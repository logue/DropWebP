import configPrettier from '@vue/eslint-config-prettier';
import { defineConfigWithVueTs, vueTsConfigs } from '@vue/eslint-config-typescript';

import markdown from '@eslint/markdown';
import comments from '@eslint-community/eslint-plugin-eslint-comments/configs';
import pluginVueI18n from '@intlify/eslint-plugin-vue-i18n';
// import pluginVitest from '@vitest/eslint-plugin';
import { globalIgnores } from 'eslint/config';
import pluginImport from 'eslint-plugin-import-x';
import pluginOxlint from 'eslint-plugin-oxlint';
// @ts-ignore
import pluginSecurity from 'eslint-plugin-security';
import pluginVue from 'eslint-plugin-vue';
import pluginVueA11y from 'eslint-plugin-vuejs-accessibility';
import pluginVuetify from 'eslint-plugin-vuetify';

import type { Linter } from 'eslint';

// Lint policy:
// 1) Keep oxlint + prettier as primary formatting/quick-check tools.
// 2) Keep ESLint focused on framework/type/import correctness.
// 3) Scope plugin presets to relevant file types to avoid cross-file crashes.
// 4) Restrict markdown lint to workspace instruction docs under .github.
// 5) Prefer small, explicit overrides over broad global exceptions.
const APP_FILES = ['**/*.{vue,ts,mts,tsx}'];
const VUE_FILES = ['*.vue', '**/*.vue'];
const MARKDOWN_FILES = ['.github/**/*.md'];
// const E2E_FILES = ['e2e/**/*.{test,spec}.{js,ts,jsx,tsx}'];
// const UNIT_TEST_FILES = ['src/**/__tests__/*'];
const GLOBAL_IGNORES = [
  '**/dist/**',
  '**/dist-ssr/**',
  '**/coverage/**',
  '.stylelintrc.yaml',
  'src/locales/**'
];

const scopeConfigsToFiles = (configs: Linter.Config[], files: string[]) =>
  configs.map(config => (config.files ? config : { ...config, files }));

const markdownRecommendedConfigs = markdown.configs.recommended.map(config => ({
  ...config,
  files: MARKDOWN_FILES
}));

const appRules: Linter.Config['rules'] = {
  '@eslint-community/eslint-comments/require-description': 'error',
  'no-unused-vars': 'off',
  '@typescript-eslint/array-type': ['error', { default: 'array' }],
  '@typescript-eslint/ban-ts-comment': 'off',
  '@typescript-eslint/consistent-generic-constructors': ['error', 'type-annotation'],
  '@typescript-eslint/consistent-type-imports': ['off', { prefer: 'type-imports' }],
  '@typescript-eslint/explicit-function-return-type': 'off',
  '@typescript-eslint/no-unused-vars': [
    'error',
    {
      args: 'all',
      argsIgnorePattern: '^_',
      caughtErrors: 'all',
      caughtErrorsIgnorePattern: '^_',
      destructuredArrayIgnorePattern: '^_',
      varsIgnorePattern: '^_',
      ignoreRestSiblings: true
    }
  ],
  '@typescript-eslint/strict-boolean-expressions': 'off',
  '@typescript-eslint/triple-slash-reference': 'off',
  'import-x/default': 'off',
  'import-x/namespace': 'off',
  'import-x/no-default-export': 'off',
  'import-x/no-named-as-default-member': 'off',
  'import-x/no-named-as-default': 'off',
  'import-x/order': [
    'error',
    {
      groups: ['builtin', 'external', 'parent', 'sibling', 'index', 'object', 'type'],
      pathGroups: [
        {
          pattern: '{vue,@/store,vue-i18n,pinia,vite,vitest,vitest/**,@vitejs/**,@vue/**}',
          group: 'external',
          position: 'before'
        },
        {
          pattern: '{@/**}',
          group: 'internal',
          position: 'before'
        }
      ],
      pathGroupsExcludedImportTypes: ['builtin'],
      alphabetize: { order: 'asc' },
      'newlines-between': 'always'
    }
  ],
  'import-x/no-relative-parent-imports': ['error', { ignore: ['^@/', '^~/'] }]
};

const appSettings = {
  'import-x/parsers': {
    espree: ['.js', '.cjs', '.mjs', '.jsx'],
    '@typescript-eslint/parser': ['.ts', '.tsx'],
    'vue-eslint-parser': ['.vue']
  },
  'import-x/resolver': {
    typescript: true,
    node: true,
    'eslint-import-resolver-custom-alias': {
      alias: { '@': './src', '~': './node_modules' },
      extensions: ['.js', '.ts', '.jsx', '.tsx', '.vue']
    }
  },
  'vue-i18n': {
    localeDir: './src/locales/*.{json,json5,yaml,yml}',
    messageSyntaxVersion: '^11.0.0'
  }
};

export default defineConfigWithVueTs(
  ...markdownRecommendedConfigs,

  globalIgnores(GLOBAL_IGNORES),

  ...scopeConfigsToFiles(pluginVue.configs['flat/recommended'], VUE_FILES),
  ...scopeConfigsToFiles(pluginVueA11y.configs['flat/recommended'], VUE_FILES),
  ...scopeConfigsToFiles(pluginVuetify.configs['flat/recommended'], VUE_FILES),
  ...scopeConfigsToFiles(pluginVueI18n.configs['flat/recommended'], VUE_FILES),
  vueTsConfigs.recommended,
  comments.recommended,

  { ...pluginImport.flatConfigs.recommended, files: APP_FILES },
  { ...pluginImport.flatConfigs.typescript, files: APP_FILES },
  { ...pluginSecurity.configs.recommended, files: APP_FILES },
  {
    name: 'app/rules',
    files: APP_FILES,
    settings: appSettings,
    rules: appRules
  },
  {
    name: 'vue/rules',
    files: VUE_FILES,
    rules: {
      'vue/component-api-style': ['error', ['script-setup']],
      'vue/define-props-declaration': ['error', 'type-based'],
      'vue/define-emits-declaration': ['error', 'type-based'],
      'vue/enforce-style-attribute': ['error', { allow: ['scoped'] }],
      'vue/attributes-order': [
        'warn',
        {
          order: [
            'DEFINITION',
            'LIST_RENDERING',
            'CONDITIONALS',
            'RENDER_MODIFIERS',
            'UNIQUE',
            'TWO_WAY_BINDING',
            'OTHER_DIRECTIVES',
            'ATTR_DYNAMIC',
            'ATTR_STATIC',
            'ATTR_SHORTHAND_BOOL',
            'EVENTS',
            'CONTENT'
          ],
          alphabetical: false
        }
      ],
      'vue/html-self-closing': ['error', { html: { void: 'always' } }],
      'vue/multi-word-component-names': 'warn',
      'vuejs-accessibility/label-has-for': [
        'error',
        {
          components: ['VLabel'],
          controlComponents: ['VInput'],
          required: { some: ['nesting', 'id'] }
        }
      ],
      'vuejs-accessibility/no-autofocus': 'warn',
      'vuejs-accessibility/anchor-has-content': 'error',
      '@intlify/vue-i18n/no-missing-keys': 'off'
    }
  },
  {
    name: 'vue/components-strict',
    files: ['src/components/**/*.vue'],
    rules: { 'vue/multi-word-component-names': 'error' }
  },
  {
    name: 'app/root-style-exception',
    files: ['src/App.vue'],
    rules: { 'vue/enforce-style-attribute': 'off' }
  },

  ...pluginOxlint.buildFromOxlintConfigFile('../.oxlintrc.json'),
  configPrettier,
  {
    name: 'markdown/final-overrides',
    files: MARKDOWN_FILES,
    language: 'markdown/gfm',
    rules: { 'prettier/prettier': 'off' }
  }
);
