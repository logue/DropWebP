<script setup lang="ts">
import { useI18nLocale } from '@/composables/useI18nLocale';
import { SupportedLocales, type SupportedLocale } from '@/types/SupportedLocales';

// useI18nLocale composableを使用
const { currentLocale, setLocaleAndNavigate } = useI18nLocale();

const setLocale = async (newLocale: SupportedLocale) => {
  await setLocaleAndNavigate(newLocale);
};
</script>

<template>
  <v-menu>
    <template #activator="{ props }">
      <v-btn v-bind="props" variant="plain" icon="mdi-translate" />
    </template>
    <v-list>
      <v-list-item
        v-for="locale of SupportedLocales"
        :key="locale"
        :active="locale === currentLocale"
        @click="setLocale(locale)"
      >
        <v-list-item-title>{{ t(locale) }}</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<i18n lang="yaml">
en:
  en: 🇺🇸 English
  fr: 🇫🇷 French
  ja: 🇯🇵 Japanese
  ko: 🇰🇷 Korean
  zhHant: 🇹🇼 Traditional Chinese
  zhHans: 🇨🇳 Simplified Chinese
ja:
  en: 🇺🇸 英語
  fr: 🇫🇷 フランス語
  ja: 🇯🇵 日本語
  ko: 🇰🇷 韓国語
  zhHant: 🇹🇼 繁体字中国語
  zhHans: 🇨🇳 简体字中国語
ko:
  en: 🇺🇸 영어
  fr: 🇫🇷 프랑스어
  ja: 🇯🇵 일본어
  ko: 🇰🇷 한국어
  zhHant: 🇹🇼 번체자 중국어
  zhHans: 🇨🇳 간체자 중국어
zhHant:
  en: 🇺🇸 英語
  fr: 🇫🇷 法語
  ja: 🇯🇵 日語
  ko: 🇰🇷 韓語
  zhHant: 🇹🇼 繁体字中文
  zhHans: 🇨🇳 簡体字中文
zhHans:
  en: 🇺🇸 英语
  fr: 🇫🇷 法语
  ja: 🇯🇵 日语
  ko: 🇰🇷 韩语
  zhHant: 🇹🇼 繁体字中文
  zhHans: 🇨🇳 简体字中文
</i18n>
