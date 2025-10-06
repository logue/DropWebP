/**
 * Root ESLint Config - Basic workspace settings only
 * Individual projects have their own eslint.config.ts
 */
export default [
  {
    name: 'root/workspace-ignore',
    ignores: [
      // App specific
      'app/dist/',
      'app/public/',
      'app/src-tauri/target/',
      // Docs specific
      'docs/.nuxt/',
      'docs/.output/',
      // Common
      '**/node_modules/',
      '**/.vite/',
      '**/.cache/',
      '**/*.d.ts',
      '**/coverage/'
    ]
  }
];
