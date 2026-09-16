# Security Model

ReTheme reduces theme risk through a narrow declarative protocol. It does not claim that a desktop binary can keep every embedded client value secret from a determined local attacker.

## Trust boundaries

- Community authors upload source ZIP files containing only a manifest, CSS, and images.
- The server validates and normalizes a submission, then produces the signed `.ctheme` package.
- The desktop downloads packages only through the authenticated API and verifies the platform Ed25519 signature before installation.
- Installed packages are stored in a device-encrypted cache and are decrypted only by the Rust runtime when applied.
- Runtime assets are served by a loopback-only session URL. They are not extracted into a public theme directory.
- Local development directories intentionally bypass package signing and encryption, but are never installed as community packages.
- Signed compatibility data is selected by ChatGPT version and rejects tampering, expiry mismatches, and rollback.
- Tauri updater artifacts are signed by a separate update key and verified before installation.
- macOS applications and DMGs are signed with the ReTheme Apple Developer ID identity, use the hardened runtime, and are notarized and stapled before publication.

## Theme restrictions

Themes cannot include JavaScript, external URLs, remote fonts, `@import`, active SVG content, path traversal, undeclared slots, global unscoped CSS, or arbitrary permissions. Archive size, extracted size, image size, file count, MIME type, and asset mounts are bounded by the Rust validator.

## Secrets

This public desktop repository commits templates and public verification keys only. GitHub Actions receives production build values from repository secrets and deletes generated TOML files after each build. The platform's private signing keys remain in the private server/source repository.

API identifiers, client signing material, and package decryption values compiled into a desktop application should be treated as recoverable defense-in-depth values. Server authorization, signed responses, short-lived download URLs, account state, replay protection, and entitlement checks remain authoritative.

Apple Developer ID signing and Tauri updater signing use separate keys and protect different boundaries. Passing updater signature verification does not imply that Gatekeeper will accept an unsigned or unnotarized macOS application.

Please report security issues privately through the contact channel on [retheme.app](https://retheme.app) rather than publishing exploit details in a public issue.
