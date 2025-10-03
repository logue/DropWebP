<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

// import { WebPImageHint } from '@/types/WebpTypes';

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
    step="0.1"
    thumb-label="always"
    type="number"
  />
  <!--
  <v-slicer
    v-model="settingsStore.webpOptions.method"
    :label="t('method') + ' (' + settingsStore.webpOptions.method + ')'"
    :max="6"
    :min="0"
    color="primary"
    step="1"
    thumb-label="always"
    type="number"
  />
  <v-switch
    v-model="settingsStore.webpOptions.autofilter"
    :label="t('autofilter')"
    color="primary"
    inline
  />
  <v-select
    v-model="settingsStore.webpOptions.hint"
    :items="[
      { text: t('image_hint_default'), value: WebPImageHint.Default },
      { text: t('image_hint_picture'), value: WebPImageHint.Picture },
      { text: t('image_hint_photo'), value: WebPImageHint.Photo },
      { text: t('image_hint_graph'), value: WebPImageHint.Graph }
    ]"
    :label="t('image_hint')"
    item-title="text"
    item-value="value"
    persistent-hint
  />
  -->
  <v-btn
    color="warning"
    prepend-icon="mdi-rotate-left"
    variant="text"
    @click="settingsStore.resetWebpOptions()"
  >
    {{ t('reset_webp_options') }}
  </v-btn>
</template>

<i18n lang="yaml">
en:
  lossless: 'Lossless'
  quality: 'Quality (0-100)'
  image_hint: 'Image Hint'
  image_hint_default: 'Default'
  image_hint_picture: 'Picture'
  image_hint_photo: 'Photo'
  image_hint_graph: 'Graph'
  method: 'quality/speed trade-off (0=fast, 6=slower-better)'
  autofilter: 'Auto Filter'
  reset_webp_options: 'Reset WebP Options'
ja:
  lossless: 'ロスレス'
  quality: '品質 (0-100)'
  image_hint: '画像のヒント'
  image_hint_default: 'デフォルト'
  image_hint_picture: '画像'
  image_hint_photo: '写真'
  image_hint_graph: 'グラフ'
  method: '品質/速度のトレードオフ (0=高速、6=高品質)'
  autofilter: '自動フィルタリング'
  reset_webp_options: 'WebPオプションをリセット'
kr:
  lossless: '무손실'
  quality: '품질 (0-100)'
  image_hint: '이미지 힌트'
  image_hint_default: '기본값'
  image_hint_picture: '사진'
  image_hint_photo: '사진'
  image_hint_graph: '그래프'
  method: '품질/속도 절충 (0=빠름, 6=느리지만 더 좋음)'
  autofilter: '자동 필터링'
  reset_webp_options: 'WebP 옵션 재설정'
zh:
  lossless: '無損'
  quality: '質量 (0-100)'
  image_hint: '圖像提示'
  image_hint_default: '預設'
  image_hint_picture: '圖片'
  image_hint_photo: '照片'
  image_hint_graph: '圖表'
  method: '品質/速度的折衷 (0=快速，6=高品質)'
  autofilter: '自動過濾'
  reset_webp_options: '重置 WebP 選項'
</i18n>
