import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import path from "node:path";

/** Project root directory. */
const projectRoot = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "..",
);

/**
 * Parse .env style content while preserving values that include spaces.
 *
 * @param {string} content Raw .env content.
 * @returns {Record<string, string>} Parsed key-value pairs.
 */
const parseDotEnv = (content) => {
  const result = {};

  for (const rawLine of content.split(/\r?\n/u)) {
    const line = rawLine.trim();
    if (!line || line.startsWith("#")) {
      continue;
    }

    const separatorIndex = line.indexOf("=");
    if (separatorIndex === -1) {
      continue;
    }

    const key = line.slice(0, separatorIndex).trim();
    if (!key) {
      continue;
    }

    const value = line.slice(separatorIndex + 1).trim();
    result[key] = value;
  }

  return result;
};

/**
 * Read and parse root .env file.
 *
 * @returns {Record<string, string>} Parsed environment map.
 */
const readRootEnv = () => {
  const envPath = path.join(projectRoot, ".env");
  const envContent = readFileSync(envPath, "utf-8");
  return parseDotEnv(envContent);
};

/**
 * Write JSON file with stable formatting.
 *
 * @param {string} relativePath File path relative to project root.
 * @param {unknown} data JSON-serializable data.
 * @returns {void}
 */
const writeJson = (relativePath, data) => {
  const targetPath = path.join(projectRoot, relativePath);
  writeFileSync(targetPath, `${JSON.stringify(data, null, 2)}\n`, "utf-8");
};

/**
 * Update first crate package version in Cargo.toml.
 *
 * @param {string} version Version string.
 * @returns {void}
 */
const syncCargoVersion = (version) => {
  const cargoPath = path.join(projectRoot, "backend/Cargo.toml");
  const current = readFileSync(cargoPath, "utf-8");
  const updated = current.replace(/^version = ".*"/m, `version = "${version}"`);
  writeFileSync(cargoPath, updated, "utf-8");
};

/**
 * Synchronize app metadata from .env into build/runtime config files.
 *
 * @returns {void}
 */
const main = () => {
  const env = readRootEnv();

  const version = env.VERSION ?? "0.0.0";
  const identifier = env.APP_IDENTIFIER ?? "com.tauri.vue3.app";
  const appName = env.VITE_APP_NAME ?? env.APP_NAME ?? "Tauri Vue3 App";
  const summary = env.APP_SUMMARY ?? "Modern desktop application template";
  const description =
    env.APP_DESCRIPTION ??
    "A modern desktop application built with Tauri v2 and Vue 3";

  const frontendPackage = JSON.parse(
    readFileSync(path.join(projectRoot, "frontend/package.json"), "utf-8"),
  );
  frontendPackage.version = version;
  writeJson("frontend/package.json", frontendPackage);

  const tauriConfig = JSON.parse(
    readFileSync(path.join(projectRoot, "backend/tauri.conf.json"), "utf-8"),
  );
  tauriConfig.version = version;
  tauriConfig.identifier = identifier;
  tauriConfig.productName = appName;
  tauriConfig.mainBinaryName = appName;

  if (
    Array.isArray(tauriConfig?.app?.windows) &&
    tauriConfig.app.windows.length > 0
  ) {
    tauriConfig.app.windows[0].title = appName;
  }

  if (tauriConfig?.bundle) {
    tauriConfig.bundle.shortDescription = summary;
    tauriConfig.bundle.longDescription = description;
    if (
      typeof tauriConfig.bundle.publisher !== "string" ||
      !tauriConfig.bundle.publisher.trim()
    ) {
      tauriConfig.bundle.publisher = env.AUTHOR_NAME ?? "Your Name";
    }
  }

  writeJson("backend/tauri.conf.json", tauriConfig);

  syncCargoVersion(version);

  console.log("Synced app metadata from .env");
  console.log(`- version: ${version}`);
  console.log(`- identifier: ${identifier}`);
  console.log(`- appName: ${appName}`);
};

main();
