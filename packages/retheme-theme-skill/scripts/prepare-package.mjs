import { cpSync, existsSync, rmSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const packageDirectory = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repository = resolve(packageDirectory, "../..");
const skill = resolve(repository, "skills/retheme-theme-development");

if (!existsSync(skill)) {
  throw new Error(`Skill source not found: ${skill}`);
}

rmSync(resolve(packageDirectory, "skill"), { recursive: true, force: true });
cpSync(skill, resolve(packageDirectory, "skill"), { recursive: true });
cpSync(resolve(repository, "LICENSE"), resolve(packageDirectory, "LICENSE"));
