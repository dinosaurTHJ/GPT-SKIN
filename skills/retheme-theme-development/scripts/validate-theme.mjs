#!/usr/bin/env node

import { existsSync, statSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const input = process.argv[2];
if (!input) {
  console.error("Usage: validate-theme.mjs <theme-directory-or-source.zip>");
  process.exit(2);
}

const themePath = resolve(input);
if (!existsSync(themePath)) {
  console.error(`Theme path does not exist: ${themePath}`);
  process.exit(2);
}

const scriptDirectory = dirname(fileURLToPath(import.meta.url));
const executable = process.platform === "win32" ? "retheme-theme-validator.exe" : "retheme-theme-validator";
const validator = resolve(scriptDirectory, "..", "bin", executable);
if (!existsSync(validator)) {
  console.error("The Skill validator is missing. Reinstall with: pnpm dlx @duxweb/retheme-theme-skill install");
  process.exit(127);
}

const mode = statSync(themePath).isDirectory() ? "--directory" : "--source";
const result = spawnSync(validator, [mode, themePath], { stdio: "inherit" });
if (result.error) {
  console.error(result.error.message);
  process.exit(126);
}
process.exit(result.status ?? 1);
