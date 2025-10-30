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
  <!-- Settings -->
  <settings-dialog />
</template>

<i18n lang="yaml">
en:
  toggle_dark_mode_btn: Toggle Dark Mode
fr:
  toggle_dark_mode_btn: Basculer en mode sombre
ja:
  toggle_dark_mode_btn: ダークモード切り替え
ko:
  toggle_dark_mode_btn: 다크 모드 전환
zhHant:
  toggle_dark_mode_btn: 切換深色模式
zhHans:
  toggle_dark_mode_btn: 切换深色模式
</i18n>
