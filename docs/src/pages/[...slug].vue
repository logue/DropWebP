<script setup lang="ts">
import type { Collections } from '@nuxt/content';
import { useI18n } from 'vue-i18n';

import { withLeadingSlash } from 'ufo';

const route = useRoute();
const { locale, t } = useI18n();
const slug = computed(() => withLeadingSlash(String(route.params.slug)));

const { data: page } = await useAsyncData(
  // キーにロケールを含めてSSG時に各言語で別々にキャッシュされるようにする
  () => `page-${locale.value}-${slug.value}`,
  async () => {
    // Build collection name based on current locale
    const collection = ('content_' + locale.value) as keyof Collections;
    const content = await queryCollection(collection).path(slug.value).first();

    // Optional: fallback to default locale if content is missing
    if (!content && locale.value !== 'en') {
      return await queryCollection('content_en').path(slug.value).first();
    }

    return content;
  },
  {
    watch: [locale] // Refetch when locale changes
  }
);
</script>

<template>
  <content-renderer v-if="page" :value="page" class="markdown-body" tag="article" />
  <v-alert v-else :title="t('error')" type="error" variant="tonal">
    {{ t('error_description', { locale }) }}
  </v-alert>
</template>

<style lang="scss">
@use 'sass:map';
/* Optimized GitHub Markdown Styles with SCSS and Vuetify Dark Mode Support */
// Color variables for light and dark themes
$colors-light: (
  link: #0969da,
  text: #1f2328,
  text-secondary: #59636e,
  bg: #ffffff,
  bg-secondary: #f6f8fa,
  border: #d1d9e0,
  border-alpha: #d1d9e0b3,
  code-bg: #818b981f,
  mark-bg: #fff8c5,
  kbd-bg: #f6f8fa,
  focus: #0969da,
  alert-note: #0969da,
  alert-tip: #1a7f37,
  alert-important: #8250df,
  alert-warning: #9a6700,
  alert-caution: #cf222e
);

$colors-dark: (
  link: #4493f8,
  text: #f0f6fc,
  text-secondary: #9198a1,
  bg: #0d1117,
  bg-secondary: #151b23,
  border: #3d444d,
  border-alpha: #3d444db3,
  code-bg: #656c7633,
  mark-bg: #bb800926,
  kbd-bg: #151b23,
  focus: #1f6feb,
  alert-note: #4493f8,
  alert-tip: #3fb950,
  alert-important: #ab7df8,
  alert-warning: #d29922,
  alert-caution: #f85149
);

// Mixin for theme colors
@mixin theme-colors($theme-colors) {
  --md-link: #{map.get($theme-colors, link)};
  --md-text: #{map.get($theme-colors, text)};
  --md-text-secondary: #{map.get($theme-colors, text-secondary)};
  --md-bg: #{map.get($theme-colors, bg)};
  --md-bg-secondary: #{map.get($theme-colors, bg-secondary)};
  --md-border: #{map.get($theme-colors, border)};
  --md-border-alpha: #{map.get($theme-colors, border-alpha)};
  --md-code-bg: #{map.get($theme-colors, code-bg)};
  --md-mark-bg: #{map.get($theme-colors, mark-bg)};
  --md-kbd-bg: #{map.get($theme-colors, kbd-bg)};
  --md-focus: #{map.get($theme-colors, focus)};
  --md-alert-note: #{map.get($theme-colors, alert-note)};
  --md-alert-tip: #{map.get($theme-colors, alert-tip)};
  --md-alert-important: #{map.get($theme-colors, alert-important)};
  --md-alert-warning: #{map.get($theme-colors, alert-warning)};
  --md-alert-caution: #{map.get($theme-colors, alert-caution)};
}

// Apply light theme by default
:root {
  @include theme-colors($colors-light);
}

// Apply dark theme when Vuetify dark mode is active
.v-theme--dark {
  @include theme-colors($colors-dark);
}

.markdown-body {
  color-scheme: light;
  overflow-wrap: break-word;
  color: var(--md-text);

  // Dark mode
  .v-theme--dark & {
    color-scheme: dark;
  }

  // Base elements
  .octicon {
    display: inline-block;
    fill: currentColor;
    vertical-align: text-bottom;
  }

  details,
  figcaption,
  figure {
    display: block;
  }

  summary {
    display: list-item;
  }

  [hidden] {
    display: none !important;
  }

  // Links
  a {
    background-color: transparent;
    color: var(--md-link);
    text-decoration: none;

    &:hover {
      text-decoration: underline;
    }

    &:not([href]) {
      color: inherit;
      text-decoration: none;
    }

    &:focus,
    &:focus-visible {
      outline: 2px solid var(--md-focus);
      outline-offset: -2px;
      box-shadow: none;
    }

    &:focus:not(:focus-visible) {
      outline: solid 1px transparent;
    }

    &:not([class]):focus,
    &:not([class]):focus-visible {
      outline-offset: 0;
    }
  }

  // Text formatting
  abbr[title] {
    border-bottom: none;
    text-decoration: underline dotted;
  }

  b,
  strong {
    font-weight: 600;
  }

  mark {
    background-color: var(--md-mark-bg);
    color: var(--md-text);
  }

  small {
    font-size: 90%;
  }

  sub,
  sup {
    font-size: 75%;
    line-height: 0;
    position: relative;
    vertical-align: baseline;
  }

  sub {
    bottom: -0.25em;
  }

  sup {
    top: -0.5em;
  }

  // Images
  img {
    border-style: none;
    max-width: 100%;
    box-sizing: content-box;
    background-color: var(--md-bg);

    &[align='right'] {
      padding-left: 20px;
    }

    &[align='left'] {
      padding-right: 20px;
    }
  }

  .emoji {
    max-width: none;
    vertical-align: text-top;
    background-color: transparent;
  }

  // Typography
  h1,
  h2,
  h3,
  h4,
  h5,
  h6 {
    margin-top: 1.5rem;
    margin-bottom: 1rem;
    font-weight: 600;
    line-height: 1.25;

    .octicon-link {
      color: var(--md-text);
      vertical-align: middle;
      visibility: hidden;
    }

    &:hover .anchor {
      text-decoration: none;

      .octicon-link {
        visibility: visible;

        &:before {
          width: 16px;
          height: 16px;
          content: ' ';
          display: inline-block;
          background-color: currentColor;
          -webkit-mask-image: url("data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16' version='1.1' aria-hidden='true'><path fill-rule='evenodd' d='M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z'></path></svg>");
          mask-image: url("data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16' version='1.1' aria-hidden='true'><path fill-rule='evenodd' d='M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z'></path></svg>");
        }
      }
    }

    tt,
    code {
      padding: 0 0.2em;
      font-size: inherit;
    }
  }

  h1 {
    margin: 0.67em 0;
    padding-bottom: 0.3em;
    font-size: 2em;
    border-bottom: 1px solid var(--md-border-alpha);
  }

  h2 {
    padding-bottom: 0.3em;
    font-size: 1.5em;
    border-bottom: 1px solid var(--md-border-alpha);
  }

  h3 {
    font-size: 1.25em;
  }

  h4 {
    font-size: 1em;
  }

  h5 {
    font-size: 0.875em;
  }

  h6 {
    font-size: 0.85em;
    color: var(--md-text-secondary);
  }

  p {
    margin-top: 0;
    margin-bottom: 10px;
  }

  blockquote {
    margin: 0;
    padding: 0 1em;
    color: var(--md-text-secondary);
    border-left: 0.25em solid var(--md-border);

    > :first-child {
      margin-top: 0;
    }

    > :last-child {
      margin-bottom: 0;
    }
  }

  // Lists
  ul,
  ol {
    margin-top: 0;
    margin-bottom: 0;
    padding-left: 2em;

    ul,
    ol {
      margin-top: 0;
      margin-bottom: 0;
    }
  }

  ol ol,
  ul ol {
    list-style-type: lower-roman;
  }

  ul ul ol,
  ul ol ol,
  ol ul ol,
  ol ol ol {
    list-style-type: lower-alpha;
  }

  ul.no-list,
  ol.no-list {
    padding: 0;
    list-style-type: none;
  }

  ol[type='a s'] {
    list-style-type: lower-alpha;
  }

  ol[type='A s'] {
    list-style-type: upper-alpha;
  }

  ol[type='i s'] {
    list-style-type: lower-roman;
  }

  ol[type='I s'] {
    list-style-type: upper-roman;
  }

  ol[type='1'],
  div > ol:not([type]) {
    list-style-type: decimal;
  }

  li {
    > p {
      margin-top: 1rem;
    }

    + li {
      margin-top: 0.25em;
    }
  }

  dl {
    padding: 0;

    dt {
      padding: 0;
      margin-top: 1rem;
      font-size: 1em;
      font-style: italic;
      font-weight: 600;
    }

    dd {
      padding: 0 1rem;
      margin-bottom: 1rem;
      margin-left: 0;
    }
  }

  // Code
  code,
  kbd,
  pre,
  samp,
  tt {
    font-family: ui-monospace, 'Noto Sans Mono', monospace;
  }

  code,
  kbd,
  pre,
  samp {
    font-size: 1em;
  }

  tt,
  code,
  samp {
    font-size: 12px;
  }

  code,
  tt {
    padding: 0.2em 0.4em;
    margin: 0;
    font-size: 85%;
    white-space: break-spaces;
    background-color: var(--md-code-bg);
    border-radius: 6px;

    br {
      display: none;
    }
  }

  del code {
    text-decoration: inherit;
  }

  samp {
    font-size: 85%;
  }

  pre {
    margin-top: 0;
    margin-bottom: 0;
    font-size: 12px;
    overflow-wrap: normal;

    code {
      font-size: 100%;
    }

    > code {
      padding: 0;
      margin: 0;
      word-break: normal;
      white-space: pre;
      background: transparent;
      border: 0;
    }
  }

  .highlight pre,
  pre {
    padding: 1rem;
    overflow: auto;
    font-size: 85%;
    line-height: 1.45;
    color: var(--md-text);
    background-color: var(--md-bg-secondary);
    border-radius: 6px;
  }

  .highlight {
    margin-bottom: 1rem;

    pre {
      margin-bottom: 0;
      word-break: normal;
    }
  }

  pre code,
  pre tt {
    display: inline;
    max-width: auto;
    padding: 0;
    margin: 0;
    overflow: visible;
    line-height: inherit;
    overflow-wrap: normal;
    background-color: transparent;
    border: 0;
  }

  kbd {
    display: inline-block;
    padding: 0.25rem;
    font:
      11px ui-monospace,
      'Noto Sans Mono',
      monospace;
    line-height: 10px;
    color: var(--md-text);
    vertical-align: middle;
    background-color: var(--md-kbd-bg);
    border: solid 1px var(--md-border-alpha);
    border-bottom-color: var(--md-border-alpha);
    border-radius: 6px;
    box-shadow: inset 0 -1px 0 var(--md-border-alpha);
  }

  // Tables
  table {
    border-spacing: 0;
    border-collapse: collapse;
    display: block;
    width: max-content;
    max-width: 100%;
    overflow: auto;

    th {
      font-weight: 600;
    }

    th,
    td {
      padding: 6px 13px;
      border: 1px solid var(--md-border);
    }

    td > :last-child {
      margin-bottom: 0;
    }

    tr {
      background-color: var(--md-bg);
      border-top: 1px solid var(--md-border-alpha);

      &:nth-child(2n) {
        background-color: var(--md-bg-secondary);
      }
    }

    img {
      background-color: transparent;
    }
  }

  // Horizontal rule
  hr {
    box-sizing: content-box;
    overflow: hidden;
    background: transparent;
    border-bottom: 1px solid var(--md-border-alpha);
    height: 0.25em;
    padding: 0;
    margin: 1.5rem 0;
    background-color: var(--md-border);
    border: 0;

    &::before,
    &::after {
      display: table;
      content: '';
    }

    &::after {
      clear: both;
    }
  }

  // Forms
  input {
    font: inherit;
    margin: 0;
    overflow: visible;
    font-family: inherit;
    font-size: inherit;
    line-height: inherit;

    &::-webkit-outer-spin-button,
    &::-webkit-inner-spin-button {
      margin: 0;
      appearance: none;
    }
  }

  [type='button'],
  [type='reset'],
  [type='submit'] {
    appearance: auto;
  }

  [type='checkbox'],
  [type='radio'] {
    box-sizing: border-box;
    padding: 0;

    &:focus,
    &:focus-visible {
      outline: 2px solid var(--md-focus);
      outline-offset: -2px;
      box-shadow: none;
    }

    &:focus:not(:focus-visible) {
      outline: solid 1px transparent;
    }

    &:focus,
    &:focus-visible {
      outline-offset: 0;
    }
  }

  [type='number']::-webkit-inner-spin-button,
  [type='number']::-webkit-outer-spin-button {
    height: auto;
  }

  [type='search']::-webkit-search-cancel-button,
  [type='search']::-webkit-search-decoration {
    appearance: none;
  }

  ::-webkit-input-placeholder {
    color: inherit;
    opacity: 0.54;
  }

  ::-webkit-file-upload-button {
    appearance: auto;
    font: inherit;
  }

  ::placeholder {
    color: var(--md-text-secondary);
    opacity: 1;
  }

  // Misc elements
  figure {
    margin: 1em 2.5rem;
  }

  td,
  th {
    padding: 0;
  }

  details summary {
    cursor: pointer;
  }

  .anchor {
    float: left;
    padding-right: 0.25rem;
    margin-left: -20px;
    line-height: 1;

    &:focus {
      outline: none;
    }
  }

  .mr-2 {
    margin-right: 0.5rem !important;
  }

  // Clearfix
  &::before,
  &::after {
    display: table;
    content: '';
  }

  &::after {
    clear: both;
  }

  // First and last child spacing
  > *:first-child {
    margin-top: 0 !important;
  }

  > *:last-child {
    margin-bottom: 0 !important;
  }

  // Common element spacing
  p,
  blockquote,
  ul,
  ol,
  dl,
  table,
  pre,
  details {
    margin-top: 0;
    margin-bottom: 1rem;
  }

  // Summary headings
  summary {
    h1,
    h2,
    h3,
    h4,
    h5,
    h6 {
      display: inline-block;

      .anchor {
        margin-left: -40px;
      }
    }

    h1,
    h2 {
      padding-bottom: 0;
      border-bottom: 0;
    }
  }

  // Markdown alerts
  .markdown-alert {
    padding: 0.5rem 1rem;
    margin-bottom: 1rem;
    color: inherit;
    border-left: 0.25em solid var(--md-border);

    > :first-child {
      margin-top: 0;
    }

    > :last-child {
      margin-bottom: 0;
    }

    .markdown-alert-title {
      display: flex;
      font-weight: 500;
      align-items: center;
      line-height: 1;
    }

    &.markdown-alert-note {
      border-left-color: var(--md-alert-note);

      .markdown-alert-title {
        color: var(--md-alert-note);
      }
    }

    &.markdown-alert-important {
      border-left-color: var(--md-alert-important);

      .markdown-alert-title {
        color: var(--md-alert-important);
      }
    }

    &.markdown-alert-warning {
      border-left-color: var(--md-alert-warning);

      .markdown-alert-title {
        color: var(--md-alert-warning);
      }
    }

    &.markdown-alert-tip {
      border-left-color: var(--md-alert-tip);

      .markdown-alert-title {
        color: var(--md-alert-tip);
      }
    }

    &.markdown-alert-caution {
      border-left-color: var(--md-alert-caution);

      .markdown-alert-title {
        color: var(--md-alert-caution);
      }
    }
  }

  // Task lists
  .task-list-item {
    list-style-type: none;

    label {
      font-weight: 400;
    }

    &.enabled label {
      cursor: pointer;
    }

    + .task-list-item {
      margin-top: 0.25rem;
    }

    .handle {
      display: none;
    }

    &-checkbox {
      margin: 0 0.2em 0.25em -1.4em;
      vertical-align: middle;
    }
  }

  ul:dir(rtl) .task-list-item-checkbox,
  ol:dir(rtl) .task-list-item-checkbox {
    margin: 0 -1.6em 0.25em 0.2em;
  }

  // Emoji
  g-emoji {
    display: inline-block;
    min-width: 1ch;
    font-family:
      'Apple Color Emoji', 'Segoe UI Emoji', 'Noto Color Emoji', 'Segoe UI Symbol', sans-serif;
    font-size: 1em;
    font-style: normal !important;
    font-weight: 400;
    line-height: 1;
    vertical-align: -0.075em;

    img {
      width: 1em;
      height: 1em;
    }
  }

  // Focus styles
  [role='button'],
  button,
  summary,
  a {
    &:focus:not(:focus-visible) {
      outline: none;
      box-shadow: none;
    }
  }

  [role='button'],
  [role='tabpanel'][tabindex='0'],
  [tabindex='0'],
  details-dialog {
    &:focus:not(:focus-visible) {
      outline: none;
    }
  }

  // Other
  > *:first-child > .heading-element:first-child {
    margin-top: 0 !important;
  }
}
</style>

<i18n lang="yaml">
en:
  error: Page not found
  error_description: "This page doesn't exist in {locale} language."
fr:
  error: Page non trouvée
  error_description: "Cette page n'existe pas dans la langue {locale}."
ja:
  error: ページが見つかりません
  error_description: このページは{locale}言語に存在しません。
ko:
  error: 페이지를 찾을 수 없습니다
  error_description: 이 페이지는 {locale} 언어에 존재하지 않습니다.
zhHant:
  error: 頁面未找到
  error_description: 此頁面在{locale}語言中不存在。
zhHans:
  error: 页面未找到
  error_description: 此页面在{locale}语言中不存在。
</i18n>
