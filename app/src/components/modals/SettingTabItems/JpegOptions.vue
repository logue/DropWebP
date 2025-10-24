<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

// TODO:

const { t } = useI18n();
const settingsStore = useSettingsStore();
</script>

<template>
  <v-slider
    v-model="settingsStore.jpegOptions.quality"
    :hint="t('quality_hint', { min: 1, max: 100 })"
    :label="t('quality')"
    color="primary"
    max="100"
    min="1"
    persistent-hint
    step="1"
    thumb-label="always"
    type="number"
  />
  <v-switch
    v-model="settingsStore.jpegOptions.progressive"
    :hint="t('progressive_hint')"
    :label="t('progressive')"
    color="primary"
    persistent-hint
  />
  <v-switch
    v-model="settingsStore.jpegOptions.optimize"
    :hint="t('optimize_hint')"
    :label="t('optimize')"
    color="primary"
    persistent-hint
  />
  <v-btn
    color="warning"
    prepend-icon="mdi-rotate-left"
    variant="text"
    @click="settingsStore.resetJpegOptions()"
  >
    {{ t('reset_jpeg_options') }}
  </v-btn>
</template>

<i18n lang="yaml">
en:
  quality: Quality
  quality_hint: 'Set the JPEG quality ({{ min }}-{{ max }}). Lower quality results in smaller file size. (Recommended: 85 and above)'
  progressive: Progressive (Recommended)
  progressive_hint: Enable progressive encoding for JPEG images.
  optimize: Optimize (Recommended)
  optimize_hint: Enable optimization for JPEG images.
  reset_jpeg_options: Reset JPEG Options
fr:
  quality: Qualité
  quality_hint: 'Définissez la qualité JPEG ({{ min }}-{{ max }}). Une qualité inférieure entraîne une taille de fichier plus petite. (Recommandé : 85 et plus)'
  progressive: Progressif (Recommandé)
  progressive_hint: Activez le codage progressif pour les images JPEG.
  optimize: Optimiser (Recommandé)
  optimize_hint: Activez l'optimisation pour les images JPEG.
  reset_jpeg_options: Réinitialiser les options JPEG
ja:
  quality: 品質
  quality_hint: JPEGの品質を設定します（{{ min }}-{{ max }}）。 低い品質はファイルサイズの縮小につながります。（85以上を推奨）
  progressive: プログレッシブ（推奨）
  progressive_hint: JPEG画像のプログレッシブエンコードを有効にします。
  optimize: 最適化（推奨）
  optimize_hint: JPEG画像の最適化を有効にします。
  reset_jpeg_options: JPEGオプションをリセット
ko:
  quality: 품질
  quality_hint: JPEG 품질 설정 ({{ min }}-{{ max }}). 낮은 품질은 더 작은 파일 크기를 초래합니다. (85 이상 권장)
  progressive: 프로그레시브 (권장)
  progressive_hint: JPEG 이미지에 대한 프로그레시브 인코딩을 활성화합니다.
  optimize: 최적화 (권장)
  optimize_hint: JPEG 이미지에 대한 최적화를 활성화합니다.
  reset_jpeg_options: JPEG 옵션 재설정
zhHans:
  quality: 质量
  quality_hint: '设置JPEG质量（{{ min }}-{{ max }}）。较低的质量会导致更小的文件大小。 (推荐：85及以上)'
  progressive: 渐进式 (推荐)
  progressive_hint: 为JPEG图像启用渐进式编码。
  optimize: 优化 (推荐)
  optimize_hint: 为JPEG图像启用优化。
  reset_jpeg_options: 重置JPEG选项
zhHant:
  quality: 質量
  quality_hint: 設定JPEG質量（{{ min }}-{{ max }}）。較低的質量會導致較小的檔案大小。 (建議：85及以上)
  progressive: 漸進式 (建議)
  progressive_hint: 為JPEG圖像啟用漸進式編碼。
  optimize: 優化 (建議)
  optimize_hint: 為JPEG圖像啟用優化。
  reset_jpeg_options: 重置JPEG選項
</i18n>
