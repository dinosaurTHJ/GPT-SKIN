import assert from "node:assert/strict";
import { chmod, mkdtemp, readFile, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";
import test from "node:test";

const script = path.resolve("scripts/publish-release.sh");

async function runPublish({ draft = "true", tag = "v0.1.0", args = ["12345", "v0.1.0"] } = {}) {
  const directory = await mkdtemp(path.join(os.tmpdir(), "retheme-publish-test-"));
  const calls = path.join(directory, "calls.log");
  const gh = path.join(directory, "gh");
  await writeFile(gh, `#!/bin/bash
set -euo pipefail
printf '%s\\n' "$*" >> "$GH_CALLS"
if [[ "$*" == *"--method PATCH"* ]]; then
  printf 'v0.1.0\\tfalse\\n'
elif [[ "$*" == *"--jq .tag_name"* ]]; then
  printf '%s\\n' "$GH_TAG"
elif [[ "$*" == *"--jq .draft"* ]]; then
  printf '%s\\n' "$GH_DRAFT"
else
  exit 2
fi
`);
  await chmod(gh, 0o755);
  const result = spawnSync("bash", [script, ...args], {
    encoding: "utf8",
    env: {
      ...process.env,
      PATH: `${directory}:${process.env.PATH}`,
      GITHUB_REPOSITORY: "duxweb/ReTheme",
      GH_CALLS: calls,
      GH_DRAFT: draft,
      GH_TAG: tag,
    },
  });
  return { result, calls: await readFile(calls, "utf8").catch(() => "") };
}

test("publishes an untagged draft by numeric release ID", async () => {
  const { result, calls } = await runPublish({ tag: "untagged-8b508d66d9b9f462fe0c" });
  assert.equal(result.status, 0, result.stderr);
  assert.match(calls, /--method PATCH repos\/duxweb\/ReTheme\/releases\/12345/);
  assert.match(calls, /-f tag_name=v0\.1\.0/);
  assert.match(calls, /-F draft=false/);
  assert.doesNotMatch(calls, /release edit/);
});

test("refuses a release ID that belongs to another tag", async () => {
  const { result, calls } = await runPublish({ tag: "v0.2.0" });
  assert.equal(result.status, 1);
  assert.match(result.stderr, /belongs to v0\.2\.0, expected v0\.1\.0/);
  assert.doesNotMatch(calls, /--method PATCH/);
});

test("treats an already public release as an idempotent success", async () => {
  const { result, calls } = await runPublish({ draft: "false" });
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /already public/);
  assert.doesNotMatch(calls, /--method PATCH/);
});
