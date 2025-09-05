<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

import { BitDepth, ColorModel, AlphaColorMode } from '@/types/AvifTypes';

const { t } = useI18n();
const settingsStore = useSettingsStore();
</script>

<template>
  <v-select
    v-model="settingsStore.avifOptions.bitDepth"
    :items="[
      { text: t('bit_depth_8'), value: BitDepth.Eight },
      { text: t('bit_depth_10'), value: BitDepth.Ten },
      { text: t('bit_depth_auto'), value: BitDepth.Auto }
    ]"
    :label="t('bit_depth')"
    item-title="text"
    item-value="value"
  />
  <v-slider
    v-model="settingsStore.avifOptions.quality"
    :label="t('quality')"
    :max="100"
    :min="0"
    color="primary"
    step="1"
    thumb-label="always"
    type="number"
  />
  <v-slider
    v-model="settingsStore.avifOptions.alphaQuality"
    :label="t('alpha_quality')"
    :max="100"
    :min="0"
    color="primary"
    step="1"
    thumb-label="always"
  />
  <v-select
    v-model="settingsStore.avifOptions.alphaColorMode"
    :items="[
      { text: 'UnassociatedDirty', value: AlphaColorMode.UnassociatedDirty },
      { text: 'UnassociatedClean', value: AlphaColorMode.UnassociatedClean },
      { text: 'Premultiplied', value: AlphaColorMode.Premultiplied }
    ]"
    :label="t('alpha_color_mode')"
    item-title="text"
    item-value="value"
  />
  <v-slider
    v-model="settingsStore.avifOptions.speed"
    :label="t('speed')"
    :max="10"
    :min="0"
    color="primary"
    step="1"
    thumb-label="always"
    type="number"
  />
  <v-select
    v-model="settingsStore.avifOptions.colorModel"
    :items="[
      { text: 'YCbCr', value: ColorModel.YCbCr },
      { text: 'RGB', value: ColorModel.RGB }
    ]"
    :label="t('color_model')"
    item-title="text"
    item-value="value"
  />
  <v-number-input
    v-model="settingsStore.avifOptions.threads"
    :label="t('threads')"
    type="number"
    clearable
    :min="1"
  />
  <v-btn @click="settingsStore.resetAvifOptions()">{{ t('reset_avif_options') }}</v-btn>
</template>

<i18n lang="yaml">
en:
  bit_depth: 'Bit Depth'
  bit_depth_8: '8-bit'
  bit_depth_10: '10-bit'
  bit_depth_auto: 'Auto'
  quality: 'Quality (0-100)'
  alpha_quality: 'Alpha Channel Quality (0-100)'
  alpha_color_mode: 'Alpha Color Mode'
  speed: 'Encoding Speed (0-10)'
  color_model: 'Color Model'
  threads: 'Max Threads to Use (Leave Blank for Auto)'
  reset_avif_options: 'Reset AVIF Options'
ja:
  bit_depth: 'ビット深度'
  bit_depth_8: '8ビット'
  bit_depth_10: '10ビット'
  bit_depth_auto: '自動'
  quality: '品質 (0-100)'
  alpha_quality: 'アルファチャンネルの品質 (0-100)'
  alpha_color_mode: 'アルファカラーモード'
  speed: 'エンコード速度 (0-10)'
  color_model: 'カラーモデル'
  threads: '最大スレッド数 (空欄で自動設定)'
  reset_avif_options: 'AVIFオプションをリセット'
</i18n>
