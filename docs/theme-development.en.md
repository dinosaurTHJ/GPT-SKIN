# ReTheme Theme Development Specification v1

This specification is the shared contract for the ReTheme desktop runtime, community review, and AI-assisted theme creation. Use the runnable [`theme-example/package`](theme-example/package), the complete [`slot catalog`](theme-slots.md), the [`Banner and image-generation guide`](theme-banner-assets.md), the [`AI workflow`](theme-ai-workflow.md), and the machine-readable [`JSON Schema`](theme.schema.json) together.

## 1. Boundary

A ReTheme theme is a declarative package containing JSON, CSS, and local static images. The engine detects the active ChatGPT version, maps native nodes to stable slots, mounts controlled images, mirrors appearance and locale state, and restores the original interface when the theme stops.

A theme may change colors, borders, radii, shadows, opacity, backgrounds, icon colors, text hierarchy, and non-interactive decoration. It may provide a home Hero, compact conversation Banner, controlled assets, and localized copy.

A theme must not:

- Execute JavaScript or alter events, ARIA semantics, business state, or native DOM order.
- Depend on minified ChatGPT class names, text content, DOM depth, or version-specific attributes.
- Override the font family. Always inherit ChatGPT's native font.
- reposition structural areas with fixed/absolute positioning or layout offsets.
- hide essential buttons, fields, messages, focus indicators, or accessible labels.

## 2. Runtime states

The root receives stable state attributes:

```css
:root[data-ct-theme="studio.example.my-theme"]
:root[data-ct-color-scheme="light"]
:root[data-ct-color-scheme="dark"]
:root[data-ct-view="home"]
:root[data-ct-view="home-compact"]
:root[data-ct-view="conversation"]
:root[data-ct-view="other"]
```

| View | Meaning | Required checks |
|:--|:--|:--|
| `home` | Empty home screen | Full Hero, prompt, suggestion cards, composer |
| `home-compact` | Draft text or an open workspace panel | Engine hides full Hero/cards and may show the compact Banner |
| `conversation` | Started or historical conversation | Compact Banner sits above the message viewport; native composer position remains intact |
| `other` | Settings and other pages | Page, settings, and menu slots appear only when their DOM exists |

The engine owns these attributes. Theme CSS must never set or simulate them.

## 3. Package layout

```text
my-theme/
├── manifest.json
├── styles/
│   ├── tokens.css
│   └── overrides.css
└── assets/
    ├── hero.webp
    ├── hero-foreground.webp
    └── conversation.webp
```

- Local development selects the directory containing `manifest.json`.
- A community source ZIP must place `manifest.json` directly at its root.
- CSS belongs under `styles/`; images belong under `assets/`.
- Paths use UTF-8, `/`, and canonical relative components. Absolute paths, `..`, backslashes, drive letters, and symlinks are rejected.
- A development directory is not a `.ctheme`. Only the platform creates signed `.ctheme` releases.

## 4. Deterministic workflow and validator

1. Copy [`theme-example/package`](theme-example/package) and read [`manifest.annotated.jsonc`](theme-example/manifest.annotated.jsonc).
2. Replace the ID, name, version, author, copy, and preview colors.
3. Define light and dark semantic tokens before writing slot overrides.
4. Load the directory through “Load local theme”.
5. Test English/Chinese, light/dark, home/compact/conversation/settings/menu, macOS/Windows, and narrow/medium/wide/maximized windows.
6. Remove structural CSS, overflow, flicker, poor contrast, and interactive overlays.
7. Bump SemVer, ZIP the directory contents, and submit the source ZIP.

Validate a directory or ZIP without cloning ReTheme or installing Rust:

```bash
pnpm dlx @duxweb/retheme-theme-skill validate /absolute/path/to/my-theme
pnpm dlx @duxweb/retheme-theme-skill validate /absolute/path/to/my-theme.zip
```

The npm package invokes the compiled Rust validator for the current platform and writes one JSON report to stdout. Exit code `0` means valid, `1` means a protocol failure, and `2` means invalid CLI usage. Desktop, server, and authoring tools share the same protocol implementation; it is not WASM and is not duplicated in JavaScript or PHP.

## 5. Strict Manifest

Unknown fields, misspelled fields, duplicate `styles`, and duplicate `slots` fail validation.

| Field | Required | Contract |
|:--|:--:|:--|
| `schemaVersion` | yes | Exactly `1` |
| `id` | yes | Global lowercase reverse-domain ID; max 128; immutable after publication |
| `name` | yes | 1–120 characters in the base language |
| `description` | yes | 1–240 characters in the base language |
| `version` | yes | Strict SemVer |
| `author` | yes | `{ id, name }`; name is 1–100 characters |
| `testedCodexVersions` | no | Versions actually tested; never a runtime lock |
| `styles` | yes | At least one unique package-relative CSS path |
| `slots` | yes | Unique entries from the 164-slot allowlist |
| `permissions` | yes | Exactly `[]` in v1 |
| `preview` | yes | `background`, `surface`, and `accent` as `#RRGGBB` |
| `experience` | yes | Engine-created copy and controlled assets |
| `locales` | no | Locale tag to localized copy |
| `access` | platform | Omit from source ZIPs |
| `integrity` | platform | Omit from source ZIPs |
| `signature` | platform | Omit from source ZIPs |

`supportedCodexVersions` is a legacy read alias. Do not use it in new themes. The target is called ChatGPT; the v1 field name remains `testedCodexVersions` for protocol stability.

`slots` declares the capabilities used by the theme. Controlled `experience.assets` entries must also be declared in `slots`. Current compatibility permits ordinary CSS to hit an undeclared runtime slot, but new themes must list every slot they use so review and migration remain deterministic.

## 6. Experience and controlled resources

### Home Hero

`experience.homeHero` is required:

| Property | Contract |
|:--|:--|
| `eyebrow` | 1–80 characters |
| `title` | 1–120 characters |
| `description` | 1–240 characters |
| `asset` | Package image path |
| `fit` | `cover` or `contain` |
| `position` | `center`, `top`, `bottom`, `left`, or `right` |
| `foreground` | Optional transparent foreground image |
| `divider` | Optional `{ label, asset? }`; label is 1–80 characters |

The Hero is the first flow item in `home.stage`. A theme may define visual height or minimum height, but never detach it from document flow or move cards into it.

### Home prompt and conversation Banner

`homePrompt.title` optionally replaces the native home prompt and restores it when the theme stops.

`conversationBanner` is optional and uses the Hero image/layout fields except `divider`. The engine creates a stable conversation header above the message viewport and aligns it to the conversation content column. The theme owns height, colors, border, copy, and foreground visuals only.

### Dedicated decorations

| Manifest property | Runtime slot | Parent |
|:--|:--|:--|
| `composerSubmit` | `composer.submit.decoration` | Submit button |
| `composerDecoration` | `composer.decoration` | Composer surface |
| `conversationSummaryDecoration` | `conversation.summary.decoration` | Summary/task card |
| `sidebarSectionDecoration` | `sidebar.section.decoration` | Project section |

Each value is `{ "asset": "assets/file.svg" }`. Mounts are non-interactive and must not obscure controls.

### General asset slots

Each `experience.assets` entry uses `{ "slot", "asset" }`. Optional `lightAsset` and `darkAsset` paths provide appearance-specific images; the engine switches them in place and falls back to `asset` when a matching variant is absent.

`experience.assets` accepts only:

`app.background`, `main.background`, `main.overlay`, `main.frame`, `sidebar.brand.icon`, `sidebar.brand.badge`, `sidebar.header.background`, `sidebar.header.decoration`, `sidebar.frame`, `home.card.background`, `home.card.arrow.asset`.

Declare each slot once in both `experience.assets` and top-level `slots`. Every declared path must reference an image inside the package; prefer compressed WebP for complex backgrounds.

`experience.decorations` may be omitted and defaults to `[]`. It accepts `decoration.top-right` and `decoration.bottom-right` only.

## 7. Assets and local serving

| Limit | Value |
|:--|:--|
| Formats | SVG, PNG, JPEG, WebP |
| One image | 8 MiB |
| Other file | 1 MiB |
| Source ZIP | 30 MiB protocol maximum; the community endpoint may impose a lower upload limit |
| Extracted total | 60 MiB |
| File count | 256 |

ReTheme serves assets through a temporary loopback-only `127.0.0.1` service with a session token. Base64 is unnecessary. CSS `url()` is rejected; every image must enter through a controlled Manifest field.

SVG validation rejects scripts, `foreignObject`, event attributes, external URLs, and JavaScript URLs. Use WebP for complex illustrations, transparent WebP/PNG for foregrounds, and SVG for simple static vectors.

Read [`theme-banner-assets.md`](theme-banner-assets.md) for exact home, compact Banner, and transparent foreground dimensions, safe areas, export checks, and Chinese/English generation prompts.

## 8. CSS contract

Every top-level selector begins with the exact theme root:

```css
:root[data-ct-theme="studio.example.my-theme"] [data-ct-slot="sidebar"] {
  background: var(--rt-sidebar-background) !important;
}
```

Every entry in a selector list repeats the root. `:is()` and `:where()` may contain commas; the validator parses CSS with an AST.

Allowed selector attributes are `data-ct-theme`, `data-ct-color-scheme`, `data-ct-view`, `data-ct-slot`, `data-ct-mount`, `role="button"`, and `role="switch"`. Platform classes are limited to the documented `ct-home-hero__*` classes and `ct-decoration`. IDs, ChatGPT internal classes, `data-app-*`, and arbitrary attributes are rejected.

Allowed rule types are ordinary style rules, `@media`, and `@keyframes`. Animation names start with `ct-`. CSS nesting, `@import`, external URLs, every `url()`, `expression()`, `javascript:`, and `</style` are rejected.

Do not use `font`, `font-family`, font custom properties, or `--ct-font*`. Local `font-size`, `font-weight`, `line-height`, and `letter-spacing` remain available when readability requires them.

The engine owns structural display, positioning, scrolling, and responsive alignment for home, conversation, composer, page, and settings containers. If a design requires structural changes, request a stable engine slot instead of adding a selector workaround.

## 9. Tokens and appearance

Keep theme-owned semantic variables under a unique prefix such as `--rt-*`:

```css
:root[data-ct-theme="studio.example.my-theme"] {
  --rt-app-background: #eef4ff;
  --rt-surface: rgba(255, 255, 255, 0.84);
  --rt-text: #18213d;
  --rt-text-muted: #5e6885;
  --rt-accent: #526ee8;
  --rt-border: rgba(82, 110, 232, 0.22);
  --rt-radius-card: 14px;
}

:root[data-ct-theme="studio.example.my-theme"][data-ct-color-scheme="dark"] {
  --rt-app-background: #101521;
  --rt-surface: rgba(28, 36, 55, 0.86);
  --rt-text: #edf2ff;
}
```

Do not redefine engine `--ct-*` variables except the documented Hero/foreground visual variables. Dynamic content width, titlebar safe area, summary width, and card count variables are read-only.

Both light and dark palettes are mandatory. Use `data-ct-color-scheme`, never `prefers-color-scheme`, to follow ChatGPT. Maintain WCAG contrast, visible focus and selection states, and readable translucent surfaces in both modes.

## 10. Localization

Provide one complete base language and override the other language in `locales`. ReTheme currently normalizes its interface locale to `zh-CN` or `en`.

Localizable values include theme name/description; Hero eyebrow/title/description/divider label; home prompt title; and conversation Banner eyebrow/title/description. Assets and layout are not duplicated by locale. Test overflow and wrapping in both languages at narrow widths.

## 11. Motion

Prefer `transform` and `opacity`; avoid layout-changing animation and continuous large-area filters. Decoration layers use `pointer-events: none`. Every animated theme provides reduced motion:

```css
@media (prefers-reduced-motion: reduce) {
  :root[data-ct-theme="studio.example.my-theme"] *,
  :root[data-ct-theme="studio.example.my-theme"] *::before,
  :root[data-ct-theme="studio.example.my-theme"] *::after {
    animation: none !important;
    transition-duration: 0.01ms !important;
  }
}
```

## 12. Required QA matrix

Test all combinations of Chinese/English and light/dark. On macOS and Windows, test narrow, medium, wide, and maximized windows, plus an open workspace panel.

Cover empty home, drafted home, expanded suggestion cards, new and historical conversations, long-message scrolling, summary/task cards, empty/single-line/multiline/attachment composers, sidebar states, every settings navigation section, controls and switches, menus and separators, code, inline code, diff, terminal, reduced motion, theme switching, timeout, and restoration.

Publication blockers include horizontal scroll, a Banner covering messages, composer relocation, unclickable controls, poor contrast, persistent settings flash, missing light/dark or localization coverage, hover flicker, scroll rebound, and leftover mounts after restoration.

## 13. Publish and security boundary

Upload a source ZIP, not `.ctheme`. Remove unused files, verify that the ZIP root contains `manifest.json`, increment SemVer, and omit `access`, `integrity`, and `signature`.

The platform revalidates the same protocol, runs business/content review, renders a cover, normalizes the Manifest, creates `integrity.json`, signs it with Ed25519, and generates the downloadable `.ctheme`. Platform private keys, remote adaptation selectors, and compatibility records never belong in theme source.

## 14. Compatibility ownership

Themes depend only on stable slots. ReTheme selects signed compatibility data by ChatGPT and engine version, so older and newer ChatGPT releases can coexist. `testedCodexVersions` records regression coverage and never locks compatibility.

When a slot disappears after a ChatGPT update, reproduce with another known-good theme. If multiple themes lose the same slot, update compatibility data or the engine. Never copy the new internal ChatGPT selector into a theme.

## 15. AI entry point

Install the complete Skill instead of asking an AI to reconstruct rules from this document:

```bash
pnpm dlx @duxweb/retheme-theme-skill install
```

Restart Codex, then ask it to use `retheme-theme-development` to create, improve, or review a theme. The Skill loads its bundled protocol, slot, QA, Banner, and template references as needed and does not require a ReTheme repository checkout.
