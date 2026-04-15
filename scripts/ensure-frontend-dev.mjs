import { spawn } from "node:child_process";

const devUrl = process.env.TAURI_DEV_URL ?? "http://localhost:1420/";

async function isDevServerUp(url) {
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), 1500);

  try {
    const response = await fetch(url, {
      method: "GET",
      signal: controller.signal,
    });
    return response.ok;
  } catch {
    return false;
  } finally {
    clearTimeout(timeoutId);
  }
}

if (await isDevServerUp(devUrl)) {
  console.log(`Frontend dev server is already running: ${devUrl}`);
  process.exit(0);
}

console.log(`Starting frontend dev server: ${devUrl}`);
const command =
  process.platform === "win32"
    ? ["cmd.exe", ["/d", "/s", "/c", "pnpm --filter frontend dev"]]
    : ["pnpm", ["--filter", "frontend", "dev"]];

const child = spawn(command[0], command[1], {
  stdio: "inherit",
  shell: false,
});

function shutdown(signal) {
  if (child.exitCode === null) {
    child.kill(signal);
  }
}

process.on("SIGINT", () => shutdown("SIGINT"));
process.on("SIGTERM", () => shutdown("SIGTERM"));

child.on("exit", (code) => {
  process.exit(code ?? 0);
});
