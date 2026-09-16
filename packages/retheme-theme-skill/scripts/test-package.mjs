import { existsSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { arch, platform, tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const packageDirectory = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repository = resolve(packageDirectory, "../..");
const temporary = mkdtempSync(join(tmpdir(), "retheme-skill-package-"));
const packageVersion = JSON.parse(readFileSync(resolve(packageDirectory, "package.json"), "utf8")).version;

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: options.cwd || packageDirectory,
    encoding: "utf8",
    env: process.env
  });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(" ")} failed\n${result.stdout}\n${result.stderr}`);
  }
  return result.stdout.trim();
}

try {
  const keys = {
    "darwin:arm64": "darwin-arm64",
    "darwin:x64": "darwin-x64",
    "win32:x64": "win32-x64",
    "linux:x64": "linux-x64"
  };
  const key = keys[`${platform()}:${arch()}`];
  const executable = platform() === "win32" ? "retheme-theme-validator.exe" : "retheme-theme-validator";
  if (!key || !existsSync(resolve(packageDirectory, "native", key, executable))) {
    run("node", ["scripts/stage-local-validator.mjs"]);
  }
  const archiveName = run("pnpm", ["pack", "--pack-destination", temporary]).split("\n").at(-1);
  const archive = resolve(temporary, archiveName);
  const skill = resolve(temporary, "installed-skill");
  const createdTheme = resolve(temporary, "created-theme");
  const sourceZip = resolve(temporary, "created-theme.zip");

  const version = run("pnpm", ["dlx", archive, "--version"]);
  if (version !== packageVersion) throw new Error(`Unexpected CLI version: ${version}`);

  run("pnpm", ["dlx", archive, "install", "--target", skill]);
  run("pnpm", ["dlx", archive, "create", createdTheme]);
  const validation = run("node", [
    resolve(skill, "scripts/validate-theme.mjs"),
    resolve(repository, "docs/theme-example/package")
  ]);
  const report = JSON.parse(validation);
  if (!report.ok) throw new Error(`Installed validator rejected the example: ${validation}`);

  const direct = JSON.parse(run("pnpm", [
    "dlx",
    archive,
    "validate",
    resolve(repository, "docs/theme-example/package")
  ]));
  if (!direct.ok) throw new Error("Package validator rejected the example.");

  run("zip", ["-qr", sourceZip, "."], { cwd: createdTheme });
  const source = JSON.parse(run("pnpm", ["dlx", archive, "validate", sourceZip]));
  if (!source.ok) throw new Error("Package validator rejected the created source ZIP.");

  const manifest = JSON.parse(readFileSync(resolve(skill, "assets/theme-template/manifest.json"), "utf8"));
  if (manifest.schemaVersion !== 1) throw new Error("Installed Skill template is invalid.");
  console.log("ReTheme theme Skill package passed installation and validation tests.");
} finally {
  rmSync(temporary, { recursive: true, force: true });
}
