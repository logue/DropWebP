import { execSync } from "node:child_process";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { syncTauriConfigFromEnv } from "./sync-tauri-config-from-env.mjs";

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(scriptDir, "..");
const tauriConfigPath = resolve(repoRoot, "backend", "tauri.conf.json");
const tauriBin = resolve(
  repoRoot,
  "frontend",
  "node_modules",
  ".bin",
  process.platform === "win32" ? "tauri.cmd" : "tauri",
);
const extraArgs = process.argv.slice(2).join(" ");

const syncResult = syncTauriConfigFromEnv(repoRoot);
console.log("Synced Tauri config from .env");
console.log(`  version=${syncResult.version}`);
console.log(`  identifier=${syncResult.identifier}`);

process.chdir(repoRoot);

try {
  execSync(`"${tauriBin}" build --config "${tauriConfigPath}" ${extraArgs}`, {
    stdio: "inherit",
    env: process.env,
    shell: true,
  });
  process.exit(0);
} catch {
  process.exit(1);
}
