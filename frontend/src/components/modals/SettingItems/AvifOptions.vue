<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

import { BitDepth, ColorModel } from '@/types/AvifTypes';

const { t } = useI18n();
const settingsStore = useSettingsStore();
</script>

<template>
  <v-card :title="t('title')" flat>
    <v-card-text>
      <v-slider
        v-model="settingsStore.avifOptions.quality"
        :label="t('quality', { min: 1, max: 100 })"
        :max="100"
        :min="1"
        color="primary"
        step="0.1"
        type="number"
        persistent-hint
      >
        <template #append>
          <v-text-field v-model="settingsStore.avifOptions.quality" variant="underlined" readonly />
        </template>
      </v-slider>
      <v-slider
        v-model="settingsStore.avifOptions.alphaQuality"
        :label="t('alpha_quality', { min: 1, max: 100 })"
        :max="100"
        :min="1"
        color="primary"
        step="0.1"
      >
        <template #append>
          <v-text-field
            v-model="settingsStore.avifOptions.alphaQuality"
            variant="underlined"
            readonly
          />
        </template>
      </v-slider>
      <v-select
        v-model="settingsStore.avifOptions.bitDepth"
        :items="[
          { text: t('bit_depth_items[0]'), value: BitDepth.Auto },
          { text: t('bit_depth_items[1]'), value: BitDepth.Eight },
          { text: t('bit_depth_items[2]'), value: BitDepth.Ten },
          { text: t('bit_depth_items[3]'), value: BitDepth.Twelve }
        ]"
        :hint="t('bit_depth_hint')"
        :label="t('bit_depth')"
        item-title="text"
        item-value="value"
        persistent-hint
      />
      <v-select
        v-model="settingsStore.avifOptions.colorModel"
        :items="[
          { text: 'YCbCr', value: ColorModel.YCbCr },
          { text: 'RGB', value: ColorModel.RGB }
        ]"
        :hint="t('color_model_hint')"
        :label="t('color_model')"
        item-title="text"
        item-value="value"
        persistent-hint
      />
      <v-slider
        v-model="settingsStore.avifOptions.speed"
        :hint="t('speed_hint')"
        :label="t('speed', { min: 1, max: 10 })"
        :max="10"
        :min="1"
        color="primary"
        step="1"
        type="number"
        persistent-hint
      >
        <template #append>
          <v-text-field v-model="settingsStore.avifOptions.speed" variant="underlined" readonly />
        </template>
      </v-slider>
      <v-number-input
        v-model="settingsStore.avifOptions.threads"
        :hint="t('threads_hint')"
        :label="t('threads')"
        :max="10"
        :min="1"
        type="number"
        clearable
        persistent-hint
      />
    </v-card-text>
    <v-card-actions>
      <v-btn
        :text="t('reset_avif_options')"
        color="warning"
        prepend-icon="mdi-rotate-left"
        variant="tonal"
        @click="settingsStore.resetAvifOptions()"
      />
    </v-card-actions>
  </v-card>
</template>

<i18n lang="yaml">
en:
  title: AVIF Options
  bit_depth: Bit Depth
  bit_depth_hint: Auto (recommended) intelligently selects bit depth based on image content. 8-bit is fast for standard images, 10-bit is slower but preserves quality for HDR/wide gamut images, 12-bit is very slow for archival purposes.
  bit_depth_items:
    - Auto (Recommended)
    - 8-bit (Fast, Standard)
    - 10-bit (Slower, HDR/Wide Gamut)
    - 12-bit (Very Slow, Archival)
  quality: Quality ({min}-{max})
  alpha_quality: Alpha Channel Quality ({min}-{max})
  speed: Encoding Speed ({min}-{max})
  speed_hint: Speed 6 (default) balances quality and time. Higher values = faster but larger files, lower values = slower but better compression
  color_model: Color Model
  color_model_hint: YCbCr generally offers better compression, but RGB may yield better results for some images
  threads: Max Threads to Use
  threads_hint: If left blank, it will be set automatically based on the number of logical cores in the system
  reset_avif_options: Reset AVIF Options
fr:
  title: Options AVIF
  bit_depth: Profondeur de bits
  bit_depth_hint: Auto (recommandé) sélectionne intelligemment la profondeur de bits en fonction du contenu de l'image. 8 bits est rapide pour les images standard, 10 bits est plus lent mais préserve la qualité pour les images HDR/gamme étendue, 12 bits est très lent pour l'archivage.
  bit_depth_items:
    - Auto (Recommandé)
    - 8 bits (Rapide, Standard)
    - 10 bits (Plus lent, HDR/Gamme étendue)
    - 12 bits (Très lent, Archivage)
  quality: Qualité ({min}-{max})
  alpha_quality: Qualité du canal alpha ({min}-{max})
  speed: Vitesse d'encodage ({min}-{max})
  speed_hint: Vitesse 6 (par défaut) équilibre qualité et temps. Valeurs plus élevées = plus rapide mais fichiers plus gros, valeurs plus basses = plus lent mais meilleure compression
  color_model: Modèle de couleur
  color_model_hint: YCbCr offre généralement une meilleure compression, mais RGB peut donner de meilleurs résultats pour certaines images
  threads: Nombre maximum de threads à utiliser
  threads_hint: S'il est laissé vide, il sera défini automatiquement en fonction du nombre de cœurs logiques du système
  reset_avif_options: Réinitialiser les options AVIF
ja:
  title: AVIF 設定
  bit_depth: ビット深度
  bit_depth_hint: 自動（推奨）は画像内容に基づいて最適なビット深度を選択します。8ビットは標準画像に高速、10ビットはHDR/広色域画像に高品質（遅い）、12ビットはアーカイブ用（非常に遅い）。
  bit_depth_items:
    - 自動（推奨）
    - 8ビット（高速・標準）
    - 10ビット（低速・HDR/広色域）
    - 12ビット（最遅・アーカイブ）
  quality: 品質（{min}～{max}）
  alpha_quality: アルファチャンネルの品質（{min}～{max}）
  speed: エンコード速度（{min}～{max}）
  speed_hint: 速度6（デフォルト）は品質と時間のバランスが良好。高い値 = 高速だがファイルサイズ大、低い値 = 低速だが圧縮効率良
  color_model: カラーモデル
  color_model_hint: YCbCrは一般的により良い圧縮を提供しますが、RGBは一部の画像でより良い結果をもたらす場合があります
  threads: 最大スレッド数
  threads_hint: 空欄の場合、システムの論理コア数に基づいて自動的に設定されます
  reset_avif_options: AVIFオプションをリセット
ko:
  title: AVIF 옵션
  bit_depth: 비트 깊이
  bit_depth_hint: 자동(권장)은 이미지 내용에 따라 비트 깊이를 지능적으로 선택합니다. 8비트는 표준 이미지에 빠름, 10비트는 HDR/광색역 이미지에 고품질(느림), 12비트는 아카이브용(매우 느림).
  bit_depth_items:
    - 자동 (권장)
    - 8비트 (빠름, 표준)
    - 10비트 (느림, HDR/광색역)
    - 12비트 (매우 느림, 아카이브)
  quality: 품질 ({min}-{max})
  alpha_quality: 알파 채널 품질 ({min}-{max})
  speed: 인코딩 속도 ({min}-{max})
  speed_hint: 속도 6(기본값)은 품질과 시간의 균형이 좋습니다. 높은 값 = 빠르지만 파일 크기 증가, 낮은 값 = 느리지만 압축 효율 증가
  color_model: 색상 모델
  color_model_hint: YCbCr는 일반적으로 더 나은 압축을 제공하지만 RGB는 일부 이미지에서 더 나은 결과를 얻을 수 있습니다
  threads: 사용할 최대 스레드 수
  threads_hint: 비워 두면 시스템의 논리 코어 수에 따라 자동으로 설정됩니다
  reset_avif_options: AVIF 옵션 재설정
zhHant:
  title: AVIF 選項
  bit_depth: 位深
  bit_depth_hint: 自動（推薦）根據圖像內容智能選擇位深。8位元適用於標準圖像（快速），10位元適用於HDR/廣色域圖像（較慢），12位元適用於存檔（非常慢）。
  bit_depth_items:
    - 自動（推薦）
    - 8位元（快速，標準）
    - 10位元（較慢，HDR/廣色域）
    - 12位元（非常慢，存檔）
  quality: 質量 ({min}-{max})
  alpha_quality: Alpha通道質量 ({min}-{max})
  speed: 編碼速度 ({min}-{max})
  speed_hint: 速度6（預設）平衡品質與時間。較高的值 = 較快但檔案較大，較低的值 = 較慢但壓縮效率更好
  color_model: 顏色模式
  color_model_hint: YCbCr通常提供更好的壓縮，但RGB可能對某些圖像產生更好的效果
  threads: 使用的最大线程数
  threads_hint: 如果留空，将根据系统中的逻辑核心数自动设置
  reset_avif_options: 重置AVIF选项
zhHans:
  title: AVIF 设置
  bit_depth: 位深
  bit_depth_hint: 自动（推荐）根据图像内容智能选择位深。8位元适用于标准图像（快速），10位元适用于HDR/广色域图像（较慢），12位元适用于存档（非常慢）。
  bit_depth_items:
    - 自动（推荐）
    - 8位元（快速，标准）
    - 10位元（较慢，HDR/广色域）
    - 12位元（非常慢，存档）
  quality: 质量 ({min}-{max})
  alpha_quality: Alpha通道质量 ({min}-{max})
  speed: 编码速度 ({min}-{max})
  speed_hint: 速度6（默认）平衡质量与时间。较高的值 = 较快但文件较大，较低的值 = 较慢但压缩效率更好
  color_model: 颜色模式
  color_model_hint: YCbCr通常提供更好的压缩，但RGB可能对某些图像产生更好的效果
  threads: 使用的最大线程数
  threads_hint: 如果留空，将根据系统中的逻辑核心数自动设置
  reset_avif_options: 重置AVIF选项
</i18n>
