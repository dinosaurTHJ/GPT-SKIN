#!/usr/bin/env node

import { readFile, writeFile } from "node:fs/promises";

const [metadataPath, assetsPath, repository, tag] = process.argv.slice(2);

if (!metadataPath || !assetsPath || !repository || !tag) {
  console.error("usage: normalize-updater-metadata.mjs <latest.json> <release-assets.json> <repository> <tag>");
  process.exit(1);
}

const metadata = JSON.parse(await readFile(metadataPath, "utf8"));
const assets = JSON.parse(await readFile(assetsPath, "utf8"));
const assetNames = new Map();

for (const asset of assets) {
  assetNames.set(String(asset.id), asset.name);
  assetNames.set(asset.name, asset.name);
  if (asset.url) assetNames.set(asset.url, asset.name);
  if (asset.browser_download_url) assetNames.set(asset.browser_download_url, asset.name);
}

const releaseRoot = `https://github.com/${repository}/releases/download/${tag}/`;

for (const platform of Object.values(metadata.platforms ?? {})) {
  const url = platform.url;
  const finalSegment = decodeURIComponent(new URL(url).pathname.split("/").pop());
  const assetName = assetNames.get(url) ?? assetNames.get(finalSegment);
  if (!assetName) {
    throw new Error(`Unable to map updater URL to a release asset: ${url}`);
  }
  platform.url = `${releaseRoot}${assetName}`;
}

await writeFile(metadataPath, `${JSON.stringify(metadata, null, 2)}\n`);
