<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

import { BitDepth, ColorModel, AlphaColorMode } from '@/types/AvifTypes';

const { t } = useI18n();
const settingsStore = useSettingsStore();
const isLossless = ref(false);
</script>

<template>
  <v-switch v-model="isLossless" :label="t('lossless')" color="primary" disabled inline />
  <v-slider
    v-model="settingsStore.avifOptions.quality"
    :disabled="isLossless"
    :label="t('quality')"
    :max="100"
    :min="1"
    color="primary"
    step="0.1"
    thumb-label="always"
    type="number"
  />
  <v-slider
    v-model="settingsStore.avifOptions.alphaQuality"
    :disabled="isLossless"
    :label="t('alpha_quality')"
    :max="100"
    :min="1"
    color="primary"
    step="0.1"
    thumb-label="always"
  />
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
  lossless: 'Lossless Compression (Larger File Size)'
  bit_depth: 'Bit Depth'
  bit_depth_8: '8-bit'
  bit_depth_10: '10-bit'
  bit_depth_auto: 'Auto'
  quality: 'Quality (1-100)'
  alpha_quality: 'Alpha Channel Quality (1-100)'
  alpha_color_mode: 'Alpha Color Mode'
  speed: 'Encoding Speed (0-10)'
  color_model: 'Color Model'
  threads: 'Max Threads to Use (Leave Blank for Auto)'
  reset_avif_options: 'Reset AVIF Options'
ja:
  lossless: 'ロスレス圧縮 (ファイルサイズは大きくなります)'
  bit_depth: 'ビット深度'
  bit_depth_8: '8ビット'
  bit_depth_10: '10ビット'
  bit_depth_auto: '自動'
  quality: '品質 (1-100)'
  alpha_quality: 'アルファチャンネルの品質 (1-100)'
  alpha_color_mode: 'アルファカラーモード'
  speed: 'エンコード速度 (0-10)'
  color_model: 'カラーモデル'
  threads: '最大スレッド数 (空欄で自動設定)'
  reset_avif_options: 'AVIFオプションをリセット'
kr:
  lossless: '무손실 압축 (파일 크기가 커질 수 있음)'
  bit_depth: '비트 깊이'
  bit_depth_8: '8비트'
  bit_depth_10: '10비트'
  bit_depth_auto: '자동'
  quality: '품질 (1-100)'
  alpha_quality: '알파 채널 품질 (1-100)'
  alpha_color_mode: '알파 색상 모드'
  speed: '인코딩 속도 (0-10)'
  color_model: '색상 모델'
  threads: '사용할 최대 스레드 수 (자동 설정하려면 비워두기)'
  reset_avif_options: 'AVIF 옵션 재설정'
zh:
  lossless: '無損壓縮 (文件大小可能會增大)'
  bit_depth: '位深'
  bit_depth_8: '8位'
  bit_depth_10: '10位'
  bit_depth_auto: '自動'
  quality: '質量 (1-100)'
  alpha_quality: 'Alpha通道質量 (1-100)'
  alpha_color_mode: 'Alpha顏色模式'
  speed: '編碼速度 (0-10)'
  color_model: '顏色模型'
  threads: '使用的最大線程數 (留空則自動設置)'
  reset_avif_options: '重置 AVIF 選項'
</i18n>
