<script setup lang="ts">
import { useConfigStore } from '@/store';
import { useI18n } from 'vue-i18n';

import LocaleSelector from './LocaleSelector.vue';
import AboutDialog from './modals/AboutDialog.vue';
import SettingsDialog from './modals/SettingsDialog.vue';

const { t } = useI18n();
/** Config Store */
const configStore = useConfigStore();
</script>

<template>
  <!-- About -->
  <about-dialog />
  <!-- Locale Menu -->
  <v-menu location="bottom">
    <template #activator="{ props }">
      <v-btn v-bind="props" icon variant="plain">
        <v-icon>mdi-translate</v-icon>
        <v-tooltip :text="t('locale_btn')" activator="parent" location="bottom" />
      </v-btn>
    </template>
    <v-list density="compact">
      <locale-selector :tooltip="t('locale_btn')" />
    </v-list>
  </v-menu>
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
  <settings-dialog :tooltip="t('settings_btn')" />
</template>

<i18n lang="yaml">
en:
  locale_btn: Select Language
  toggle_dark_mode_btn: Toggle Dark Mode
  settings_btn: Settings
fr:
  locale_btn: Choisir la langue
  toggle_dark_mode_btn: Basculer en mode sombre
  settings_btn: Paramètres
ja:
  locale_btn: 言語を選択
  toggle_dark_mode_btn: ダークモード切り替え
  settings_btn: 設定
ko:
  locale_btn: 언어 선택
  toggle_dark_mode_btn: 다크 모드 전환
  settings_btn: 설정
zhHant:
  locale_btn: 選擇語言
  toggle_dark_mode_btn: 切換深色模式
  settings_btn: 設定
zhHans:
  locale_btn: 选择语言
  toggle_dark_mode_btn: 切换深色模式
  settings_btn: 设置
</i18n>
