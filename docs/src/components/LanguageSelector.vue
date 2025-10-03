<template>
  <v-menu>
    <template #activator="{ props }">
      <v-btn v-bind="props" variant="text" icon="mdi-translate">
        <v-icon>mdi-translate</v-icon>
      </v-btn>
    </template>
    <v-list>
      <v-list-item
        v-for="locale in availableLocales"
        :key="locale.code"
        :active="locale.code === currentLocale"
        @click="setLocale(locale.code)"
      >
        <v-list-item-title>{{ locale.name }}</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const { locale } = useI18n();
const route = useRoute();
// const router = useRouter();

const currentLocale = computed(() => locale.value);

const availableLocales = [
  { code: 'ja', name: '日本語' },
  { code: 'en', name: 'English' },
  { code: 'ko', name: '한국어' },
  { code: 'zh-tw', name: '繁體中文' }
];

const setLocale = async (newLocale: string) => {
  // i18nのロケールを更新
  locale.value = newLocale;

  // ローカルストレージに保存
  if (process.client) {
    localStorage.setItem('locale', newLocale);
  }

  // URLを言語対応に変更
  const currentPath = route.path;
  const supportedLocales = ['ja', 'en', 'ko', 'zh-tw'];

  // 現在のパスから言語コードを除去
  const pathSegments = currentPath.split('/').filter(Boolean);
  const firstSegment = pathSegments[0] || '';

  let cleanPath = currentPath;
  if (supportedLocales.includes(firstSegment)) {
    // 言語コードを除去したパスを作成
    cleanPath = '/' + pathSegments.slice(1).join('/');
  }

  // 新しい言語のURLを構築
  let newPath = cleanPath;
  if (newLocale !== 'ja') {
    // 日本語以外の場合は言語コードを追加
    newPath = `/${newLocale}${cleanPath}`;
  }

  // 同じパスの場合は何もしない
  if (newPath !== currentPath) {
    await navigateTo(newPath);
  }
};
</script>
