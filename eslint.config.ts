/**
 * Root ESLint Config - Basic workspace settings only
 * Individual projects have their own eslint.config.ts
 */
export default [
  {
    name: 'root/workspace-ignore',
    ignores: [
      // App specific
      'frontend/dist/',
      'frontend/public/',
      'backend/target/',
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
