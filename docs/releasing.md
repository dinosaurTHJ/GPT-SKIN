# Release Guide

ReTheme releases are built by `.github/workflows/release.yml` and published to `duxweb/ReTheme` GitHub Releases.

## Required repository secrets

- `RETHEME_API_SECRET_ID`
- `RETHEME_API_SECRET_KEY`
- `RETHEME_PACKAGE_KEY`
- `RETHEME_LICENSE_PUBLIC_KEY`
- `RETHEME_COMPATIBILITY_PUBLIC_KEY`
- `RETHEME_THEME_PUBLIC_KEY`
- `TAURI_SIGNING_PRIVATE_KEY`
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`
- `APPLE_CERTIFICATE`: base64-encoded Developer ID Application `.p12`
- `APPLE_CERTIFICATE_PASSWORD`: password used when exporting the `.p12`
- `APPLE_SIGNING_IDENTITY`: `Developer ID Application: xinhua li (USA572LS4F)`
- `APPLE_API_KEY`: App Store Connect API key ID
- `APPLE_API_ISSUER`: App Store Connect API issuer UUID
- `APPLE_API_PRIVATE_KEY_BASE64`: base64-encoded App Store Connect `.p8`

Do not paste these values into source files, workflow logs, release notes, or issues. The private source repository is the authoritative encrypted/offline backup location.

## Preflight

```bash
pnpm install --frozen-lockfile
pnpm test
pnpm test:release
pnpm build
pnpm version:check
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
```

The tag is authoritative. Before each build, the workflow maps `vX.Y.Z` to `X.Y.Z` in `package.json`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, and the root package entry in `Cargo.lock`, then verifies consistency. The workflow also requires matching entries in `CHANGELOG.md` and `CHANGELOG.zh-CN.md`. It generates `api.toml` and `security.toml` from Actions Secrets, then removes them in an `always()` cleanup step.

## Publish

```bash
git tag v0.1.0
git push origin main
git push origin v0.1.0
```

Release asset names are stable and omit versions, for example `ReTheme-darwin-aarch64.dmg`, `ReTheme-darwin-x64.dmg`, and `ReTheme-windows-x64-setup.exe`. GitHub release notes and Tauri `latest.json.notes` use the same bilingual changelog entry.

Published macOS builds fail before compilation if any Apple signing or notarization secret is missing. The workflow never falls back to ad-hoc signing. The matrix builds:

- macOS Apple Silicon on `macos-15` for `aarch64-apple-darwin` (`app` updater archive and DMG)
- macOS Intel on `macos-15-intel` for `x86_64-apple-darwin` (`app` updater archive and DMG)
- Windows x64 on `windows-latest` for `x86_64-pc-windows-msvc`

The Windows NSIS installer offers Simplified Chinese and English. Matrix builds update one draft release serially, then the publish job verifies all updater platform entries before marking it as the latest public release.

## Verify

1. All three matrix jobs pass.
2. Both macOS apps and DMGs pass strict Developer ID signature checks and contain Team ID `USA572LS4F`.
3. Both macOS apps and DMGs contain valid stapled notarization tickets and pass Gatekeeper assessment.
4. The updater archives contain the same signed and notarized apps and have valid Tauri updater `.sig` files.
5. The release contains both fixed-name macOS DMGs, the fixed-name Windows NSIS installer, updater archives/signatures, and `latest.json`.
6. `latest.json` points to the same release, contains all supported platforms, and carries the bilingual changelog in `notes`.
7. Install each native artifact and verify launch, language, tray, deep links, theme apply/restore, and update checks.
8. Keep the tag immutable. Publish a new SemVer tag for every correction.

Apple Developer ID signing and Tauri updater signing are independent. Apple signing and notarization allow macOS to trust and launch the application. Tauri updater signatures let an already installed ReTheme client verify an update archive. A release is not valid unless both checks pass.

The ignored Rust integration tests require ChatGPT to be installed and may launch isolated windows. Run them explicitly on maintained macOS and Windows test machines before a production rollout.
