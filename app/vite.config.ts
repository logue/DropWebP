import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath, URL } from 'node:url';

import vue from '@vitejs/plugin-vue';
import { defineConfig, type UserConfig } from 'vite';

import VueI18nPlugin from '@intlify/unplugin-vue-i18n/vite';
import { visualizer } from 'rollup-plugin-visualizer';
import { checker } from 'vite-plugin-checker';
import vueDevTools from 'vite-plugin-vue-devtools';
import vuetify, { transformAssetUrls } from 'vite-plugin-vuetify';

const host = process.env.TAURI_DEV_HOST;

// Load version from .env file
const loadVersionFromEnv = (): string => {
  try {
    const envPath = fileURLToPath(new URL('../.env', import.meta.url));
    const envContent = readFileSync(envPath, 'utf-8');
    const versionMatch = envContent.match(/^VERSION=(.+)$/m);
    return versionMatch ? versionMatch[1].trim() : '0.0.0';
  } catch {
    console.warn('Failed to load version from .env, using default version');
    return '0.0.0';
  }
};

const version = loadVersionFromEnv();

/**
 * Vite Configure
 *
 * @see {@link https://vitejs.dev/config/}
 */
export default defineConfig(({ command, mode }): UserConfig => {
  const config: UserConfig = {
    // https://vitejs.dev/config/shared-options.html#base
    base: './',
    // https://vitejs.dev/config/shared-options.html#define
    define: { 'process.env': {} },
    plugins: [
      // Vue3
      vue({
        template: {
          // https://github.com/vuetifyjs/vuetify-loader/tree/next/packages/vite-plugin#image-loading
          transformAssetUrls
        }
      }),
      vueDevTools(),
      // Vuetify Loader
      // https://github.com/vuetifyjs/vuetify-loader/tree/master/packages/vite-plugin
      vuetify({
        autoImport: true,
        styles: { configFile: 'src/styles/settings.scss' }
      }),
      // vite-plugin-checker
      // https://github.com/fi3ework/vite-plugin-checker
      checker({
        typescript: true
        // vueTsc: true,
        // eslint: { lintCommand: 'eslint' },
        // stylelint: { lintCommand: 'stylelint' },
      }),
      VueI18nPlugin({
        // trueにすると、<i18n>ブロックの警告が出なくなります
        // See https://github.com/intlify/bundle-tools/issues/22
        compositionOnly: false,
        // <i18n>カスタムブロックを持つファイルを指定
        include: fileURLToPath(new URL('./src/locales', import.meta.url))
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
    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // 1. prevent Vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
      port: 1420,
      strictPort: true,
      host: host || false,
      hmr: host
        ? {
            protocol: 'ws',
            host,
            port: 1421
          }
        : undefined,
      watch: {
        // 3. tell Vite to ignore watching `src-tauri` and other unnecessary directories
        ignored: [
          '**/src-tauri/**',
          '**/target/**',
          '**/node_modules/**',
          '**/.git/**',
          '**/dist/**',
          '**/*.lock',
          '**/Meta.ts', // Prevent infinite loop from auto-generated file
          '**/.DS_Store',
          '**/*.swp',
          '**/.turbo/**',
          '**/pnpm-lock.yaml',
          '**/.pnpm-debug.log',
          '**/*.log'
        ],
        // macOS FSEvents対応：ポーリングを無効にし、バッチ処理を強化
        usePolling: false,
        interval: 500,
        batchTimeout: 200,
        awaitWriteFinish: {
          // ファイルの書き込みが完了するまで待機
          stabilityThreshold: 100,
          pollInterval: 100
        }
      }
    },
    envPrefix: ['VITE_', 'TAURI_'],
    // Build Options
    // https://vitejs.dev/config/build-options.html
    build: {
      // Build Target
      // https://vitejs.dev/config/build-options.html#build-target
      target: ['es2021', 'chrome97', 'safari13'],
      // Minify option
      // https://vitejs.dev/config/build-options.html#build-minify
      minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
      sourcemap: !!process.env.TAURI_DEBUG,
      // Rollup Options
      // https://vitejs.dev/config/build-options.html#build-rollupoptions
      rollupOptions: {
        output: {
          manualChunks: (id: string) => {
            // Split external library from transpiled code.
            if (
              id.includes('/node_modules/vuetify') ||
              id.includes('/node_modules/webfontloader') ||
              id.includes('/node_modules/@mdi')
            ) {
              // Split Vuetify before vue.
              return 'vuetify';
            }
            if (
              id.includes('/node_modules/@vue/') ||
              id.includes('/node_modules/vue') ||
              id.includes('/node_modules/pinia')
            ) {
              // Combine Vue and Pinia into a single chunk.
              // This is because Pinia is a state management library for Vue.
              return 'vue';
            }
            // Others
            if (id.includes('/node_modules/')) {
              return 'vendor';
            }
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
    esbuild: {
      // Drop console when production build.
      drop: command === 'serve' ? [] : ['console'],
      supported: {
        'top-level-await': true //browsers can handle top-level-await features
      }
    }
  };

  // Write meta data only during build, not during dev to prevent watch loops
  if (command === 'build') {
    writeFileSync(
      fileURLToPath(new URL('./src/Meta.ts', import.meta.url)),

      `import type MetaInterface from '@/interfaces/MetaInterface';

// This file is auto-generated by the build system.
const meta: MetaInterface = {
  version: '${version}',
  date: '${new Date().toISOString()}',
};
export default meta;
`
    );
  }

  return config;
});
