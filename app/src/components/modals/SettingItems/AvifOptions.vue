<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

import { BitDepth, ColorModel, AlphaColorMode } from '@/types/AvifTypes';

const { t } = useI18n();
const settingsStore = useSettingsStore();
</script>

<template>
  <v-card elevation="0">
    <v-card-text>
      <v-slider
        v-model="settingsStore.avifOptions.quality"
        :label="t('quality', { min: 1, max: 100 })"
        :max="100"
        :min="1"
        color="primary"
        persistent-hint
        step="0.1"
        type="number"
      >
        <template #append>
          <v-text-field v-model="settingsStore.avifOptions.quality" readonly variant="underlined" />
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
            readonly
            variant="underlined"
          />
        </template>
      </v-slider>
      <v-select
        v-model="settingsStore.avifOptions.bitDepth"
        :items="[
          { text: t('bit_depth_items[0]'), value: BitDepth.Eight },
          { text: t('bit_depth_items[1]'), value: BitDepth.Ten },
          { text: t('bit_depth_items[2]'), value: BitDepth.Auto }
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
      <v-select
        v-model="settingsStore.avifOptions.alphaColorMode"
        :items="[
          { text: t('alpha_color_mode_items[0]'), value: AlphaColorMode.UnassociatedDirty },
          { text: t('alpha_color_mode_items[1]'), value: AlphaColorMode.UnassociatedClean },
          { text: t('alpha_color_mode_items[2]'), value: AlphaColorMode.Premultiplied }
        ]"
        :hint="t('alpha_color_mode_hint')"
        :label="t('alpha_color_mode')"
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
        persistent-hint
        step="1"
        type="number"
      >
        <template #append>
          <v-text-field v-model="settingsStore.avifOptions.speed" readonly variant="underlined" />
        </template>
      </v-slider>
      <v-number-input
        v-model="settingsStore.avifOptions.threads"
        :hint="t('threads_hint')"
        :label="t('threads')"
        :max="10"
        :min="1"
        clearable
        type="number"
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
  bit_depth: Bit Depth
  bit_depth_hint: Select the bit depth. Higher bit depth provides better quality but results in larger file sizes.
  bit_depth_items:
    - 8-bit
    - 10-bit
    - Auto
  quality: Quality ({min}-{max})
  alpha_quality: Alpha Channel Quality ({min}-{max})
  alpha_color_mode: Alpha Color Mode
  alpha_color_mode_hint: Specifies how to handle the colors of the alpha channel
  alpha_color_mode_items:
    - Unassociated alpha (dirty)
    - Unassociated alpha (clean)
    - Premultiplied alpha
  speed: Encoding Speed ({min}-{max})
  speed_hint: Higher values are faster but lower quality
  color_model: Color Model
  color_model_hint: YCbCr generally offers better compression, but RGB may yield better results for some images
  threads: Max Threads to Use
  threads_hint: If left blank, it will be set automatically based on the number of logical cores in the system
  reset_avif_options: Reset AVIF Options
fr:
  bit_depth: Profondeur de bits
  bit_depth_hint: Sélectionnez la profondeur de bits. Une profondeur de bits plus élevée offre une meilleure qualité mais entraîne des tailles de fichier plus importantes.
  bit_depth_items:
    - 8 bits
    - 10 bits
    - Auto
  quality: Qualité ({min}-{max})
  alpha_quality: Qualité du canal alpha ({min}-{max})
  alpha_color_mode: Mode de couleur alpha
  alpha_color_mode_hint: Spécifie comment gérer les couleurs du canal alpha
  alpha_color_mode_items:
    - UnassociatedDirty (Alpha non associé (sale))
    - UnassociatedClean (Alpha non associé (propre))
    - Premultiplied (Alpha prémultiplié)
  speed: Vitesse d'encodage ({min}-{max})
  speed_hint: Des valeurs plus élevées sont plus rapides mais de qualité inférieure
  color_model: Modèle de couleur
  color_model_hint: YCbCr offre généralement une meilleure compression, mais RGB peut donner de meilleurs résultats pour certaines images
  threads: Nombre maximum de threads à utiliser
  threads_hint: S'il est laissé vide, il sera défini automatiquement en fonction du nombre de cœurs logiques du système
  reset_avif_options: Réinitialiser les options AVIF
ja:
  bit_depth: ビット深度
  bit_depth_hint: ビット深度を選択します。高いビット深度はより良い品質を提供しますが、ファイルサイズも大きくなります。
  bit_depth_items:
    - 8ビット
    - 10ビット
    - 自動
  quality: 品質（{min}～{max}）
  alpha_quality: アルファチャンネルの品質（{min}～{max}）
  alpha_color_mode: アルファカラーモード
  alpha_color_mode_hint: アルファチャンネルの色の扱い方を指定します
  alpha_color_mode_items:
    - UnassociatedDirty (非関連（ダーティ）)
    - UnassociatedClean (非関連（クリーン）)
    - Premultiplied (乗算済みアルファ)
  speed: エンコード速度（{min}～{max}）
  speed_hint: 値が高いほど速度は速くなりますが、品質が悪くなります
  color_model: カラーモデル
  color_model_hint: YCbCrは一般的により良い圧縮を提供しますが、RGBは一部の画像でより良い結果をもたらす場合があります
  threads: 最大スレッド数
  threads_hint: 空欄の場合、システムの論理コア数に基づいて自動的に設定されます
  reset_avif_options: AVIFオプションをリセット
ko:
  bit_depth: 비트 깊이
  bit_depth_hint: 비트 깊이를 선택합니다. 더 높은 비트 깊이는 더 나은 품질을 제공하지만 파일 크기가 커집니다.
  bit_depth_items:
    - 8비트
    - 10비트
    - 자동
  quality: 품질 ({min}-{max})
  alpha_quality: 알파 채널 품질 ({min}-{max})
  alpha_color_mode: 알파 색상 모드
  alpha_color_mode_hint: 알파 채널의 색상을 처리하는 방법을 지정합니다
  alpha_color_mode_items:
    - UnassociatedDirty (비연관 알파(더티))
    - UnassociatedClean (비연관 알파(클린))
    - Premultiplied (프리멀티플라이드 알파)
  speed: 인코딩 속도 ({min}-{max})
  speed_hint: 값이 높을수록 빠르지만 품질이 낮아집니다
  color_model: 색상 모델
  color_model_hint: YCbCr는 일반적으로 더 나은 압축을 제공하지만 RGB는 일부 이미지에서 더 나은 결과를 얻을 수 있습니다
  threads: 사용할 최대 스레드 수
  threads_hint: 비워 두면 시스템의 논리 코어 수에 따라 자동으로 설정됩니다
  reset_avif_options: AVIF 옵션 재설정
zh-tw:
  bit_depth: 位深
  bit_depth_hint: 選擇位深。較高的位深提供更好的質量，但會導致檔案大小增加。
  bit_depth_items:
    - 8位元
    - 10位元
    - 自動
  quality: 質量 ({min}-{max})
  alpha_quality: Alpha通道質量 ({min}-{max})
  alpha_color_mode: Alpha顏色模式
  alpha_color_mode_hint: 指定如何處理Alpha通道的顏色
  alpha_color_mode_items:
    - UnassociatedDirty (非關聯Alpha（髒）)
    - UnassociatedClean (非關聯Alpha（乾淨）)
    - Premultiplied (預乘Alpha)
  speed: 編碼速度 ({min}-{max})
  speed_hint: 較高的值較快但質量較低
  color_model: 顏色模式
  color_model_hint: YCbCr通常提供更好的壓縮，但RGB可能對某些圖像產生更好的效果
  threads: 使用的最大线程数
  threads_hint: 如果留空，将根据系统中的逻辑核心数自动设置
  reset_avif_options: 重置AVIF选项
zh-cn:
  bit_depth: 位深
  bit_depth_hint: 选择位深。较高的位深提供更好的质量，但会导致文件大小增加。
  bit_depth_items:
    - 8位元
    - 10位元
    - 自动
  quality: 质量 ({min}-{max})
  alpha_quality: Alpha通道质量 ({min}-{max})
  alpha_color_mode: Alpha颜色模式
  alpha_color_mode_hint: 指定如何处理Alpha通道的颜色
  alpha_color_mode_items:
    - UnassociatedDirty (非关联Alpha（脏）)
    - UnassociatedClean (非关联Alpha（干净）)
    - Premultiplied (预乘Alpha)
  speed: 编码速度 ({min}-{max})
  speed_hint: 较高的值较快但质量较低
  color_model: 颜色模式
  color_model_hint: YCbCr通常提供更好的压缩，但RGB可能对某些图像产生更好的效果
  threads: 使用的最大线程数
  threads_hint: 如果留空，将根据系统中的逻辑核心数自动设置
  reset_avif_options: 重置AVIF选项
</i18n>
