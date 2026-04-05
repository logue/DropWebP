import { execSync } from "node:child_process";
import { existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { syncTauriConfigFromEnv } from "./sync-tauri-config-from-env.mjs";

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(scriptDir, "..");

const syncResult = syncTauriConfigFromEnv(repoRoot);
console.log("Synced Tauri config from .env");
console.log(`  version=${syncResult.version}`);
console.log(`  identifier=${syncResult.identifier}`);

const env = {
  ...process.env,
  RUST_LOG: process.env.RUST_LOG ?? "info",
  CARGO_INCREMENTAL: process.env.CARGO_INCREMENTAL ?? "1",
};

const pathKey = process.platform === "win32" ? "Path" : "PATH";
const pathValue = env[pathKey] ?? "";

function prependPath(directory) {
  if (!directory || pathValue.toLowerCase().includes(directory.toLowerCase())) {
    return;
  }

  env[pathKey] = `${directory};${env[pathKey] ?? ""}`;
}

function configureWindowsTooling() {
  if (process.platform !== "win32") {
    return [];
  }

  const warnings = [];
  const llvmBin = "C:/Program Files/LLVM/bin";
  const nasmBin = "C:/Program Files/NASM";

  if (!env.LIBCLANG_PATH && existsSync(join(llvmBin, "libclang.dll"))) {
    env.LIBCLANG_PATH = llvmBin;
  }

  if (existsSync(join(llvmBin, "clang.exe"))) {
    prependPath(llvmBin);
  } else {
    warnings.push(
      "LLVM/Clang が見つかりません。LLVM をインストールし、clang.exe と libclang.dll を利用可能にしてください。",
    );
  }

  if (existsSync(join(nasmBin, "nasm.exe"))) {
    prependPath(nasmBin);
  } else {
    warnings.push("NASM が見つかりません。AVIF 関連のビルドで必要です。");
  }

  if (
    !env.LIBCLANG_PATH ||
    !existsSync(join(env.LIBCLANG_PATH, "libclang.dll"))
  ) {
    warnings.push(
      "LIBCLANG_PATH が未設定です。通常は C:/Program Files/LLVM/bin を指す必要があります。",
    );
  }

  return warnings;
}

const warnings = configureWindowsTooling();

if (warnings.length > 0) {
  console.error("Missing Windows build prerequisites:");
  for (const warning of warnings) {
    console.error(`- ${warning}`);
  }
  process.exit(1);
}

process.chdir(repoRoot);
execSync("tauri dev --config backend/tauri.conf.json", {
  stdio: "inherit",
  env,
  shell: true,
});
