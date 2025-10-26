<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const settingsStore = useSettingsStore();
</script>

<template>
  <v-card flat>
    <v-card-text>
      <v-switch
        v-model="settingsStore.jpegOptions.progressive"
        :hint="t('progressive_hint')"
        color="primary"
        persistent-hint
      >
        <template #label>
          {{ t('progressive') }}
          <v-chip :text="t('recommended')" size="x-small" />
        </template>
      </v-switch>
      <v-switch
        v-model="settingsStore.jpegOptions.optimize"
        :hint="t('optimize_hint')"
        :label="t('optimize')"
        color="primary"
        persistent-hint
      >
        <template #label>
          {{ t('optimize') }}
          <v-chip :text="t('recommended')" size="x-small" />
        </template>
      </v-switch>
      <v-slider
        v-model="settingsStore.jpegOptions.quality"
        :hint="t('quality_hint')"
        :label="t('quality', { min: 1, max: 100 })"
        color="primary"
        max="100"
        min="1"
        persistent-hint
        step="1"
        type="number"
      >
        <template #append>
          <v-text-field v-model="settingsStore.jpegOptions.quality" readonly variant="underlined" />
        </template>
      </v-slider>
    </v-card-text>
    <v-card-actions>
      <v-btn
        :text="t('reset_jpeg_options')"
        color="warning"
        prepend-icon="mdi-rotate-left"
        variant="tonal"
        @click="settingsStore.resetJpegOptions()"
      />
    </v-card-actions>
  </v-card>
</template>

<i18n lang="yaml">
en:
  recommended: Recommended
  quality: Quality ({min}-{max})
  quality_hint: 'Set the JPEG quality. Lower quality results in smaller file size. (Recommended: 85 and above)'
  progressive: Progressive (Recommended)
  progressive_hint: Enable progressive encoding for JPEG images.
  optimize: Optimize (Recommended)
  optimize_hint: Enable optimization for JPEG images.
  reset_jpeg_options: Reset JPEG Options
fr:
  recommended: Recommandé
  quality: Qualité ({min}-{max})
  quality_hint: 'Définissez la qualité JPEG. Une qualité inférieure entraîne une taille de fichier plus petite. (Recommandé : 85 et plus)'
  progressive: Progressif
  progressive_hint: Activez le codage progressif pour les images JPEG.
  optimize: Optimiser
  optimize_hint: Activez l'optimisation pour les images JPEG.
  reset_jpeg_options: Réinitialiser les options JPEG
ja:
  recommended: 推奨
  quality: 品質（{min}-{max}）
  quality_hint: JPEGの品質を設定します。 低い品質はファイルサイズの縮小につながります。（85以上を推奨）
  progressive: プログレッシブ
  progressive_hint: JPEG画像のプログレッシブエンコードを有効にします。
  optimize: 最適化
  optimize_hint: JPEG画像の最適化を有効にします。
  reset_jpeg_options: JPEGオプションをリセット
ko:
  recommended: 권장
  quality: 품질 ({min}-{max})
  quality_hint: JPEG 품질 설정. 낮은 품질은 더 작은 파일 크기를 초래합니다. (85 이상 권장)
  progressive: 프로그레시브
  progressive_hint: JPEG 이미지에 대한 프로그레시브 인코딩을 활성화합니다.
  optimize: 최적화
  optimize_hint: JPEG 이미지에 대한 최적화를 활성화합니다.
  reset_jpeg_options: JPEG 옵션 재설정
zhHans:
  recommended: 推荐
  quality: 质量（{min}-{max}）
  quality_hint: '设置JPEG质量。较低的质量会导致更小的文件大小。 (推荐：85及以上)'
  progressive: 渐进式
  progressive_hint: 为JPEG图像启用渐进式编码。
  optimize: 优化
  optimize_hint: 为JPEG图像启用优化。
  reset_jpeg_options: 重置JPEG选项
zhHant:
  recommended: 推薦
  quality: 質量（{min}-{max}）
  quality_hint: 設定JPEG質量。較低的質量會導致較小的檔案大小。 (建議：85及以上)
  progressive: 漸進式
  progressive_hint: 為JPEG圖像啟用漸進式編碼。
  optimize: 優化
  optimize_hint: 為JPEG圖像啟用優化。
  reset_jpeg_options: 重置JPEG選項
</i18n>
