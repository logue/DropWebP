<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

import { WebPImageHint, WebPPreset } from '@/types/WebpTypes';

const { t } = useI18n();
const settingsStore = useSettingsStore();
</script>

<template>
  <v-card elevation="0">
    <v-card-text>
      <!-- ロスレス/ロッシー切り替え -->
      <v-switch
        v-model="settingsStore.webpOptions.lossless"
        :label="t('lossless')"
        :hint="t('lossless_hint')"
        color="primary"
        persistent-hint
        inline
      />

      <!-- エンコード方法 -->
      <v-slider
        v-model="settingsStore.webpOptions.method"
        :disabled="settingsStore.webpOptions.lossless"
        :hint="t('method_hint')"
        :label="t('method', { min: 0, max: 6 })"
        :max="6"
        :min="0"
        color="primary"
        persistent-hint
        step="1"
        type="number"
      >
        <template #append>
          <v-text-field v-model="settingsStore.webpOptions.method" readonly variant="underlined" />
        </template>
      </v-slider>

      <!-- プリセット選択 -->
      <v-select
        v-model="settingsStore.webpOptions.preset"
        :disabled="settingsStore.webpOptions.lossless"
        :items="[
          { text: t('preset_default'), value: WebPPreset.Default },
          { text: t('preset_picture'), value: WebPPreset.Picture },
          { text: t('preset_photo'), value: WebPPreset.Photo },
          { text: t('preset_drawing'), value: WebPPreset.Drawing },
          { text: t('preset_icon'), value: WebPPreset.Icon },
          { text: t('preset_text'), value: WebPPreset.Text }
        ]"
        :hint="t('preset_hint')"
        :label="t('preset')"
        item-title="text"
        item-value="value"
        persistent-hint
      />

      <!-- 画像ヒント -->
      <v-select
        v-model="settingsStore.webpOptions.hint"
        :disabled="settingsStore.webpOptions.lossless"
        :items="[
          { text: t('image_hint_default'), value: WebPImageHint.Default },
          { text: t('image_hint_picture'), value: WebPImageHint.Picture },
          { text: t('image_hint_photo'), value: WebPImageHint.Photo },
          { text: t('image_hint_graph'), value: WebPImageHint.Graph }
        ]"
        :hint="t('image_hint_hint')"
        :label="t('image_hint')"
        item-title="text"
        item-value="value"
        persistent-hint
      />

      <!-- 品質スライダー（ロスレス時は無効化） -->
      <v-slider
        v-model="settingsStore.webpOptions.quality"
        :disabled="settingsStore.webpOptions.lossless"
        :hint="t('quality_hint')"
        :label="t('quality', { min: 0, max: 100 })"
        :max="100"
        :min="0"
        color="primary"
        persistent-hint
        step="0.1"
        type="number"
      >
        <template #append>
          <v-text-field v-model="settingsStore.webpOptions.quality" readonly variant="underlined" />
        </template>
      </v-slider>

      <!-- アルファ品質 -->
      <v-slider
        v-model="settingsStore.webpOptions.alphaQuality"
        :disabled="settingsStore.webpOptions.lossless"
        :hint="t('alpha_quality_hint')"
        :label="t('alpha_quality', { min: 0, max: 100 })"
        :max="100"
        :min="0"
        color="primary"
        persistent-hint
        step="1"
        type="number"
      >
        <template #append>
          <v-text-field
            v-model="settingsStore.webpOptions.alphaQuality"
            readonly
            variant="underlined"
          />
        </template>
      </v-slider>

      <!-- 自動フィルタリング -->
      <v-switch
        v-model="settingsStore.webpOptions.autofilter"
        :disabled="settingsStore.webpOptions.lossless"
        :label="t('autofilter')"
        :hint="t('autofilter_hint')"
        color="primary"
        persistent-hint
        inline
      />

      <!-- フィルタ強度（自動フィルタOFF時のみ有効） -->
      <v-slider
        v-model="settingsStore.webpOptions.filterStrength"
        :disabled="settingsStore.webpOptions.autofilter || settingsStore.webpOptions.lossless"
        :hint="t('filter_strength_hint')"
        :label="t('filter_strength', { min: 0, max: 100 })"
        :max="100"
        :min="0"
        color="primary"
        persistent-hint
        step="1"
        type="number"
      >
        <template #append>
          <v-text-field
            v-model="settingsStore.webpOptions.filterStrength"
            readonly
            variant="underlined"
          />
        </template>
      </v-slider>

      <!-- フィルタシャープネス -->
      <v-slider
        v-model="settingsStore.webpOptions.filterSharpness"
        :disabled="settingsStore.webpOptions.autofilter || settingsStore.webpOptions.lossless"
        :hint="t('filter_sharpness_hint')"
        :label="t('filter_sharpness', { min: 0, max: 7 })"
        :max="7"
        :min="0"
        color="primary"
        persistent-hint
        step="1"
        type="number"
      >
        <template #append>
          <v-text-field
            v-model="settingsStore.webpOptions.filterSharpness"
            readonly
            variant="underlined"
          />
        </template>
      </v-slider>

      <!-- SNS強度 -->
      <v-slider
        v-model="settingsStore.webpOptions.snsStrength"
        :disabled="settingsStore.webpOptions.autofilter || settingsStore.webpOptions.lossless"
        :hint="t('sns_strength_hint')"
        :label="t('sns_strength', { min: 0, max: 100 })"
        :max="100"
        :min="0"
        color="primary"
        persistent-hint
        step="1"
        type="number"
      >
        <template #append>
          <v-text-field
            v-model="settingsStore.webpOptions.snsStrength"
            readonly
            variant="underlined"
          />
        </template>
      </v-slider>
    </v-card-text>
    <v-card-actions>
      <v-btn
        :text="t('reset_webp_options')"
        color="warning"
        prepend-icon="mdi-rotate-left"
        variant="tonal"
        @click="settingsStore.resetWebpOptions()"
      />
    </v-card-actions>
  </v-card>
</template>

<i18n lang="yaml">
en:
  lossless: Lossless Compression
  lossless_hint: Enable lossless compression (perfect quality, larger file size)
  quality: Quality ({min}-{max})
  quality_hint: Compression quality. Higher values = better quality but larger files. Ignored in lossless mode.
  preset: Preset
  preset_hint: Optimization profile for different image types. Automatically adjusts quality and settings.
  preset_default: Default
  preset_picture: Picture (digital photos)
  preset_photo: Photo (outdoor, +5% quality)
  preset_drawing: Drawing (line art, -10% quality)
  preset_icon: Icon (small images, -20% quality, prefers lossless)
  preset_text: Text (maximum quality, prefers lossless)
  image_hint: Image Hint
  image_hint_hint: Hint for encoder optimization based on image content
  image_hint_default: Default
  image_hint_picture: Picture
  image_hint_photo: Photo
  image_hint_graph: Graph (sharp edges)
  method: Encoding Method ({min}-{max})
  method_hint: Speed/quality trade-off. 0=fastest, 6=slowest but best quality. Recommended=4.
  autofilter: Auto Filter
  autofilter_hint: Automatically select optimal filter strength
  filter_strength: Filter Strength ({min}-{max})
  filter_strength_hint: Deblocking filter strength. 0=none (sharp), 100=maximum (smooth). Recommended=20-50.
  filter_sharpness: Filter Sharpness ({min}-{max})
  filter_sharpness_hint: Sharpness level. 0=sharpest, 7=smoothest. Photos=0-4, Illustrations=5-7.
  sns_strength: SNS Strength ({min}-{max})
  sns_strength_hint: Spatial Noise Shaping strength. Higher values reduce noise and file size. Photos=50-80, Illustrations=0-30.
  alpha_quality: Alpha Quality ({min}-{max})
  alpha_quality_hint: Transparency quality for RGBA images. 100=lossless alpha. Recommended=80-100.
  reset_webp_options: Reset WebP Options
fr:
  lossless: Compression sans perte
  lossless_hint: Activer la compression sans perte (qualité parfaite, taille de fichier plus grande)
  quality: Qualité ({min}-{max})
  quality_hint: Qualité de compression. Valeurs plus élevées = meilleure qualité mais fichiers plus volumineux. Ignoré en mode sans perte.
  preset: Préréglage
  preset_hint: Profil d'optimisation pour différents types d'images. Ajuste automatiquement la qualité et les paramètres.
  preset_default: Par défaut
  preset_picture: Image (photos numériques)
  preset_photo: Photo (extérieur, +5% qualité)
  preset_drawing: Dessin (art linéaire, -10% qualité)
  preset_icon: Icône (petites images, -20% qualité, préfère sans perte)
  preset_text: Texte (qualité maximale, préfère sans perte)
  image_hint: Indice d'image
  image_hint_hint: Indice pour l'optimisation de l'encodeur basé sur le contenu de l'image
  image_hint_default: Par défaut
  image_hint_picture: Image
  image_hint_photo: Photo
  image_hint_graph: Graphique (bords nets)
  method: Méthode d'encodage ({min}-{max})
  method_hint: Compromis vitesse/qualité. 0=le plus rapide, 6=le plus lent mais meilleure qualité. Recommandé=4.
  autofilter: Filtrage automatique
  autofilter_hint: Sélectionner automatiquement la force de filtrage optimale
  filter_strength: Force du filtre ({min}-{max})
  filter_strength_hint: Force du filtre de déblocage. 0=aucun (net), 100=maximum (lisse). Recommandé=20-50.
  filter_sharpness: Netteté du filtre ({min}-{max})
  filter_sharpness_hint: Niveau de netteté. 0=le plus net, 7=le plus lisse. Photos=0-4, Illustrations=5-7.
  sns_strength: Force SNS ({min}-{max})
  sns_strength_hint: Force du façonnage du bruit spatial. Des valeurs plus élevées réduisent le bruit et la taille du fichier. Photos=50-80, Illustrations=0-30.
  alpha_quality: Qualité alpha ({min}-{max})
  alpha_quality_hint: Qualité de transparence pour les images RGBA. 100=alpha sans perte. Recommandé=80-100.
  reset_webp_options: Réinitialiser les options WebP
ja:
  lossless: ロスレス圧縮
  lossless_hint: ロスレス圧縮を有効化（完全な品質、ファイルサイズは大きくなります）
  quality: 品質（{min}～{max}）
  quality_hint: 圧縮品質。値が大きいほど品質は良いがファイルサイズも大きくなる。ロスレス時は無視されます。
  preset: プリセット
  preset_hint: 画像タイプに応じた最適化プロファイル。品質と設定を自動調整します。
  preset_default: デフォルト
  preset_picture: 画像（デジタル写真）
  preset_photo: 写真（屋外、品質+5%）
  preset_drawing: 描画（線画、品質-10%）
  preset_icon: アイコン（小画像、品質-20%、ロスレス推奨）
  preset_text: テキスト（最高品質、ロスレス推奨）
  image_hint: 画像のヒント
  image_hint_hint: 画像内容に基づくエンコーダー最適化のヒント
  image_hint_default: デフォルト
  image_hint_picture: 画像
  image_hint_photo: 写真
  image_hint_graph: グラフ（シャープエッジ）
  method: エンコード方法（{min}～{max}）
  method_hint: 速度/品質のトレードオフ。0=最速、6=最も遅いが最高品質。推奨=4。
  autofilter: 自動フィルタリング
  autofilter_hint: 最適なフィルタ強度を自動選択
  filter_strength: フィルタ強度（{min}～{max}）
  filter_strength_hint: デブロッキングフィルタ強度。0=なし（シャープ）、100=最大（滑らか）。推奨=20-50。
  filter_sharpness: フィルタシャープネス（{min}～{max}）
  filter_sharpness_hint: シャープネスレベル。0=最もシャープ、7=最も滑らか。写真=0-4、イラスト=5-7。
  sns_strength: SNS強度（{min}～{max}）
  sns_strength_hint: 空間ノイズシェーピング強度。値が大きいとノイズとファイルサイズが減少。写真=50-80、イラスト=0-30。
  alpha_quality: アルファ品質（{min}～{max}）
  alpha_quality_hint: RGBA画像の透明度品質。100=ロスレスアルファ。推奨=80-100。
  reset_webp_options: WebPオプションをリセット
ko:
  lossless: 무손실 압축
  lossless_hint: 무손실 압축 활성화 (완벽한 품질, 파일 크기 증가)
  quality: 품질 ({min}-{max})
  quality_hint: 압축 품질. 값이 높을수록 품질은 좋지만 파일 크기가 커집니다. 무손실 모드에서는 무시됩니다.
  preset: 프리셋
  preset_hint: 다양한 이미지 유형에 대한 최적화 프로파일. 품질과 설정을 자동 조정합니다.
  preset_default: 기본값
  preset_picture: 사진 (디지털 사진)
  preset_photo: 사진 (야외, +5% 품질)
  preset_drawing: 그림 (라인 아트, -10% 품질)
  preset_icon: 아이콘 (작은 이미지, -20% 품질, 무손실 선호)
  preset_text: 텍스트 (최고 품질, 무손실 선호)
  image_hint: 이미지 힌트
  image_hint_hint: 이미지 내용에 따른 인코더 최적화 힌트
  image_hint_default: 기본값
  image_hint_picture: 사진
  image_hint_photo: 사진
  image_hint_graph: 그래프 (선명한 모서리)
  method: 인코딩 방법 ({min}-{max})
  method_hint: 속도/품질 절충. 0=가장 빠름, 6=가장 느리지만 최고 품질. 권장=4.
  autofilter: 자동 필터링
  autofilter_hint: 최적의 필터 강도 자동 선택
  filter_strength: 필터 강도 ({min}-{max})
  filter_strength_hint: 디블로킹 필터 강도. 0=없음(선명), 100=최대(부드러움). 권장=20-50.
  filter_sharpness: 필터 선명도 ({min}-{max})
  filter_sharpness_hint: 선명도 수준. 0=가장 선명, 7=가장 부드러움. 사진=0-4, 일러스트=5-7.
  sns_strength: SNS 강도 ({min}-{max})
  sns_strength_hint: 공간 노이즈 셰이핑 강도. 높은 값은 노이즈와 파일 크기를 줄입니다. 사진=50-80, 일러스트=0-30.
  alpha_quality: 알파 품질 ({min}-{max})
  alpha_quality_hint: RGBA 이미지의 투명도 품질. 100=무손실 알파. 권장=80-100.
  reset_webp_options: WebP 옵션 재설정
zh-tw:
  lossless: 無損壓縮
  lossless_hint: 啟用無損壓縮（完美品質，檔案較大）
  quality: 品質 ({min}-{max})
  quality_hint: 壓縮品質。值越高=品質越好但檔案越大。無損模式下會被忽略。
  preset: 預設
  preset_hint: 針對不同圖像類型的優化配置。自動調整品質和設定。
  preset_default: 預設值
  preset_picture: 圖像（數位照片）
  preset_photo: 照片（戶外，+5% 品質）
  preset_drawing: 繪圖（線條藝術，-10% 品質）
  preset_icon: 圖示（小圖像，-20% 品質，偏好無損）
  preset_text: 文字（最高品質，偏好無損）
  image_hint: 圖像提示
  image_hint_hint: 基於圖像內容的編碼器優化提示
  image_hint_default: 預設值
  image_hint_picture: 圖像
  image_hint_photo: 照片
  image_hint_graph: 圖表（銳利邊緣）
  method: 編碼方法 ({min}-{max})
  method_hint: 速度/品質權衡。0=最快，6=最慢但最佳品質。建議=4。
  autofilter: 自動過濾
  autofilter_hint: 自動選擇最佳過濾強度
  filter_strength: 過濾強度 ({min}-{max})
  filter_strength_hint: 去區塊過濾強度。0=無（銳利），100=最大（平滑）。建議=20-50。
  filter_sharpness: 過濾銳利度 ({min}-{max})
  filter_sharpness_hint: 銳利度等級。0=最銳利，7=最平滑。照片=0-4，插圖=5-7。
  sns_strength: SNS 強度 ({min}-{max})
  sns_strength_hint: 空間雜訊整形強度。較高的值會減少雜訊和檔案大小。照片=50-80，插圖=0-30。
  alpha_quality: Alpha 品質 ({min}-{max})
  alpha_quality_hint: RGBA 圖像的透明度品質。100=無損 alpha。建議=80-100。
  reset_webp_options: 重置 WebP 選項
zh-cn:
  lossless: 无损压缩
  lossless_hint: 启用无损压缩（完美品质，文件较大）
  quality: 品质 ({min}-{max})
  quality_hint: 压缩品质。值越高=品质越好但文件越大。无损模式下会被忽略。
  preset: 预设
  preset_hint: 针对不同图像类型的优化配置。自动调整品质和设置。
  preset_default: 默认值
  preset_picture: 图像（数字照片）
  preset_photo: 照片（户外，+5% 品质）
  preset_drawing: 绘图（线条艺术，-10% 品质）
  preset_icon: 图标（小图像，-20% 品质，偏好无损）
  preset_text: 文本（最高品质，偏好无损）
  image_hint: 图像提示
  image_hint_hint: 基于图像内容的编码器优化提示
  image_hint_default: 默认值
  image_hint_picture: 图像
  image_hint_photo: 照片
  image_hint_graph: 图表（锐利边缘）
  method: 编码方法 ({min}-{max})
  method_hint: 速度/品质权衡。0=最快，6=最慢但最佳品质。建议=4。
  autofilter: 自动过滤
  autofilter_hint: 自动选择最佳过滤强度
  filter_strength: 过滤强度 ({min}-{max})
  filter_strength_hint: 去块过滤强度。0=无（锐利），100=最大（平滑）。建议=20-50。
  filter_sharpness: 过滤锐利度 ({min}-{max})
  filter_sharpness_hint: 锐利度等级。0=最锐利，7=最平滑。照片=0-4，插图=5-7。
  sns_strength: SNS 强度 ({min}-{max})
  sns_strength_hint: 空间噪声整形强度。较高的值会减少噪声和文件大小。照片=50-80，插图=0-30。
  alpha_quality: Alpha 品质 ({min}-{max})
  alpha_quality_hint: RGBA 图像的透明度品质。100=无损 alpha。建议=80-100。
  reset_webp_options: 重置 WebP 选项
</i18n>
