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
        <v-tooltip :text="t('locale')" activator="parent" location="bottom" />
      </v-btn>
    </template>
    <v-list density="compact">
      <locale-selector />
    </v-list>
  </v-menu>
  <!-- Toggle Dark mode -->
  <v-tooltip :text="t('toggle-dark-mode')" location="bottom">
    <template #activator="{ props }">
      <v-btn
        v-bind="props"
        icon="mdi-theme-light-dark"
        variant="plain"
        @click="configStore.toggleTheme"
      />
    </template>
  </v-tooltip>
  <settings-dialog />
</template>

<i18n lang="yaml">
en:
  title: 'Drop Compress Image'
  about: 'About {appname}'
  locale: 'Select Language'
  toggle-dark-mode: 'Toggle Dark Mode'
ja:
  title: 'Drop Compress Image'
  about: '{appname}について'
  locale: '言語を選択'
  toggle-dark-mode: 'ダークモードを切り替え'
</i18n>
