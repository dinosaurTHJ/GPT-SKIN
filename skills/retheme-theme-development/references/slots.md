# ReTheme Stable Slot Reference

Read the canonical repository catalog at `docs/theme-slots.md` when available; it includes node ownership, parent, appearance conditions, recommended changes, and prohibited changes for every slot. This standalone reference preserves the complete v1 allowlist and the rules needed to select slots safely.

## Selection rules

1. Prefer the narrowest semantic slot: color `home.card.icon.glyph`, not every SVG under `home.card`.
2. Prefer explicit state slots such as `sidebar.item.active`, `menu.item.checked`, and `settings.control.checked` over inferred states.
3. Treat absent conditional slots as normal. Never create business DOM or use internal selectors to force them.
4. Change visuals only on native nodes. Engine mounts accept non-interactive decoration only.
5. Keep layout, scroll, responsive alignment, event behavior, and accessibility under engine/native ownership.
6. Declare every slot used by CSS and every controlled asset slot in Manifest.

## App, titlebar, main, page

`app.shell`, `app.background`, `titlebar`, `main`, `main.fade`, `main.content.frame`, `main.background`, `main.overlay`, `main.frame`, `page`, `page.surface`, `page.header`, `page.content`.

- `app.background`, `main.background`, `main.overlay`, and `main.frame` are engine resource layers.
- Do not change root/main/page display, positioning, width calculation, or scroll ownership.
- Use `titlebar`, `page.header`, and `main.fade` for platform/header surface, border, and overlay visuals without altering window controls.

## Sidebar

`sidebar`, `sidebar.scroll`, `sidebar.resize`, `sidebar.resize.indicator`, `sidebar.header`, `sidebar.header.icon`, `sidebar.header.label`, `sidebar.header.background`, `sidebar.header.decoration`, `sidebar.brand`, `sidebar.brand.icon`, `sidebar.brand.badge`, `sidebar.frame`, `sidebar.section`, `sidebar.section.projects`, `sidebar.section.header`, `sidebar.section.toggle`, `sidebar.section.label`, `sidebar.section.actions`, `sidebar.section.action`, `sidebar.section.action.icon`, `sidebar.section.decoration`, `sidebar.item`, `sidebar.item.icon`, `sidebar.item.label`, `sidebar.item.active`, `sidebar.item.active.icon`, `sidebar.item.active.label`, `sidebar.footer`, `sidebar.footer.item`, `sidebar.footer.icon`, `sidebar.footer.label`, `sidebar.footer.brand`, `sidebar.footer.brand.label`, `sidebar.footer.brand.timer`, `sidebar.footer.brand.pro`, `sidebar.footer.brand.version`.

- Resource/mount slots: header background/decoration, brand icon/badge, frame, section decoration, and footer brand status.
- Style labels and glyphs separately. Never use an outer icon container color when the actual glyph slot exists.
- Do not resize the sidebar, move footer ownership, change collapse/resize behavior, or fake Pro/timer state.

## Menu

`menu`, `menu.item`, `menu.item.active`, `menu.item.checked`, `menu.icon`, `menu.label`, `menu.shortcut`, `menu.separator`.

- Style normal, active, and checked separately.
- Preserve item hit targets, placement, keyboard focus, selected indication, labels, and shortcuts.

## Composer

`composer`, `composer.backdrop`, `composer.context`, `composer.context.item`, `composer.context.item.icon`, `composer.context.item.label`, `composer.editor`, `composer.action`, `composer.action.icon`, `composer.action.label`, `composer.permission`, `composer.permission.icon`, `composer.permission.label`, `composer.submit`, `composer.submit.icon`, `composer.submit.decoration`, `composer.decoration`, `composer.panel`, `composer.panel.item`, `composer.panel.icon`, `composer.panel.separator`, `composer.region`.

- Style outer surface, backdrop, context bar, controls, editor text, submit, and panels independently.
- Do not set fixed composer/editor height, editor background, structural padding, bottom positioning, or expanded-state layout.
- Dedicated decorations remain pointer-transparent and cannot cover icons or hit targets.

## Home Hero and cards

`home.hero`, `home.hero.viewport`, `home.hero.copy`, `home.hero.eyebrow`, `home.hero.title`, `home.hero.description`, `home.hero.media`, `home.hero.media.asset`, `home.hero.foreground`, `home.hero.foreground.asset`, `home.hero.divider`, `home.hero.divider.icon`, `home.hero.divider.label`, `home.hero.divider.line`, `home.layout`, `home.content.region`, `home.stage`, `home.brand`, `home.prompt`, `home.prompt.title`, `home.cards`, `home.cards.layout`, `home.cards.grid`, `home.card`, `home.card.background`, `home.card.content`, `home.card.icon`, `home.card.icon.glyph`, `home.card.label`, `home.card.arrow`, `home.card.arrow.glyph`, `home.card.arrow.asset`.

- The Hero is an engine-owned flow item before native home content. Style its height and visuals without absolute/fixed positioning or negative layout offsets.
- `home.layout`, `home.content.region`, and `home.stage` are structural and read-only apart from visual backgrounds/tokens.
- Keep cards in the native grid. Never move them into the Hero or impose a fixed total width/height.
- Color actual glyph slots, preserve labels, clicks, expansion, and responsive wrapping.

## Conversation and content

`conversation.stage`, `conversation.header`, `conversation.header.content`, `conversation.viewport`, `conversation.summary.region`, `conversation.summary`, `conversation.summary.decoration`, `conversation`, `conversation.user`, `conversation.assistant`, `conversation.banner`, `conversation.banner.copy`, `conversation.banner.eyebrow`, `conversation.banner.title`, `conversation.banner.description`, `conversation.banner.media`, `conversation.banner.media.asset`, `conversation.banner.foreground`, `conversation.banner.foreground.asset`, `code`, `code.inline`, `diff`, `terminal`, `terminal.viewport`.

- Engine structure is `conversation.stage > conversation.header + conversation.viewport`; the compact Banner sits inside header content above the message viewport.
- Never float the Banner over messages or change conversation scroll/anchor behavior.
- Summary decoration attaches visually to the task card without adding layout height.
- Preserve message, code, diff, terminal copy/select/scroll semantics.

## Settings

`settings`, `settings.header`, `settings.sidebar`, `settings.nav`, `settings.nav.item`, `settings.nav.item.active`, `settings.content`, `settings.surface`, `settings.frame`, `settings.canvas`, `settings.toolbar`, `settings.body`, `settings.section`, `settings.section.title`, `settings.card`, `settings.row`, `settings.row.title`, `settings.row.description`, `settings.row.separator`, `settings.control`, `settings.control.checked`, `settings.switch`, `settings.switch.checked`, `settings.switch.track`, `settings.switch.track.checked`, `settings.switch.thumb`.

- Use outer settings slots to remove platform canvas/overlay differences; keep form cards distinct from the outer page container.
- Style each card, row, label, separator, control, checked state, Switch track, and thumb independently.
- Preserve settings navigation, scrolling, control state, focus, and native layout.

## Corner decoration

`decoration.top-right`, `decoration.bottom-right`.

Both are engine resource mounts under the main decoration layer. Limit size and contrast, keep `pointer-events: none`, and never cover the titlebar, workspace panel, composer, or controls.

## Controlled general asset slots

Only these can appear in `experience.assets`:

`app.background`, `main.background`, `main.overlay`, `main.frame`, `sidebar.brand.icon`, `sidebar.brand.badge`, `sidebar.header.background`, `sidebar.header.decoration`, `sidebar.frame`, `home.card.background`, `home.card.arrow.asset`.

Dedicated Manifest fields create `composer.submit.decoration`, `composer.decoration`, `conversation.summary.decoration`, and `sidebar.section.decoration` instead.
