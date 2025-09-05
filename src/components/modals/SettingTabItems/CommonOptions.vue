<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

const settingsStore = useSettingsStore();
const { t } = useI18n();
</script>

<template>
  <v-row>
    <v-col cols="12" sm="6">
      <v-switch
        v-model="settingsStore.commonOptions.ignoreJpeg"
        :label="t('ignore_jpeg')"
        color="primary"
        hide-details
      />
    </v-col>
    <v-col cols="12" sm="6">
      <v-switch
        v-model="settingsStore.commonOptions.overwrite"
        :label="t('overwrite')"
        color="primary"
        disabled
        hide-details
      />
    </v-col>
    <v-col cols="12" sm="6">
      <v-switch
        v-model="settingsStore.commonOptions.sameDirectory"
        :label="t('same_directory')"
        color="primary"
        hide-details
      />
    </v-col>
    <v-col cols="12" sm="6">
      <v-switch
        v-model="settingsStore.commonOptions.deleteOriginal"
        :label="t('delete_original')"
        color="primary"
        disabled
        hide-details
      />
    </v-col>
    <v-col cols="12">
      <v-switch
        v-model="settingsStore.commonOptions.recursive"
        :label="t('recursive')"
        color="primary"
        hide-details
      />
    </v-col>
  </v-row>
  <v-text-field v-model="settingsStore.commonOptions.outputPath" :label="t('output_path')" readonly>
    <template #append>
      <v-btn icon="mdi-folder-open" variant="plain" @click="settingsStore.browseOutputPath()" />
    </template>
  </v-text-field>
  <v-btn @click="settingsStore.resetCommonOptions()">{{ t('reset') }}</v-btn>
</template>

<i18n lang="yaml">
en:
  ignore_jpeg: Ignore JPEG
  overwrite: Overwrite
  same_directory: Output Same Directory
  delete_original: Delete Original
  recursive: Include Subdirectories
  output_path: Default output path
  browse: Browse
  reset: Reset common options
ja:
  ignore_jpeg: JPEGを無視
  overwrite: 上書きする
  same_directory: 同じディレクトリに出力
  delete_original: 元ファイルを削除する
  recursive: サブディレクトリを含める
  output_path: デフォルトの出力先のパス
  browse: ブラウズ
  reset: 共通オプションをリセット
</i18n>
