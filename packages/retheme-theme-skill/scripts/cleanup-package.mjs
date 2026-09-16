import { rmSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const packageDirectory = resolve(dirname(fileURLToPath(import.meta.url)), "..");
rmSync(resolve(packageDirectory, "skill"), { recursive: true, force: true });
rmSync(resolve(packageDirectory, "LICENSE"), { force: true });
