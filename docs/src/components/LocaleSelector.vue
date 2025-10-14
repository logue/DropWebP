<template>
  <v-menu>
    <template v-slot:activator="{ props }">
      <v-btn icon v-bind="props">
        <v-icon>mdi-translate</v-icon>
      </v-btn>
    </template>

    <v-list>
      <v-list-item
        v-for="localeItem in availableLocales"
        :key="localeItem.code"
        @click="switchLanguage(localeItem.code)"
        :class="{ 'v-list-item--active': localeItem.code === locale }"
      >
        <v-list-item-title>{{ localeItem.name }}</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<script setup lang="ts">
const { locale, locales } = useI18n();
const switchLocalePath = useSwitchLocalePath();

// TypeScript対応のためのキャスト
const availableLocales = computed(() =>
  (locales.value as any[]).filter((l: any) => l.code !== locale.value)
);

const switchLanguage = async (code: string) => {
  await navigateTo(switchLocalePath(code as any));
};
</script>
