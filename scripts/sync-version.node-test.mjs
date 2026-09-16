import assert from "node:assert/strict";
import { cpSync, mkdirSync, mkdtempSync, readFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import test from "node:test";

const projectRoot = resolve(import.meta.dirname, "..");

test("synchronizes every desktop version source", () => {
  const root = mkdtempSync(join(tmpdir(), "retheme-version-"));
  for (const relativePath of ["package.json", "src-tauri/tauri.conf.json", "src-tauri/Cargo.toml", "src-tauri/Cargo.lock"]) {
    mkdirSync(dirname(join(root, relativePath)), { recursive: true });
    cpSync(join(projectRoot, relativePath), join(root, relativePath), { recursive: true });
  }

  const sync = spawnSync(process.execPath, [join(projectRoot, "scripts/sync-version.mjs"), "v1.2.3", "--root", root], { encoding: "utf8" });
  assert.equal(sync.status, 0, sync.stderr);
  assert.equal(JSON.parse(readFileSync(join(root, "package.json"), "utf8")).version, "1.2.3");
  assert.equal(JSON.parse(readFileSync(join(root, "src-tauri/tauri.conf.json"), "utf8")).version, "1.2.3");
  assert.match(readFileSync(join(root, "src-tauri/Cargo.toml"), "utf8"), /\[package\][\s\S]*?version = "1\.2\.3"/);
  assert.match(readFileSync(join(root, "src-tauri/Cargo.lock"), "utf8"), /name = "retheme"\nversion = "1\.2\.3"/);

  const check = spawnSync(process.execPath, [join(projectRoot, "scripts/sync-version.mjs"), "1.2.3", "--check", "--root", root], { encoding: "utf8" });
  assert.equal(check.status, 0, check.stderr);
});

test("uses the positional version when root is the current directory", () => {
  const root = mkdtempSync(join(tmpdir(), "retheme-version-cwd-"));
  for (const relativePath of ["package.json", "src-tauri/tauri.conf.json", "src-tauri/Cargo.toml", "src-tauri/Cargo.lock"]) {
    mkdirSync(dirname(join(root, relativePath)), { recursive: true });
    cpSync(join(projectRoot, relativePath), join(root, relativePath), { recursive: true });
  }

  const sync = spawnSync(process.execPath, [join(projectRoot, "scripts/sync-version.mjs"), "v1.2.3"], { cwd: root, encoding: "utf8" });
  assert.equal(sync.status, 0, sync.stderr);
  assert.equal(JSON.parse(readFileSync(join(root, "package.json"), "utf8")).version, "1.2.3");
  assert.equal(JSON.parse(readFileSync(join(root, "src-tauri/tauri.conf.json"), "utf8")).version, "1.2.3");
});
