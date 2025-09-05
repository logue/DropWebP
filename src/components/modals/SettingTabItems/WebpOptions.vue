<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const settingsStore = useSettingsStore();
</script>

<template>
  <v-switch v-model="settingsStore.webpOptions.lossless" :label="t('lossless')" color="primary" />
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
  <v-btn @click="settingsStore.resetWebpOptions()">{{ t('reset_webp_options') }}</v-btn>
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
</i18n>
