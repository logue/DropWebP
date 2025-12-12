<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const settingsStore = useSettingsStore();
</script>

<template>
  <v-card flat :title="t('title')">
    <v-card-text>
      <v-slider
        v-model="settingsStore.pngOptions.optimizationLevel"
        :label="t('optimization_level')"
        :hint="t('optimization_level_hint')"
        :min="0"
        :max="6"
        :step="1"
        color="primary"
        persistent-hint
      >
        <template #append>
          <v-text-field
            v-model="settingsStore.pngOptions.optimizationLevel"
            readonly
            variant="underlined"
            style="width: 60px"
          />
        </template>
      </v-slider>

      <v-switch
        v-model="settingsStore.pngOptions.useZopfli"
        :label="t('use_zopfli')"
        :hint="t('use_zopfli_hint')"
        color="primary"
        persistent-hint
      />

      <v-switch
        v-model="settingsStore.pngOptions.stripMetadata"
        :label="t('strip_metadata')"
        :hint="t('strip_metadata_hint')"
        color="primary"
        persistent-hint
      />

      <v-switch
        v-model="settingsStore.pngOptions.bitDepthReduction"
        :label="t('bit_depth_reduction')"
        :hint="t('bit_depth_reduction_hint')"
        color="primary"
        persistent-hint
      />

      <v-switch
        v-model="settingsStore.pngOptions.colorTypeReduction"
        :label="t('color_type_reduction')"
        :hint="t('color_type_reduction_hint')"
        color="primary"
        persistent-hint
      />

      <v-switch
        v-model="settingsStore.pngOptions.paletteReduction"
        :label="t('palette_reduction')"
        :hint="t('palette_reduction_hint')"
        color="primary"
        persistent-hint
      />

      <v-switch
        v-model="settingsStore.pngOptions.grayscaleReduction"
        :label="t('grayscale_reduction')"
        :hint="t('grayscale_reduction_hint')"
        color="primary"
        persistent-hint
      />

      <v-radio-group v-model="settingsStore.pngOptions.interlace" inline :label="t('interlace')">
        <v-tooltip :text="t('interlace_item.no_change_hint')" location="top">
          <template #activator="{ props }">
            <v-radio
              v-bind="props"
              :label="t('interlace_item.no_change')"
              color="primary"
              :value="null"
            />
          </template>
        </v-tooltip>
        <v-tooltip :text="t('interlace_item.disabled_hint')" location="top">
          <template #activator="{ props }">
            <v-radio
              v-bind="props"
              :label="t('interlace_item.disabled')"
              color="primary"
              :value="false"
            />
          </template>
        </v-tooltip>
        <v-tooltip :text="t('interlace_item.enabled_hint')" location="top">
          <template #activator="{ props }">
            <v-radio
              v-bind="props"
              :label="t('interlace_item.enabled')"
              color="primary"
              :value="true"
            />
          </template>
        </v-tooltip>
      </v-radio-group>

      <v-switch
        v-model="settingsStore.pngOptions.optimizeAlpha"
        :label="t('optimize_alpha')"
        :hint="t('optimize_alpha_hint')"
        color="primary"
        persistent-hint
      />

      <v-switch
        v-model="settingsStore.pngOptions.fastEvaluation"
        :label="t('fast_evaluation')"
        :hint="t('fast_evaluation_hint')"
        color="primary"
        persistent-hint
      />

      <v-switch
        v-model="settingsStore.pngOptions.scale16"
        :label="t('scale_16')"
        :hint="t('scale_16_hint')"
        color="primary"
        persistent-hint
      />
    </v-card-text>
    <v-card-actions>
      <v-btn
        :text="t('reset_png_options')"
        color="warning"
        prepend-icon="mdi-rotate-left"
        variant="tonal"
        @click="settingsStore.resetPngOptions()"
      />
    </v-card-actions>
  </v-card>
</template>

<i18n lang="yaml">
en:
  title: PNG (Oxipng) Options
  optimization_level: Optimization Level
  optimization_level_hint: PNG optimization level (0-6). 0=fastest/minimal compression, 6=slowest/maximum compression. PNG is lossless regardless of level.
  use_zopfli: Use Zopfli Compression
  use_zopfli_hint: Enable Zopfli compression. Much slower but achieves 3-8% better compression than standard deflate.
  strip_metadata: Strip Metadata
  strip_metadata_hint: Remove metadata chunks (safe stripping only).
  bit_depth_reduction: Bit Depth Reduction
  bit_depth_reduction_hint: Reduces the bit depth where possible to decrease file size.
  color_type_reduction: Color Type Reduction
  color_type_reduction_hint: Reduces the color type where possible to decrease file size.
  palette_reduction: Palette Reduction
  palette_reduction_hint: Uses a palette where possible to decrease file size.
  grayscale_reduction: Grayscale Reduction
  grayscale_reduction_hint: Converts to grayscale where possible to decrease file size.
  interlace: Interlace Setting
  interlace_item:
    no_change: No Change
    no_change_hint: Keep existing interlace setting.
    disabled: Disabled
    disabled_hint: No interlacing. Minimum file size.
    enabled: Enabled (Adam7)
    enabled_hint: Progressive display but larger file size.
  optimize_alpha: Optimize Alpha
  optimize_alpha_hint: Allow transparent pixels to be altered for better compression.
  fast_evaluation: Fast Evaluation
  fast_evaluation_hint: Use faster filter evaluation (recommended).
  scale_16: Scale 16-bit to 8-bit
  scale_16_hint: Force scale 16-bit images to 8-bit.
  reset_png_options: Reset PNG Options
ja:
  title: PNG (Oxipng) 設定
  optimization_level: 最適化レベル
  optimization_level_hint: PNG最適化レベル (0-6)。0=最速/最小圧縮、6=最遅/最大圧縮。どのレベルでもPNGは無劣化です。
  use_zopfli: Zopfli圧縮を使用
  use_zopfli_hint: Zopfli圧縮を有効化。通常のdeflateより3-8%高圧縮ですが、非常に遅くなります。
  strip_metadata: メタデータを削除
  strip_metadata_hint: メタデータチャンクを削除（安全な削除のみ）。
  bit_depth_reduction: ビット深度の削減
  bit_depth_reduction_hint: 可能な場合、ビット深度を削減してファイルサイズを小さくします。
  color_type_reduction: カラータイプの削減
  color_type_reduction_hint: 可能な場合、カラータイプを削減してファイルサイズを小さくします。
  palette_reduction: パレットの削減
  palette_reduction_hint: 可能な場合、パレットを使用してファイルサイズを小さくします。
  grayscale_reduction: グレースケール変換
  grayscale_reduction_hint: 可能な場合、グレースケールに変換してファイルサイズを小さくします。
  interlace: インターレース設定
  interlace_item:
    no_change: 変更なし
    no_change_hint: 既存のインターレース設定を維持。
    disabled: 無効
    disabled_hint: インターレースなし。最小ファイルサイズ。
    enabled: 有効 (Adam7)
    enabled_hint: プログレッシブ表示可能だがファイルサイズ増加。
  optimize_alpha: アルファ最適化
  optimize_alpha_hint: 透明ピクセルを変更して圧縮率を向上。
  fast_evaluation: 高速評価
  fast_evaluation_hint: 高速フィルター評価を使用（推奨）。
  scale_16: 16ビット→8ビット変換
  scale_16_hint: 16ビット画像を強制的に8ビットに変換。
  reset_png_options: PNG設定をリセット
fr:
  title: Options PNG (Oxipng)
  optimization_level: Niveau d'optimisation
  optimization_level_hint: Niveau d'optimisation PNG (0-6). 0=le plus rapide/compression minimale, 6=le plus lent/compression maximale. PNG est sans perte quel que soit le niveau.
  use_zopfli: Utiliser la compression Zopfli
  use_zopfli_hint: Activer la compression Zopfli. Beaucoup plus lent mais obtient une compression 3-8% meilleure que le deflate standard.
  strip_metadata: Supprimer les métadonnées
  strip_metadata_hint: Supprimer les chunks de métadonnées (suppression sûre uniquement).
  bit_depth_reduction: Réduction de la profondeur de bits
  bit_depth_reduction_hint: Réduit la profondeur de bits lorsque cela est possible pour diminuer la taille du fichier.
  color_type_reduction: Réduction du type de couleur
  color_type_reduction_hint: Réduit le type de couleur lorsque cela est possible pour diminuer la taille du fichier.
  palette_reduction: Réduction de la palette
  palette_reduction_hint: Utilise une palette lorsque cela est possible pour diminuer la taille du fichier.
  grayscale_reduction: Réduction en niveaux de gris
  grayscale_reduction_hint: Convertit en niveaux de gris lorsque cela est possible pour diminuer la taille du fichier.
  interlace: Paramètre d'entrelacement
  interlace_item:
    no_change: Pas de changement
    no_change_hint: Conserver le paramètre d'entrelacement existant.
    disabled: Désactivé
    disabled_hint: Pas d'entrelacement. Taille de fichier minimale.
    enabled: Activé (Adam7)
    enabled_hint: Affichage progressif mais taille de fichier plus grande.
  optimize_alpha: Optimiser Alpha
  optimize_alpha_hint: Permettre la modification des pixels transparents pour une meilleure compression.
  fast_evaluation: Évaluation rapide
  fast_evaluation_hint: Utiliser une évaluation de filtre plus rapide (recommandé).
  scale_16: Réduire 16 bits à 8 bits
  scale_16_hint: Forcer la réduction des images 16 bits à 8 bits.
  reset_png_options: Réinitialiser les options PNG
ko:
  title: PNG (Oxipng) 옵션
  optimization_level: 최적화 레벨
  optimization_level_hint: PNG 최적화 레벨 (0-6). 0=가장 빠름/최소 압축, 6=가장 느림/최대 압축. 레벨에 관계없이 PNG는 무손실입니다.
  use_zopfli: Zopfli 압축 사용
  use_zopfli_hint: Zopfli 압축을 활성화합니다. 표준 deflate보다 3-8% 더 나은 압축을 달성하지만 훨씬 느립니다.
  strip_metadata: 메타데이터 제거
  strip_metadata_hint: 메타데이터 청크 제거 (안전한 제거만).
  bit_depth_reduction: 비트 심도 감소
  bit_depth_reduction_hint: 가능한 경우 비트 심도를 줄여 파일 크기를 줄입니다.
  color_type_reduction: 색상 유형 감소
  color_type_reduction_hint: 가능한 경우 색상 유형을 줄여 파일 크기를 줄입니다.
  palette_reduction: 팔레트 감소
  palette_reduction_hint: 가능한 경우 팔레트를 사용하여 파일 크기를 줄입니다.
  grayscale_reduction: 그레이스케일 변환
  grayscale_reduction_hint: 가능한 경우 그레이스케일로 변환하여 파일 크기를 줄입니다.
  interlace: 인터레이스 설정
  interlace_item:
    no_change: 변경 없음
    no_change_hint: 기존 인터레이스 설정 유지.
    disabled: 비활성화
    disabled_hint: 인터레이스 없음. 최소 파일 크기.
    enabled: 활성화 (Adam7)
    enabled_hint: 점진적 표시 가능하지만 파일 크기 증가.
  optimize_alpha: 알파 최적화
  optimize_alpha_hint: 투명 픽셀을 변경하여 더 나은 압축을 허용합니다.
  fast_evaluation: 빠른 평가
  fast_evaluation_hint: 더 빠른 필터 평가 사용 (권장).
  scale_16: 16비트→8비트 변환
  scale_16_hint: 16비트 이미지를 강제로 8비트로 변환.
  reset_png_options: PNG 옵션 재설정
zhHant:
  title: PNG (Oxipng) 選項
  optimization_level: 優化級別
  optimization_level_hint: PNG 優化級別 (0-6)。0=最快/最小壓縮，6=最慢/最大壓縮。無論級別如何，PNG 都是無損的。
  use_zopfli: 使用 Zopfli 壓縮
  use_zopfli_hint: 啟用 Zopfli 壓縮。比標準 deflate 慢得多，但實現 3-8% 更好的壓縮。
  strip_metadata: 刪除元數據
  strip_metadata_hint: 刪除元數據塊（僅安全刪除）。
  bit_depth_reduction: 位深度減少
  bit_depth_reduction_hint: 在可能的情況下減少位深度以減小文件大小。
  color_type_reduction: 顏色類型減少
  color_type_reduction_hint: 在可能的情況下減少顏色類型以減小文件大小。
  palette_reduction: 調色板減少
  palette_reduction_hint: 在可能的情況下使用調色板以減小文件大小。
  grayscale_reduction: 灰階轉換
  grayscale_reduction_hint: 在可能的情況下轉換為灰階以減小文件大小。
  interlace: 交錯設置
  interlace_item:
    no_change: 不更改
    no_change_hint: 保持現有交錯設置。
    disabled: 禁用
    disabled_hint: 無交錯。最小文件大小。
    enabled: 啟用 (Adam7)
    enabled_hint: 漸進顯示但文件大小增加。
  optimize_alpha: 優化 Alpha
  optimize_alpha_hint: 允許更改透明像素以實現更好的壓縮。
  fast_evaluation: 快速評估
  fast_evaluation_hint: 使用更快的過濾器評估（推薦）。
  scale_16: 16位→8位縮放
  scale_16_hint: 強制將 16 位圖像縮放為 8 位。
  reset_png_options: 重置 PNG 選項
zhHans:
  title: PNG (Oxipng) 选项
  optimization_level: 优化级别
  optimization_level_hint: PNG 优化级别 (0-6)。0=最快/最小压缩，6=最慢/最大压缩。无论级别如何，PNG 都是无损的。
  use_zopfli: 使用 Zopfli 压缩
  use_zopfli_hint: 启用 Zopfli 压缩。比标准 deflate 慢得多，但实现 3-8% 更好的压缩。
  strip_metadata: 删除元数据
  strip_metadata_hint: 删除元数据块（仅安全删除）。
  bit_depth_reduction: 位深度减少
  bit_depth_reduction_hint: 在可能的情况下减少位深度以减小文件大小。
  color_type_reduction: 颜色类型减少
  color_type_reduction_hint: 在可能的情况下减少颜色类型以减小文件大小。
  palette_reduction: 调色板减少
  palette_reduction_hint: 在可能的情况下使用调色板以减小文件大小。
  grayscale_reduction: 灰阶转换
  grayscale_reduction_hint: 在可能的情况下转换为灰阶以减小文件大小。
  interlace: 交错设置
  interlace_item:
    no_change: 不更改
    no_change_hint: 保持现有交错设置。
    disabled: 禁用
    disabled_hint: 无交错。最小文件大小。
    enabled: 启用 (Adam7)
    enabled_hint: 渐进显示但文件大小增加。
  optimize_alpha: 优化 Alpha
  optimize_alpha_hint: 允许更改透明像素以实现更好的压缩。
  fast_evaluation: 快速评估
  fast_evaluation_hint: 使用更快的过滤器评估（推荐）。
  scale_16: 16位→8位缩放
  scale_16_hint: 强制将 16 位图像缩放为 8 位。
  reset_png_options: 重置 PNG 选项
</i18n>
