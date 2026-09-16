# ReTheme v1 Protocol Reference

## Contents

1. Package limits
2. Manifest fields
3. Experience resources
4. CSS boundary
5. Appearance and localization
6. Publication boundary

## 1. Package limits

```text
theme/
├── manifest.json
├── styles/*.css
└── assets/*.{svg,png,jpg,jpeg,webp}
```

- Source ZIP root contains `manifest.json` directly.
- Use canonical UTF-8 relative paths and `/` separators.
- Reject absolute paths, empty components, `.`, `..`, backslashes, drive letters, symlinks, and duplicate ZIP entries.
- Protocol limits: 30 MiB ZIP, 60 MiB extracted, 256 files, 8 MiB per image, 1 MiB per other file.
- Local directories follow the same file allowlist and safety checks; `.DS_Store` is ignored.

## 2. Manifest fields

Manifest parsing is strict at every object level. Unknown or misspelled fields fail.

| Field | Contract |
|:--|:--|
| `schemaVersion` | Required integer `1` |
| `id` | Required lowercase reverse-domain ID; `a-z`, `0-9`, `.`, `-`; max 128; contains `.` |
| `name` | Required nonblank string, max 120 characters |
| `description` | Required nonblank string, max 240 characters |
| `version` | Required strict SemVer |
| `author.id` | Required lowercase platform/author ID, max 128 |
| `author.name` | Required nonblank string, max 100 characters |
| `testedCodexVersions` | Optional unique strings; test record only, never a compatibility lock |
| `styles` | Required nonempty unique `styles/*.css` paths |
| `slots` | Required unique entries from the stable allowlist |
| `permissions` | Required exact empty array |
| `preview` | Required `background`, `surface`, `accent` as `#RRGGBB` |
| `experience` | Required controlled copy and resources |
| `locales` | Optional valid locale tags; maximum 20 entries |
| `access`, `integrity`, `signature` | Platform fields; omit from source themes |

`supportedCodexVersions` is a legacy read alias. Do not write it in new themes.

## 3. Experience resources

### Required `homeHero`

```json
{
  "eyebrow": "RETHEME",
  "title": "Theme title",
  "description": "Theme description",
  "asset": "assets/hero.webp",
  "fit": "cover",
  "position": "center",
  "foreground": "assets/foreground.webp",
  "divider": {
    "label": "Optional label",
    "asset": "assets/divider.svg"
  }
}
```

`eyebrow` max 80, `title` max 120, `description` max 240. `fit` is `cover` or `contain`. `position` is `center`, `top`, `bottom`, `left`, or `right`. `foreground` and `divider` are optional.

### Optional copy and Banner

- `homePrompt`: `{ "title": "..." }`, max 120.
- `conversationBanner`: Hero fields without `divider`; all three copy strings may be empty in the runtime structure but new themes should provide useful localized copy.

### Dedicated asset properties

- `composerSubmit` → `composer.submit.decoration`.
- `composerDecoration` → `composer.decoration`.
- `conversationSummaryDecoration` → `conversation.summary.decoration`.
- `sidebarSectionDecoration` → `sidebar.section.decoration`.

Each is `{ "asset": "assets/file.ext" }`.

### `experience.assets`

Each entry requires `{ "slot", "asset" }`. It may additionally provide `lightAsset` and `darkAsset`; the engine selects the current ChatGPT appearance and falls back to `asset` when that variant is absent.

Allowed slots:

`app.background`, `main.background`, `main.overlay`, `main.frame`, `sidebar.brand.icon`, `sidebar.brand.badge`, `sidebar.header.background`, `sidebar.header.decoration`, `sidebar.frame`, `home.card.background`, `home.card.arrow.asset`.

Each slot appears once and must also appear in top-level `slots`.

### `experience.decorations`

Optional, defaults to `[]`. Allows `decoration.top-right` and `decoration.bottom-right` only.

## 4. CSS boundary

Every selector-list member starts with the exact root:

```css
:root[data-ct-theme="studio.example.theme"] [data-ct-slot="home.card"],
:root[data-ct-theme="studio.example.theme"] [data-ct-slot="settings.card"] {
  border-color: var(--theme-border) !important;
}
```

Allowed attributes:

- `data-ct-theme="exact-theme-id"`
- `data-ct-color-scheme="light|dark"`
- `data-ct-view="home|home-compact|conversation|other"`
- Documented `data-ct-slot` and `data-ct-mount` values
- `role="button"` and `role="switch"`

Allowed platform classes are limited to `ct-home-hero__copy`, `ct-home-hero__eyebrow`, `ct-home-hero__title`, `ct-home-hero__description`, `ct-home-hero__image`, and `ct-decoration`.

Allowed rules: ordinary style rules, `@media`, and `@keyframes`. Keyframe names start with `ct-`.

Rejected:

- IDs, arbitrary attributes, ChatGPT classes, text matching, DOM-depth assumptions.
- CSS nesting, `@import`, every `url()`, remote/data URLs, `javascript:`, `expression()`, `</style`.
- `font`, `font-family`, font custom properties, and `--ct-font*`.
- Structural positioning, layout, scrolling, composer height/editor padding, and hidden business nodes.

Images enter only through Manifest. SVG rejects scripts, `foreignObject`, event handlers, external links, and JavaScript links.

## 5. Appearance and localization

Use theme-prefixed semantic tokens. Provide explicit light and dark values keyed by `data-ct-color-scheme`; never infer ChatGPT appearance with `prefers-color-scheme`.

Use the base Manifest for one complete language and `locales.en` or `locales.zh-CN` for the other. Localizable values are theme name/description, Hero copy/divider label, home prompt title, and conversation Banner copy. Assets do not change by locale in v1.

Use `@media (prefers-reduced-motion: reduce)` to remove decorative animation. Keep decoration non-interactive.

## 6. Publication boundary

Local source directories require no signature or encryption. Community submissions are source ZIPs. The platform revalidates protocol, reviews content and rights, renders a cover, adds online access metadata and integrity files, signs with Ed25519, and produces `.ctheme`.

Do not include private keys, platform access fields, compatibility selectors, or `.ctheme` output in theme source.
