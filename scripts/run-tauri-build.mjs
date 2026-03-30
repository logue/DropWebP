import { spawnSync } from "node:child_process";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(scriptDir, "..");
const extraArgs = process.argv.slice(2);

const tauriBinary =
  process.platform === "win32"
    ? resolve(repoRoot, "frontend/node_modules/.bin/tauri.cmd")
    : resolve(repoRoot, "frontend/node_modules/.bin/tauri");

const result = spawnSync(
  tauriBinary,
  ["build", "--config", "backend/tauri.conf.json", ...extraArgs],
  {
    cwd: repoRoot,
    stdio: "inherit",
    env: process.env,
    shell: false,
  },
);

if (result.error) {
  console.error(result.error.message);
  process.exit(1);
}

process.exit(result.status ?? 1);
