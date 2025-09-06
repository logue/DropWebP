<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const settingsStore = useSettingsStore();
</script>

<template>
  <v-switch
    v-model="settingsStore.webpOptions.lossless"
    :label="t('lossless')"
    color="primary"
    inline
  />
  <v-slider
    v-model="settingsStore.webpOptions.quality"
    :disabled="settingsStore.webpOptions.lossless"
    :label="t('quality')"
    :max="100"
    :min="0"
    color="primary"
    hide-details
    step="1"
    thumb-label="always"
    type="number"
  />
  <v-btn prepend-icon="mdi-rotate-left" variant="text" @click="settingsStore.resetCommonOptions()">
    {{ t('reset_webp_options') }}
  </v-btn>
</template>

<i18n lang="yaml">
en:
  lossless: 'Lossless'
  quality: 'Quality (0-100)'
  reset_webp_options: 'Reset WebP Options'
ja:
  lossless: 'ロスレス'
  quality: '品質 (0-100)'
  reset_webp_options: 'WebPオプションをリセット'
kr:
  lossless: '무손실'
  quality: '품질 (0-100)'
  reset_webp_options: 'WebP 옵션 재설정'
zh:
  lossless: '無損'
  quality: '質量 (0-100)'
  reset_webp_options: '重置 WebP 選項'
</i18n>
