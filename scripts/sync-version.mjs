import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const args = process.argv.slice(2);
const check = args.includes("--check");
const rootIndex = args.indexOf("--root");
const root = resolve(rootIndex >= 0 ? args[rootIndex + 1] : ".");
const rawVersion = args.find((arg, index) => !arg.startsWith("--") && (rootIndex < 0 || index !== rootIndex + 1));
const version = (rawVersion ?? JSON.parse(readFileSync(resolve(root, "package.json"), "utf8")).version)?.replace(/^v/, "");

if (!version || !/^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/.test(version)) {
  throw new Error("Usage: node scripts/sync-version.mjs [vX.Y.Z|X.Y.Z] [--check] [--root <path>]");
}

function updateJson(relativePath) {
  const path = resolve(root, relativePath);
  const value = JSON.parse(readFileSync(path, "utf8"));
  const current = value.version;
  value.version = version;
  return { path, current, next: `${JSON.stringify(value, null, 2)}\n` };
}

function updateText(relativePath, pattern, label) {
  const path = resolve(root, relativePath);
  const source = readFileSync(path, "utf8");
  const match = source.match(pattern);
  if (!match) throw new Error(`Unable to find ${label} version in ${relativePath}`);
  return {
    path,
    current: match[2],
    next: source.replace(pattern, `$1${version}$3`),
  };
}

const updates = [
  updateJson("package.json"),
  updateJson("src-tauri/tauri.conf.json"),
  updateText("src-tauri/Cargo.toml", /(\[package\][\s\S]*?^version\s*=\s*")([^"]+)(")/m, "Cargo package"),
  updateText("src-tauri/Cargo.lock", /(\[\[package\]\]\nname = "retheme"\nversion = ")([^"]+)(")/, "Cargo lockfile package"),
];

const mismatches = updates.filter((update) => update.current !== version);
if (check && mismatches.length > 0) {
  throw new Error(`Version ${version} is not synchronized: ${mismatches.map((update) => `${update.path}=${update.current}`).join(", ")}`);
}

if (!check) {
  for (const update of updates) writeFileSync(update.path, update.next);
}

console.log(`${check ? "Verified" : "Synchronized"} ReTheme version ${version}`);
