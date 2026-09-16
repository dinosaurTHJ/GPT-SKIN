use lightningcss::properties::PropertyId;
use lightningcss::rules::{CssRule, CssRuleList};
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use lightningcss::traits::ToCss;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::fs;
use std::io::{Cursor, Read};
use std::path::{Component, Path};
use zip::ZipArchive;

pub const MAX_ARCHIVE_SIZE: u64 = 30 * 1024 * 1024;
pub const MAX_EXTRACTED_SIZE: u64 = 60 * 1024 * 1024;
pub const MAX_IMAGE_SIZE: u64 = 8 * 1024 * 1024;
pub const MAX_FILE_COUNT: usize = 256;

pub const ALLOWED_SLOTS: [&str; 164] = [
    "app.shell",
    "app.background",
    "titlebar",
    "sidebar",
    "sidebar.scroll",
    "sidebar.resize",
    "sidebar.resize.indicator",
    "sidebar.header",
    "sidebar.header.icon",
    "sidebar.header.label",
    "sidebar.header.background",
    "sidebar.header.decoration",
    "sidebar.brand",
    "sidebar.brand.icon",
    "sidebar.brand.badge",
    "sidebar.frame",
    "sidebar.section",
    "sidebar.section.projects",
    "sidebar.section.header",
    "sidebar.section.toggle",
    "sidebar.section.label",
    "sidebar.section.actions",
    "sidebar.section.action",
    "sidebar.section.action.icon",
    "sidebar.section.decoration",
    "sidebar.item",
    "sidebar.item.icon",
    "sidebar.item.label",
    "sidebar.item.active",
    "sidebar.item.active.icon",
    "sidebar.item.active.label",
    "sidebar.footer",
    "sidebar.footer.item",
    "sidebar.footer.icon",
    "sidebar.footer.label",
    "sidebar.footer.brand",
    "sidebar.footer.brand.label",
    "sidebar.footer.brand.timer",
    "sidebar.footer.brand.pro",
    "sidebar.footer.brand.version",
    "main",
    "main.fade",
    "main.content.frame",
    "main.background",
    "main.overlay",
    "main.frame",
    "page",
    "page.surface",
    "page.header",
    "page.content",
    "menu",
    "menu.item",
    "menu.item.active",
    "menu.item.checked",
    "menu.icon",
    "menu.label",
    "menu.shortcut",
    "menu.separator",
    "composer",
    "composer.backdrop",
    "composer.context",
    "composer.context.item",
    "composer.context.item.icon",
    "composer.context.item.label",
    "composer.editor",
    "composer.action",
    "composer.action.icon",
    "composer.action.label",
    "composer.permission",
    "composer.permission.icon",
    "composer.permission.label",
    "composer.submit",
    "composer.submit.icon",
    "composer.submit.decoration",
    "composer.decoration",
    "composer.panel",
    "composer.panel.item",
    "composer.panel.icon",
    "composer.panel.separator",
    "home.hero",
    "home.hero.viewport",
    "home.hero.copy",
    "home.hero.eyebrow",
    "home.hero.title",
    "home.hero.description",
    "home.hero.media",
    "home.hero.media.asset",
    "home.hero.foreground",
    "home.hero.foreground.asset",
    "home.hero.divider",
    "home.hero.divider.icon",
    "home.hero.divider.label",
    "home.hero.divider.line",
    "home.layout",
    "home.content.region",
    "home.stage",
    "home.brand",
    "home.prompt",
    "home.prompt.title",
    "home.cards",
    "home.cards.layout",
    "home.cards.grid",
    "home.card",
    "home.card.background",
    "home.card.content",
    "home.card.icon",
    "home.card.icon.glyph",
    "home.card.label",
    "home.card.arrow",
    "home.card.arrow.glyph",
    "home.card.arrow.asset",
    "composer.region",
    "conversation.stage",
    "conversation.header",
    "conversation.header.content",
    "conversation.viewport",
    "conversation.summary.region",
    "conversation.summary",
    "conversation.summary.decoration",
    "conversation",
    "conversation.user",
    "conversation.assistant",
    "conversation.banner",
    "conversation.banner.copy",
    "conversation.banner.eyebrow",
    "conversation.banner.title",
    "conversation.banner.description",
    "conversation.banner.media",
    "conversation.banner.media.asset",
    "conversation.banner.foreground",
    "conversation.banner.foreground.asset",
    "code",
    "code.inline",
    "diff",
    "terminal",
    "terminal.viewport",
    "settings",
    "settings.header",
    "settings.sidebar",
    "settings.nav",
    "settings.nav.item",
    "settings.nav.item.active",
    "settings.content",
    "settings.surface",
    "settings.frame",
    "settings.canvas",
    "settings.toolbar",
    "settings.body",
    "settings.section",
    "settings.section.title",
    "settings.card",
    "settings.row",
    "settings.row.title",
    "settings.row.description",
    "settings.row.separator",
    "settings.control",
    "settings.control.checked",
    "settings.switch",
    "settings.switch.checked",
    "settings.switch.track",
    "settings.switch.track.checked",
    "settings.switch.thumb",
    "decoration.top-right",
    "decoration.bottom-right",
];

pub const ALLOWED_ASSET_SLOTS: [&str; 11] = [
    "app.background",
    "main.background",
    "main.overlay",
    "main.frame",
    "sidebar.brand.icon",
    "sidebar.brand.badge",
    "sidebar.header.background",
    "sidebar.header.decoration",
    "sidebar.frame",
    "home.card.background",
    "home.card.arrow.asset",
];

pub const ALLOWED_MOUNTS: [&str; 19] = [
    "home.hero",
    "app.background",
    "main.background",
    "main.overlay",
    "main.frame",
    "sidebar.brand.icon",
    "sidebar.brand.badge",
    "sidebar.header.background",
    "sidebar.header.decoration",
    "sidebar.frame",
    "sidebar.footer.brand",
    "home.card.background",
    "home.card.arrow.asset",
    "composer.submit.decoration",
    "composer.decoration",
    "conversation.summary.decoration",
    "sidebar.section.decoration",
    "decoration.top-right",
    "decoration.bottom-right",
];

#[derive(Debug, Clone)]
pub struct ThemeError(pub String);

impl fmt::Display for ThemeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ThemeError {}

impl From<std::io::Error> for ThemeError {
    fn from(error: std::io::Error) -> Self {
        Self(error.to_string())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ThemeManifest {
    pub schema_version: u32,
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: ThemeAuthor,
    #[serde(
        default,
        rename = "testedCodexVersions",
        alias = "supportedCodexVersions"
    )]
    pub tested_codex_versions: Vec<String>,
    pub styles: Vec<String>,
    pub slots: Vec<String>,
    pub permissions: Vec<String>,
    pub preview: ThemePreview,
    pub experience: ThemeExperience,
    #[serde(default)]
    pub locales: BTreeMap<String, ThemeLocalization>,
    #[serde(default)]
    pub access: Option<ThemePackageAccess>,
    #[serde(default)]
    pub integrity: Option<String>,
    #[serde(default)]
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ThemeLocalization {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub experience: Option<ThemeExperienceLocalization>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ThemeExperienceLocalization {
    #[serde(default)]
    pub home_hero: Option<ThemeHeroLocalization>,
    #[serde(default)]
    pub home_prompt: Option<ThemeHomePrompt>,
    #[serde(default)]
    pub conversation_banner: Option<ThemeBannerLocalization>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeHeroLocalization {
    #[serde(default)]
    pub eyebrow: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub divider: Option<ThemeDividerLocalization>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeBannerLocalization {
    #[serde(default)]
    pub eyebrow: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeDividerLocalization {
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ThemePackageAccess {
    pub delivery: String,
    pub slug: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeAuthor {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThemePreview {
    pub background: String,
    pub surface: String,
    pub accent: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ThemeExperience {
    pub home_hero: ThemeHero,
    #[serde(default)]
    pub home_prompt: Option<ThemeHomePrompt>,
    #[serde(default)]
    pub conversation_banner: Option<ThemeConversationBanner>,
    #[serde(default)]
    pub composer_submit: Option<ThemeAsset>,
    #[serde(default)]
    pub composer_decoration: Option<ThemeAsset>,
    #[serde(default)]
    pub conversation_summary_decoration: Option<ThemeAsset>,
    #[serde(default)]
    pub sidebar_section_decoration: Option<ThemeAsset>,
    #[serde(default)]
    pub assets: Vec<ThemeControlledAsset>,
    #[serde(default)]
    pub decorations: Vec<ThemeDecoration>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeAsset {
    pub asset: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeHero {
    pub eyebrow: String,
    pub title: String,
    pub description: String,
    pub asset: String,
    pub fit: String,
    pub position: String,
    #[serde(default)]
    pub foreground: Option<String>,
    #[serde(default)]
    pub divider: Option<ThemeHeroDivider>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeHomePrompt {
    pub title: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeConversationBanner {
    #[serde(default)]
    pub eyebrow: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    pub asset: String,
    pub fit: String,
    pub position: String,
    #[serde(default)]
    pub foreground: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeHeroDivider {
    pub label: String,
    #[serde(default)]
    pub asset: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ThemeControlledAsset {
    pub slot: String,
    pub asset: String,
    #[serde(default)]
    pub light_asset: Option<String>,
    #[serde(default)]
    pub dark_asset: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeDecoration {
    pub slot: String,
    pub asset: String,
}

#[derive(Debug)]
pub struct ValidatedTheme {
    pub manifest: ThemeManifest,
    pub css: String,
}

pub type PackageFiles = HashMap<String, Vec<u8>>;

pub fn validate_source_archive(
    source: &[u8],
) -> Result<(ValidatedTheme, PackageFiles), ThemeError> {
    if source.len() as u64 > MAX_ARCHIVE_SIZE {
        return Err(ThemeError("主题包超过 30 MB".into()));
    }
    let files = read_archive(source, false)?;
    let theme = validate_package_files(&files, false)?;
    Ok((theme, files))
}

pub fn validate_development_directory(
    root: &Path,
) -> Result<(ValidatedTheme, PackageFiles), ThemeError> {
    let files = read_development_directory(root)?;
    let theme = validate_package_files(&files, false)?;
    Ok((theme, files))
}

pub fn validate_package_files(
    files: &PackageFiles,
    release: bool,
) -> Result<ValidatedTheme, ThemeError> {
    let manifest_source = utf8_file(files, "manifest.json")?;
    let manifest: ThemeManifest = serde_json::from_str(manifest_source)
        .map_err(|error| ThemeError(format!("主题 Manifest 无效：{error}")))?;
    validate_manifest(&manifest)?;
    if release
        && (manifest.integrity.as_deref() != Some("integrity.json")
            || manifest.signature.as_deref() != Some("signature.ed25519"))
    {
        return Err(ThemeError(
            "正式主题必须声明 integrity.json 与 signature.ed25519".into(),
        ));
    }
    if let Some(source) = files.get("compatibility.json") {
        serde_json::from_slice::<serde_json::Value>(source)
            .map_err(|error| ThemeError(format!("compatibility.json 无效：{error}")))?;
    }

    let mut css = String::new();
    for style_path in &manifest.styles {
        validate_style_path(style_path)?;
        let source = utf8_file(files, style_path)?;
        validate_css(source, &manifest.id)?;
        css.push_str(source);
        css.push('\n');
    }
    for asset_path in referenced_asset_paths(&manifest) {
        let source = files
            .get(asset_path)
            .ok_or_else(|| ThemeError(format!("主题缺少资源文件：{asset_path}")))?;
        validate_image(asset_path, source)?;
    }

    Ok(ValidatedTheme { manifest, css })
}

pub fn validate_manifest(manifest: &ThemeManifest) -> Result<(), ThemeError> {
    if manifest.schema_version != 1 {
        return Err(ThemeError(format!(
            "不支持主题规范版本：{}",
            manifest.schema_version
        )));
    }
    if !is_valid_theme_id(&manifest.id) {
        return Err(ThemeError(format!("主题 ID 格式无效：{}", manifest.id)));
    }
    if !is_valid_author_id(&manifest.author.id) {
        return Err(ThemeError(format!(
            "作者 ID 格式无效：{}",
            manifest.author.id
        )));
    }
    Version::parse(&manifest.version)
        .map_err(|error| ThemeError(format!("主题版本无效：{error}")))?;
    validate_text("Theme name", &manifest.name, 120)?;
    validate_text("Theme description", &manifest.description, 240)?;
    validate_text("Author name", &manifest.author.name, 100)?;
    if !manifest.permissions.is_empty() {
        return Err(ThemeError("第一阶段主题 permissions 必须为空".into()));
    }
    if manifest.locales.len() > 20 {
        return Err(ThemeError("主题语言数量不能超过 20".into()));
    }
    for (locale, translation) in &manifest.locales {
        if !is_valid_locale(locale) {
            return Err(ThemeError(format!("主题语言标签无效：{locale}")));
        }
        validate_optional_text("Localized theme name", translation.name.as_deref(), 120)?;
        validate_optional_text(
            "Localized theme description",
            translation.description.as_deref(),
            240,
        )?;
        if let Some(experience) = &translation.experience {
            if let Some(hero) = &experience.home_hero {
                validate_optional_text("Localized Banner eyebrow", hero.eyebrow.as_deref(), 80)?;
                validate_optional_text("Localized Banner title", hero.title.as_deref(), 120)?;
                validate_optional_text(
                    "Localized Banner description",
                    hero.description.as_deref(),
                    240,
                )?;
                if let Some(divider) = &hero.divider {
                    validate_optional_text(
                        "Localized divider label",
                        divider.label.as_deref(),
                        80,
                    )?;
                }
            }
            if let Some(prompt) = &experience.home_prompt {
                validate_text("Localized home prompt title", &prompt.title, 120)?;
            }
            if let Some(banner) = &experience.conversation_banner {
                validate_optional_text(
                    "Localized conversation eyebrow",
                    banner.eyebrow.as_deref(),
                    80,
                )?;
                validate_optional_text(
                    "Localized conversation title",
                    banner.title.as_deref(),
                    120,
                )?;
                validate_optional_text(
                    "Localized conversation description",
                    banner.description.as_deref(),
                    240,
                )?;
            }
        }
    }
    if manifest.styles.is_empty() {
        return Err(ThemeError("主题至少需要一个样式文件".into()));
    }
    if manifest.styles.iter().collect::<HashSet<_>>().len() != manifest.styles.len() {
        return Err(ThemeError("主题样式文件不能重复声明".into()));
    }
    if manifest.slots.iter().collect::<HashSet<_>>().len() != manifest.slots.len() {
        return Err(ThemeError("主题逻辑插槽不能重复声明".into()));
    }
    validate_text("Banner eyebrow", &manifest.experience.home_hero.eyebrow, 80)?;
    validate_text("Banner title", &manifest.experience.home_hero.title, 120)?;
    validate_text(
        "Banner description",
        &manifest.experience.home_hero.description,
        240,
    )?;
    validate_banner_layout(
        &manifest.experience.home_hero.fit,
        &manifest.experience.home_hero.position,
        "Banner",
    )?;
    validate_asset_path(&manifest.experience.home_hero.asset)?;
    if let Some(foreground) = &manifest.experience.home_hero.foreground {
        validate_asset_path(foreground)?;
    }
    if let Some(prompt) = &manifest.experience.home_prompt {
        validate_text("Home prompt title", &prompt.title, 120)?;
    }
    if let Some(banner) = &manifest.experience.conversation_banner {
        validate_text("Conversation Banner eyebrow", &banner.eyebrow, 80)?;
        validate_text("Conversation Banner title", &banner.title, 120)?;
        validate_text("Conversation Banner description", &banner.description, 240)?;
        validate_banner_layout(&banner.fit, &banner.position, "Conversation Banner")?;
        validate_asset_path(&banner.asset)?;
        if let Some(foreground) = &banner.foreground {
            validate_asset_path(foreground)?;
        }
    }
    if let Some(divider) = &manifest.experience.home_hero.divider {
        validate_text("Banner divider label", &divider.label, 80)?;
        if let Some(asset) = &divider.asset {
            validate_asset_path(asset)?;
        }
    }
    for asset in [
        &manifest.experience.composer_submit,
        &manifest.experience.composer_decoration,
        &manifest.experience.conversation_summary_decoration,
        &manifest.experience.sidebar_section_decoration,
    ]
    .into_iter()
    .flatten()
    {
        validate_asset_path(&asset.asset)?;
    }
    let mut asset_slots = HashSet::new();
    for asset in &manifest.experience.assets {
        if !ALLOWED_ASSET_SLOTS.contains(&asset.slot.as_str()) {
            return Err(ThemeError("主题资源声明了未开放的插槽".into()));
        }
        if !manifest.slots.iter().any(|slot| slot == &asset.slot) {
            return Err(ThemeError(format!(
                "主题资源插槽未在 slots 中声明：{}",
                asset.slot
            )));
        }
        if !asset_slots.insert(asset.slot.as_str()) {
            return Err(ThemeError(format!("主题资源插槽重复：{}", asset.slot)));
        }
        validate_asset_path(&asset.asset)?;
        if let Some(path) = &asset.light_asset {
            validate_asset_path(path)?;
        }
        if let Some(path) = &asset.dark_asset {
            validate_asset_path(path)?;
        }
    }
    for decoration in &manifest.experience.decorations {
        if !matches!(
            decoration.slot.as_str(),
            "decoration.top-right" | "decoration.bottom-right"
        ) {
            return Err(ThemeError("主题装饰声明了未开放的插槽".into()));
        }
        validate_asset_path(&decoration.asset)?;
    }
    if manifest
        .slots
        .iter()
        .any(|slot| !ALLOWED_SLOTS.contains(&slot.as_str()))
    {
        return Err(ThemeError("主题声明了未开放的逻辑插槽".into()));
    }
    for color in [
        &manifest.preview.background,
        &manifest.preview.surface,
        &manifest.preview.accent,
    ] {
        if !is_hex_color(color) {
            return Err(ThemeError(format!("主题预览色无效：{color}")));
        }
    }
    Ok(())
}

fn validate_banner_layout(fit: &str, position: &str, label: &str) -> Result<(), ThemeError> {
    if !matches!(fit, "cover" | "contain") {
        return Err(ThemeError(format!("{label} fit 只允许 cover 或 contain")));
    }
    if !matches!(position, "center" | "top" | "bottom" | "left" | "right") {
        return Err(ThemeError(format!("{label} position 不在允许范围")));
    }
    Ok(())
}

fn referenced_asset_paths(manifest: &ThemeManifest) -> HashSet<&str> {
    let mut paths = HashSet::from([manifest.experience.home_hero.asset.as_str()]);
    if let Some(path) = manifest.experience.home_hero.foreground.as_deref() {
        paths.insert(path);
    }
    if let Some(divider) = &manifest.experience.home_hero.divider
        && let Some(path) = divider.asset.as_deref()
    {
        paths.insert(path);
    }
    if let Some(banner) = &manifest.experience.conversation_banner {
        paths.insert(&banner.asset);
        if let Some(path) = banner.foreground.as_deref() {
            paths.insert(path);
        }
    }
    for asset in [
        &manifest.experience.composer_submit,
        &manifest.experience.composer_decoration,
        &manifest.experience.conversation_summary_decoration,
        &manifest.experience.sidebar_section_decoration,
    ]
    .into_iter()
    .flatten()
    {
        paths.insert(&asset.asset);
    }
    for asset in &manifest.experience.assets {
        paths.insert(&asset.asset);
        if let Some(path) = asset.light_asset.as_deref() {
            paths.insert(path);
        }
        if let Some(path) = asset.dark_asset.as_deref() {
            paths.insert(path);
        }
    }
    for decoration in &manifest.experience.decorations {
        paths.insert(&decoration.asset);
    }
    paths
}

pub fn read_archive(
    source: &[u8],
    require_release_files: bool,
) -> Result<PackageFiles, ThemeError> {
    let mut archive = ZipArchive::new(Cursor::new(source))
        .map_err(|error| ThemeError(format!("主题源码不是有效 ZIP：{error}")))?;
    if archive.len() > MAX_FILE_COUNT {
        return Err(ThemeError("主题包文件数量超过 256".into()));
    }
    let mut files = HashMap::new();
    let mut extracted_size = 0_u64;
    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|error| ThemeError(format!("无法读取主题包：{error}")))?;
        let path = entry
            .enclosed_name()
            .ok_or_else(|| ThemeError("主题包包含越界路径".into()))?;
        if entry.is_dir() {
            continue;
        }
        if entry
            .unix_mode()
            .is_some_and(|mode| mode & 0o170000 == 0o120000)
        {
            return Err(ThemeError("主题包不能包含符号链接".into()));
        }
        let path = normalize_package_path(&path)?;
        validate_package_file(&path, entry.size())?;
        extracted_size = extracted_size
            .checked_add(entry.size())
            .ok_or_else(|| ThemeError("主题包解压大小溢出".into()))?;
        if extracted_size > MAX_EXTRACTED_SIZE {
            return Err(ThemeError("主题包解压后超过 60 MB".into()));
        }
        let declared_size = entry.size();
        let mut bytes = Vec::with_capacity(declared_size as usize);
        (&mut entry)
            .take(declared_size + 1)
            .read_to_end(&mut bytes)
            .map_err(|error| ThemeError(format!("无法解压 {path}：{error}")))?;
        if bytes.len() as u64 != declared_size {
            return Err(ThemeError(format!("主题文件长度不一致：{path}")));
        }
        if files.insert(path.clone(), bytes).is_some() {
            return Err(ThemeError(format!("主题包包含重复文件：{path}")));
        }
    }
    if !files.contains_key("manifest.json") {
        return Err(ThemeError("主题包缺少 manifest.json".into()));
    }
    if require_release_files {
        for required in ["integrity.json", "signature.ed25519"] {
            if !files.contains_key(required) {
                return Err(ThemeError(format!("主题包缺少 {required}")));
            }
        }
    }
    Ok(files)
}

pub fn read_development_directory(root: &Path) -> Result<PackageFiles, ThemeError> {
    let metadata = fs::symlink_metadata(root)
        .map_err(|error| ThemeError(format!("无法读取本地主题目录：{error}")))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(ThemeError("请选择包含 manifest.json 的主题目录".into()));
    }
    let mut files = HashMap::new();
    let mut total_size = 0_u64;
    read_development_entries(root, root, 0, &mut files, &mut total_size)?;
    if !files.contains_key("manifest.json") {
        return Err(ThemeError("本地主题目录缺少 manifest.json".into()));
    }
    Ok(files)
}

fn read_development_entries(
    root: &Path,
    directory: &Path,
    depth: usize,
    files: &mut PackageFiles,
    total_size: &mut u64,
) -> Result<(), ThemeError> {
    if depth > 16 {
        return Err(ThemeError("本地主题目录层级超过 16 层".into()));
    }
    for entry in fs::read_dir(directory)
        .map_err(|error| ThemeError(format!("无法读取本地主题目录：{error}")))?
    {
        let entry = entry.map_err(|error| ThemeError(format!("无法读取本地主题文件：{error}")))?;
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| ThemeError(format!("无法读取本地主题文件：{error}")))?;
        if metadata.file_type().is_symlink() {
            return Err(ThemeError("本地主题目录不能包含符号链接".into()));
        }
        if metadata.is_dir() {
            read_development_entries(root, &path, depth + 1, files, total_size)?;
            continue;
        }
        if !metadata.is_file() {
            return Err(ThemeError("本地主题目录包含不支持的文件类型".into()));
        }
        if path.file_name().and_then(|name| name.to_str()) == Some(".DS_Store") {
            continue;
        }
        if files.len() >= MAX_FILE_COUNT {
            return Err(ThemeError("本地主题文件数量超过 256".into()));
        }
        let package_path = normalize_package_path(
            path.strip_prefix(root)
                .map_err(|_| ThemeError("本地主题文件越过所选目录".into()))?,
        )?;
        validate_package_file(&package_path, metadata.len())?;
        *total_size = total_size
            .checked_add(metadata.len())
            .ok_or_else(|| ThemeError("本地主题大小溢出".into()))?;
        if *total_size > MAX_EXTRACTED_SIZE {
            return Err(ThemeError("本地主题总大小超过 60 MB".into()));
        }
        let source = fs::read(&path)
            .map_err(|error| ThemeError(format!("无法读取本地主题文件 {package_path}：{error}")))?;
        if files.insert(package_path.clone(), source).is_some() {
            return Err(ThemeError(format!("本地主题包含重复文件：{package_path}")));
        }
    }
    Ok(())
}

pub fn validate_css(source: &str, theme_id: &str) -> Result<(), ThemeError> {
    let normalized = source.to_ascii_lowercase();
    if normalized.contains("--ct-font") {
        return Err(ThemeError("主题必须使用 ChatGPT 默认字体".into()));
    }
    for forbidden in [
        "@import",
        "url(",
        "http://",
        "https://",
        "javascript:",
        "expression(",
        "</style",
        "data-app-",
    ] {
        if normalized.contains(forbidden) {
            return Err(ThemeError(format!("主题 CSS 包含禁止内容：{forbidden}")));
        }
    }
    let stylesheet = StyleSheet::parse(source, ParserOptions::default())
        .map_err(|error| ThemeError(format!("主题 CSS 语法无效：{error}")))?;
    validate_css_rules(&stylesheet.rules, theme_id)
}

fn validate_css_rules(rules: &CssRuleList<'_>, theme_id: &str) -> Result<(), ThemeError> {
    for rule in &rules.0 {
        match rule {
            CssRule::Style(style) => {
                for selector in &style.selectors.0 {
                    let selector = selector
                        .to_css_string(lightningcss::printer::PrinterOptions::default())
                        .map_err(|error| ThemeError(format!("主题选择器无效：{error}")))?;
                    validate_selector(&selector, theme_id)?;
                }
                if style
                    .declarations
                    .declarations
                    .iter()
                    .chain(style.declarations.important_declarations.iter())
                    .any(|property| match property.property_id() {
                        PropertyId::Font | PropertyId::FontFamily => true,
                        PropertyId::Custom(name) => name.as_ref().contains("font"),
                        _ => false,
                    })
                {
                    return Err(ThemeError("主题必须使用 ChatGPT 默认字体".into()));
                }
                if !style.rules.0.is_empty() {
                    return Err(ThemeError("主题 CSS 暂不允许嵌套规则".into()));
                }
            }
            CssRule::Media(media) => validate_css_rules(&media.rules, theme_id)?,
            CssRule::Keyframes(keyframes) => {
                let name = keyframes
                    .name
                    .to_css_string(lightningcss::printer::PrinterOptions::default())
                    .map_err(|error| ThemeError(format!("主题动画名称无效：{error}")))?;
                if !name.starts_with("ct-") {
                    return Err(ThemeError("主题动画名称必须使用 ct- 前缀".into()));
                }
                if keyframes.keyframes.iter().any(|keyframe| {
                    keyframe
                        .declarations
                        .declarations
                        .iter()
                        .chain(&keyframe.declarations.important_declarations)
                        .any(|property| match property.property_id() {
                            PropertyId::Font | PropertyId::FontFamily => true,
                            PropertyId::Custom(name) => name.as_ref().contains("font"),
                            _ => false,
                        })
                }) {
                    return Err(ThemeError("主题必须使用 ChatGPT 默认字体".into()));
                }
            }
            CssRule::Ignored => {}
            _ => {
                return Err(ThemeError(
                    "主题 CSS 仅允许普通规则、@media 与 @keyframes".into(),
                ));
            }
        }
    }
    Ok(())
}

fn validate_selector(selector: &str, theme_id: &str) -> Result<(), ThemeError> {
    let expected_root = format!(r#":root[data-ct-theme="{theme_id}"]"#);
    let Some(scoped_selector) = selector.strip_prefix(&expected_root) else {
        return Err(ThemeError(format!(
            "主题选择器未限定到自身作用域：{selector}"
        )));
    };
    if scoped_selector.contains('#') {
        return Err(ThemeError(format!("主题选择器禁止使用 ID：{selector}")));
    }
    let selector_without_attributes = strip_selector_attributes(scoped_selector);
    validate_selector_attributes(selector, theme_id)?;
    for class in selector_without_attributes.split('.').skip(1).map(|part| {
        part.split(|character: char| {
            !character.is_ascii_alphanumeric() && character != '-' && character != '_'
        })
        .next()
        .unwrap_or("")
    }) {
        if !matches!(
            class,
            "ct-home-hero__copy"
                | "ct-home-hero__eyebrow"
                | "ct-home-hero__title"
                | "ct-home-hero__description"
                | "ct-home-hero__image"
                | "ct-decoration"
        ) {
            return Err(ThemeError(format!("主题选择器使用了非平台类名：.{class}")));
        }
    }
    Ok(())
}

fn validate_selector_attributes(selector: &str, theme_id: &str) -> Result<(), ThemeError> {
    let mut remaining = selector;
    while let Some(start) = remaining.find('[') {
        let after_start = &remaining[start + 1..];
        let end = after_start
            .find(']')
            .ok_or_else(|| ThemeError("主题选择器属性未闭合".into()))?;
        let attribute = &after_start[..end];
        let valid = attribute == format!(r#"data-ct-theme="{theme_id}""#)
            || parse_attribute_value(attribute, "data-ct-color-scheme")
                .is_some_and(|value| matches!(value, "light" | "dark"))
            || parse_attribute_value(attribute, "data-ct-view").is_some_and(|value| {
                matches!(value, "home" | "home-compact" | "conversation" | "other")
            })
            || parse_attribute_value(attribute, "data-ct-slot")
                .is_some_and(|value| ALLOWED_SLOTS.contains(&value))
            || parse_attribute_value(attribute, "data-ct-mount")
                .is_some_and(|value| ALLOWED_MOUNTS.contains(&value))
            || matches!(attribute, r#"role="button""# | r#"role="switch""#);
        if !valid {
            return Err(ThemeError(format!(
                "主题选择器属性不在白名单：[{attribute}]"
            )));
        }
        remaining = &after_start[end + 1..];
    }
    Ok(())
}

fn parse_attribute_value<'a>(attribute: &'a str, name: &str) -> Option<&'a str> {
    attribute
        .strip_prefix(name)?
        .strip_prefix("=\"")?
        .strip_suffix('"')
}

fn strip_selector_attributes(selector: &str) -> String {
    let mut result = String::with_capacity(selector.len());
    let mut depth = 0_u32;
    for character in selector.chars() {
        match character {
            '[' => depth += 1,
            ']' => depth = depth.saturating_sub(1),
            _ if depth == 0 => result.push(character),
            _ => {}
        }
    }
    result
}

fn validate_text(label: &str, value: &str, max_length: usize) -> Result<(), ThemeError> {
    if value.trim().is_empty() || value.chars().count() > max_length {
        return Err(ThemeError(format!("{label} 必须为 1–{max_length} 个字符")));
    }
    Ok(())
}

fn validate_optional_text(
    label: &str,
    value: Option<&str>,
    max_length: usize,
) -> Result<(), ThemeError> {
    if let Some(value) = value {
        validate_text(label, value, max_length)?;
    }
    Ok(())
}

fn validate_style_path(path: &str) -> Result<(), ThemeError> {
    if !is_safe_relative_path(path)
        || !path.starts_with("styles/")
        || extension(path) != Some("css")
    {
        return Err(ThemeError(format!("主题样式路径无效：{path}")));
    }
    Ok(())
}

fn validate_asset_path(path: &str) -> Result<(), ThemeError> {
    if !is_safe_relative_path(path)
        || !path.starts_with("assets/")
        || !matches!(
            extension(path),
            Some("svg" | "png" | "jpg" | "jpeg" | "webp")
        )
    {
        return Err(ThemeError(format!("主题资源路径无效：{path}")));
    }
    Ok(())
}

pub fn validate_image(path: &str, source: &[u8]) -> Result<&'static str, ThemeError> {
    match extension(path) {
        Some("svg") => {
            let source = std::str::from_utf8(source)
                .map_err(|error| ThemeError(format!("SVG 不是 UTF-8：{error}")))?;
            validate_svg(source)?;
            Ok("image/svg+xml")
        }
        Some("png") if source.starts_with(b"\x89PNG\r\n\x1a\n") => Ok("image/png"),
        Some("jpg" | "jpeg") if source.starts_with(b"\xff\xd8\xff") => Ok("image/jpeg"),
        Some("webp")
            if source.len() >= 12 && &source[..4] == b"RIFF" && &source[8..12] == b"WEBP" =>
        {
            Ok("image/webp")
        }
        _ => Err(ThemeError(format!("图片内容与扩展名不匹配：{path}"))),
    }
}

pub fn validate_svg(source: &str) -> Result<(), ThemeError> {
    let normalized = source.to_ascii_lowercase();
    if !normalized.contains("<svg") {
        return Err(ThemeError("SVG 资源缺少根节点".into()));
    }
    let content = normalized.replace("http://www.w3.org/2000/svg", "");
    for forbidden in [
        "<script",
        "<foreignobject",
        "javascript:",
        "http://",
        "https://",
        "onload=",
        "onclick=",
    ] {
        if content.contains(forbidden) {
            return Err(ThemeError(format!("SVG 资源包含禁止内容：{forbidden}")));
        }
    }
    Ok(())
}

fn validate_package_file(path: &str, size: u64) -> Result<(), ThemeError> {
    let allowed = matches!(
        path,
        "manifest.json" | "compatibility.json" | "integrity.json" | "signature.ed25519"
    ) || path.starts_with("styles/") && extension(path) == Some("css")
        || path.starts_with("assets/")
            && matches!(
                extension(path),
                Some("svg" | "png" | "jpg" | "jpeg" | "webp")
            );
    if !allowed {
        return Err(ThemeError(format!("主题包包含不允许的文件：{path}")));
    }
    let limit = if path.starts_with("assets/") {
        MAX_IMAGE_SIZE
    } else {
        1024 * 1024
    };
    if size > limit {
        return Err(ThemeError(format!("主题文件超过大小限制：{path}")));
    }
    Ok(())
}

fn normalize_package_path(path: &Path) -> Result<String, ThemeError> {
    if path
        .components()
        .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(ThemeError("主题包路径必须是规范相对路径".into()));
    }
    let value = path
        .to_str()
        .ok_or_else(|| ThemeError("主题包路径必须是 UTF-8".into()))?
        .replace('\\', "/");
    if !is_safe_relative_path(&value) {
        return Err(ThemeError(format!("主题包路径无效：{value}")));
    }
    Ok(value)
}

fn is_safe_relative_path(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && !path.contains('\\')
        && !path.contains(':')
        && path
            .split('/')
            .all(|component| !component.is_empty() && component != "." && component != "..")
}

fn utf8_file<'a>(files: &'a PackageFiles, path: &str) -> Result<&'a str, ThemeError> {
    let source = files
        .get(path)
        .ok_or_else(|| ThemeError(format!("主题缺少文件：{path}")))?;
    std::str::from_utf8(source).map_err(|error| ThemeError(format!("{path} 不是 UTF-8：{error}")))
}

fn extension(path: &str) -> Option<&str> {
    path.rsplit_once('.').map(|(_, extension)| extension)
}

pub fn is_valid_theme_id(value: &str) -> bool {
    value.contains('.') && is_valid_author_id(value)
}

pub fn is_valid_author_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-')
        })
}

fn is_valid_locale(value: &str) -> bool {
    let mut parts = value.split('-');
    let language = parts.next().unwrap_or_default();
    (2..=3).contains(&language.len())
        && language.bytes().all(|byte| byte.is_ascii_lowercase())
        && parts.all(|part| {
            (part.len() == 2 && part.bytes().all(|byte| byte.is_ascii_uppercase()))
                || (part.len() == 4
                    && part.as_bytes()[0].is_ascii_uppercase()
                    && part.as_bytes()[1..]
                        .iter()
                        .all(|byte| byte.is_ascii_lowercase()))
        })
}

fn is_hex_color(value: &str) -> bool {
    value.len() == 7
        && value.starts_with('#')
        && value[1..].bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_lists_preserve_nested_function_commas() {
        let source = r#":root[data-ct-theme="studio.example.theme"] :is([data-ct-slot="home.card"], [data-ct-slot="settings.card"]) { color: white; }"#;
        validate_css(source, "studio.example.theme").expect("nested selector commas");
    }

    #[test]
    fn rejects_unscoped_css_and_fonts() {
        assert!(validate_css("body { color: red; }", "studio.example.theme").is_err());
        assert!(
            validate_css(
                r#":root[data-ct-theme="studio.example.theme"] { font-family: serif; }"#,
                "studio.example.theme",
            )
            .is_err()
        );
    }
}
