/// <reference types="vite/client" />
/// <reference types="@tauri-apps/api" />

/**
 * Vite environment variables available via import.meta.env
 * See: https://vitejs.dev/guide/env-and-mode.html#env-files
 */
interface ImportMetaEnv {
  /**
   * Optional app title for legacy usage.
   * Prefer __APP_NAME__ defined in app/vite.config.ts for the UI title.
   */
  readonly VITE_APP_TITLE?: string;
}

/**
 * ImportMeta typing for Vite.
 */
interface ImportMeta {
  readonly env: ImportMetaEnv;
}
