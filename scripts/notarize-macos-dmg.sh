#!/bin/bash
set -euo pipefail

target="${1:?usage: notarize-macos-dmg.sh <target-triple>}"
bundle_dir="src-tauri/target/${target}/release/bundle"

fail() {
  echo "[macos-notarization] $*" >&2
  exit 1
}

[[ -n "${APPLE_API_KEY_PATH:-}" && -f "$APPLE_API_KEY_PATH" ]] || fail "APPLE_API_KEY_PATH is missing"
[[ -n "${APPLE_API_KEY:-}" ]] || fail "APPLE_API_KEY is missing"
[[ -n "${APPLE_API_ISSUER:-}" ]] || fail "APPLE_API_ISSUER is missing"

shopt -s nullglob
dmg_paths=("${bundle_dir}/dmg/"*.dmg)
[[ ${#dmg_paths[@]} -eq 1 ]] || fail "expected exactly one DMG, found ${#dmg_paths[@]}"

echo "[macos-notarization] submitting DMG to Apple"
xcrun notarytool submit "${dmg_paths[0]}" \
  --key "$APPLE_API_KEY_PATH" \
  --key-id "$APPLE_API_KEY" \
  --issuer "$APPLE_API_ISSUER" \
  --wait
xcrun stapler staple "${dmg_paths[0]}"
xcrun stapler validate "${dmg_paths[0]}"
spctl --assess --type open --context context:primary-signature --verbose=4 "${dmg_paths[0]}"

echo "[macos-notarization] Apple remote checks passed for $target"
