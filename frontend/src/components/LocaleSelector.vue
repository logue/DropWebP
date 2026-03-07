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
  <v-menu location="bottom">
    <template #activator="{ props }">
      <v-btn v-bind="props" icon variant="plain">
        <v-icon>mdi-translate</v-icon>
        <v-tooltip :text="t('locale')" activator="parent" location="bottom" />
      </v-btn>
    </template>
    <v-list density="compact" mandatory>
      <v-list-item
        v-for="lang in availableLocales"
        :key="lang"
        :active="configStore.locale === lang"
        :title="t(`${lang}`)"
        @click="changeLocale(lang)"
      />
    </v-list>
  </v-menu>
</template>

<i18n lang="yaml">
en:
  locale: Locale
  en: 🇺🇸 English
  fr: 🇫🇷 French
  ja: 🇯🇵 Japanese
  ko: 🇰🇷 Korean
  zhHant: 🇹🇼 Traditional Chinese
  zhHans: 🇨🇳 Simplified Chinese
  locale-changed: Locale {locale} has been changed.
fr:
  locale: Langue
  en: 🇺🇸 Anglais
  fr: 🇫🇷 Français
  ja: 🇯🇵 Japonais
  ko: 🇰🇷 Coréen
  zhHant: 🇹🇼 Chinois traditionnel
  zhHans: 🇨🇳 Chinois simplifié
  locale-changed: La langue a été changée en {locale}.
ja:
  locale: 言語
  en: 🇺🇸 英語
  fr: 🇫🇷 フランス語
  ja: 🇯🇵 日本語
  ko: 🇰🇷 韓国語
  zhHant: 🇹🇼 繁体字中国語
  zhHans: 🇨🇳 簡体字中国語
  locale-changed: 言語は{locale}に変更されました。
ko:
  locale: 언어
  en: 🇺🇸 영어
  fr: 🇫🇷 프랑스어
  ja: 🇯🇵 일본어
  ko: 🇰🇷 한국어
  zhHant: 🇹🇼 번체 중국어
  zhHans: 🇨🇳 간체 중국어
  locale-changed: 언어가 {locale}(으)로 변경되었습니다.
zhHant:
  locale: 語言
  en: 🇺🇸 英文
  fr: 🇫🇷 法文
  ja: 🇯🇵 日文
  ko: 🇰🇷 韓文
  zhHant: 🇹🇼 繁體中文
  zhHans: 🇨🇳 簡體中文
  locale-changed: 語言已更改為 {locale}。
zhHans:
  locale: 语言
  en: 🇺🇸 英语
  fr: 🇫🇷 法语
  ja: 🇯🇵 日语
  ko: 🇰🇷 韩语
  zhHant: 🇹🇼 繁体中文
  zhHans: 🇨🇳 简体中文
  locale-changed: 语言已更改为 {locale}。
</i18n>
