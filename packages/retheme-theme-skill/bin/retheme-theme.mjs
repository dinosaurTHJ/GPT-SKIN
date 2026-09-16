#!/usr/bin/env node

import { chmodSync, cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, renameSync, rmSync, statSync } from "node:fs";
import { homedir } from "node:os";
import { basename, dirname, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const packageDirectory = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const packageJson = JSON.parse(readFileSync(join(packageDirectory, "package.json"), "utf8"));

function platformKey() {
  const keys = {
    "darwin:arm64": "darwin-arm64",
    "darwin:x64": "darwin-x64",
    "win32:x64": "win32-x64",
    "linux:x64": "linux-x64"
  };
  const key = keys[`${process.platform}:${process.arch}`];
  if (!key) {
    throw new Error(`Unsupported platform: ${process.platform}-${process.arch}`);
  }
  return key;
}

function validatorPath(root = packageDirectory) {
  const executable = process.platform === "win32" ? "retheme-theme-validator.exe" : "retheme-theme-validator";
  const packaged = join(root, "native", platformKey(), executable);
  if (!existsSync(packaged)) {
    throw new Error(`The validator for ${platformKey()} is missing from this package.`);
  }
  if (process.platform !== "win32") chmodSync(packaged, 0o755);
  return packaged;
}

function defaultSkillDirectory() {
  const codexHome = process.env.CODEX_HOME || join(homedir(), ".codex");
  return join(codexHome, "skills", "retheme-theme-development");
}

function valueAfter(args, option) {
  const index = args.indexOf(option);
  if (index < 0) return null;
  if (!args[index + 1]) throw new Error(`${option} requires a path.`);
  return args[index + 1];
}

function install(args) {
  const source = join(packageDirectory, "skill");
  if (!existsSync(source)) {
    throw new Error("The packaged Skill is missing.");
  }

  const destination = resolve(valueAfter(args, "--target") || defaultSkillDirectory());
  const parent = dirname(destination);
  mkdirSync(parent, { recursive: true });
  const temporary = mkdtempSync(join(parent, `.${basename(destination)}-`));

  try {
    cpSync(source, temporary, { recursive: true });
    const executable = process.platform === "win32" ? "retheme-theme-validator.exe" : "retheme-theme-validator";
    const binaryDirectory = join(temporary, "bin");
    mkdirSync(binaryDirectory, { recursive: true });
    cpSync(validatorPath(), join(binaryDirectory, executable));
    if (process.platform !== "win32") {
      chmodSync(join(binaryDirectory, executable), 0o755);
    }
    rmSync(destination, { recursive: true, force: true });
    renameSync(temporary, destination);
  } catch (error) {
    rmSync(temporary, { recursive: true, force: true });
    throw error;
  }

  console.log(`Installed retheme-theme-development to ${destination}`);
}

function create(args) {
  const target = args[0];
  if (!target) throw new Error("create requires a destination directory.");
  const destination = resolve(target);
  if (existsSync(destination)) throw new Error(`Destination already exists: ${destination}`);
  cpSync(join(packageDirectory, "skill", "assets", "theme-template"), destination, { recursive: true });
  console.log(`Created ReTheme theme at ${destination}`);
}

function validate(args) {
  const input = args[0];
  if (!input) throw new Error("validate requires a theme directory or source ZIP.");
  const path = resolve(input);
  if (!existsSync(path)) throw new Error(`Theme path does not exist: ${path}`);
  const mode = statSync(path).isDirectory() ? "--directory" : "--source";
  const result = spawnSync(validatorPath(), [mode, path], { stdio: "inherit" });
  if (result.error) throw result.error;
  process.exitCode = result.status ?? 1;
}

function help() {
  console.log(`ReTheme theme tools ${packageJson.version}

Usage:
  retheme-theme install [--target <skill-directory>]
  retheme-theme create <theme-directory>
  retheme-theme validate <theme-directory-or-source.zip>
  retheme-theme --version`);
}

try {
  const [command, ...args] = process.argv.slice(2);
  if (command === "install") install(args);
  else if (command === "create") create(args);
  else if (command === "validate") validate(args);
  else if (command === "--version" || command === "-v") console.log(packageJson.version);
  else if (!command || command === "--help" || command === "-h" || command === "help") help();
  else throw new Error(`Unknown command: ${command}`);
} catch (error) {
  console.error(error instanceof Error ? error.message : String(error));
  process.exitCode = 2;
}
