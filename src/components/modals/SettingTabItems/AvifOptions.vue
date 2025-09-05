<script setup lang="ts">
import { useSettingsStore } from '@/store';

import { BitDepth, ColorModel, AlphaColorMode } from '@/types/AvifTypes';

const settingsStore = useSettingsStore();
</script>

<template>
  <v-card class="pa-4">
    <v-card-text>
      <v-row class="mb-4">
        <v-col cols="12" sm="6">
          <v-text-field
            v-model="settingsStore.avifOptions.quality"
            label="品質 (0-100)"
            type="number"
            :min="0"
            :max="100"
          />
        </v-col>
        <v-col cols="12" sm="6">
          <v-sl
            v-model="settingsStore.avifOptions.alphaQuality"
            label="アルファチャンネルの品質 (0-100)"
            type="number"
            :min="0"
            :max="100"
            step="1"
            thumb-label="always"
          />
        </v-col>
      </v-row>

      <v-row class="mb-4">
        <v-col cols="12" sm="6">
          <v-select
            v-model="settingsStore.avifOptions.alphaColorMode"
            :items="[
              { text: 'UnassociatedDirty', value: AlphaColorMode.UnassociatedDirty },
              { text: 'UnassociatedClean', value: AlphaColorMode.UnassociatedClean },
              { text: 'Premultiplied', value: AlphaColorMode.Premultiplied }
            ]"
            label="アルファカラーモード"
          />
        </v-col>
        <v-col cols="12" sm="6">
          <v-slider
            v-model="settingsStore.avifOptions.speed"
            label="エンコード速度 (0-10)"
            type="number"
            :min="0"
            :max="10"
            step="1"
            thumb-label="always"
          />
        </v-col>
      </v-row>
      <v-row class="mb-4">
        <v-col cols="12" sm="4">
          <v-select
            v-model="settingsStore.avifOptions.bitDepth"
            :items="[
              { text: '8ビット', value: BitDepth.Eight },
              { text: '10ビット', value: BitDepth.Ten },
              { text: '自動', value: BitDepth.Auto }
            ]"
            label="ビット深度"
          />
        </v-col>
        <v-col cols="12" sm="4">
          <v-select
            v-model="settingsStore.avifOptions.colorModel"
            :items="[
              { text: 'YCbCr', value: ColorModel.YCbCr },
              { text: 'RGB', value: ColorModel.RGB }
            ]"
            label="カラーモデル"
          />
        </v-col>
        <v-col cols="12" sm="4">
          <v-number-input
            v-model="settingsStore.avifOptions.threads"
            label="使用するスレッド数 (空欄で自動設定)"
            type="number"
            :min="1"
          />
        </v-col>
      </v-row>
    </v-card-text>
    <v-card-actions>
      <v-btn @click="settingsStore.resetAvifOptions()">AVIFオプションをリセット</v-btn>
    </v-card-actions>
  </v-card>
</template>
