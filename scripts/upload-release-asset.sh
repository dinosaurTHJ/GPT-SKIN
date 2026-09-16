#!/bin/bash
set -euo pipefail

release_id="${1:?usage: upload-release-asset.sh <release-id> <asset-name> <file>}"
asset_name="${2:?usage: upload-release-asset.sh <release-id> <asset-name> <file>}"
file_path="${3:?usage: upload-release-asset.sh <release-id> <asset-name> <file>}"
repository="${GITHUB_REPOSITORY:?GITHUB_REPOSITORY is missing}"

[[ -f "$file_path" ]] || { echo "Release asset not found: $file_path" >&2; exit 1; }
[[ "$asset_name" =~ ^[A-Za-z0-9._-]+$ ]] || { echo "Invalid release asset name: $asset_name" >&2; exit 1; }

while read -r asset_id; do
  [[ -z "$asset_id" ]] || gh api --method DELETE "repos/$repository/releases/assets/$asset_id"
done < <(gh api "repos/$repository/releases/$release_id/assets?per_page=100" \
  --jq ".[] | select(.name == \"$asset_name\") | .id")

upload_url="$(gh api "repos/$repository/releases/$release_id" --jq .upload_url)"
upload_url="${upload_url%%\{*}"
curl --fail-with-body \
  --request POST \
  --header "Authorization: Bearer $GH_TOKEN" \
  --header "X-GitHub-Api-Version: 2022-11-28" \
  --header "Content-Type: application/octet-stream" \
  --data-binary "@$file_path" \
  "${upload_url}?name=${asset_name}" \
  >/dev/null

echo "Uploaded $asset_name to release ID $release_id"
