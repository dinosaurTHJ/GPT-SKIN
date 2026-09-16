import assert from "node:assert/strict";
import { mkdtempSync, readFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import test from "node:test";

const projectRoot = resolve(import.meta.dirname, "..");

test("builds bilingual updater notes for the release version", () => {
  const outputPath = join(mkdtempSync(join(tmpdir(), "retheme-notes-")), "notes.md");
  const result = spawnSync("bash", [
    join(projectRoot, "scripts/release/build-release-notes.sh"),
    "0.1.0",
    outputPath,
    join(projectRoot, "CHANGELOG.md"),
    join(projectRoot, "CHANGELOG.zh-CN.md"),
  ], { encoding: "utf8" });
  assert.equal(result.status, 0, result.stderr);
  const notes = readFileSync(outputPath, "utf8");
  assert.match(notes, /## 中文/);
  assert.match(notes, /## English/);
  assert.match(notes, /签名自动更新/);
  assert.match(notes, /signed automatic updates/);
});

test("rejects a release without matching bilingual changelog entries", () => {
  const outputPath = join(mkdtempSync(join(tmpdir(), "retheme-notes-")), "notes.md");
  const result = spawnSync("bash", [
    join(projectRoot, "scripts/release/build-release-notes.sh"),
    "9.9.9",
    outputPath,
    join(projectRoot, "CHANGELOG.md"),
    join(projectRoot, "CHANGELOG.zh-CN.md"),
  ], { encoding: "utf8" });
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /No changelog entry found for version 9\.9\.9/);
});
