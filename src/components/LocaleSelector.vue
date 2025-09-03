<script setup lang="ts">
/** 言語セレクター */
import { useConfigStore } from '@/store';
import { useI18n } from 'vue-i18n';

/** vue i18n */
const { t, availableLocales } = useI18n();
/** グローバルストア */
const configStore = useConfigStore();
// localeの変更はストアのアクション経由で行う
function changeLocale(newLocale: string) {
  configStore.setLocale(newLocale);
}
</script>

<template>
  <v-list mandatory>
    <v-list-item
      v-for="lang in availableLocales"
      :key="lang"
      :active="configStore.locale === lang"
      @click="changeLocale(lang)"
    >
      <v-list-item-title>
        {{ t(`${lang}`) }}
      </v-list-item-title>
    </v-list-item>
  </v-list>
</template>

<i18n lang="yaml">
en:
  locale: Locale
  en: English
  ja: Japanese
  locale-changed: Locale {locale} has been changed.
ja:
  locale: 言語
  en: 英語
  ja: 日本語
  locale-changed: 言語は{locale}に変更されました。
</i18n>
