import { readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

function unquote(value) {
  if (
    (value.startsWith('"') && value.endsWith('"')) ||
    (value.startsWith("'") && value.endsWith("'"))
  ) {
    return value.slice(1, -1);
  }
  return value;
}

function parseEnvFile(envPath) {
  const env = {};
  const raw = readFileSync(envPath, "utf8");

  for (const line of raw.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#")) {
      continue;
    }

    const separatorIndex = line.indexOf("=");
    if (separatorIndex < 1) {
      continue;
    }

    const key = line.slice(0, separatorIndex).trim();
    const value = unquote(line.slice(separatorIndex + 1).trim());

    if (key) {
      env[key] = value;
    }
  }

  return env;
}

export function syncTauriConfigFromEnv(repoRoot) {
  const envPath = resolve(repoRoot, ".env");
  const tauriConfPath = resolve(repoRoot, "backend/tauri.conf.json");

  const env = parseEnvFile(envPath);
  const version = env.VERSION;
  const identifier = env.APP_IDENTIFIER;
  const appName = env.VITE_APP_NAME || env.APP_NAME;
  const appNameKebab = env.APP_NAME_KEBAB;

  if (!version) {
    throw new Error("VERSION is not defined in .env");
  }

  if (!identifier) {
    throw new Error("APP_IDENTIFIER is not defined in .env");
  }

  if (!appNameKebab) {
    throw new Error("APP_NAME_KEBAB is not defined in .env");
  }

  const tauriConf = JSON.parse(readFileSync(tauriConfPath, "utf8"));
  tauriConf.version = version;
  tauriConf.identifier = identifier;
  tauriConf.mainBinaryName = appNameKebab;

  if (appName) {
    tauriConf.productName = appName;

    if (
      Array.isArray(tauriConf?.app?.windows) &&
      tauriConf.app.windows.length > 0
    ) {
      tauriConf.app.windows[0].title = appName;
    }
  }

  writeFileSync(
    tauriConfPath,
    `${JSON.stringify(tauriConf, null, 2)}\n`,
    "utf8",
  );

  return { version, identifier, appName, appNameKebab, tauriConfPath };
}

const currentScriptPath = fileURLToPath(import.meta.url);
if (process.argv[1] === currentScriptPath) {
  const repoRoot = resolve(dirname(currentScriptPath), "..");
  const { version, identifier, appName, appNameKebab, tauriConfPath } =
    syncTauriConfigFromEnv(repoRoot);
  console.log(`Synced ${tauriConfPath}`);
  console.log(`  version=${version}`);
  console.log(`  identifier=${identifier}`);
  if (appName) {
    console.log(`  productName=${appName}`);
  }
  console.log(`  mainBinaryName=${appNameKebab}`);
}
