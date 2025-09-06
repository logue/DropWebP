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
    :hint="t('bit_depth_hint')"
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
    :hint="t('alpha_color_mode_hint')"
    item-title="text"
    item-value="value"
  />
  <v-slider
    v-model="settingsStore.avifOptions.speed"
    :label="t('speed')"
    :hint="t('speed_hint')"
    :max="10"
    :min="1"
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
    :hint="t('color_model_hint')"
    item-title="text"
    item-value="value"
  />
  <v-number-input
    v-model="settingsStore.avifOptions.threads"
    :hint="t('threads_hint')"
    :label="t('threads')"
    :max="10"
    :min="1"
    clearable
    type="number"
  />
  <v-btn prepend-icon="mdi-rotate-left" variant="text" @click="settingsStore.resetCommonOptions()">
    {{ t('reset_avif_options') }}
  </v-btn>
</template>

<i18n lang="yaml">
en:
  lossless: 'Lossless Compression (Larger File Size)'
  bit_depth: 'Bit Depth'
  bit_depth_hint: 'Select the bit depth. Higher bit depth provides better quality but results in larger file sizes.'
  bit_depth_8: '8-bit'
  bit_depth_10: '10-bit'
  bit_depth_auto: 'Auto'
  quality: 'Quality (1-100)'
  alpha_quality: 'Alpha Channel Quality (1-100)'
  alpha_color_mode: 'Alpha Color Mode'
  alpha_color_mode_hint: 'Specifies how to handle the colors of the alpha channel'
  speed: 'Encoding Speed (1-10)'
  speed_hint: 'Higher speed results in lower quality'
  color_model: 'Color Model'
  color_model_hint: 'YCbCr generally offers better compression, but RGB may yield better results for some images'
  threads: 'Max Threads to Use (Leave Blank for Auto)'
  threads_hint: 'If left blank, it will be set automatically based on the number of logical cores in the system'
  reset_avif_options: 'Reset AVIF Options'
ja:
  lossless: 'ロスレス圧縮 (ファイルサイズは大きくなります)'
  bit_depth: 'ビット深度'
  bit_depth_hint: 'ビット深度を選択します。高いビット深度はより良い品質を提供しますが、ファイルサイズも大きくなります。'
  bit_depth_8: '8ビット'
  bit_depth_10: '10ビット'
  bit_depth_auto: '自動'
  quality: '品質 (1-100)'
  alpha_quality: 'アルファチャンネルの品質 (1-100)'
  alpha_color_mode: 'アルファカラーモード'
  alpha_color_mode_hint: 'アルファチャンネルの色の扱い方を指定します'
  speed: 'エンコード速度 (1-10)'
  speed_hint: '速度が速いほど品質が低くなります'
  color_model: 'カラーモデル'
  color_model_hint: 'YCbCrは一般的により良い圧縮を提供しますが、RGBは一部の画像でより良い結果をもたらす場合があります'
  threads: '最大スレッド数 (空欄で自動設定)'
  threads_hint: '空欄の場合、システムの論理コア数に基づいて自動的に設定されます'
  reset_avif_options: 'AVIFオプションをリセット'
kr:
  lossless: '무손실 압축 (파일 크기가 커질 수 있음)'
  bit_depth: '비트 깊이'
  bit_depth_hint: '비트 깊이를 선택합니다. 비트 깊이가 높을수록 더 나은 품질을 제공하지만 파일 크기도 커집니다.'
  bit_depth_8: '8비트'
  bit_depth_10: '10비트'
  bit_depth_auto: '자동'
  quality: '품질 (1-100)'
  alpha_quality: '알파 채널 품질 (1-100)'
  alpha_color_mode: '알파 색상 모드'
  alpha_color_mode_hint: '알파 채널의 색상을 처리하는 방법을 지정합니다'
  speed: '인코딩 속도 (1-10)'
  speed_hint: '속도가 높을수록 품질이 낮아집니다'
  color_model: '색상 모델'
  color_model_hint: 'YCbCr는 일반적으로 더 나은 압축을 제공하지만, RGB는 일부 이미지에서 더 나은 결과를 제공할 수 있습니다'
  threads: '사용할 최대 스레드 수 (자동 설정하려면 비워두기)'
  threads_hint: '비워두면 시스템의 논리 코어 수에 따라 자동으로 설정됩니다'
  reset_avif_options: 'AVIF 옵션 재설정'
zh:
  lossless: '無損壓縮 (文件大小可能會增大)'
  bit_depth: '位深'
  bit_depth_hint: '選擇位深。較高的位深提供更好的質量，但文件大小也會增大。'
  bit_depth_8: '8位'
  bit_depth_10: '10位'
  bit_depth_auto: '自動'
  quality: '質量 (1-100)'
  alpha_quality: 'Alpha通道質量 (1-100)'
  alpha_color_mode: 'Alpha顏色模式'
  alpha_color_mode_hint: '指定如何處理Alpha通道的顏色'
  speed: '編碼速度 (1-10)'
  speed_hint: '速度越快，質量越低'
  color_model: '顏色模型'
  color_model_hint: 'YCbCr通常提供更好的壓縮，但RGB可能對某些圖像效果更好'
  threads: '使用的最大線程數 (留空則自動設置)'
  threads_hint: '如果留空，將根據系統中的邏輯核心數自動設置'
  reset_avif_options: '重置 AVIF 選項'
</i18n>
