/// <reference types="vite/client" />
/// <reference types="@tauri-apps/api" />

/**
 * Vite environment variables available via import.meta.env
 * See: https://vitejs.dev/guide/env-and-mode.html#env-files
 */
interface ImportMetaEnv {
  /**
   * Application display name from the root .env file.
   */
  readonly VITE_APP_NAME?: string;

  /**
   * Optional app title for legacy usage.
   * Prefer __APP_NAME__ defined in app/vite.config.ts for the UI title.
   */
  readonly VITE_APP_TITLE?: string;

  /**
   * Project repository URL used in the UI.
   */
  readonly VITE_PROJECT_URL?: string;
}

/**
 * ImportMeta typing for Vite.
 */
interface ImportMeta {
  readonly env: ImportMetaEnv;
}

declare const __APP_VERSION__: string;
declare const __BUILD_DATE__: string;
