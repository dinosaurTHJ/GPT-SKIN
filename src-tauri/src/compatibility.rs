use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use ed25519_dalek::{Signature, VerifyingKey};
use reqwest::Client;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{
    fmt, fs,
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{api, security_config};

const SCHEMA_VERSION: u8 = 1;
const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct CompatibilityError(String);

impl fmt::Display for CompatibilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for CompatibilityError {}

impl From<std::io::Error> for CompatibilityError {
    fn from(error: std::io::Error) -> Self {
        Self(error.to_string())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CompatibilityConfig {
    schema: u8,
    revision: u64,
    issued_at: u64,
    expires_at: u64,
    engine: VersionRange,
    adapter: CodexAdapter,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompatibilityStatus {
    pub adapter_id: String,
    pub revision: Option<u64>,
    pub source: CompatibilitySource,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CompatibilitySource {
    SignedRemote,
    BuiltIn,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct VersionRange {
    minimum: String,
    maximum_exclusive: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CodexAdapter {
    pub(crate) id: String,
    codex: VersionRange,
    pub(crate) probes: Vec<String>,
    pub(crate) selectors: AdapterSelectors,
    strategies: AdapterStrategies,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AdapterSelectors {
    pub(crate) titlebar: String,
    #[serde(default)]
    pub(crate) application_menu: Option<String>,
    pub(crate) main: String,
    #[serde(default)]
    pub(crate) main_top_fade: Option<String>,
    #[serde(default)]
    pub(crate) main_content_frame: Option<String>,
    #[serde(default)]
    pub(crate) workspace_panel: Option<String>,
    pub(crate) sidebar_scroll: String,
    pub(crate) sidebar_section: String,
    pub(crate) composer: String,
    pub(crate) composer_root: String,
    #[serde(default)]
    pub(crate) composer_utility_bar: Option<String>,
    pub(crate) home_source: String,
    pub(crate) home_cards: String,
    pub(crate) home_brand: Option<String>,
    pub(crate) conversation: String,
    #[serde(default)]
    pub(crate) conversation_summary_region: Option<String>,
    pub(crate) settings_item: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AdapterStrategies {
    hero_mount: HeroMountStrategy,
    composer_surface: ComposerSurfaceStrategy,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum HeroMountStrategy {
    MainPrepend,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ComposerSurfaceStrategy {
    VisualAncestor,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SignedCompatibilityResponse {
    payload: String,
    signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CachedCompatibility {
    payload: String,
    signature: String,
}

#[derive(Clone)]
pub struct CompatibilityRepository {
    cache_dir: PathBuf,
    client: Client,
    verifying_key: VerifyingKey,
}

impl CompatibilityRepository {
    pub fn new(data_dir: PathBuf) -> Result<Self, CompatibilityError> {
        fs::create_dir_all(&data_dir)?;
        Ok(Self {
            cache_dir: data_dir,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .map_err(|error| {
                    CompatibilityError(format!("无法初始化兼容配置客户端：{error}"))
                })?,
            verifying_key: compatibility_verifying_key()?,
        })
    }

    pub fn adapter(&self, codex_version: &str) -> CodexAdapter {
        self.load_cached(codex_version, unix_time())
            .ok()
            .map(|config| config.adapter.with_builtin_defaults(codex_version))
            .unwrap_or_else(|| builtin_adapter(codex_version))
    }

    pub fn status(&self, codex_version: &str) -> CompatibilityStatus {
        if let Ok(config) = self.load_cached(codex_version, unix_time()) {
            return CompatibilityStatus {
                adapter_id: config.adapter.id,
                revision: Some(config.revision),
                source: CompatibilitySource::SignedRemote,
            };
        }
        CompatibilityStatus {
            adapter_id: builtin_adapter(codex_version).id,
            revision: None,
            source: CompatibilitySource::BuiltIn,
        }
    }

    pub async fn refresh(&self, codex_version: &str) -> Result<bool, CompatibilityError> {
        self.cache_path(codex_version)?;
        let request = self
            .client
            .get(api::url("/compatibility/codex"))
            .header(reqwest::header::ACCEPT, "application/json")
            .query(&[
                ("codex_version", codex_version),
                ("engine_version", ENGINE_VERSION),
            ]);
        let request = api::sign(request).map_err(CompatibilityError)?;
        let response = self
            .client
            .execute(request)
            .await
            .map_err(|error| CompatibilityError(format!("无法下载 Codex 兼容配置：{error}")))?
            .error_for_status()
            .map_err(|error| CompatibilityError(format!("Codex 兼容配置下载失败：{error}")))?;
        let envelope: ApiEnvelope = response
            .json()
            .await
            .map_err(|error| CompatibilityError(format!("Codex 兼容配置响应无效：{error}")))?;
        if envelope.code >= 400 {
            return Err(CompatibilityError(envelope.message));
        }
        let signed: SignedCompatibilityResponse = serde_json::from_value(envelope.data)
            .map_err(|error| CompatibilityError(format!("Codex 兼容配置数据无效：{error}")))?;
        let config = verify_signed_payload(
            &signed.payload,
            &signed.signature,
            &self.verifying_key,
            unix_time(),
            codex_version,
        )?;
        self.persist_signed(codex_version, signed, config, unix_time())
    }

    fn persist_signed(
        &self,
        codex_version: &str,
        signed: SignedCompatibilityResponse,
        config: CompatibilityConfig,
        now: u64,
    ) -> Result<bool, CompatibilityError> {
        if let Ok(current) = self.load_cached(codex_version, now)
            && config.revision < current.revision
        {
            return Ok(false);
        }
        write_atomic(
            &self.cache_path(codex_version)?,
            &serde_json::to_vec(&CachedCompatibility {
                payload: signed.payload,
                signature: signed.signature,
            })
            .map_err(|error| CompatibilityError(format!("无法编码兼容配置缓存：{error}")))?,
        )?;
        Ok(true)
    }

    fn load_cached(
        &self,
        codex_version: &str,
        now: u64,
    ) -> Result<CompatibilityConfig, CompatibilityError> {
        let cached: CachedCompatibility =
            serde_json::from_slice(&fs::read(self.cache_path(codex_version)?)?)
                .map_err(|error| CompatibilityError(format!("Codex 兼容配置缓存无效：{error}")))?;
        verify_signed_payload(
            &cached.payload,
            &cached.signature,
            &self.verifying_key,
            now,
            codex_version,
        )
    }

    fn cache_path(&self, codex_version: &str) -> Result<PathBuf, CompatibilityError> {
        let codex_version = parse_version(codex_version, "Codex")?;
        let engine_version = parse_version(ENGINE_VERSION, "ReTheme 引擎")?;
        Ok(self.cache_dir.join(format!(
            "codex-{codex_version}-engine-{engine_version}.json"
        )))
    }
}

#[derive(Debug, Deserialize)]
struct ApiEnvelope {
    code: u16,
    message: String,
    data: serde_json::Value,
}

pub fn builtin_adapter(codex_version: &str) -> CodexAdapter {
    let Some(version) = Version::parse(codex_version).ok() else {
        return builtin_latest_adapter();
    };
    let adapters = builtin_adapters();
    if let Some(adapter) = adapters
        .iter()
        .find(|adapter| range_contains(&adapter.codex, &version))
    {
        return adapter.clone();
    }
    adapters
        .iter()
        .rev()
        .find(|adapter| {
            Version::parse(&adapter.codex.minimum).is_ok_and(|minimum| minimum <= version)
        })
        .cloned()
        .unwrap_or_else(|| adapters[0].clone())
}

fn builtin_adapters() -> Vec<CodexAdapter> {
    vec![
        adapter(
            "codex-2026-home-v1",
            VersionRange {
                minimum: "26.707.0".to_owned(),
                maximum_exclusive: "26.715.0".to_owned(),
            },
        ),
        builtin_latest_adapter(),
    ]
}

fn builtin_latest_adapter() -> CodexAdapter {
    adapter(
        "codex-2026-home-v2",
        VersionRange {
            minimum: "26.715.0".to_owned(),
            maximum_exclusive: "27.0.0".to_owned(),
        },
    )
}

fn adapter(id: &str, codex: VersionRange) -> CodexAdapter {
    CodexAdapter {
        id: id.to_owned(),
        codex,
        probes: vec![
            "[data-app-shell-main-content-layout]".to_owned(),
            "[data-codex-composer-root]".to_owned(),
        ],
        selectors: AdapterSelectors {
            titlebar: "[data-app-shell-header-edge-scroll]".to_owned(),
            application_menu: Some("[class*=\"application-menu-top-bar\"]".to_owned()),
            main: "main".to_owned(),
            main_top_fade: Some(".app-shell-main-content-top-fade".to_owned()),
            main_content_frame: Some(".app-shell-main-content-frame".to_owned()),
            workspace_panel: Some("[data-app-shell-tabs]".to_owned()),
            sidebar_scroll: "[data-app-action-sidebar-scroll]".to_owned(),
            sidebar_section: "[data-app-action-sidebar-section]".to_owned(),
            composer: "[data-codex-composer], textarea, [contenteditable=\"true\"]".to_owned(),
            composer_root: "[data-codex-composer-root]".to_owned(),
            composer_utility_bar: None,
            home_source: "[data-feature=\"game-source\"]".to_owned(),
            home_cards: "section".to_owned(),
            home_brand: Some("[data-testid=\"home-icon\"]".to_owned()),
            conversation: "[data-app-action-timeline-scroll]".to_owned(),
            conversation_summary_region: Some(
                "[data-pip-obstacle=\"thread-summary-panel\"]".to_owned(),
            ),
            settings_item: "[data-settings-panel-slug]".to_owned(),
        },
        strategies: AdapterStrategies {
            hero_mount: HeroMountStrategy::MainPrepend,
            composer_surface: ComposerSurfaceStrategy::VisualAncestor,
        },
    }
}

fn verify_signed_payload(
    payload: &str,
    signature: &str,
    key: &VerifyingKey,
    now: u64,
    codex_version: &str,
) -> Result<CompatibilityConfig, CompatibilityError> {
    let signature = URL_SAFE_NO_PAD
        .decode(signature)
        .map_err(|_| CompatibilityError("Codex 兼容配置签名格式无效".into()))?;
    let signature = Signature::from_slice(&signature)
        .map_err(|_| CompatibilityError("Codex 兼容配置签名长度无效".into()))?;
    key.verify_strict(payload.as_bytes(), &signature)
        .map_err(|_| CompatibilityError("Codex 兼容配置签名验证失败".into()))?;
    let config: CompatibilityConfig = serde_json::from_str(payload)
        .map_err(|error| CompatibilityError(format!("Codex 兼容配置内容无效：{error}")))?;
    validate_config(&config, now, codex_version)?;
    Ok(config)
}

fn validate_config(
    config: &CompatibilityConfig,
    now: u64,
    codex_version: &str,
) -> Result<(), CompatibilityError> {
    if config.schema != SCHEMA_VERSION || config.revision == 0 {
        return Err(CompatibilityError("Codex 兼容配置协议无效".into()));
    }
    if config.issued_at > now.saturating_add(300) || config.expires_at <= now {
        return Err(CompatibilityError("Codex 兼容配置尚未生效或已过期".into()));
    }
    let engine = parse_version(ENGINE_VERSION, "ReTheme 引擎")?;
    validate_range(&config.engine, "ReTheme 引擎")?;
    if !range_contains(&config.engine, &engine) {
        return Err(CompatibilityError("远程兼容配置要求升级 ReTheme".into()));
    }
    let codex_version = parse_version(codex_version, "Codex")?;
    validate_range(&config.adapter.codex, "Codex")?;
    if !range_contains(&config.adapter.codex, &codex_version) {
        return Err(CompatibilityError(
            "远程适配器与当前 Codex 版本不匹配".into(),
        ));
    }
    if config.adapter.id.is_empty() || config.adapter.probes.is_empty() {
        return Err(CompatibilityError("Codex 适配器声明不完整".into()));
    }
    for selector in config
        .adapter
        .selectors
        .all()
        .into_iter()
        .chain(&config.adapter.probes)
    {
        validate_selector(selector)?;
    }
    Ok(())
}

fn parse_version(version: &str, name: &str) -> Result<Version, CompatibilityError> {
    Version::parse(version).map_err(|error| CompatibilityError(format!("{name}版本无效：{error}")))
}

fn validate_range(range: &VersionRange, name: &str) -> Result<(), CompatibilityError> {
    let minimum = parse_version(&range.minimum, name)?;
    let maximum = parse_version(&range.maximum_exclusive, name)?;
    if minimum >= maximum {
        return Err(CompatibilityError(format!("{name}兼容范围无效")));
    }
    Ok(())
}

fn range_contains(range: &VersionRange, version: &Version) -> bool {
    Version::parse(&range.minimum)
        .ok()
        .zip(Version::parse(&range.maximum_exclusive).ok())
        .is_some_and(|(minimum, maximum)| version >= &minimum && version < &maximum)
}

impl AdapterSelectors {
    fn all(&self) -> Vec<&String> {
        let mut selectors = vec![
            &self.titlebar,
            &self.main,
            &self.sidebar_scroll,
            &self.sidebar_section,
            &self.composer,
            &self.composer_root,
            &self.home_source,
            &self.home_cards,
            &self.conversation,
            &self.settings_item,
        ];
        if let Some(application_menu) = &self.application_menu {
            selectors.push(application_menu);
        }
        if let Some(composer_utility_bar) = &self.composer_utility_bar {
            selectors.push(composer_utility_bar);
        }
        if let Some(home_brand) = &self.home_brand {
            selectors.push(home_brand);
        }
        if let Some(main_top_fade) = &self.main_top_fade {
            selectors.push(main_top_fade);
        }
        if let Some(main_content_frame) = &self.main_content_frame {
            selectors.push(main_content_frame);
        }
        if let Some(workspace_panel) = &self.workspace_panel {
            selectors.push(workspace_panel);
        }
        if let Some(conversation_summary_region) = &self.conversation_summary_region {
            selectors.push(conversation_summary_region);
        }
        selectors
    }
}

impl CodexAdapter {
    fn with_builtin_defaults(mut self, codex_version: &str) -> Self {
        if self.selectors.application_menu.is_none() {
            self.selectors.application_menu =
                builtin_adapter(codex_version).selectors.application_menu;
        }
        if self.selectors.main_top_fade.is_none() {
            self.selectors.main_top_fade = builtin_adapter(codex_version).selectors.main_top_fade;
        }
        if self.selectors.main_content_frame.is_none() {
            self.selectors.main_content_frame =
                builtin_adapter(codex_version).selectors.main_content_frame;
        }
        if self.selectors.workspace_panel.is_none() {
            self.selectors.workspace_panel =
                builtin_adapter(codex_version).selectors.workspace_panel;
        }
        if self.selectors.conversation_summary_region.is_none() {
            self.selectors.conversation_summary_region = builtin_adapter(codex_version)
                .selectors
                .conversation_summary_region;
        }
        self
    }
}

fn validate_selector(selector: &str) -> Result<(), CompatibilityError> {
    if selector.is_empty()
        || selector.len() > 240
        || selector.chars().any(|character| {
            !character.is_ascii_alphanumeric()
                && !matches!(
                    character,
                    ' ' | '-' | '_' | '[' | ']' | '=' | '"' | '\'' | '.' | '#' | ':' | ',' | '*'
                )
        })
        || selector.contains(":has")
        || selector.contains("data-ct-")
    {
        return Err(CompatibilityError(format!(
            "Codex 兼容配置包含不安全选择器：{selector}"
        )));
    }
    Ok(())
}

fn compatibility_verifying_key() -> Result<VerifyingKey, CompatibilityError> {
    let mut bytes = [0_u8; 32];
    hex::decode_to_slice(security_config::compatibility_public_key(), &mut bytes)
        .map_err(|error| CompatibilityError(format!("兼容配置公钥无效：{error}")))?;
    VerifyingKey::from_bytes(&bytes).map_err(|_| CompatibilityError("兼容配置公钥无效".into()))
}

fn write_atomic(path: &Path, source: &[u8]) -> Result<(), CompatibilityError> {
    let parent = path
        .parent()
        .ok_or_else(|| CompatibilityError("兼容配置缓存路径无效".into()))?;
    fs::create_dir_all(parent)?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    temporary.write_all(source)?;
    temporary.as_file().sync_all()?;
    temporary.persist(path).map_err(|error| {
        CompatibilityError(format!("无法原子写入兼容配置缓存：{}", error.error))
    })?;
    Ok(())
}

fn unix_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    const TEST_SIGNING_SEED: [u8; 32] = [17; 32];

    fn signed_config(
        revision: u64,
        now: u64,
        adapter: CodexAdapter,
        signing_key: &SigningKey,
    ) -> SignedCompatibilityResponse {
        let config = CompatibilityConfig {
            schema: SCHEMA_VERSION,
            revision,
            issued_at: now - 10,
            expires_at: now + 600,
            engine: VersionRange {
                minimum: "0.1.0".to_owned(),
                maximum_exclusive: "1.0.0".to_owned(),
            },
            adapter,
        };
        let payload = serde_json::to_string(&config).expect("serialize config");
        let signature = URL_SAFE_NO_PAD.encode(signing_key.sign(payload.as_bytes()).to_bytes());
        SignedCompatibilityResponse { payload, signature }
    }

    fn configured_signing_key() -> SigningKey {
        SigningKey::from_bytes(&TEST_SIGNING_SEED)
    }

    fn test_repository(data_dir: PathBuf) -> CompatibilityRepository {
        fs::create_dir_all(&data_dir).expect("compatibility directory");
        CompatibilityRepository {
            cache_dir: data_dir,
            client: Client::new(),
            verifying_key: configured_signing_key().verifying_key(),
        }
    }

    #[test]
    fn selects_builtin_adapter_by_codex_version() {
        assert_eq!(builtin_adapter("26.707.91948").id, "codex-2026-home-v1");
        assert_eq!(builtin_adapter("26.715.21316").id, "codex-2026-home-v2");
        assert_eq!(builtin_adapter("26.600.1").id, "codex-2026-home-v1");
        assert_eq!(builtin_adapter("27.1.0").id, "codex-2026-home-v2");
        assert!(
            builtin_adapter("26.715.21316")
                .selectors
                .composer_utility_bar
                .is_none()
        );
    }

    #[test]
    fn verifies_signed_single_adapter_config() {
        let signing_key = SigningKey::from_bytes(&[17; 32]);
        let signed = signed_config(
            2,
            1_800_000_000,
            builtin_adapter("26.715.21316"),
            &signing_key,
        );
        let config = verify_signed_payload(
            &signed.payload,
            &signed.signature,
            &signing_key.verifying_key(),
            1_800_000_000,
            "26.715.21316",
        )
        .expect("signed config");
        assert_eq!(config.revision, 2);
        assert_eq!(config.adapter.id, "codex-2026-home-v2");
    }

    #[test]
    fn rejects_tampering_expiration_mismatch_and_unsafe_config() {
        let signing_key = SigningKey::from_bytes(&[17; 32]);
        let signed = signed_config(
            2,
            1_800_000_000,
            builtin_adapter("26.715.21316"),
            &signing_key,
        );
        let verify = |payload: &str, now: u64, codex_version: &str| {
            verify_signed_payload(
                payload,
                &signed.signature,
                &signing_key.verifying_key(),
                now,
                codex_version,
            )
        };
        assert!(
            verify(
                &(signed.payload.clone() + " "),
                1_800_000_000,
                "26.715.21316"
            )
            .is_err()
        );
        assert!(verify(&signed.payload, 1_800_001_000, "26.715.21316").is_err());
        assert!(verify(&signed.payload, 1_800_000_000, "26.707.91948").is_err());
        assert!(validate_selector("main:has(script)").is_err());
        assert!(validate_selector("[data-codex-composer-root]").is_ok());

        let mut config: CompatibilityConfig =
            serde_json::from_str(&signed.payload).expect("compatibility config");
        config.engine.minimum = "1.0.0".to_owned();
        config.engine.maximum_exclusive = "2.0.0".to_owned();
        let payload = serde_json::to_string(&config).expect("serialize config");
        let signature = URL_SAFE_NO_PAD.encode(signing_key.sign(payload.as_bytes()).to_bytes());
        assert!(
            verify_signed_payload(
                &payload,
                &signature,
                &signing_key.verifying_key(),
                1_800_000_000,
                "26.715.21316",
            )
            .is_err()
        );
    }

    #[test]
    fn keeps_versioned_caches_separate() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let repository = test_repository(directory.path().to_path_buf());
        let signing_key = configured_signing_key();
        let now = unix_time();
        for (version, revision) in [("26.707.91948", 2), ("26.715.21316", 3)] {
            let signed = signed_config(revision, now, builtin_adapter(version), &signing_key);
            let config = verify_signed_payload(
                &signed.payload,
                &signed.signature,
                &signing_key.verifying_key(),
                now,
                version,
            )
            .expect("signed config");
            assert!(
                repository
                    .persist_signed(version, signed, config, now)
                    .expect("persist cache")
            );
        }
        assert_ne!(
            repository.cache_path("26.707.91948").expect("legacy path"),
            repository.cache_path("26.715.21316").expect("current path")
        );
        assert_eq!(repository.adapter("26.707.91948").id, "codex-2026-home-v1");
        assert_eq!(repository.adapter("26.715.21316").id, "codex-2026-home-v2");
    }

    #[test]
    fn reports_signed_and_builtin_compatibility_sources() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let repository = test_repository(directory.path().to_path_buf());
        assert_eq!(
            repository.status("26.715.21316"),
            CompatibilityStatus {
                adapter_id: "codex-2026-home-v2".to_owned(),
                revision: None,
                source: CompatibilitySource::BuiltIn,
            }
        );

        let now = unix_time();
        let signed = signed_config(
            12,
            now,
            builtin_adapter("26.715.21316"),
            &configured_signing_key(),
        );
        let config = verify_signed_payload(
            &signed.payload,
            &signed.signature,
            &configured_signing_key().verifying_key(),
            now,
            "26.715.21316",
        )
        .expect("signed config");
        repository
            .persist_signed("26.715.21316", signed, config, now)
            .expect("persist cache");

        assert_eq!(
            repository.status("26.715.21316"),
            CompatibilityStatus {
                adapter_id: "codex-2026-home-v2".to_owned(),
                revision: Some(12),
                source: CompatibilitySource::SignedRemote,
            }
        );
    }

    #[test]
    fn fills_new_optional_selectors_for_legacy_signed_cache() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let repository = test_repository(directory.path().to_path_buf());
        let signing_key = configured_signing_key();
        let now = unix_time();
        let mut legacy_adapter = builtin_adapter("26.715.21316");
        legacy_adapter.selectors.main_top_fade = None;
        legacy_adapter.selectors.main_content_frame = None;
        legacy_adapter.selectors.conversation_summary_region = None;
        let signed = signed_config(1, now, legacy_adapter, &signing_key);
        let config = verify_signed_payload(
            &signed.payload,
            &signed.signature,
            &signing_key.verifying_key(),
            now,
            "26.715.21316",
        )
        .expect("legacy signed config");
        repository
            .persist_signed("26.715.21316", signed, config, now)
            .expect("persist legacy config");

        assert_eq!(
            repository
                .adapter("26.715.21316")
                .selectors
                .main_top_fade
                .as_deref(),
            Some(".app-shell-main-content-top-fade")
        );
        let selectors = repository.adapter("26.715.21316").selectors;
        assert_eq!(
            selectors.main_content_frame.as_deref(),
            Some(".app-shell-main-content-frame")
        );
        assert_eq!(
            selectors.conversation_summary_region.as_deref(),
            Some("[data-pip-obstacle=\"thread-summary-panel\"]")
        );
    }

    #[test]
    fn rejects_cache_rollback_and_falls_back_safely() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let repository = test_repository(directory.path().to_path_buf());
        let signing_key = configured_signing_key();
        let now = unix_time();
        for revision in [5, 4] {
            let mut adapter = builtin_adapter("26.715.21316");
            adapter.id = format!("remote-revision-{revision}");
            let signed = signed_config(revision, now, adapter, &signing_key);
            let config = verify_signed_payload(
                &signed.payload,
                &signed.signature,
                &signing_key.verifying_key(),
                now,
                "26.715.21316",
            )
            .expect("signed config");
            let persisted = repository
                .persist_signed("26.715.21316", signed, config, now)
                .expect("persist cache");
            assert_eq!(persisted, revision == 5);
        }
        assert_eq!(repository.adapter("26.715.21316").id, "remote-revision-5");

        fs::write(
            repository
                .cache_path("26.707.91948")
                .expect("legacy cache path"),
            b"invalid",
        )
        .expect("invalid cache");
        assert_eq!(
            repository.adapter("26.707.91948"),
            builtin_adapter("26.707.91948")
        );
        assert!(repository.cache_path("../unsafe").is_err());
    }
}
