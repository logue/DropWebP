<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

import { WebPImageHint } from '@/types/WebpTypes';

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
    :label="t('quality', { min: 0, max: 100 })"
    :max="100"
    :min="0"
    color="primary"
    step="0.1"
    thumb-label="always"
    type="number"
  />
  <v-slider
    v-model="settingsStore.webpOptions.method"
    :label="t('method', { min: 0, max: 6 })"
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
  lossless: Lossless
  quality: Quality ({min}-{max})
  image_hint: Image Hint
  image_hint_default: Default
  image_hint_picture: Picture
  image_hint_photo: Photo
  image_hint_graph: Graph
  method: Quality/Speed trade-off ({min}=fast, {max}=slower-better)
  autofilter: Auto Filter
  reset_webp_options: Reset WebP Options
fr:
  lossless: Compression sans perte
  quality: Qualité ({min}-{max})
  image_hint: Indice d'image
  image_hint_default: Par défaut
  image_hint_picture: Image
  image_hint_photo: Photo
  image_hint_graph: Graphique
  method: Compromis qualité/vitesse ({min}=rapide, {max}=lent-mais-mieux)
  autofilter: Filtrage automatique
  reset_webp_options: Réinitialiser les options WebP
ja:
  lossless: ロスレス
  quality: 品質 ({min}-{max})
  image_hint: 画像のヒント
  image_hint_default: デフォルト
  image_hint_picture: 画像
  image_hint_photo: 写真
  image_hint_graph: グラフ
  method: 品質/速度のトレードオフ ({min}=高速、{max}=高品質)
  autofilter: 自動フィルタリング
  reset_webp_options: WebPオプションをリセット
ko:
  lossless: 무손실
  quality: 품질 ({min}-{max})
  image_hint: 이미지 힌트
  image_hint_default: 기본값
  image_hint_picture: 사진
  image_hint_photo: 사진
  image_hint_graph: 그래프
  method: 품질/속도 절충 ({min}=빠름, {max}=느리지만 더 좋음)
  autofilter: 자동 필터링
  reset_webp_options: WebP 옵션 재설정
zh-tw:
  lossless: 無損壓縮
  quality: 品質 ({min}-{max})
  image_hint: 圖像提示
  image_hint_default: 預設值
  image_hint_picture: 圖像
  image_hint_photo: 照片
  image_hint_graph: 圖表
  method: 品質/速度權衡 ({min}=快速, {max}=較慢但較好)
  autofilter: 自動過濾
  reset_webp_options: 重置 WebP 選項
zh-cn:
  lossless: 无损压缩
  quality: 品质 ({min}-{max})
  image_hint: 图像提示
  image_hint_default: 默认值
  image_hint_picture: 图像
  image_hint_photo: 照片
  image_hint_graph: 图表
  method: 品质/速度权衡 ({min}=快速, {max}=较慢但较好)
  autofilter: 自动过滤
  reset_webp_options: 重置 WebP 选项
</i18n>
