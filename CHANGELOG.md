# Changelog

All notable ReTheme desktop changes are documented here.

## [0.1.4] - 2026-07-20

### Added

- Add a GitHub Star support entry to the overview for users who want to support the open-source project.

### Fixed

- Fix theme activation for the Microsoft Store/MSIX ChatGPT package installed under `WindowsApps`.
- Launch the isolated ChatGPT instance through its system AppUserModelId and reclaim only the process returned by that activation.

## [0.1.3] - 2026-07-19

### Added

- Allow controlled theme assets to declare `lightAsset` and `darkAsset` variants that switch with the ChatGPT appearance and fall back to `asset`.
- Share the appearance-aware asset schema across desktop loading, the theme-development Skill, and server-side community validation.

## [0.1.2] - 2026-07-19

### Added

- Add one shared Rust theme protocol and CLI validator for desktop loading, local authoring, AI workflows, and server-side community review.
- Document the complete v1 Manifest schema, 164 stable slots, annotated examples, Banner asset specifications, generation prompts, and a reusable theme-development Skill.

### Fixed

- Refresh only the affected home slots while editing the composer, avoiding full runtime reinjection and related layout flicker.
- Fully release theme observers and animation frames when replacing or restoring a runtime.

### Security

- Strictly reject unknown Manifest fields, unsafe archive paths, unscoped or structural CSS, external asset references, invalid image content, and unsafe SVG markup through the shared protocol gate.

## [0.1.1] - 2026-07-19

### Fixed

- Recover installed themes after reinstalling ReTheme on Windows when an old device-encrypted cache can no longer be opened.
- Show a clear result after manually checking for updates, including when ReTheme is already up to date.
- Build tagged releases with the tag version across application metadata and updater manifests.

## [0.1.0] - 2026-07-19

### Added

- Manage installed themes, community downloads, local theme development, account state, cloud favorites, and device history.
- Apply signed themes through the local Rust runtime with remote ChatGPT compatibility data.
- Support Simplified Chinese and English, tray behavior, deep links, light and dark appearance, and signed automatic updates.
- Ship signed installers for macOS Apple Silicon, macOS Intel, and Windows x64.

### Security

- Verify theme packages and compatibility data before use and serve theme assets only through a bounded loopback session.
- Restore the official ChatGPT interface before ReTheme exits or installs an application update.
