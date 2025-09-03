<script setup lang="ts">
import { useConfigStore } from '@/store';
import { type Ref, ref } from 'vue';
import { useI18n } from 'vue-i18n';

import AboutModal from './AboutModal.vue';
import LocaleSelector from './LocaleSelector.vue';

const { t } = useI18n();
/** Config Store */
const configStore = useConfigStore();

/** アバウトモーダル */
const aboutModal: Ref<InstanceType<typeof AboutModal> | undefined> = ref();

/** アバウトを表示 */
const showAbout = () => aboutModal.value?.open();
</script>

<template>
  <!-- About -->
  <v-tooltip :text="t('about', { appname: t('title') })" location="bottom">
    <template #activator="{ props }">
      <v-btn v-bind="props" icon="mdi-information-outline" variant="plain" @click="showAbout" />
    </template>
  </v-tooltip>
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
  <about-modal ref="aboutModal" />
</template>

<i18n lang="yaml">
en:
  title: 'DropWebP'
  about: 'About {appname}'
  locale: 'Select Language'
  toggle-dark-mode: 'Toggle Dark Mode'
ja:
  title: 'DropWebP'
  about: '{appname}について'
  locale: '言語を選択'
  toggle-dark-mode: 'ダークモードを切り替え'
</i18n>
