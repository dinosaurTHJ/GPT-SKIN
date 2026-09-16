#!/bin/bash
set -euo pipefail

version="${1:?usage: verify-published-release.sh <version>}"
repository="${GITHUB_REPOSITORY:?GITHUB_REPOSITORY is missing}"
expected_identity="Developer ID Application: xinhua li (USA572LS4F)"
expected_team_id="USA572LS4F"
download_root="https://github.com/${repository}/releases/latest/download"
work_dir="$(mktemp -d)"
trap 'rm -rf "$work_dir"' EXIT

download() {
  local name="$1"
  curl --fail --location --retry 6 --retry-delay 5 --retry-all-errors \
    "$download_root/$name" --output "$work_dir/$name"
}

download ReTheme-darwin-aarch64.dmg
download ReTheme-darwin-x64.dmg
download ReTheme-windows-x64-setup.exe
download latest.json

jq -e --arg version "$version" '.version == $version' "$work_dir/latest.json" >/dev/null

release_root="https://github.com/${repository}/releases/download/v${version}/"
jq -e --arg release_root "$release_root" '
  [.platforms[].url | startswith($release_root)] | all
' "$work_dir/latest.json" >/dev/null

while read -r updater_url; do
  curl --fail --head --location --retry 6 --retry-delay 5 --retry-all-errors \
    "$updater_url" --output /dev/null
done < <(jq -r '.platforms | to_entries | map(.value.url) | unique[]' "$work_dir/latest.json")

for dmg_path in "$work_dir"/*.dmg; do
  codesign --verify --strict --verbose=2 "$dmg_path"
  signature_info="$(codesign -dvvv "$dmg_path" 2>&1)"
  grep -Fq "Authority=$expected_identity" <<< "$signature_info"
  grep -Fq "TeamIdentifier=$expected_team_id" <<< "$signature_info"
  xcrun stapler validate "$dmg_path"
  spctl --assess --type open --context context:primary-signature --verbose=4 "$dmg_path"
done

echo "Published release downloads and Apple signatures are valid"
