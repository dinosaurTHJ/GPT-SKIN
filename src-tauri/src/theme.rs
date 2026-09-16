use chacha20poly1305::{
    XChaCha20Poly1305, XNonce,
    aead::{Aead, KeyInit, Payload},
};
use ed25519_dalek::{Signature, VerifyingKey};
use rand_core::{OsRng, RngCore};
#[cfg(test)]
use retheme_theme_protocol::{
    ALLOWED_ASSET_SLOTS, ALLOWED_SLOTS, is_valid_author_id, is_valid_theme_id, validate_css,
    validate_manifest, validate_svg,
};
use retheme_theme_protocol::{
    MAX_ARCHIVE_SIZE, ThemeError, ThemeManifest, ThemePreview, read_archive,
    read_development_directory, validate_image, validate_package_files,
};
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::security_config;

const ONLINE_CACHE_MAGIC: &[u8; 4] = b"RTC1";

#[derive(Debug, Clone)]
pub struct ThemePackage {
    manifest: ThemeManifest,
    css: String,
    runtime_assets: HashMap<String, ThemeRuntimeAsset>,
}

#[derive(Debug, Clone)]
pub(crate) struct ThemeRuntimeAsset {
    pub path: String,
    pub mime: String,
    pub source: Arc<[u8]>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeSummary {
    id: String,
    name: String,
    description: String,
    version: String,
    author: String,
    preview: ThemePreview,
    built_in: bool,
    online_slug: Option<String>,
    requires_account: bool,
    locales: BTreeMap<String, ThemeSummaryLocalization>,
}

#[derive(Debug, Clone, Serialize)]
struct ThemeSummaryLocalization {
    name: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeInstallReport {
    theme: ThemeSummary,
    replaced: bool,
    package_digest: String,
    signature_verified: bool,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct ThemeRegistry {
    themes: BTreeMap<String, InstalledTheme>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct InstalledTheme {
    version: String,
    digest: String,
    online: OnlineThemeSource,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct OnlineThemeSource {
    slug: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct IntegrityIndex {
    algorithm: String,
    files: BTreeMap<String, String>,
}

#[derive(Clone)]
pub struct ThemeRepository {
    root: PathBuf,
    verifying_key: VerifyingKey,
    cache_key: Option<[u8; 32]>,
}

impl ThemePackage {
    pub fn id(&self) -> &str {
        &self.manifest.id
    }

    pub fn css(&self) -> &str {
        &self.css
    }

    pub(crate) fn version(&self) -> &str {
        &self.manifest.version
    }

    pub(crate) fn runtime_assets(&self) -> Vec<ThemeRuntimeAsset> {
        self.runtime_assets.values().cloned().collect()
    }

    pub(crate) fn runtime_config_with_asset_urls(
        &self,
        asset_urls: &HashMap<String, String>,
    ) -> Result<Value, ThemeError> {
        build_runtime_config_with(&self.manifest, |path| {
            asset_urls
                .get(path)
                .cloned()
                .ok_or_else(|| ThemeError(format!("主题资源 URL 缺失：{path}")))
        })
    }

    pub fn preview_summary(&self) -> ThemeSummary {
        self.summary(false, None)
    }

    fn summary(&self, built_in: bool, online: Option<&OnlineThemeSource>) -> ThemeSummary {
        ThemeSummary {
            id: self.manifest.id.clone(),
            name: self.manifest.name.clone(),
            description: self.manifest.description.clone(),
            version: self.manifest.version.clone(),
            author: self.manifest.author.name.clone(),
            preview: self.manifest.preview.clone(),
            built_in,
            online_slug: online.map(|source| source.slug.clone()),
            requires_account: false,
            locales: self
                .manifest
                .locales
                .iter()
                .map(|(locale, translation)| {
                    (
                        locale.clone(),
                        ThemeSummaryLocalization {
                            name: translation.name.clone(),
                            description: translation.description.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl ThemeRepository {
    pub fn new_local(root: PathBuf) -> Result<Self, ThemeError> {
        Ok(Self {
            root,
            verifying_key: platform_verifying_key()?,
            cache_key: None,
        })
    }

    #[cfg(test)]
    pub fn new(root: PathBuf) -> Result<Self, ThemeError> {
        Ok(Self {
            root,
            verifying_key: platform_verifying_key()?,
            cache_key: None,
        })
    }

    pub fn new_with_cache_key(root: PathBuf, cache_key: [u8; 32]) -> Result<Self, ThemeError> {
        Ok(Self {
            root,
            verifying_key: platform_verifying_key()?,
            cache_key: Some(cache_key),
        })
    }

    pub fn list(&self) -> Result<Vec<ThemeSummary>, ThemeError> {
        let mut registry = self.read_registry()?;
        let mut themes = Vec::new();
        let mut invalid_theme_ids = Vec::new();
        for (theme_id, installed) in &registry.themes {
            match self.load_installed(theme_id, installed) {
                Ok(package) => themes.push(package.summary(false, Some(&installed.online))),
                Err(_) => invalid_theme_ids.push(theme_id.clone()),
            }
        }
        if !invalid_theme_ids.is_empty() {
            let removed = invalid_theme_ids
                .into_iter()
                .filter_map(|theme_id| registry.themes.remove(&theme_id))
                .collect::<Vec<_>>();
            let registry_bytes = serde_json::to_vec_pretty(&registry)
                .map_err(|error| ThemeError(format!("无法保存主题注册表：{error}")))?;
            write_atomic(&self.registry_path(), &registry_bytes)?;
            for installed in removed {
                if !registry
                    .themes
                    .values()
                    .any(|item| item.digest == installed.digest)
                {
                    let _ = fs::remove_file(self.store_path(&installed.digest));
                }
            }
        }
        Ok(themes)
    }

    pub fn load(&self, theme_id: &str) -> Result<ThemePackage, ThemeError> {
        let registry = self.read_registry()?;
        let installed = registry
            .themes
            .get(theme_id)
            .ok_or_else(|| ThemeError(format!("未知主题：{theme_id}")))?;
        self.load_installed(theme_id, installed)
    }

    pub fn online_slug(&self, theme_id: &str) -> Result<String, ThemeError> {
        let registry = self.read_registry()?;
        let installed = registry
            .themes
            .get(theme_id)
            .ok_or_else(|| ThemeError(format!("未知主题：{theme_id}")))?;
        Ok(installed.online.slug.clone())
    }

    pub fn load_development(&self, theme_path: &Path) -> Result<ThemePackage, ThemeError> {
        let files = read_development_directory(theme_path)?;
        parse_package_files(&files, false)
    }

    pub fn install_online(
        &self,
        archive_bytes: &[u8],
        slug: String,
    ) -> Result<ThemeInstallReport, ThemeError> {
        self.install_bytes(archive_bytes, OnlineThemeSource { slug })
    }

    pub fn uninstall(&self, theme_id: &str) -> Result<bool, ThemeError> {
        let mut registry = self.read_registry()?;
        let installed = registry
            .themes
            .remove(theme_id)
            .ok_or_else(|| ThemeError(format!("主题未安装：{theme_id}")))?;
        let cache_is_shared = registry
            .themes
            .values()
            .any(|item| item.digest == installed.digest);
        let registry_bytes = serde_json::to_vec_pretty(&registry)
            .map_err(|error| ThemeError(format!("无法保存主题注册表：{error}")))?;
        fs::create_dir_all(&self.root)?;
        write_atomic(&self.registry_path(), &registry_bytes)?;
        if !cache_is_shared {
            let cache_path = self.store_path(&installed.digest);
            if cache_path.exists() {
                fs::remove_file(&cache_path).map_err(|error| {
                    ThemeError(format!(
                        "主题已卸载，但无法删除缓存 {}：{error}",
                        cache_path.display()
                    ))
                })?;
            }
        }
        Ok(true)
    }

    fn install_bytes(
        &self,
        archive_bytes: &[u8],
        online: OnlineThemeSource,
    ) -> Result<ThemeInstallReport, ThemeError> {
        if archive_bytes.len() as u64 > MAX_ARCHIVE_SIZE {
            return Err(ThemeError("主题包超过 30 MB".into()));
        }
        let files = read_archive(archive_bytes, true)?;
        verify_integrity_and_signature(&files, &self.verifying_key)?;
        let package = parse_package_files(&files, true)?;
        validate_package_access(&package.manifest, &online)?;

        let digest = sha256_hex(archive_bytes);
        let mut registry = self.read_registry()?;
        let replaced = registry.themes.contains_key(package.id());
        if let Some(current) = registry.themes.get(package.id()) {
            let current_version = Version::parse(&current.version)
                .map_err(|error| ThemeError(format!("已安装主题版本损坏：{error}")))?;
            let next_version = Version::parse(&package.manifest.version)
                .map_err(|error| ThemeError(format!("主题版本无效：{error}")))?;
            if next_version < current_version {
                return Err(ThemeError("不能用较旧版本覆盖已安装主题".into()));
            }
            if next_version == current_version && digest != current.digest {
                return Err(ThemeError("同一主题版本发布后不可覆盖内容".into()));
            }
        }

        fs::create_dir_all(self.store_dir())?;
        let store_path = self.store_path(&digest);
        let stored = self.encrypt_online_cache(&digest, archive_bytes)?;
        write_atomic(&store_path, &stored)?;
        registry.themes.insert(
            package.id().to_owned(),
            InstalledTheme {
                version: package.manifest.version.clone(),
                digest: digest.clone(),
                online,
            },
        );
        fs::create_dir_all(&self.root)?;
        let registry_bytes = serde_json::to_vec_pretty(&registry)
            .map_err(|error| ThemeError(format!("无法保存主题注册表：{error}")))?;
        write_atomic(&self.registry_path(), &registry_bytes)?;

        Ok(ThemeInstallReport {
            theme: package.summary(
                false,
                registry.themes.get(package.id()).map(|item| &item.online),
            ),
            replaced,
            package_digest: digest,
            signature_verified: true,
        })
    }

    fn load_installed(
        &self,
        theme_id: &str,
        installed: &InstalledTheme,
    ) -> Result<ThemePackage, ThemeError> {
        if !is_sha256(&installed.digest) {
            return Err(ThemeError(format!("主题 {theme_id} 的注册表摘要无效")));
        }
        let stored = fs::read(self.store_path(&installed.digest))
            .map_err(|error| ThemeError(format!("无法读取已安装主题 {theme_id}：{error}")))?;
        let archive_bytes = self.decrypt_online_cache(&installed.digest, &stored)?;
        if sha256_hex(&archive_bytes) != installed.digest {
            return Err(ThemeError(format!("已安装主题 {theme_id} 的缓存已被篡改")));
        }
        let files = read_archive(&archive_bytes, true)?;
        verify_integrity_and_signature(&files, &self.verifying_key)?;
        let package = parse_package_files(&files, true)?;
        if package.id() != theme_id || package.manifest.version != installed.version {
            return Err(ThemeError(format!("主题 {theme_id} 与注册表不一致")));
        }
        validate_package_access(&package.manifest, &installed.online)?;
        Ok(package)
    }

    fn read_registry(&self) -> Result<ThemeRegistry, ThemeError> {
        let path = self.registry_path();
        if !path.exists() {
            return Ok(ThemeRegistry::default());
        }
        let source = fs::read(&path)?;
        serde_json::from_slice(&source)
            .map_err(|error| ThemeError(format!("主题注册表损坏：{error}")))
    }

    fn registry_path(&self) -> PathBuf {
        self.root.join("registry.json")
    }

    fn store_dir(&self) -> PathBuf {
        self.root.join("store")
    }

    fn store_path(&self, digest: &str) -> PathBuf {
        self.store_dir().join(format!("{digest}.cache"))
    }

    fn encrypt_online_cache(&self, digest: &str, source: &[u8]) -> Result<Vec<u8>, ThemeError> {
        let key = self
            .cache_key
            .ok_or_else(|| ThemeError("当前设备无法保护在线主题缓存".into()))?;
        let cipher = XChaCha20Poly1305::new_from_slice(&key)
            .map_err(|_| ThemeError("在线主题缓存密钥无效".into()))?;
        let mut nonce = [0_u8; 24];
        OsRng.fill_bytes(&mut nonce);
        let ciphertext = cipher
            .encrypt(
                XNonce::from_slice(&nonce),
                Payload {
                    msg: source,
                    aad: digest.as_bytes(),
                },
            )
            .map_err(|_| ThemeError("无法加密在线主题缓存".into()))?;
        let mut stored =
            Vec::with_capacity(ONLINE_CACHE_MAGIC.len() + nonce.len() + ciphertext.len());
        stored.extend_from_slice(ONLINE_CACHE_MAGIC);
        stored.extend_from_slice(&nonce);
        stored.extend_from_slice(&ciphertext);
        Ok(stored)
    }

    fn decrypt_online_cache(&self, digest: &str, stored: &[u8]) -> Result<Vec<u8>, ThemeError> {
        if stored.len() <= ONLINE_CACHE_MAGIC.len() + 24
            || &stored[..ONLINE_CACHE_MAGIC.len()] != ONLINE_CACHE_MAGIC
        {
            return Err(ThemeError("在线主题缓存格式已过期，请重新下载".into()));
        }
        let key = self
            .cache_key
            .ok_or_else(|| ThemeError("当前设备无法读取在线主题缓存".into()))?;
        let cipher = XChaCha20Poly1305::new_from_slice(&key)
            .map_err(|_| ThemeError("在线主题缓存密钥无效".into()))?;
        let nonce_end = ONLINE_CACHE_MAGIC.len() + 24;
        cipher
            .decrypt(
                XNonce::from_slice(&stored[ONLINE_CACHE_MAGIC.len()..nonce_end]),
                Payload {
                    msg: &stored[nonce_end..],
                    aad: digest.as_bytes(),
                },
            )
            .map_err(|_| ThemeError("在线主题缓存与当前设备不匹配或已损坏".into()))
    }
}

fn validate_package_access(
    manifest: &ThemeManifest,
    online: &OnlineThemeSource,
) -> Result<(), ThemeError> {
    match manifest.access.as_ref() {
        Some(access) if access.delivery == "online" && access.slug == online.slug => Ok(()),
        None => Err(ThemeError(
            "在线主题包缺少签名授权策略，请重新下载最新版本".into(),
        )),
        Some(_) => Err(ThemeError("在线主题包授权策略与服务端不一致".into())),
    }
}

#[cfg(test)]
const TEST_THEME_MANIFEST: &str = r##"{
  "schemaVersion": 1,
  "id": "studio.example.test-theme",
  "name": "Test Theme",
  "description": "A protocol test theme.",
  "version": "1.0.0",
  "author": { "id": "studio.example", "name": "Example Studio" },
  "testedCodexVersions": ["26.707.91948"],
  "styles": ["styles/theme.css"],
  "slots": [
    "app.shell",
    "home.hero",
    "conversation.banner",
    "composer.decoration",
    "conversation.summary.decoration"
  ],
  "permissions": [],
  "preview": { "background": "#111111", "surface": "#222222", "accent": "#aaff44" },
  "experience": {
    "homeHero": {
      "eyebrow": "TEST",
      "title": "Protocol theme",
      "description": "Theme engine verification",
      "asset": "assets/hero.svg",
      "fit": "cover",
      "position": "center"
    },
    "homePrompt": { "title": "What should we test?" },
    "conversationBanner": {
      "eyebrow": "SESSION",
      "title": "Test conversation",
      "description": "Narrow banner verification",
      "asset": "assets/banner-chat.svg",
      "fit": "contain",
      "position": "right"
    },
    "composerDecoration": { "asset": "assets/ornament.svg" },
    "conversationSummaryDecoration": { "asset": "assets/ornament.svg" },
    "decorations": []
  },
  "locales": {
    "zh-CN": {
      "name": "测试主题",
      "description": "用于验证主题协议。",
      "experience": {
        "homePrompt": { "title": "今天测试什么？" }
      }
    }
  }
}"##;

#[cfg(test)]
const TEST_THEME_CSS: &str = r#":root[data-ct-theme="studio.example.test-theme"] [data-ct-slot="app.shell"] { color: white; }"#;

#[cfg(test)]
const TEST_THEME_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><path d="M0 0h10v10H0z"/></svg>"#;

#[cfg(test)]
fn test_theme_files() -> HashMap<String, Vec<u8>> {
    HashMap::from([
        (
            "manifest.json".to_owned(),
            TEST_THEME_MANIFEST.as_bytes().to_vec(),
        ),
        (
            "styles/theme.css".to_owned(),
            TEST_THEME_CSS.as_bytes().to_vec(),
        ),
        (
            "assets/hero.svg".to_owned(),
            TEST_THEME_SVG.as_bytes().to_vec(),
        ),
        (
            "assets/banner-chat.svg".to_owned(),
            TEST_THEME_SVG.as_bytes().to_vec(),
        ),
        (
            "assets/ornament.svg".to_owned(),
            TEST_THEME_SVG.as_bytes().to_vec(),
        ),
    ])
}

#[cfg(test)]
pub(crate) fn test_theme_summary() -> ThemeSummary {
    test_theme_package()
        .expect("test theme should be valid")
        .preview_summary()
}

#[cfg(test)]
fn test_theme_package() -> Result<ThemePackage, ThemeError> {
    parse_package_files(&test_theme_files(), false)
}

fn parse_package_files(
    files: &HashMap<String, Vec<u8>>,
    signed_package: bool,
) -> Result<ThemePackage, ThemeError> {
    let validated = validate_package_files(files, signed_package)?;
    let manifest = validated.manifest;
    let runtime_config = build_runtime_config(&manifest, files)?;
    let runtime_assets = collect_runtime_assets(files, &runtime_config)?;

    Ok(ThemePackage {
        manifest,
        css: validated.css,
        runtime_assets,
    })
}

fn build_runtime_config(
    manifest: &ThemeManifest,
    files: &HashMap<String, Vec<u8>>,
) -> Result<Value, ThemeError> {
    build_runtime_config_with(manifest, |path| {
        let source = files
            .get(path)
            .ok_or_else(|| ThemeError(format!("主题缺少资源文件：{path}")))?;
        validate_image(path, source)?;
        Ok(path.to_owned())
    })
}

fn build_runtime_config_with(
    manifest: &ThemeManifest,
    load_asset_url: impl Fn(&str) -> Result<String, ThemeError>,
) -> Result<Value, ThemeError> {
    let hero = &manifest.experience.home_hero;
    let hero_asset = load_asset_url(&hero.asset)?;
    let hero_foreground_asset = hero
        .foreground
        .as_deref()
        .map(&load_asset_url)
        .transpose()?;
    let home_prompt = manifest
        .experience
        .home_prompt
        .as_ref()
        .map(|prompt| json!({ "title": prompt.title }));
    let conversation_banner = manifest
        .experience
        .conversation_banner
        .as_ref()
        .map(|banner| -> Result<Value, ThemeError> {
            Ok(json!({
                "eyebrow": banner.eyebrow,
                "title": banner.title,
                "description": banner.description,
                "assetUrl": load_asset_url(&banner.asset)?,
                "foregroundAssetUrl": banner
                    .foreground
                    .as_deref()
                    .map(&load_asset_url)
                    .transpose()?,
                "fit": banner.fit,
                "position": banner.position,
            }))
        })
        .transpose()?;
    let hero_divider = hero
        .divider
        .as_ref()
        .map(|divider| -> Result<Value, ThemeError> {
            Ok(json!({
                "label": divider.label,
                "assetUrl": divider
                    .asset
                    .as_deref()
                    .map(&load_asset_url)
                    .transpose()?,
            }))
        })
        .transpose()?;
    let decorations = manifest
        .experience
        .decorations
        .iter()
        .map(|decoration| {
            Ok(json!({
                "slot": decoration.slot,
                "assetUrl": load_asset_url(&decoration.asset)?,
            }))
        })
        .collect::<Result<Vec<_>, ThemeError>>()?;
    let composer_submit = manifest
        .experience
        .composer_submit
        .as_ref()
        .map(|visual| load_asset_url(&visual.asset))
        .transpose()?;
    let composer_decoration = manifest
        .experience
        .composer_decoration
        .as_ref()
        .map(|visual| load_asset_url(&visual.asset))
        .transpose()?;
    let conversation_summary_decoration = manifest
        .experience
        .conversation_summary_decoration
        .as_ref()
        .map(|visual| load_asset_url(&visual.asset))
        .transpose()?;
    let sidebar_section_decoration = manifest
        .experience
        .sidebar_section_decoration
        .as_ref()
        .map(|visual| load_asset_url(&visual.asset))
        .transpose()?;
    let assets = manifest
        .experience
        .assets
        .iter()
        .map(|asset| {
            Ok(json!({
                "slot": asset.slot,
                "assetUrl": load_asset_url(&asset.asset)?,
                "lightAssetUrl": asset
                    .light_asset
                    .as_deref()
                    .map(&load_asset_url)
                    .transpose()?,
                "darkAssetUrl": asset
                    .dark_asset
                    .as_deref()
                    .map(&load_asset_url)
                    .transpose()?,
            }))
        })
        .collect::<Result<Vec<_>, ThemeError>>()?;

    Ok(json!({
        "hero": {
            "eyebrow": hero.eyebrow,
            "title": hero.title,
            "description": hero.description,
            "assetUrl": hero_asset,
            "foregroundAssetUrl": hero_foreground_asset,
            "fit": hero.fit,
            "position": hero.position,
            "divider": hero_divider,
        },
        "homePrompt": home_prompt,
        "conversationBanner": conversation_banner,
        "composerSubmit": composer_submit.map(|asset_url| json!({ "assetUrl": asset_url })),
        "composerDecoration": composer_decoration.map(|asset_url| json!({ "assetUrl": asset_url })),
        "conversationSummaryDecoration": conversation_summary_decoration
            .map(|asset_url| json!({ "assetUrl": asset_url })),
        "sidebarSectionDecoration": sidebar_section_decoration
            .map(|asset_url| json!({ "assetUrl": asset_url })),
        "assets": assets,
        "decorations": decorations,
        "locales": manifest.locales,
    }))
}

fn collect_runtime_assets(
    files: &HashMap<String, Vec<u8>>,
    runtime_config: &Value,
) -> Result<HashMap<String, ThemeRuntimeAsset>, ThemeError> {
    let mut paths = HashSet::new();
    collect_runtime_asset_paths(runtime_config, &mut paths);
    paths
        .into_iter()
        .map(|path| {
            let source = files
                .get(path)
                .ok_or_else(|| ThemeError(format!("主题缺少资源文件：{path}")))?;
            let mime = validate_image(path, source)?.to_owned();
            Ok((
                path.to_owned(),
                ThemeRuntimeAsset {
                    path: path.to_owned(),
                    mime,
                    source: Arc::from(source.clone()),
                },
            ))
        })
        .collect()
}

fn collect_runtime_asset_paths<'a>(value: &'a Value, paths: &mut HashSet<&'a str>) {
    match value {
        Value::Object(object) => {
            for (key, value) in object {
                if matches!(
                    key.as_str(),
                    "assetUrl" | "foregroundAssetUrl" | "lightAssetUrl" | "darkAssetUrl"
                ) {
                    if let Some(path) = value.as_str() {
                        paths.insert(path);
                    }
                } else {
                    collect_runtime_asset_paths(value, paths);
                }
            }
        }
        Value::Array(values) => {
            for value in values {
                collect_runtime_asset_paths(value, paths);
            }
        }
        _ => {}
    }
}

fn verify_integrity_and_signature(
    files: &HashMap<String, Vec<u8>>,
    verifying_key: &VerifyingKey,
) -> Result<(), ThemeError> {
    let integrity_source = files
        .get("integrity.json")
        .ok_or_else(|| ThemeError("主题包缺少 integrity.json".into()))?;
    let integrity: IntegrityIndex = serde_json::from_slice(integrity_source)
        .map_err(|error| ThemeError(format!("integrity.json 无效：{error}")))?;
    if integrity.algorithm != "sha256" {
        return Err(ThemeError("完整性算法必须为 sha256".into()));
    }
    let expected_files: HashSet<&str> = files
        .keys()
        .filter(|path| *path != "integrity.json" && *path != "signature.ed25519")
        .map(String::as_str)
        .collect();
    let indexed_files: HashSet<&str> = integrity.files.keys().map(String::as_str).collect();
    if expected_files != indexed_files {
        return Err(ThemeError(
            "integrity.json 必须完整且仅覆盖主题内容文件".into(),
        ));
    }
    for (path, expected_digest) in &integrity.files {
        if !is_sha256(expected_digest) {
            return Err(ThemeError(format!("完整性摘要格式无效：{path}")));
        }
        if sha256_hex(&files[path]) != *expected_digest {
            return Err(ThemeError(format!("主题文件完整性校验失败：{path}")));
        }
    }
    let signature_source = files
        .get("signature.ed25519")
        .ok_or_else(|| ThemeError("主题包缺少 signature.ed25519".into()))?;
    let signature = Signature::from_slice(signature_source)
        .map_err(|_| ThemeError("Ed25519 签名必须是 64 字节".into()))?;
    verifying_key
        .verify_strict(integrity_source, &signature)
        .map_err(|_| ThemeError("主题平台签名验证失败".into()))
}

fn write_atomic(path: &Path, source: &[u8]) -> Result<(), ThemeError> {
    let parent = path
        .parent()
        .ok_or_else(|| ThemeError("主题存储路径无效".into()))?;
    fs::create_dir_all(parent)?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    temporary.write_all(source)?;
    temporary.as_file().sync_all()?;
    temporary
        .persist(path)
        .map_err(|error| ThemeError(format!("无法原子写入主题存储：{}", error.error)))?;
    Ok(())
}

fn platform_verifying_key() -> Result<VerifyingKey, ThemeError> {
    let mut bytes = [0_u8; 32];
    hex::decode_to_slice(security_config::theme_public_key(), &mut bytes)
        .map_err(|error| ThemeError(format!("内置平台公钥无效：{error}")))?;
    VerifyingKey::from_bytes(&bytes).map_err(|_| ThemeError("内置平台公钥无效".into()))
}

fn sha256_hex(source: &[u8]) -> String {
    hex::encode(Sha256::digest(source))
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use std::io::Cursor;
    use zip::ZipWriter;
    use zip::write::SimpleFileOptions;

    const TEST_SECRET: [u8; 32] = [7; 32];

    #[test]
    fn codex_tested_versions_are_optional_and_accept_legacy_field() {
        let mut manifest: Value = serde_json::from_str(TEST_THEME_MANIFEST).expect("test manifest");
        manifest
            .as_object_mut()
            .expect("manifest object")
            .remove("testedCodexVersions");
        let without_versions: ThemeManifest =
            serde_json::from_value(manifest.clone()).expect("versions are optional");
        assert!(without_versions.tested_codex_versions.is_empty());

        manifest["supportedCodexVersions"] = json!(["26.707.91948"]);
        let legacy: ThemeManifest =
            serde_json::from_value(manifest).expect("legacy version field remains readable");
        assert_eq!(legacy.tested_codex_versions, ["26.707.91948"]);
    }

    #[test]
    fn rejects_unknown_manifest_fields_in_nested_protocol_objects() {
        let mut manifest: Value = serde_json::from_str(TEST_THEME_MANIFEST).expect("test manifest");
        manifest["experience"]["conversationSummaryDecoraton"] = json!({
            "asset": "assets/ornament.svg"
        });
        let error = serde_json::from_value::<ThemeManifest>(manifest)
            .expect_err("misspelled protocol fields must fail")
            .to_string();
        assert!(error.contains("unknown field `conversationSummaryDecoraton`"));
    }

    #[test]
    fn decorations_default_to_empty_when_omitted() {
        let mut manifest: Value = serde_json::from_str(TEST_THEME_MANIFEST).expect("test manifest");
        manifest["experience"]
            .as_object_mut()
            .expect("experience object")
            .remove("decorations");
        let manifest: ThemeManifest =
            serde_json::from_value(manifest).expect("decorations should be optional");
        assert!(manifest.experience.decorations.is_empty());
        validate_manifest(&manifest).expect("manifest without decorations should remain valid");
    }

    #[test]
    fn rejects_duplicate_styles_and_slots() {
        let mut manifest: Value = serde_json::from_str(TEST_THEME_MANIFEST).expect("test manifest");
        manifest["styles"] = json!(["styles/theme.css", "styles/theme.css"]);
        let duplicate_styles: ThemeManifest =
            serde_json::from_value(manifest.clone()).expect("manifest");
        assert!(
            validate_manifest(&duplicate_styles)
                .expect_err("duplicate styles must fail")
                .to_string()
                .contains("样式文件不能重复")
        );

        manifest["styles"] = json!(["styles/theme.css"]);
        manifest["slots"] = json!(["app.shell", "app.shell"]);
        let duplicate_slots: ThemeManifest = serde_json::from_value(manifest).expect("manifest");
        assert!(
            validate_manifest(&duplicate_slots)
                .expect_err("duplicate slots must fail")
                .to_string()
                .contains("逻辑插槽不能重复")
        );
    }

    #[test]
    fn rejects_overlong_base_manifest_text() {
        for (field, length, expected) in [
            ("name", 121, "Theme name"),
            ("description", 241, "Theme description"),
        ] {
            let mut manifest: Value =
                serde_json::from_str(TEST_THEME_MANIFEST).expect("test manifest");
            manifest[field] = json!("x".repeat(length));
            let manifest: ThemeManifest = serde_json::from_value(manifest).expect("manifest");
            assert!(
                validate_manifest(&manifest)
                    .expect_err("overlong text must fail")
                    .to_string()
                    .contains(expected)
            );
        }

        let mut manifest: Value = serde_json::from_str(TEST_THEME_MANIFEST).expect("test manifest");
        manifest["author"]["name"] = json!("x".repeat(101));
        let manifest: ThemeManifest = serde_json::from_value(manifest).expect("manifest");
        assert!(
            validate_manifest(&manifest)
                .expect_err("overlong author name must fail")
                .to_string()
                .contains("Author name")
        );
    }

    #[test]
    fn selector_lists_preserve_nested_function_commas() {
        let source = r#":root[data-ct-theme="studio.example.test-theme"] :is([data-ct-slot="home.card"], [data-ct-slot="settings.card"]) { color: white; }"#;
        validate_css(source, "studio.example.test-theme")
            .expect("nested selector commas must not split the root scope");
    }

    #[test]
    fn home_prompt_and_conversation_banner_are_optional() {
        let mut manifest: Value = serde_json::from_str(TEST_THEME_MANIFEST).expect("test manifest");
        manifest["experience"]
            .as_object_mut()
            .expect("experience object")
            .retain(|key, _| {
                !matches!(key.as_str(), "homePrompt" | "conversationBanner" | "assets")
            });
        assert!(manifest["experience"].get("homePrompt").is_none());
        assert!(manifest["experience"].get("conversationBanner").is_none());
        let manifest: ThemeManifest =
            serde_json::from_value(manifest).expect("optional experience fields");
        validate_manifest(&manifest).expect("legacy theme remains valid");
        let files = test_theme_files();
        let config = build_runtime_config(&manifest, &files).expect("runtime config");
        assert!(config["homePrompt"].is_null());
        assert!(config["conversationBanner"].is_null());
    }

    #[test]
    fn builds_home_prompt_and_conversation_banner_runtime_config() {
        let mut manifest: Value = serde_json::from_str(TEST_THEME_MANIFEST).expect("test manifest");
        manifest["experience"]
            .as_object_mut()
            .expect("experience object")
            .remove("assets");
        manifest["experience"]["homePrompt"] = json!({
            "title": "今天想一起做什么？"
        });
        manifest["experience"]["conversationBanner"] = json!({
            "eyebrow": "SESSION",
            "title": "保持专注",
            "description": "会话页使用独立窄图。",
            "asset": "assets/banner-chat.svg",
            "fit": "contain",
            "position": "right"
        });
        let manifest: ThemeManifest = serde_json::from_value(manifest).expect("theme manifest");
        validate_manifest(&manifest).expect("experience fields should be valid");
        let files = test_theme_files();
        let config = build_runtime_config(&manifest, &files).expect("runtime config");
        assert_eq!(config["homePrompt"]["title"], "今天想一起做什么？");
        assert_eq!(config["conversationBanner"]["fit"], "contain");
        assert_eq!(config["conversationBanner"]["position"], "right");
        assert!(
            config["conversationBanner"]["assetUrl"]
                .as_str()
                .is_some_and(|value| value == "assets/banner-chat.svg")
        );
    }

    #[test]
    fn builds_controlled_asset_slot_runtime_config() {
        let mut manifest: Value = serde_json::from_str(TEST_THEME_MANIFEST).expect("test manifest");
        manifest["experience"]["assets"] = json!([
            {
                "slot": "app.background",
                "asset": "assets/hero.svg",
                "lightAsset": "assets/hero.svg",
                "darkAsset": "assets/ornament.svg"
            },
            { "slot": "main.background", "asset": "assets/hero.svg" },
            { "slot": "main.overlay", "asset": "assets/ornament.svg" },
            { "slot": "main.frame", "asset": "assets/ornament.svg" },
            { "slot": "sidebar.brand.icon", "asset": "assets/ornament.svg" },
            { "slot": "sidebar.brand.badge", "asset": "assets/ornament.svg" },
            { "slot": "sidebar.header.decoration", "asset": "assets/ornament.svg" },
            { "slot": "sidebar.frame", "asset": "assets/ornament.svg" },
            { "slot": "home.card.arrow.asset", "asset": "assets/ornament.svg" }
        ]);
        manifest["slots"] = json!(
            manifest["slots"]
                .as_array()
                .expect("slots")
                .iter()
                .cloned()
                .chain(ALLOWED_ASSET_SLOTS.map(Value::from))
                .collect::<Vec<_>>()
        );
        let manifest: ThemeManifest = serde_json::from_value(manifest).expect("theme manifest");
        validate_manifest(&manifest).expect("controlled asset slots should be valid");
        let files = test_theme_files();
        let config = build_runtime_config(&manifest, &files).expect("runtime config");
        assert_eq!(config["assets"].as_array().map(Vec::len), Some(9));
        assert_eq!(config["assets"][0]["slot"], "app.background");
        assert_eq!(config["assets"][0]["lightAssetUrl"], "assets/hero.svg");
        assert_eq!(config["assets"][0]["darkAssetUrl"], "assets/ornament.svg");
        assert_eq!(config["assets"][6]["slot"], "sidebar.header.decoration");
        assert_eq!(config["assets"][7]["slot"], "sidebar.frame");
        assert!(
            config["assets"][8]["assetUrl"]
                .as_str()
                .is_some_and(|value| value == "assets/ornament.svg")
        );
    }

    #[test]
    fn replaces_every_runtime_asset_path_with_session_urls() {
        let theme = test_theme_package().expect("test theme");
        let assets = theme.runtime_assets();
        assert!(!assets.is_empty());
        let urls = assets
            .iter()
            .map(|asset| {
                (
                    asset.path.clone(),
                    format!("http://127.0.0.1:49152/token/{}", asset.path),
                )
            })
            .collect::<HashMap<_, _>>();
        let config = theme
            .runtime_config_with_asset_urls(&urls)
            .expect("session runtime config");
        assert_runtime_urls_are_local(&config);
        assert_eq!(
            config["conversationBanner"]["assetUrl"],
            urls["assets/banner-chat.svg"]
        );
    }

    fn assert_runtime_urls_are_local(value: &Value) {
        match value {
            Value::Object(object) => {
                for (key, value) in object {
                    if matches!(
                        key.as_str(),
                        "assetUrl" | "foregroundAssetUrl" | "lightAssetUrl" | "darkAssetUrl"
                    ) {
                        if let Some(url) = value.as_str() {
                            assert!(url.starts_with("http://127.0.0.1:"), "invalid URL: {url}");
                        }
                    } else {
                        assert_runtime_urls_are_local(value);
                    }
                }
            }
            Value::Array(values) => {
                for value in values {
                    assert_runtime_urls_are_local(value);
                }
            }
            _ => {}
        }
    }

    #[test]
    fn rejects_unknown_or_duplicate_asset_slots() {
        let mut manifest: Value = serde_json::from_str(TEST_THEME_MANIFEST).expect("test manifest");
        manifest["experience"]["assets"] = json!([
            { "slot": "main.unknown", "asset": "assets/ornament.svg" }
        ]);
        let unknown: ThemeManifest = serde_json::from_value(manifest.clone()).expect("manifest");
        assert!(
            validate_manifest(&unknown)
                .expect_err("unknown asset slot must fail")
                .to_string()
                .contains("未开放")
        );

        manifest["experience"]["assets"] = json!([
            { "slot": "sidebar.frame", "asset": "assets/ornament.svg" }
        ]);
        let undeclared: ThemeManifest = serde_json::from_value(manifest.clone()).expect("manifest");
        assert!(
            validate_manifest(&undeclared)
                .expect_err("undeclared asset slot must fail")
                .to_string()
                .contains("slots")
        );

        manifest["slots"]
            .as_array_mut()
            .expect("slots")
            .push(json!("sidebar.frame"));
        manifest["experience"]["assets"] = json!([
            { "slot": "sidebar.frame", "asset": "assets/ornament.svg" },
            { "slot": "sidebar.frame", "asset": "assets/ornament.svg" }
        ]);
        let duplicate: ThemeManifest = serde_json::from_value(manifest).expect("manifest");
        assert!(
            validate_manifest(&duplicate)
                .expect_err("duplicate asset slot must fail")
                .to_string()
                .contains("重复")
        );
    }

    #[test]
    fn rejects_invalid_home_prompt_and_conversation_banner() {
        let mut manifest: Value = serde_json::from_str(TEST_THEME_MANIFEST).expect("test manifest");
        manifest["experience"]["homePrompt"] = json!({ "title": "" });
        let manifest_with_empty_prompt: ThemeManifest =
            serde_json::from_value(manifest.clone()).expect("theme manifest");
        assert!(
            validate_manifest(&manifest_with_empty_prompt)
                .expect_err("empty prompt must fail")
                .to_string()
                .contains("Home prompt title")
        );

        manifest["experience"]["homePrompt"] = json!({ "title": "Valid" });
        manifest["experience"]["conversationBanner"] = json!({
            "eyebrow": "SESSION",
            "title": "Conversation",
            "description": "Independent narrow banner",
            "asset": "assets/banner-chat.svg",
            "fit": "stretch",
            "position": "center"
        });
        let manifest_with_invalid_fit: ThemeManifest =
            serde_json::from_value(manifest.clone()).expect("theme manifest");
        assert!(
            validate_manifest(&manifest_with_invalid_fit)
                .expect_err("invalid fit must fail")
                .to_string()
                .contains("fit")
        );

        manifest["experience"]["conversationBanner"]["fit"] = json!("cover");
        manifest["experience"]["conversationBanner"]["position"] = json!("center 20%");
        let manifest_with_invalid_position: ThemeManifest =
            serde_json::from_value(manifest).expect("theme manifest");
        assert!(
            validate_manifest(&manifest_with_invalid_position)
                .expect_err("invalid position must fail")
                .to_string()
                .contains("position")
        );
    }

    #[test]
    fn installs_and_loads_signed_online_theme() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let package = signed_online_test_package();
        let signing_key = SigningKey::from_bytes(&TEST_SECRET);
        let repository = ThemeRepository {
            root: directory.path().join("repository"),
            verifying_key: signing_key.verifying_key(),
            cache_key: Some([9; 32]),
        };
        let installed_before = repository.list().expect("list themes before install").len();

        let report = repository
            .install_online(&package, "test-theme".into())
            .expect("install signed theme");

        assert!(report.signature_verified);
        assert_eq!(report.theme.id, "studio.example.test-theme");
        assert!(!report.theme.built_in);
        assert_eq!(
            repository.list().expect("list themes").len(),
            installed_before + 1
        );
        let installed = repository.read_registry().expect("registry");
        let digest = installed
            .themes
            .get("studio.example.test-theme")
            .expect("installed theme")
            .digest
            .clone();
        let cache_path = repository.store_path(&digest);
        assert!(cache_path.is_file());

        assert!(
            repository
                .uninstall("studio.example.test-theme")
                .expect("uninstall signed theme")
        );
        assert_eq!(
            repository.list().expect("list themes").len(),
            installed_before
        );
        assert!(!cache_path.exists());
    }

    #[test]
    fn online_theme_uses_device_encrypted_cache() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let package = signed_online_test_package();
        let signing_key = SigningKey::from_bytes(&TEST_SECRET);
        let repository = ThemeRepository {
            root: directory.path().join("repository"),
            verifying_key: signing_key.verifying_key(),
            cache_key: Some([9; 32]),
        };

        repository
            .install_online(&package, "test-theme".into())
            .expect("authorized online install");
        let registry = repository.read_registry().expect("registry");
        let installed = registry
            .themes
            .get("studio.example.test-theme")
            .expect("installed theme");
        let stored = fs::read(repository.store_path(&installed.digest)).expect("cache");
        assert!(stored.starts_with(ONLINE_CACHE_MAGIC));
        assert!(
            !stored
                .windows(package.len())
                .any(|window| window == package)
        );
        repository
            .load("studio.example.test-theme")
            .expect("device-bound cache loads");

        let mut registry = repository.read_registry().expect("registry");
        registry
            .themes
            .get_mut("studio.example.test-theme")
            .expect("installed theme")
            .online
            .slug = "different-theme".into();
        write_atomic(
            &repository.registry_path(),
            &serde_json::to_vec_pretty(&registry).expect("serialize registry"),
        )
        .expect("tamper registry");
        let error = repository
            .load("studio.example.test-theme")
            .expect_err("registry source tampering must fail");
        assert!(error.to_string().contains("授权策略与服务端不一致"));
    }

    #[test]
    fn removes_online_cache_encrypted_for_another_device() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let root = directory.path().join("repository");
        let signing_key = SigningKey::from_bytes(&TEST_SECRET);
        let first_device = ThemeRepository {
            root: root.clone(),
            verifying_key: signing_key.verifying_key(),
            cache_key: Some([9; 32]),
        };
        first_device
            .install_online(&signed_online_test_package(), "test-theme".into())
            .expect("first device install");

        let current_device = ThemeRepository {
            root,
            verifying_key: signing_key.verifying_key(),
            cache_key: Some([10; 32]),
        };
        assert!(
            current_device
                .list()
                .expect("stale cache cleanup")
                .is_empty()
        );
        assert!(
            current_device
                .read_registry()
                .expect("cleaned registry")
                .themes
                .is_empty()
        );
    }

    #[test]
    fn reinstall_reencrypts_online_cache_for_current_device() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let root = directory.path().join("repository");
        let package = signed_online_test_package();
        let signing_key = SigningKey::from_bytes(&TEST_SECRET);
        let first_device = ThemeRepository {
            root: root.clone(),
            verifying_key: signing_key.verifying_key(),
            cache_key: Some([9; 32]),
        };
        first_device
            .install_online(&package, "test-theme".into())
            .expect("first device install");

        let current_device = ThemeRepository {
            root,
            verifying_key: signing_key.verifying_key(),
            cache_key: Some([10; 32]),
        };
        current_device
            .install_online(&package, "test-theme".into())
            .expect("current device reinstall");
        assert_eq!(
            current_device
                .load("studio.example.test-theme")
                .expect("current device cache")
                .id(),
            "studio.example.test-theme"
        );
    }

    #[test]
    fn loads_unsigned_development_directory_without_installing_it() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let theme_path = directory.path().join("development-theme");
        fs::create_dir_all(theme_path.join("styles")).expect("styles directory");
        fs::create_dir_all(theme_path.join("assets")).expect("assets directory");
        fs::write(theme_path.join("manifest.json"), TEST_THEME_MANIFEST)
            .expect("development manifest");
        fs::write(theme_path.join("styles/theme.css"), TEST_THEME_CSS).expect("development CSS");
        for asset in ["hero.svg", "banner-chat.svg", "ornament.svg"] {
            fs::write(theme_path.join("assets").join(asset), TEST_THEME_SVG)
                .expect("development asset");
        }
        let repository =
            ThemeRepository::new(directory.path().join("repository")).expect("theme repository");
        let installed_before = repository
            .list()
            .expect("installed themes before load")
            .len();

        let package = repository
            .load_development(&theme_path)
            .expect("development theme should load");

        assert_eq!(package.id(), "studio.example.test-theme");
        assert_eq!(
            repository.list().expect("installed themes").len(),
            installed_before
        );
        assert!(!directory.path().join("repository/registry.json").exists());
    }

    #[test]
    fn documented_example_theme_matches_the_runtime_protocol() {
        let repository_root = tempfile::tempdir().expect("temporary repository");
        let repository =
            ThemeRepository::new(repository_root.path().to_path_buf()).expect("theme repository");
        let theme_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../docs/theme-example/package");

        let package = repository
            .load_development(&theme_path)
            .expect("documented example theme should remain loadable");

        assert_eq!(package.id(), "studio.example.protocol-preview");
        assert_eq!(package.version(), "1.0.0");
        assert_eq!(package.runtime_assets().len(), 1);
    }

    #[test]
    fn documented_schema_and_slot_catalog_match_the_runtime_protocol() {
        let docs = Path::new(env!("CARGO_MANIFEST_DIR")).join("../docs");
        let schema: Value = serde_json::from_str(
            &fs::read_to_string(docs.join("theme.schema.json")).expect("theme schema"),
        )
        .expect("valid theme schema JSON");
        let schema_slots = schema["$defs"]["slot"]["enum"]
            .as_array()
            .expect("schema slot enum")
            .iter()
            .map(|slot| slot.as_str().expect("string slot"))
            .collect::<Vec<_>>();
        assert_eq!(schema_slots, ALLOWED_SLOTS);

        let catalog = fs::read_to_string(docs.join("theme-slots.md")).expect("theme slot catalog");
        for slot in ALLOWED_SLOTS {
            assert!(
                catalog.contains(&format!("`{slot}`")),
                "slot catalog is missing {slot}"
            );
        }
    }

    #[test]
    fn accepts_platform_and_account_author_ids_without_weakening_theme_ids() {
        assert!(is_valid_author_id("retheme"));
        assert!(is_valid_author_id("123456789"));
        assert!(is_valid_author_id("studio.example"));
        assert!(is_valid_theme_id("studio.example.community-theme"));
        assert!(!is_valid_theme_id("community-theme"));
        assert!(!is_valid_author_id("Invalid Author"));
    }

    #[test]
    fn development_directory_keeps_package_file_restrictions() {
        let directory = tempfile::tempdir().expect("temporary directory");
        fs::write(directory.path().join("manifest.json"), TEST_THEME_MANIFEST)
            .expect("development manifest");
        fs::write(directory.path().join("unsafe.js"), "alert(1)").expect("unsafe file");
        let repository =
            ThemeRepository::new(directory.path().join("repository")).expect("theme repository");

        let error = repository
            .load_development(directory.path())
            .expect_err("unexpected files must fail");

        assert!(error.to_string().contains("不允许的文件"));
    }

    #[test]
    fn development_loader_rejects_ctheme_files() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let package_path = directory.path().join("theme.ctheme");
        fs::write(&package_path, signed_test_package(false)).expect("write package");
        let repository =
            ThemeRepository::new(directory.path().join("repository")).expect("theme repository");

        let error = repository
            .load_development(&package_path)
            .expect_err("packed themes are not development directories");

        assert!(
            error
                .to_string()
                .contains("请选择包含 manifest.json 的主题目录")
        );
    }

    #[test]
    fn rejects_tampered_package() {
        let files = read_archive(&signed_test_package(true), true).expect("read archive");
        let signing_key = SigningKey::from_bytes(&TEST_SECRET);
        let error = verify_integrity_and_signature(&files, &signing_key.verifying_key())
            .expect_err("tampering must fail");
        assert!(error.to_string().contains("完整性校验失败"));
    }

    #[test]
    fn rejects_same_version_with_different_content() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let signing_key = SigningKey::from_bytes(&TEST_SECRET);
        let repository = ThemeRepository {
            root: directory.path().join("repository"),
            verifying_key: signing_key.verifying_key(),
            cache_key: Some([9; 32]),
        };
        repository
            .install_online(&signed_online_test_package(), "test-theme".into())
            .expect("first install");

        let changed_package =
            signed_online_test_package_with_description("Changed package content");
        let error = repository
            .install_online(&changed_package, "test-theme".into())
            .expect_err("same version replacement must fail");

        assert!(error.to_string().contains("不可覆盖"));
    }

    #[test]
    fn rejects_archive_path_traversal() {
        let mut cursor = Cursor::new(Vec::new());
        let mut writer = ZipWriter::new(&mut cursor);
        writer
            .start_file("../escape.css", SimpleFileOptions::default())
            .expect("start unsafe entry");
        writer.write_all(b"x").expect("write unsafe entry");
        writer.finish().expect("finish archive");
        let error = read_archive(&cursor.into_inner(), true).expect_err("unsafe path must fail");
        assert!(error.to_string().contains("路径"));
    }

    #[test]
    fn rejects_unscoped_css() {
        let error = validate_css("body { color: red; }", "studio.example.theme")
            .expect_err("unscoped CSS should fail");
        assert!(error.to_string().contains("作用域"));
    }

    #[test]
    fn rejects_theme_font_overrides() {
        for css in [
            r#":root[data-ct-theme="studio.example.theme"] { font-family: serif; }"#,
            r#":root[data-ct-theme="studio.example.theme"] { --theme-font: serif; }"#,
        ] {
            let error = validate_css(css, "studio.example.theme")
                .expect_err("theme font override should fail");
            assert!(error.to_string().contains("默认字体"));
        }
    }

    #[test]
    fn accepts_namespaced_keyframes_and_rejects_global_animation_names() {
        validate_css(
            r#"@keyframes ct-theme-float { from { opacity: 0; } to { opacity: 1; } }
               :root[data-ct-theme="studio.example.theme"] { animation: ct-theme-float 1s; }"#,
            "studio.example.theme",
        )
        .expect("namespaced theme keyframes should be allowed");
        let error = validate_css(
            "@keyframes float { from { opacity: 0; } to { opacity: 1; } }",
            "studio.example.theme",
        )
        .expect_err("global animation names should fail");
        assert!(error.to_string().contains("ct- 前缀"));
    }

    #[test]
    fn accepts_only_supported_color_schemes() {
        validate_css(
            r#":root[data-ct-theme="studio.example.theme"][data-ct-color-scheme="light"] { color: black; }"#,
            "studio.example.theme",
        )
        .expect("light scheme should be allowed");
        let error = validate_css(
            r#":root[data-ct-theme="studio.example.theme"][data-ct-color-scheme="system"] { color: black; }"#,
            "studio.example.theme",
        )
        .expect_err("unknown scheme should fail");
        assert!(error.to_string().contains("白名单"));
    }

    #[test]
    fn accepts_only_supported_runtime_views() {
        validate_css(
            r#":root[data-ct-theme="studio.example.theme"][data-ct-view="home-compact"] { color: black; }"#,
            "studio.example.theme",
        )
        .expect("compact home view should be allowed");
        let error = validate_css(
            r#":root[data-ct-theme="studio.example.theme"][data-ct-view="custom"] { color: black; }"#,
            "studio.example.theme",
        )
        .expect_err("unknown runtime view should fail");
        assert!(error.to_string().contains("白名单"));
    }

    #[test]
    fn accepts_page_menu_and_detailed_settings_slots() {
        for slot in [
            "page.surface",
            "page.header",
            "page.content",
            "menu.item.active",
            "menu.item.checked",
            "menu.separator",
            "settings.surface",
            "settings.body",
            "settings.section.title",
            "settings.card",
            "settings.row.title",
            "settings.row.description",
            "settings.row.separator",
            "settings.switch.checked",
            "settings.switch.track.checked",
            "settings.switch.thumb",
            "main.content.frame",
            "home.layout",
            "composer.region",
            "composer.backdrop",
            "conversation.stage",
            "conversation.header",
            "conversation.header.content",
            "conversation.viewport",
            "conversation.summary.region",
            "conversation.summary",
            "conversation.summary.decoration",
        ] {
            validate_css(
                &format!(
                    r#":root[data-ct-theme="studio.example.theme"] [data-ct-slot="{slot}"] {{ color: black; }}"#
                ),
                "studio.example.theme",
            )
            .unwrap_or_else(|error| panic!("{slot} should be allowed: {error}"));
        }
    }

    #[test]
    fn accepts_controlled_asset_mount_selectors() {
        for slot in ALLOWED_ASSET_SLOTS {
            validate_css(
                &format!(
                    r#":root[data-ct-theme="studio.example.theme"] [data-ct-mount="{slot}"] {{ pointer-events: none; }}"#
                ),
                "studio.example.theme",
            )
            .unwrap_or_else(|error| panic!("asset mount {slot} should be allowed: {error}"));
        }
        validate_css(
            r#":root[data-ct-theme="studio.example.theme"] [data-ct-mount="app.background"] > img { object-fit: cover; object-position: center; opacity: 0.8; }"#,
            "studio.example.theme",
        )
        .expect("app background image presentation should be themeable");
    }

    #[test]
    fn rejects_active_svg_content() {
        let error =
            validate_svg(r#"<svg xmlns="http://www.w3.org/2000/svg" onload="alert(1)"></svg>"#)
                .expect_err("active SVG content should be rejected");
        assert!(error.to_string().contains("onload="));
    }

    fn signed_test_package(tamper: bool) -> Vec<u8> {
        let package = signed_test_package_with_description("A signed test theme.");
        if !tamper {
            return package;
        }
        let files = read_archive(&package, true).expect("read signed package");
        build_test_archive(files.into_iter().map(|(path, source)| {
            if path == "styles/theme.css" {
                (path, b"tampered".to_vec())
            } else {
                (path, source)
            }
        }))
    }

    fn signed_test_package_with_description(description: &str) -> Vec<u8> {
        signed_test_package_with_access(description, None)
    }

    fn signed_online_test_package() -> Vec<u8> {
        signed_online_test_package_with_description("An online test theme.")
    }

    fn signed_online_test_package_with_description(description: &str) -> Vec<u8> {
        signed_test_package_with_access(
            description,
            Some(json!({
                "delivery": "online",
                "slug": "test-theme"
            })),
        )
    }

    fn signed_test_package_with_access(description: &str, access: Option<Value>) -> Vec<u8> {
        let manifest = format!(
            r##"{{
          "schemaVersion": 1,
          "id": "studio.example.test-theme",
          "name": "Test Theme",
          "description": "{description}",
          "version": "1.0.0",
          "author": {{ "id": "studio.example", "name": "Example Studio" }},
          "testedCodexVersions": ["26.707.91948"],
          "styles": ["styles/theme.css"],
          "slots": ["app.shell", "home.hero"],
          "permissions": [],{access}
          "preview": {{ "background": "#111111", "surface": "#222222", "accent": "#aaff44" }},
          "experience": {{
            "homeHero": {{
              "eyebrow": "TEST",
              "title": "Signed package",
              "description": "Installer verification",
              "asset": "assets/hero.svg",
              "fit": "cover",
              "position": "center"
            }},
            "decorations": []
          }},
          "integrity": "integrity.json",
          "signature": "signature.ed25519"
        }}"##,
            access = access
                .map(|value| format!("\n          \"access\": {value},"))
                .unwrap_or_default()
        );
        let css = r#":root[data-ct-theme="studio.example.test-theme"] [data-ct-slot="app.shell"] { color: white; }"#;
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><path d="M0 0h10v10H0z"/></svg>"#;
        let mut files = BTreeMap::from([
            ("manifest.json".to_owned(), manifest.into_bytes()),
            ("styles/theme.css".to_owned(), css.as_bytes().to_vec()),
            ("assets/hero.svg".to_owned(), svg.as_bytes().to_vec()),
        ]);
        let integrity = IntegrityIndex {
            algorithm: "sha256".into(),
            files: files
                .iter()
                .map(|(path, source)| (path.clone(), sha256_hex(source)))
                .collect(),
        };
        let integrity_source = serde_json::to_vec(&integrity).expect("serialize integrity");
        let signing_key = SigningKey::from_bytes(&TEST_SECRET);
        let signature = signing_key.sign(&integrity_source).to_bytes().to_vec();
        files.insert("integrity.json".to_owned(), integrity_source);
        files.insert("signature.ed25519".to_owned(), signature);

        build_test_archive(files)
    }

    fn build_test_archive(files: impl IntoIterator<Item = (String, Vec<u8>)>) -> Vec<u8> {
        let mut cursor = Cursor::new(Vec::new());
        let mut writer = ZipWriter::new(&mut cursor);
        for (path, source) in files {
            writer
                .start_file(path, SimpleFileOptions::default())
                .expect("start package file");
            writer.write_all(&source).expect("write package file");
        }
        writer.finish().expect("finish package");
        cursor.into_inner()
    }
}
