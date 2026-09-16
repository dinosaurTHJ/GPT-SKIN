import assert from "node:assert/strict";
import { mkdtemp, readFile, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";
import test from "node:test";

const script = path.resolve("scripts/normalize-updater-metadata.mjs");

test("maps GitHub asset IDs in updater URLs back to stable filenames", async () => {
  const directory = await mkdtemp(path.join(os.tmpdir(), "retheme-updater-test-"));
  const metadataPath = path.join(directory, "latest.json");
  const assetsPath = path.join(directory, "assets.json");
  await writeFile(metadataPath, JSON.stringify({
    version: "0.1.2",
    platforms: {
      "darwin-aarch64": {
        url: "https://api.github.com/repos/duxweb/ReTheme/releases/assets/482331448",
        signature: "signed",
      },
      "windows-x86_64": {
        url: "https://github.com/duxweb/ReTheme/releases/download/v0.1.2/482335639",
        signature: "signed",
      },
    },
  }));
  await writeFile(assetsPath, JSON.stringify([
    { id: 482331448, name: "ReTheme-darwin-aarch64.app.tar.gz" },
    { id: 482335639, name: "ReTheme-windows-x64-setup.exe" },
  ]));

  const result = spawnSync("node", [script, metadataPath, assetsPath, "duxweb/ReTheme", "v0.1.2"], {
    encoding: "utf8",
  });
  assert.equal(result.status, 0, result.stderr);

  const metadata = JSON.parse(await readFile(metadataPath, "utf8"));
  assert.equal(
    metadata.platforms["darwin-aarch64"].url,
    "https://github.com/duxweb/ReTheme/releases/download/v0.1.2/ReTheme-darwin-aarch64.app.tar.gz",
  );
  assert.equal(
    metadata.platforms["windows-x86_64"].url,
    "https://github.com/duxweb/ReTheme/releases/download/v0.1.2/ReTheme-windows-x64-setup.exe",
  );
});

test("rejects updater URLs that do not match a release asset", async () => {
  const directory = await mkdtemp(path.join(os.tmpdir(), "retheme-updater-test-"));
  const metadataPath = path.join(directory, "latest.json");
  const assetsPath = path.join(directory, "assets.json");
  await writeFile(metadataPath, JSON.stringify({
    platforms: { unknown: { url: "https://api.github.com/releases/assets/999", signature: "signed" } },
  }));
  await writeFile(assetsPath, "[]");

  const result = spawnSync("node", [script, metadataPath, assetsPath, "duxweb/ReTheme", "v0.1.2"], {
    encoding: "utf8",
  });
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /Unable to map updater URL/);
});
