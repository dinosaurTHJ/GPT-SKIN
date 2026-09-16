---
name: retheme-theme-development
description: Create, update, validate, and review ReTheme v1 themes for the ChatGPT desktop app. Use when working with ReTheme manifest.json files, stable data-ct-slot CSS, light/dark palettes, zh-CN/en theme copy, home Hero or compact conversation Banner assets, local theme directories, community source ZIPs, or when deciding whether a visual defect belongs to a theme or the ReTheme engine.
---

# ReTheme Theme Development

Build themes as declarative JSON, scoped CSS, and local images. Keep ChatGPT structure and behavior under engine ownership.

## Required reading

Read these references before editing:

1. Read [`references/protocol.md`](references/protocol.md) for package, Manifest, CSS, asset, localization, and security rules.
2. Read [`references/slots.md`](references/slots.md) before choosing or changing slots.
3. Read [`references/qa.md`](references/qa.md) before debugging or declaring completion.
4. Read [`references/banner-generation.md`](references/banner-generation.md) before generating or editing a Hero, compact Banner, or transparent foreground.

Do not infer fields or slots from memory. Treat the bundled validator as the protocol authority.

## Workflow

### 1. Establish inputs

Obtain or determine:

- Globally unique lowercase reverse-domain theme ID.
- Theme SemVer and real author identity.
- Base language plus complete `zh-CN` and `en` copy.
- Visual concept, palette, references, prohibited styles, and asset rights.
- Required coverage: home, compact home, conversation, composer, sidebar, settings, and menu.
- Target macOS/Windows and narrow/medium/wide/maximized test states.

Ask only for missing inputs that change protocol, identity, rights, or visual direction. Never invent author identity or asset ownership.

### 2. Start from the template

Copy [`assets/theme-template`](assets/theme-template) to a new directory. Keep `manifest.json` at the directory root.

Replace every example ID in Manifest and CSS. Do not copy layout workarounds or internal selectors from an existing complex theme.

### 3. Plan semantic tokens

Define light and dark values before component CSS:

- App, main, sidebar, page, and settings backgrounds.
- Surface, hover, active, selected, and disabled states.
- Primary, secondary, muted, icon, link, and focus colors.
- Border, strong border, separator, radius, and shadow.
- Composer, card, menu, settings, code, diff, and terminal surfaces.

Use a theme-owned prefix. Never set a font family or redefine private engine variables.

### 4. Plan assets

For every image, record filename, format, purpose, Manifest field or controlled asset slot, transparency, source dimensions, and maximum rendered size.

Do not reference images from CSS. Put them under `assets/` and declare them in Manifest. Do not bake required copy into images.

### 5. Write Manifest

- Use only fields defined by the v1 protocol.
- Use optional `lightAsset` and `darkAsset` only on controlled `experience.assets` entries when an image needs explicit appearance variants; keep `asset` as the fallback.
- Keep `styles` and `slots` unique.
- Declare every used stable slot.
- Also declare every `experience.assets[].slot` in top-level `slots`.
- Keep `permissions` equal to `[]`.
- Provide complete base copy and the other supported language under `locales`.
- Omit platform-managed `access`, `integrity`, and `signature` from source themes.

Use valid JSON without comments. Keep explanations outside the package.

### 6. Write CSS

- Put semantic variables and appearance variants in `styles/tokens.css`.
- Order `styles/overrides.css` by app, sidebar, main/page, home, conversation, composer, settings, menu, content, and motion.
- Start every selector-list entry with the exact theme root.
- Prefer the most specific stable slot and explicit state slots.
- Change visuals only. Do not change structural display, positioning, scrolling, DOM order, composer height, editor padding, or responsive alignment.
- Keep generated decoration non-interactive with `pointer-events: none`.
- Prefix keyframes with `ct-` and add reduced-motion rules.

Never use ChatGPT internal classes, localized text selectors, arbitrary attributes, IDs, external URLs, `url()`, imports, scripts, fonts, or Emoji as visual assets.

### 7. Validate continuously

Run the validator bundled in this installed Skill:

```bash
node /absolute/path/to/retheme-theme-development/scripts/validate-theme.mjs /absolute/path/to/theme
```

Resolve the script relative to this `SKILL.md`, so the command works with custom `CODEX_HOME` paths and on Windows. Treat every validator error as blocking. Do not look for a ReTheme source checkout, require Cargo, or weaken the validator to accept a theme workaround.

### 8. Test in ChatGPT

Complete the matrix in [`references/qa.md`](references/qa.md). At minimum, test:

- `zh-CN` and `en` in light and dark.
- Home, drafted compact home, new conversation, historical conversation, settings, and open menus.
- Empty, single-line, multiline, attachment, permission, and expanded composer states.
- Narrow, medium, wide, maximized, and open workspace-panel layouts.
- macOS and Windows.
- Reduced motion, theme switching, timeout, and full restoration.

### 9. Diagnose ownership

Treat a defect as a theme issue when it affects one theme or comes from its colors, sizes, borders, opacity, animation, assets, or structural CSS.

Treat it as an engine/compatibility issue when multiple themes lose the same slot, a mount enters the wrong parent, navigation removes mounts, restoration leaves nodes behind, or a ChatGPT version changes the native DOM mapping.

Reproduce engine defects with the minimal template and another known-good theme. Never patch an internal ChatGPT selector into a theme.

### 10. Prepare community source

- Remove unused assets and operating-system metadata.
- Increment SemVer.
- Ensure the ZIP root directly contains `manifest.json`.
- Validate both the directory and final ZIP.
- Upload the source ZIP, not `.ctheme`.
- Report the QA matrix, ChatGPT versions tested, and known protocol limitations.

The platform owns content review, package normalization, integrity indexing, Ed25519 signing, and `.ctheme` generation.

## Completion standard

Do not report completion because the theme merely loads. Complete only when validation passes, both languages and appearances pass, layouts remain native, interactions work, scrolling does not rebound, Hover does not flicker, and restoration leaves no theme styles or mounts.
