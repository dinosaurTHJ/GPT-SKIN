#!/bin/bash
set -euo pipefail

version="${1:?usage: extract-release-notes.sh <version> [changelog_path]}"
changelog_path="${2:-CHANGELOG.md}"

[[ -f "$changelog_path" ]] || { echo "Changelog not found: $changelog_path" >&2; exit 1; }

notes="$(awk -v version="$version" '
  $0 ~ "^## \\[" version "\\]" { found=1; next }
  /^## \[/ && found { exit }
  found { print }
' "$changelog_path")"

notes="$(printf '%s\n' "$notes" | sed -e '/./,$!d')"
[[ -n "${notes//[[:space:]]/}" ]] || { echo "No changelog entry found for version $version" >&2; exit 1; }
printf '%s\n' "$notes"
