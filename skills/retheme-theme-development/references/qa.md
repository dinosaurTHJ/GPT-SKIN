# ReTheme Theme QA

## Static gate

- Shared validator returns `ok: true` for the directory and final source ZIP.
- Manifest has no unknown fields, duplicate styles/slots, platform fields, or missing assets.
- Every CSS selector is exactly theme-scoped and uses stable allowlisted slots.
- CSS contains no fonts, internal selectors, arbitrary attributes, external URLs, `url()`, imports, scripts, structural layout patches, or hidden essential controls.
- Every image is referenced, under limits, and matches its extension/magic bytes.
- Every animation uses a `ct-` name and reduced-motion behavior.

## Runtime matrix

Run every important page in `zh-CN` and `en`, light and dark, on macOS and Windows.

| Area | States |
|:--|:--|
| Home | Empty; full Hero; prompt; cards; card Hover/click/expansion |
| Compact home | Type and delete draft; open/close workspace panel; no full Hero overlap |
| Conversation | New; historical; long messages; scroll top/middle/bottom; summary/task panel |
| Composer | Empty; one line; multiple lines; attachment/image; context bar; permission; panel; submit disabled/enabled |
| Sidebar | Normal/active items; project section; collapse; resize; footer; timer/Pro state when applicable |
| Settings | First entry; every nav item; cards; rows; controls; Switch off/on; return to conversation |
| Menu | Open/close; normal; pointer/keyboard active; checked; shortcut; separator |
| Content | User/assistant; links; inline code; code block; diff; terminal; horizontal content scroll |
| Window | Narrow; medium; wide; maximized; workspace panel; platform titlebar differences |
| Motion | Normal; `prefers-reduced-motion`; no Hover flicker or large repaint jank |
| Lifecycle | Apply; switch; restore; local timeout; logout; manager exit; no residual CSS/mounts |

## Blocking failures

- Any horizontal page scroll or clipped essential content.
- Home Hero/cards overlap or a compact Banner covers messages.
- Composer moves, collapses, gains forced height/padding, or changes after first input.
- Message scrolling rebounds, remounts, or loses position.
- Any control becomes unclickable, loses focus indication, or reports a false state.
- Light/dark or Chinese/English loses contrast, artwork, or required copy.
- Hover alternates continuously because decoration changes the hit target.
- Settings flashes/stays on a wrong canvas, or navigation removes theme background/Banner unexpectedly.
- Theme restoration leaves `data-ct-*`, injected mounts, asset service references, or CSS behind.

## Ownership triage

| Evidence | Theme issue | Engine/compatibility issue |
|:--|:--:|:--:|
| Only one theme fails | likely | unlikely |
| Color, radius, border, opacity, animation, image composition | yes | no |
| Theme sets structure/position/height and layout breaks | yes | no |
| Multiple themes lose the same slot on one ChatGPT version | no | yes |
| Mount appears under wrong parent or disappears after navigation | no | yes |
| Restore leaves mounts/styles | no | yes |
| Platform-only native canvas/header mismatch across themes | verify mapping | likely after multi-theme reproduction |

Reproduce suspected engine defects with the minimal template plus one known-good full theme. Capture platform, ChatGPT version, ReTheme/engine version, locale, appearance, width, view, missing slot, and exact navigation sequence.

## Delivery report

Report:

- Theme ID and version.
- Validator command and result.
- Tested ChatGPT versions and platforms.
- Completed language/appearance/page/window matrix.
- Assets added with source/rights statement.
- Known limitations that cannot be safely implemented in v1.

Do not describe an untested state as passed.
