# Graph Report - ChatGPT Skin Studio_final-source  (2026-09-16)

## Corpus Check
- 111 files · ~406,981 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 2326 nodes · 4249 edges · 230 communities (124 shown, 106 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 145 edges (avg confidence: 0.79)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- account.rs
- injector.mjs
- App.tsx
- compatibility.rs
- config-utf8.ps1
- common-windows.ps1
- codex.rs
- lib.rs
- CodexError
- theme.rs
- manifest.json
- lib.rs
- manifest.json
- manifest.json
- theme-windows.ps1
- theme-package-validator.mjs
- theme-package-validator.mjs
- devDependencies
- String
- theme-package-validator.mjs
- safe-css-validator.mjs
- safe-css-validator.mjs
- ReTheme 主题开发规范 v1
- safe-css-validator.mjs
- ThemeError
- tauri.conf.json
- apply-community-theme.ps1
- package.json
- start_theme_package_preview
- api.rs
- start_development_theme_preview
- properties
- Result
- $defs
- asset
- properties
- Rebuild-DreamSkinTrayMenu
- Get-DreamSkinThemePaths
- String
- compilerOptions
- Changelog
- 更新日志
- properties
- ReTheme Theme Development Specification v1
- name
- definitions
- definitions
- definitions
- ReTheme Banner 与视觉资源规范
- styles
- main.rs
- 3. 产出顺序
- README.en.md
- .start
- supportedCodexVersions
- ReTheme v1 稳定插槽目录
- preview
- retheme-theme.mjs
- README.md
- sync-version.mjs
- ReTheme Stable Slot Reference
- Workflow
- theme.schema.json
- access
- experienceLocalization
- Banner and Asset Generation
- permissions
- webviews
- permissions
- webviews
- permissions
- webviews
- divider
- properties
- properties
- properties
- Parser
- decodeAndValidateSafeCss
- Parser
- Invoke-DreamSkinCommunityStartAndVerify
- Parser
- security_config.rs
- CapabilityRemote
- CapabilityRemote
- CapabilityRemote
- ReTheme v1 Protocol Reference
- compilerOptions
- IsolatedProcess
- locales
- id
- stage-local-validator.mjs
- SKILL.md
- ReTheme Theme QA
- Release Guide
- renew_theme_lease
- 3. Experience resources
- default.json
- decodeAndValidateSafeCss
- decodeAndValidateSafeCss
- Security Model
- theme-development.md
- 6. Experience and controlled resources
- Caishen Readable ReTheme source example
- test-package.mjs
- normalize-updater-metadata.mjs
- validate-theme.mjs
- conversationBanner
- local
- homePrompt
- nonEmptyText
- prepare-package.mjs
- write-build-config.mjs
- Capability
- description
- local
- Capability
- description
- local
- Capability
- description
- local
- ReTheme 主题示例
- themeName
- title
- publish-release.node-test.mjs
- verify-macos-release.sh
- verify-published-release.sh
- Number
- PermissionEntry
- Number
- PermissionEntry
- Number
- PermissionEntry
- tsconfig.json
- cleanup-package.mjs
- normalize-updater-metadata.node-test.mjs
- publish-release.sh
- build-release-notes.sh
- extract-release-notes.sh
- release-notes.node-test.mjs
- sync-version.node-test.mjs
- upload-release-asset.sh
- build.rs
- SafeCssValidationError
- main.rs
- install-dream-skin.ps1
- Arc
- BTreeMap
- Client
- Display
- Error
- Formatter
- Mutex
- Option
- Path
- PathBuf
- Result
- Self
- SigningKey
- String
- T
- Value
- Vec
- VerifyingKey
- Result
- String
- Arc
- Display
- Error
- Formatter
- From
- HashMap
- Mutex
- Option
- Path
- PathBuf
- Result
- Self
- String
- T
- Value
- Vec
- Client
- Display
- Error
- Formatter
- From
- Option
- Path
- PathBuf
- Result
- Self
- SigningKey
- String
- Value
- Vec
- VerifyingKey
- Mutex
- Option
- Path
- PathBuf
- Result
- String
- Vec
- String
- Arc
- BTreeMap
- HashMap
- HashSet
- Option
- Path
- PathBuf
- Result
- Self
- String
- Value
- Vec
- VerifyingKey

## God Nodes (most connected - your core abstractions)
1. `ThemeError` - 48 edges
2. `CodexError` - 48 edges
3. `AccountError` - 42 edges
4. `AccountRuntime` - 42 edges
5. `Assert-DreamSkinNoReparseComponents()` - 34 edges
6. `Install-DreamSkinBaseTheme()` - 30 edges
7. `Import-DreamSkinThemeZip()` - 28 edges
8. `$defs` - 27 edges
9. `Read-DreamSkinTheme()` - 22 edges
10. `start_theme_package_preview()` - 22 edges

## Surprising Connections (you probably didn't know these)
- `parse_package_files()` --calls--> `validate_package_files()`  [INFERRED]
  src-tauri/src/theme.rs → crates/theme-protocol/src/lib.rs
- `decorations_default_to_empty_when_omitted()` --calls--> `validate_manifest()`  [INFERRED]
  src-tauri/src/theme.rs → crates/theme-protocol/src/lib.rs
- `rejects_archive_path_traversal()` --calls--> `read_archive()`  [INFERRED]
  src-tauri/src/theme.rs → crates/theme-protocol/src/lib.rs
- `rejects_tampered_package()` --calls--> `read_archive()`  [INFERRED]
  src-tauri/src/theme.rs → crates/theme-protocol/src/lib.rs
- `signed_test_package()` --calls--> `read_archive()`  [INFERRED]
  src-tauri/src/theme.rs → crates/theme-protocol/src/lib.rs

## Import Cycles
- None detected.

## Communities (230 total, 106 thin omitted)

### Community 0 - "account.rs"
Cohesion: 0.06
Nodes (64): AccountDeviceSummary, AccountError, AccountInfo, AccountRuntime, AccountSession, AccountStatus, AccountSync, AccountThemeAuthor (+56 more)

### Community 1 - "injector.mjs"
Cohesion: 0.05
Nodes (75): normalizeThemeColor(), normalizeThemeText(), ascii(), classifyImageDimensions(), jpegDimensions(), pngDimensions(), readImageMetadata(), readRawDimensions() (+67 more)

### Community 2 - "App.tsx"
Cohesion: 0.05
Nodes (53): App(), formatImageSize(), ImageCard(), Page, desktop, Window, createDemoPreview(), DEMO_INSTALLATION (+45 more)

### Community 3 - "compatibility.rs"
Cohesion: 0.12
Nodes (37): adapter(), AdapterSelectors, AdapterStrategies, ApiEnvelope, builtin_adapter(), builtin_adapters(), builtin_latest_adapter(), CachedCompatibility (+29 more)

### Community 4 - "config-utf8.ps1"
Cohesion: 0.11
Nodes (57): Add-DreamSkinDesktopSection(), Archive-DreamSkinConfigBackup(), Assert-DreamSkinDesktopShapeSupported(), Assert-DreamSkinFileUnchanged(), Assert-DreamSkinJsonObjectShape(), Assert-DreamSkinTomlLineEditingSafe(), Compare-DreamSkinAppearanceMarkerSnapshot(), Compare-DreamSkinManagedAppearanceEntry() (+49 more)

### Community 5 - "common-windows.ps1"
Cohesion: 0.07
Nodes (56): Assert-DreamSkinCodexDirectLaunchTarget(), Assert-DreamSkinPort(), Assert-DreamSkinRuntimeTree(), Assert-DreamSkinTrustedNodeImage(), ConvertFrom-DreamSkinUtf8Base64(), ConvertTo-DreamSkinArgumentLine(), ConvertTo-DreamSkinCodexInstall(), ConvertTo-DreamSkinUtcTimestamp() (+48 more)

### Community 6 - "codex.rs"
Cohesion: 0.16
Nodes (21): chatgpt_app_loads_session_asset_urls(), CodexInstallation, detect(), detects_installed_chatgpt_package(), detects_installed_codex(), DevToolsTarget, external_theme_hides_native_main_fade(), find_codex_target() (+13 more)

### Community 7 - "lib.rs"
Cohesion: 0.13
Nodes (41): App, AppHandle, allow_image_library_root(), apply_live_image_theme(), canonical_image_library_root(), canonical_path_is_inside(), clear_live_state(), get_image_library_root() (+33 more)

### Community 8 - "CodexError"
Cohesion: 0.20
Nodes (15): activate_windows_store_app(), activate_windows_store_app_attached(), activate_windows_store_app_with_ownership(), CodexError, connect_theme_channel(), evaluate(), inspect_macos_app(), plist_string() (+7 more)

### Community 9 - "theme.rs"
Cohesion: 0.08
Nodes (34): selector_lists_preserve_nested_function_commas(), validate_css(), accepts_controlled_asset_mount_selectors(), accepts_namespaced_keyframes_and_rejects_global_animation_names(), accepts_only_supported_color_schemes(), accepts_only_supported_runtime_views(), accepts_page_menu_and_detailed_settings_slots(), build_test_archive() (+26 more)

### Community 10 - "manifest.json"
Cohesion: 0.05
Nodes (40): author, id, name, asset, description, eyebrow, fit, position (+32 more)

### Community 11 - "lib.rs"
Cohesion: 0.13
Nodes (24): parse_attribute_value(), referenced_asset_paths(), strip_selector_attributes(), ThemeAsset, ThemeAuthor, ThemeBannerLocalization, ThemeControlledAsset, ThemeConversationBanner (+16 more)

### Community 12 - "manifest.json"
Cohesion: 0.05
Nodes (39): author, id, name, asset, description, eyebrow, fit, position (+31 more)

### Community 13 - "manifest.json"
Cohesion: 0.05
Nodes (39): author, id, name, asset, description, eyebrow, fit, position (+31 more)

### Community 14 - "theme-windows.ps1"
Cohesion: 0.05
Nodes (36): dependencies, react, react-dom, @tauri-apps/api, @tauri-apps/plugin-deep-link, @tauri-apps/plugin-dialog, @tauri-apps/plugin-opener, @tauri-apps/plugin-process (+28 more)

### Community 15 - "theme-package-validator.mjs"
Cohesion: 0.12
Nodes (36): assertExactKeys(), assertObject(), assertString(), assertStringSet(), BACKGROUND_MEDIA, codePointLength(), COLOR_KEYS, compareSemver() (+28 more)

### Community 16 - "theme-package-validator.mjs"
Cohesion: 0.12
Nodes (36): assertExactKeys(), assertObject(), assertString(), assertStringSet(), BACKGROUND_MEDIA, codePointLength(), COLOR_KEYS, compareSemver() (+28 more)

### Community 17 - "devDependencies"
Cohesion: 0.06
Nodes (34): 10. 明暗主题, 11. 双语, 12. 动画, 13. 插槽使用原则, 14. 本地开发、试用与恢复, 15. 验收矩阵, 16. 社区发布, 17. 常见失败 (+26 more)

### Community 18 - "String"
Cohesion: 0.14
Nodes (13): assert_runtime_urls_are_local(), build_runtime_config_with(), collect_runtime_asset_paths(), collect_runtime_assets(), parse_package_files(), replaces_every_runtime_asset_path_with_session_urls(), test_theme_package(), test_theme_summary() (+5 more)

### Community 19 - "theme-package-validator.mjs"
Cohesion: 0.13
Nodes (35): assertExactKeys(), assertObject(), assertString(), assertStringSet(), BACKGROUND_MEDIA, codePointLength(), COLOR_KEYS, compareSemver() (+27 more)

### Community 20 - "safe-css-validator.mjs"
Cohesion: 0.08
Nodes (40): alphaChannel(), backdropFilterValue(), COLOR_PROPERTIES, COLOR_VARIABLES, colorChannel(), colorValue(), compileRuntimeCss(), compileSafeCss() (+32 more)

### Community 21 - "safe-css-validator.mjs"
Cohesion: 0.11
Nodes (33): alphaChannel(), backdropFilterValue(), COLOR_PROPERTIES, COLOR_VARIABLES, colorChannel(), colorValue(), CORE_BACKGROUND_IMAGE_PARTS, fontFamilyValue() (+25 more)

### Community 22 - "ReTheme 主题开发规范 v1"
Cohesion: 0.07
Nodes (28): app, security, windows, enable, scope, build, beforeBuildCommand, beforeDevCommand (+20 more)

### Community 23 - "safe-css-validator.mjs"
Cohesion: 0.12
Nodes (32): alphaChannel(), backdropFilterValue(), COLOR_PROPERTIES, COLOR_VARIABLES, colorChannel(), colorValue(), CORE_BACKGROUND_IMAGE_PARTS, fontFamilyValue() (+24 more)

### Community 24 - "ThemeError"
Cohesion: 0.08
Nodes (25): bin, retheme-theme, retheme-theme-skill, bugs, url, description, engines, node (+17 more)

### Community 25 - "tauri.conf.json"
Cohesion: 0.09
Nodes (23): pattern, type, allOf, maxLength, $defs, assetPath, copyDescription, description (+15 more)

### Community 26 - "apply-community-theme.ps1"
Cohesion: 0.10
Nodes (21): additionalProperties, properties, $ref, required, type, additionalProperties, properties, required (+13 more)

### Community 27 - "package.json"
Cohesion: 0.26
Nodes (8): installs_and_loads_signed_online_theme(), is_sha256(), online_theme_uses_device_encrypted_cache(), sha256_hex(), ThemeInstallReport, ThemeRepository, verify_integrity_and_signature(), write_atomic()

### Community 28 - "start_theme_package_preview"
Cohesion: 0.20
Nodes (14): AtomicU64, IdleInstance, replace_theme_package_preview(), start_theme_package_preview(), start_theme_preview_until(), stop_theme_preview(), stop_theme_preview_if_session(), ThemePreviewReport (+6 more)

### Community 29 - "api.rs"
Cohesion: 0.16
Nodes (17): adds_all_dux_signature_headers(), ApiConfig, base(), config(), is_placeholder_credential(), language(), normalize_language(), set_language() (+9 more)

### Community 30 - "start_development_theme_preview"
Cohesion: 0.21
Nodes (21): applies_and_restores_external_theme(), dispatch_escape(), dispatch_mouse_click(), dispatch_mouse_move(), evaluate_value(), evaluate_value_until(), external_test_theme(), external_theme_loads_all_session_assets() (+13 more)

### Community 31 - "properties"
Cohesion: 0.10
Nodes (20): description, $ref, $ref, const, description, const, $ref, properties (+12 more)

### Community 32 - "Result"
Cohesion: 0.21
Nodes (19): extension(), is_safe_relative_path(), normalize_package_path(), read_archive(), read_development_directory(), read_development_entries(), ThemeError, utf8_file() (+11 more)

### Community 33 - "$defs"
Cohesion: 0.12
Nodes (20): additionalProperties, properties, required, type, conversationBanner, hero, $ref, $ref (+12 more)

### Community 34 - "asset"
Cohesion: 0.11
Nodes (17): compilerOptions, allowJs, allowSyntheticDefaultImports, esModuleInterop, forceConsistentCasingInFileNames, isolatedModules, jsx, lib (+9 more)

### Community 35 - "properties"
Cohesion: 0.12
Nodes (15): Preflight, Publish, Release Guide, Required repository secrets, Verify, 下载, 主题开发, 主题预览 (+7 more)

### Community 36 - "Rebuild-DreamSkinTrayMenu"
Cohesion: 0.12
Nodes (15): [0.1.0] - 2026-07-19, [0.1.1] - 2026-07-19, [0.1.2] - 2026-07-19, [0.1.3] - 2026-07-19, [0.1.4] - 2026-07-20, Added, Added, Added (+7 more)

### Community 37 - "Get-DreamSkinThemePaths"
Cohesion: 0.12
Nodes (15): [0.1.0] - 2026-07-19, [0.1.1] - 2026-07-19, [0.1.2] - 2026-07-19, [0.1.3] - 2026-07-19, [0.1.4] - 2026-07-20, 修复, 修复, 修复 (+7 more)

### Community 38 - "String"
Cohesion: 0.13
Nodes (16): AtomicBool, dropping_test_instance_only_terminates_its_process_group(), expired_development_preview_clears_its_session(), get_theme_asset_response(), process_group_exists(), serve_theme_asset_request(), serves_theme_assets_only_from_the_session_url(), signal_process_group() (+8 more)

### Community 39 - "compilerOptions"
Cohesion: 0.12
Nodes (16): items, type, $ref, $ref, $ref, experience, additionalProperties, properties (+8 more)

### Community 40 - "Changelog"
Cohesion: 0.12
Nodes (16): anyOf, description, definitions, Application, Number, PermissionEntry, Target, Value (+8 more)

### Community 41 - "更新日志"
Cohesion: 0.12
Nodes (16): anyOf, description, definitions, Application, Number, PermissionEntry, Target, Value (+8 more)

### Community 42 - "properties"
Cohesion: 0.12
Nodes (16): anyOf, description, definitions, Application, Number, PermissionEntry, Target, Value (+8 more)

### Community 43 - "ReTheme Theme Development Specification v1"
Cohesion: 0.13
Nodes (15): 10. Localization, 11. Motion, 12. Required QA matrix, 13. Publish and security boundary, 14. Compatibility ownership, 15. AI entry point, 1. Boundary, 2. Runtime states (+7 more)

### Community 44 - "name"
Cohesion: 0.13
Nodes (15): additionalProperties, properties, required, type, author, localization, $ref, additionalProperties (+7 more)

### Community 45 - "definitions"
Cohesion: 0.21
Nodes (14): is_hex_color(), is_valid_author_id(), is_valid_locale(), is_valid_theme_id(), validate_banner_layout(), validate_manifest(), validate_optional_text(), validate_text() (+6 more)

### Community 46 - "definitions"
Cohesion: 0.14
Nodes (14): 1. 通用原则, 2. 规格总表, 3. 首页 Hero 背景, 4. 会话窄 Banner 背景, 5. 透明人物或物品前景, 6. 明暗模式策略, 7. 导出与验收, English prompt template (+6 more)

### Community 47 - "definitions"
Cohesion: 0.14
Nodes (14): default, items, type, $ref, decorations, slots, styles, items (+6 more)

### Community 48 - "ReTheme Banner 与视觉资源规范"
Cohesion: 0.15
Nodes (13): 1. 开始前必须获得的输入, 2. 读取顺序, 3. 产出顺序, 4. AI 禁止行为, 5. 问题归属判断, 6. 最终交付清单, ReTheme AI 主题开发工作流, 阶段 A：视觉 token (+5 more)

### Community 49 - "styles"
Cohesion: 0.18
Nodes (13): divider, heroLocalization, additionalProperties, properties, $ref, required, type, additionalProperties (+5 more)

### Community 50 - "main.rs"
Cohesion: 0.44
Nodes (8): documented_directory_returns_stable_success_report(), execute(), main(), print_report(), protocol_failure_and_usage_have_distinct_exit_codes(), Report, ReportMessage, ExitCode

### Community 51 - "3. 产出顺序"
Cohesion: 0.15
Nodes (11): ReTheme Theme Skill, Berry · Light Journal Style, Build a Theme, Development, Download, Highlights, Links, Manual workflow (+3 more)

### Community 52 - "README.en.md"
Cohesion: 0.15
Nodes (13): description, properties, required, type, Capability, Identifier, description, oneOf (+5 more)

### Community 53 - ".start"
Cohesion: 0.05
Nodes (108): Assert-DreamSkinCommunityActiveBaseline(), Confirm-DreamSkinCommunityApply(), Copy-DreamSkinActiveThemeSnapshot(), Copy-DreamSkinImportedThemeSnapshot(), Ensure-DreamSkinCommunityActiveBaseline(), Format-DreamSkinCommunitySuccessMessage(), Get-DreamSkinCommunityActiveState(), Get-DreamSkinCommunityHttpResponse() (+100 more)

### Community 54 - "supportedCodexVersions"
Cohesion: 0.15
Nodes (13): description, properties, required, type, Capability, Identifier, description, oneOf (+5 more)

### Community 55 - "ReTheme v1 稳定插槽目录"
Cohesion: 0.17
Nodes (12): type, supportedCodexVersions, testedCodexVersions, deprecated, description, items, type, uniqueItems (+4 more)

### Community 56 - "preview"
Cohesion: 0.17
Nodes (12): App 与标题栏, Manifest 资源插槽补充, ReTheme v1 稳定插槽目录, 主区、页面与菜单, 会话、代码与终端, 侧栏, 归属判断, 角落装饰 (+4 more)

### Community 57 - "retheme-theme.mjs"
Cohesion: 0.27
Nodes (8): defaultSkillDirectory(), install(), packageDirectory, packageJson, platformKey(), validate(), validatorPath(), valueAfter()

### Community 58 - "README.md"
Cohesion: 0.18
Nodes (11): $ref, $ref, preview, additionalProperties, properties, required, type, accent (+3 more)

### Community 59 - "sync-version.mjs"
Cohesion: 0.18
Nodes (8): args, check, mismatches, rawVersion, root, rootIndex, updates, version

### Community 60 - "ReTheme Stable Slot Reference"
Cohesion: 0.18
Nodes (11): additionalProperties, properties, type, bannerLocalization, homePrompt, additionalProperties, properties, required (+3 more)

### Community 61 - "Workflow"
Cohesion: 0.18
Nodes (11): App, titlebar, main, page, Composer, Controlled general asset slots, Conversation and content, Corner decoration, Home Hero and cards, Menu, ReTheme Stable Slot Reference (+3 more)

### Community 62 - "theme.schema.json"
Cohesion: 0.18
Nodes (11): 10. Prepare community source, 1. Establish inputs, 2. Start from the template, 3. Plan semantic tokens, 4. Plan assets, 5. Write Manifest, 6. Write CSS, 7. Validate continuously (+3 more)

### Community 63 - "access"
Cohesion: 0.20
Nodes (10): additionalProperties, properties, required, type, access, const, delivery, slug (+2 more)

### Community 64 - "experienceLocalization"
Cohesion: 0.20
Nodes (9): additionalProperties, description, $id, not, required, required, $schema, title (+1 more)

### Community 65 - "Banner and Asset Generation"
Cohesion: 0.20
Nodes (10): $ref, experienceLocalization, additionalProperties, properties, type, $ref, $ref, conversationBanner (+2 more)

### Community 66 - "permissions"
Cohesion: 0.20
Nodes (10): Appearance strategy, Asset QA, Banner and Asset Generation, Compact Banner prompt — Chinese, Compact Banner prompt — English, Home Hero prompt — Chinese, Home Hero prompt — English, Specifications (+2 more)

### Community 67 - "webviews"
Cohesion: 0.20
Nodes (10): properties, type, default, description, type, identifier, local, remote (+2 more)

### Community 68 - "permissions"
Cohesion: 0.20
Nodes (10): $ref, description, items, type, uniqueItems, description, items, type (+2 more)

### Community 69 - "webviews"
Cohesion: 0.20
Nodes (10): type, webviews, windows, items, description, items, type, description (+2 more)

### Community 70 - "permissions"
Cohesion: 0.20
Nodes (10): $ref, description, items, type, uniqueItems, description, items, type (+2 more)

### Community 71 - "webviews"
Cohesion: 0.20
Nodes (10): type, webviews, windows, items, description, items, type, description (+2 more)

### Community 72 - "divider"
Cohesion: 0.20
Nodes (10): $ref, description, items, type, uniqueItems, description, items, type (+2 more)

### Community 73 - "properties"
Cohesion: 0.20
Nodes (10): type, webviews, windows, items, description, items, type, description (+2 more)

### Community 74 - "properties"
Cohesion: 0.25
Nodes (3): Completion standard, Required reading, ReTheme Theme Development

### Community 75 - "properties"
Cohesion: 0.25
Nodes (8): description, properties, required, type, CapabilityRemote, urls, description, type

### Community 76 - "Parser"
Cohesion: 0.25
Nodes (8): description, properties, required, type, CapabilityRemote, urls, description, type

### Community 77 - "decodeAndValidateSafeCss"
Cohesion: 0.28
Nodes (8): compileRuntimeCss(), compileSafeCss(), decodeAndValidateSafeCss(), parseSafeCss(), validateSafeCss(), validationResult(), [fileArg], filePath

### Community 79 - "Invoke-DreamSkinCommunityStartAndVerify"
Cohesion: 0.25
Nodes (8): description, properties, required, type, CapabilityRemote, urls, description, type

### Community 81 - "security_config.rs"
Cohesion: 0.43
Nodes (6): compatibility_public_key(), config(), license_public_key(), package_key(), SecurityConfig, theme_public_key()

### Community 82 - "CapabilityRemote"
Cohesion: 0.25
Nodes (7): compilerOptions, composite, module, moduleResolution, noEmit, skipLibCheck, include

### Community 83 - "CapabilityRemote"
Cohesion: 0.29
Nodes (7): $ref, additionalProperties, default, propertyNames, type, locales, pattern

### Community 84 - "CapabilityRemote"
Cohesion: 0.29
Nodes (7): allOf, description, maxLength, minLength, pattern, type, id

### Community 85 - "ReTheme v1 Protocol Reference"
Cohesion: 0.29
Nodes (7): 1. Package limits, 2. Manifest fields, 4. CSS boundary, 5. Appearance and localization, 6. Publication boundary, Contents, ReTheme v1 Protocol Reference

### Community 86 - "compilerOptions"
Cohesion: 0.33
Nodes (6): apply_theme(), missing_startup_slots(), normalize_theme_locale(), platform_runtime_css(), startup_slots_ready(), ThemeApplication

### Community 87 - "IsolatedProcess"
Cohesion: 0.33
Nodes (6): 3. Experience resources, Dedicated asset properties, `experience.assets`, `experience.decorations`, Optional copy and Banner, Required `homeHero`

### Community 88 - "locales"
Cohesion: 0.33
Nodes (6): Blocking failures, Delivery report, Ownership triage, ReTheme Theme QA, Runtime matrix, Static gate

### Community 89 - "id"
Cohesion: 0.33
Nodes (5): description, identifier, permissions, $schema, windows

### Community 90 - "stage-local-validator.mjs"
Cohesion: 0.29
Nodes (6): build, destination, keys, packageDirectory, repository, source

### Community 91 - "SKILL.md"
Cohesion: 0.40
Nodes (4): Secrets, Security Model, Theme restrictions, Trust boundaries

### Community 93 - "ReTheme Theme QA"
Cohesion: 0.40
Nodes (5): 6. Experience and controlled resources, Dedicated decorations, General asset slots, Home Hero, Home prompt and conversation Banner

### Community 94 - "Release Guide"
Cohesion: 0.40
Nodes (4): Caishen Readable ReTheme source example, Package, Source and privacy, Validate

### Community 95 - "renew_theme_lease"
Cohesion: 0.05
Nodes (27): builds_msix_activation_command_line_with_isolated_profile(), clears_preview_status_after_codex_exits(), completes_isolated_cdp_smoke_test(), completes_isolated_cdp_smoke_test_on_windows(), DevToolsVersion, exited_test_instance(), get_json(), get_json_with_timeout() (+19 more)

### Community 96 - "3. Experience resources"
Cohesion: 0.40
Nodes (4): anyOf, description, $schema, title

### Community 97 - "default.json"
Cohesion: 0.40
Nodes (4): anyOf, description, $schema, title

### Community 98 - "decodeAndValidateSafeCss"
Cohesion: 0.40
Nodes (4): anyOf, description, $schema, title

### Community 99 - "decodeAndValidateSafeCss"
Cohesion: 0.47
Nodes (6): compileRuntimeCss(), compileSafeCss(), decodeAndValidateSafeCss(), parseSafeCss(), validateSafeCss(), validationResult()

### Community 100 - "Security Model"
Cohesion: 0.50
Nodes (4): description, required, type, Capability

### Community 101 - "theme-development.md"
Cohesion: 0.50
Nodes (4): default, description, type, description

### Community 102 - "6. Experience and controlled resources"
Cohesion: 0.50
Nodes (4): default, description, type, description

### Community 103 - "Caishen Readable ReTheme source example"
Cohesion: 0.50
Nodes (4): default, description, type, local

### Community 104 - "test-package.mjs"
Cohesion: 0.40
Nodes (3): packageDirectory, repository, temporary

### Community 105 - "normalize-updater-metadata.mjs"
Cohesion: 0.40
Nodes (4): assetNames, assets, metadata, [metadataPath, assetsPath, repository, tag]

### Community 106 - "validate-theme.mjs"
Cohesion: 0.40
Nodes (4): result, scriptDirectory, themePath, validator

### Community 107 - "conversationBanner"
Cohesion: 0.50
Nodes (4): default, description, type, description

### Community 108 - "local"
Cohesion: 0.50
Nodes (4): default, description, type, local

### Community 110 - "nonEmptyText"
Cohesion: 0.67
Nodes (3): hexColor, pattern, type

### Community 111 - "prepare-package.mjs"
Cohesion: 0.50
Nodes (3): packageDirectory, repository, skill

### Community 113 - "Capability"
Cohesion: 0.67
Nodes (3): themeName, allOf, maxLength

### Community 114 - "description"
Cohesion: 0.67
Nodes (3): title, allOf, maxLength

### Community 115 - "local"
Cohesion: 0.67
Nodes (3): signature, const, description

### Community 119 - "Capability"
Cohesion: 0.67
Nodes (3): Identifier, description, oneOf

## Knowledge Gaps
- **778 isolated node(s):** `ThemePreview`, `schemaVersion`, `id`, `name`, `description` (+773 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **106 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `hexColor` connect `nonEmptyText` to `tauri.conf.json`, `safe-css-validator.mjs`, `safe-css-validator.mjs`, `safe-css-validator.mjs`?**
  _High betweenness centrality (0.045) - this node is a cross-community bridge._
- **Why does `$defs` connect `tauri.conf.json` to `experienceLocalization`, `$defs`, `Banner and Asset Generation`, `README.md`, `compilerOptions`, `name`, `nonEmptyText`, `styles`, `Capability`, `description`, `apply-community-theme.ps1`, `ReTheme Stable Slot Reference`, `access`?**
  _High betweenness centrality (0.039) - this node is a cross-community bridge._
- **Why does `colorValue()` connect `safe-css-validator.mjs` to `nonEmptyText`?**
  _High betweenness centrality (0.031) - this node is a cross-community bridge._
- **Are the 12 inferred relationships involving `Assert-DreamSkinNoReparseComponents()` (e.g. with `Copy-DreamSkinImportedThemeSnapshot()` and `Invoke-DreamSkinCommunityApply()`) actually correct?**
  _`Assert-DreamSkinNoReparseComponents()` has 12 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ThemePreview`, `schemaVersion`, `id` to the rest of the system?**
  _778 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `account.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.05733005733005733 - nodes in this community are weakly interconnected._
- **Should `injector.mjs` be split into smaller, more focused modules?**
  _Cohesion score 0.053728949478749 - nodes in this community are weakly interconnected._