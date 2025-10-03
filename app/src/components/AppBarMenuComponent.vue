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
  title: 'Drop Compress Image'
  about_btn: 'About {appname}'
  locale_btn: 'Select Language'
  toggle_dark_mode_btn: 'Toggle Dark Mode'
  setttings_btn: 'Settings'
ja:
  title: 'Drop Compress Image'
  about_btn: '{appname}について'
  locale_btn: '言語を選択'
  toggle_dark_mode_btn: 'ダークモードを切り替え'
  settings_btn: '設定'
kr:
  title: 'Drop Compress Image'
  about_btn: '{appname} 정보'
  locale_btn: '언어 선택'
  toggle_dark_mode_btn: '다크 모드 전환'
  settings_btn: '설정'
zh:
  title: 'Drop Compress Image'
  about_btn: '關於 {appname}'
  locale_btn: '選擇語言'
  toggle_dark_mode_btn: '切換深色模式'
  settings_btn: '設置'
</i18n>
