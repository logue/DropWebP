import { execSync } from "node:child_process";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(scriptDir, "..");
const bootMode = process.argv[2] ?? "full";
const mainContentMode = process.argv[3] ?? "";
const safeBoot = process.argv[4] ?? "";

const env = {
  ...process.env,
  VITE_BOOT_MODE: bootMode,
};

if (mainContentMode) {
  env.VITE_MAIN_CONTENT_MODE = mainContentMode;
}

if (safeBoot) {
  env.VITE_SAFE_BOOT = safeBoot;
}

process.chdir(resolve(repoRoot, "frontend"));
execSync("pnpm run dev:tauri", {
  stdio: "inherit",
  env,
  shell: true,
});
