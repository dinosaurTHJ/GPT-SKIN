#!/bin/bash
set -euo pipefail

version="${1:?usage: build-release-notes.sh <version> <output_path>}"
output_path="${2:?usage: build-release-notes.sh <version> <output_path>}"
script_dir="$(cd "$(dirname "$0")" && pwd)"

english_notes="$(bash "$script_dir/extract-release-notes.sh" "$version" "${3:-CHANGELOG.md}")"
chinese_notes="$(bash "$script_dir/extract-release-notes.sh" "$version" "${4:-CHANGELOG.zh-CN.md}")"

mkdir -p "$(dirname "$output_path")"
{
  printf '## 中文\n\n%s\n\n---\n\n' "$chinese_notes"
  printf '## English\n\n%s\n' "$english_notes"
} > "$output_path"
