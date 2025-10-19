<script setup lang="ts">
import LocaleSelector from './LocaleSelector.vue';

import useConfigStore from '@/store/ConfigStore';

const { t } = useI18n();
/** Config Store */
const configStore = useConfigStore();
</script>

<template>
  <!-- Locale Menu -->
  <locale-selector />
  <!-- Toggle Dark mode -->
  <v-tooltip :text="t('toggle_dark_mode_btn')" location="bottom">
    <template #activator="{ props }">
      <v-btn
        v-bind="props"
        icon="mdi-theme-light-dark"
        variant="plain"
        @click="configStore.toggleTheme"
      />
    </template>
  </v-tooltip>
  <v-tooltip :text="t('github_btn')" location="bottom">
    <template #activator="{ props }">
      <v-btn
        v-bind="props"
        icon="mdi-github"
        variant="plain"
        href="http://github.com/logue/DropWebP"
        target="_blank"
      />
    </template>
  </v-tooltip>
</template>

<i18n lang="yaml">
en:
  locale_btn: Select Language
  toggle_dark_mode_btn: Toggle Dark Mode
  github_btn: GitHub Repository
fr:
  locale_btn: Choisir la langue
  toggle_dark_mode_btn: Basculer en mode sombre
  github_btn: Dépôt GitHub
ja:
  locale_btn: 言語を選択
  toggle_dark_mode_btn: ダークモード切り替え
  github_btn: GitHub リポジトリ
ko:
  locale_btn: 언어 선택
  toggle_dark_mode_btn: 다크 모드 전환
  github_btn: GitHub 저장소
zhHant:
  locale_btn: 選擇語言
  toggle_dark_mode_btn: 切換深色模式
  github_btn: GitHub 倉庫
zhHans:
  locale_btn: 选择语言
  toggle_dark_mode_btn: 切换深色模式
  github_btn: GitHub 仓库
</i18n>
