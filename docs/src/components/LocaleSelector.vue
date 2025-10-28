<template>
  <v-menu>
    <template #activator="{ props }">
      <v-btn icon="mdi-translate" variant="plain" v-bind="props" />
    </template>

    <v-list>
      <v-list-item
        v-for="localeItem in availableLocales"
        :key="localeItem.code"
        :class="{ 'v-list-item--active': localeItem.code === locale }"
        @click="switchLanguage(localeItem.code)"
      >
        <v-list-item-title>{{ localeItem.name }}</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<script setup lang="ts">
import type { Locale } from '@/types/LocaleType';

const { locale, locales } = useI18n();
const switchLocalePath = useSwitchLocalePath();

// TypeScript対応のためのキャスト
const availableLocales = computed(() =>
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  (locales.value as any[]).filter((l: any) => l.code !== locale.value)
);

const switchLanguage = async (code: Locale) => {
  await navigateTo(switchLocalePath(code));
};
</script>
