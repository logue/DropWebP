import { readFileSync } from 'node:fs';
import { fileURLToPath, URL } from 'node:url';

import vue from '@vitejs/plugin-vue';
import { defineConfig, loadEnv, type UserConfig } from 'vite';

import VueI18nPlugin from '@intlify/unplugin-vue-i18n/vite';
import { visualizer } from 'rollup-plugin-visualizer';
import { checker } from 'vite-plugin-checker';
import vueDevTools from 'vite-plugin-vue-devtools';
import vuetify, { transformAssetUrls } from 'vite-plugin-vuetify';

const pkg = JSON.parse(readFileSync('./package.json', 'utf-8')) as {
  name: string;
  version: string;
};

/**
 * Vite Configure
 *
 * @see {@link https://vitejs.dev/config/}
 */
export default defineConfig(({ command, mode }): UserConfig => {
  const buildDate = new Date().toISOString();
  const envRoot = fileURLToPath(new URL('..', import.meta.url));
  const env = loadEnv(mode, envRoot, '');
  const appName = env.VITE_APP_NAME || 'Tauri Vue3 App';
  const appVersion = env.VERSION || pkg.version;

  const config: UserConfig = {
    // https://vitejs.dev/config/shared-options.html#base
    base: './',
    // https://vitejs.dev/config/shared-options.html#define
    define: {
      'process.env': {},
      __APP_NAME__: JSON.stringify(appName),
      __APP_VERSION__: JSON.stringify(appVersion),
      __BUILD_DATE__: JSON.stringify(buildDate)
    },
    envDir: '..',
    plugins: [
      // Vue3
      vue({
        template: {
          // https://github.com/vuetifyjs/vuetify-loader/tree/next/packages/vite-plugin#image-loading
          transformAssetUrls
        }
      }),
      vueDevTools(),
      // vite-plugin-checker
      // https://github.com/fi3ework/vite-plugin-checker
      checker({
        typescript: true
        // vueTsc: true,
        // eslint: { lintCommand: 'eslint' },
        // stylelint: { lintCommand: 'stylelint' },
      }),
      // Vue I18n (YAML locale transform)
      VueI18nPlugin({
        compositionOnly: false,
        include: [fileURLToPath(new URL('./src/locales/**/*.yaml', import.meta.url))],
        runtimeOnly: false
      }),
      // Vuetify Loader
      // https://github.com/vuetifyjs/vuetify-loader/tree/master/packages/vite-plugin
      vuetify({
        autoImport: true,
        styles: { configFile: 'src/styles/settings.scss' }
      })
    ],
    // Resolver
    resolve: {
      // https://vitejs.dev/config/shared-options.html#resolve-alias
      alias: {
        '@': fileURLToPath(new URL('./src', import.meta.url)),
        '~': fileURLToPath(new URL('./node_modules', import.meta.url))
      },
      extensions: ['.js', '.json', '.jsx', '.mjs', '.ts', '.tsx', '.vue']
    },
    // Build Options
    // https://vitejs.dev/config/build-options.html
    build: {
      // Build Target
      // https://vitejs.dev/config/build-options.html#build-target
      // Tauri WebView (WKWebView on macOS, WebView2 on Windows, WebKitGTK on Linux)
      // supports modern JS natively, no need for legacy browser targets.
      target: 'esnext',
      // Minify option
      // https://vitejs.dev/config/build-options.html#build-minify
      minify: 'esbuild',
      // Rollup Options
      // https://vitejs.dev/config/build-options.html#build-rollupoptions
      rollupOptions: {
        output: {
          manualChunks: (id: string) => {
            // Split external library from transpiled code.
            if (id.includes('/node_modules/vuetify') || id.includes('/node_modules/@mdi')) {
              // Split Vuetify before vue.
              return 'vuetify';
            }
            if (
              id.includes('/node_modules/@vue/') ||
              id.includes('/node_modules/vue') ||
              id.includes('/node_modules/pinia') ||
              id.includes('/node_modules/destr/') || // pinia-plugin-persistedstate uses destr.
              id.includes('/node_modules/deep-pick-omit/') // pinia-plugin-persistedstate uses deep-pick-omit.
            ) {
              // Combine Vue and Pinia into a single chunk.
              // This is because Pinia is a state management library for Vue.
              return 'vue';
            }

            return 'vendor';
          },
          plugins: [
            mode === 'analyze'
              ? // rollup-plugin-visualizer
                // https://github.com/btd/rollup-plugin-visualizer
                visualizer({
                  open: true,
                  filename: 'dist/stats.html'
                })
              : undefined
          ]
        }
      }
    },
    // Keep Vite dev server URL stable for Tauri's configured devUrl.
    server: {
      port: 1420,
      strictPort: true
    },
    esbuild: {
      // Drop console when production build.
      drop: command === 'serve' ? [] : ['console']
    }
  };

  return config;
});
