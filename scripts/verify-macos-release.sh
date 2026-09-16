#!/bin/bash
set -euo pipefail

target="${1:?usage: verify-macos-release.sh <target-triple>}"
expected_identity="Developer ID Application: xinhua li (USA572LS4F)"
expected_team_id="USA572LS4F"
bundle_dir="src-tauri/target/${target}/release/bundle"
app_path="${bundle_dir}/macos/ReTheme.app"

fail() {
  echo "[macos-release] $*" >&2
  exit 1
}

[[ -d "$app_path" ]] || fail "app bundle not found: $app_path"

echo "[macos-release] verifying app signature"
codesign --verify --deep --strict --verbose=2 "$app_path"

signature_info="$(codesign -dvvv "$app_path" 2>&1)"
grep -Fq "Authority=${expected_identity}" <<< "$signature_info" || fail "Developer ID authority is missing"
grep -Fq "TeamIdentifier=${expected_team_id}" <<< "$signature_info" || fail "unexpected or missing TeamIdentifier"
grep -Fq "Runtime Version=" <<< "$signature_info" || fail "hardened runtime is missing"

echo "[macos-release] verifying app notarization ticket"
xcrun stapler validate "$app_path"
spctl --assess --type execute --verbose=4 "$app_path"

shopt -s nullglob
dmg_paths=("${bundle_dir}/dmg/"*.dmg)
[[ ${#dmg_paths[@]} -eq 1 ]] || fail "expected exactly one DMG, found ${#dmg_paths[@]}"

echo "[macos-release] verifying DMG signature"
codesign --verify --strict --verbose=2 "${dmg_paths[0]}"
dmg_signature_info="$(codesign -dvvv "${dmg_paths[0]}" 2>&1)"
grep -Fq "Authority=${expected_identity}" <<< "$dmg_signature_info" || fail "DMG Developer ID authority is missing"
grep -Fq "TeamIdentifier=${expected_team_id}" <<< "$dmg_signature_info" || fail "DMG TeamIdentifier is missing"

archive_paths=("${bundle_dir}/macos/"*.app.tar.gz)
[[ ${#archive_paths[@]} -eq 1 ]] || fail "expected exactly one updater archive, found ${#archive_paths[@]}"
[[ -s "${archive_paths[0]}.sig" ]] || fail "updater signature is missing"

archive_dir="$(mktemp -d)"
trap 'rm -rf "$archive_dir"' EXIT
tar -xzf "${archive_paths[0]}" -C "$archive_dir"
archive_app="$(find "$archive_dir" -maxdepth 2 -type d -name '*.app' -print -quit)"
[[ -n "$archive_app" ]] || fail "updater archive does not contain an app bundle"

echo "[macos-release] verifying updater archive app signature"
codesign --verify --deep --strict --verbose=2 "$archive_app"
archive_signature_info="$(codesign -dvvv "$archive_app" 2>&1)"
grep -Fq "Authority=${expected_identity}" <<< "$archive_signature_info" || fail "updater app Developer ID authority is missing"
grep -Fq "TeamIdentifier=${expected_team_id}" <<< "$archive_signature_info" || fail "updater app TeamIdentifier is missing"
xcrun stapler validate "$archive_app"

echo "[macos-release] all Apple distribution checks passed for $target"
