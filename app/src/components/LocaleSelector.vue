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
  kr: Korean
  zh: Chinese
  locale-changed: Locale {locale} has been changed.
ja:
  locale: 言語
  en: 英語
  ja: 日本語
  kr: 韓国語
  zh: 中国語
  locale-changed: 言語は{locale}に変更されました。
kr:
  locale: 언어
  en: 영어
  ja: 일본어
  kr: 한국어
  zh: 중국어
  locale-changed: 언어가 {locale}(으)로 변경되었습니다.
zh:
  locale: 語言
  en: 英語
  ja: 日語
  kr: 韓語
  zh: 中文
  locale-changed: 語言已更改為{locale}。
</i18n>
