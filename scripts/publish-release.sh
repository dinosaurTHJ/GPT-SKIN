#!/bin/bash
set -euo pipefail

release_id="${1:?usage: publish-release.sh <release-id> <tag>}"
expected_tag="${2:?usage: publish-release.sh <release-id> <tag>}"
repository="${GITHUB_REPOSITORY:?GITHUB_REPOSITORY is missing}"

[[ "$release_id" =~ ^[0-9]+$ ]] || { echo "Invalid release ID: $release_id" >&2; exit 1; }
[[ "$expected_tag" =~ ^v[0-9]+\.[0-9]+\.[0-9]+([-.][0-9A-Za-z.-]+)?$ ]] || {
  echo "Invalid release tag: $expected_tag" >&2
  exit 1
}

release_endpoint="repos/$repository/releases/$release_id"
actual_tag="$(gh api "$release_endpoint" --jq .tag_name)"
[[ "$actual_tag" == "$expected_tag" || "$actual_tag" == untagged-* ]] || {
  echo "Release ID $release_id belongs to $actual_tag, expected $expected_tag" >&2
  exit 1
}

draft="$(gh api "$release_endpoint" --jq .draft)"
if [[ "$draft" == "false" ]]; then
  echo "Release $expected_tag (ID $release_id) is already public"
  exit 0
fi
[[ "$draft" == "true" ]] || { echo "Invalid draft state for release ID $release_id: $draft" >&2; exit 1; }

published="$(gh api --method PATCH "$release_endpoint" \
  -f tag_name="$expected_tag" \
  -F draft=false \
  -f make_latest=true \
  --jq '[.tag_name, .draft] | @tsv')"
[[ "$published" == "$expected_tag"$'\t'"false" ]] || {
  echo "GitHub did not publish release ID $release_id: $published" >&2
  exit 1
}

echo "Published $expected_tag from release ID $release_id"
