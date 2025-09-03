<script setup lang="ts">
import { useI18n } from 'vue-i18n';

import ProgressDialog from './ProgressDialog.vue';

import { useImageConversionController } from '@/composables/useImageConversionController';

const { t } = useI18n();

const {
  isLossless,
  quality,
  isRecursive,
  ignoreJpeg,
  isOverwrite,
  dialog,
  inProgress,
  currentFile,
  progress,
  isSameDirectory,
  isDeleteOriginal,
  outputPath,
  browse,
  convertByDialog,
  convertByDirDialog
} = useImageConversionController(t);
</script>

<template>
  <v-container>
    <h2>{{ t('info') }}</h2>
    <v-row>
      <v-col>
        <v-switch v-model="isRecursive" :label="t('recursive')" color="primary" hide-details />
      </v-col>
      <v-col>
        <v-switch v-model="ignoreJpeg" :label="t('ignore_jpeg')" color="primary" hide-details />
      </v-col>
    </v-row>
    <v-row>
      <v-col cols="3">
        <v-switch v-model="isLossless" :label="t('lossless')" color="primary" hide-details />
      </v-col>
      <v-col cols="9">
        <v-slider
          v-model="quality"
          :disabled="isLossless"
          :max="100"
          :min="1"
          :step="1"
          color="primary"
          hide-details
          :label="t('quality')"
        >
          <template #append>
            <span>{{ quality }}</span>
          </template>
        </v-slider>
      </v-col>
    </v-row>

    <v-btn prepend-icon="mdi-file-multiple" class="mr-2" @click="convertByDialog">
      {{ t('select_files') }}
    </v-btn>
    <v-btn prepend-icon="mdi-folder-open" @click="convertByDirDialog">
      {{ t('select_folder') }}
    </v-btn>
    <v-row>
      <v-col>
        <v-switch
          v-model="isSameDirectory"
          :label="t('same_directory')"
          color="primary"
          hide-details
        />
      </v-col>
      <v-col>
        <v-switch v-model="isOverwrite" :label="t('overwrite')" color="primary" hide-details />
      </v-col>
      <v-col>
        <v-switch
          v-model="isDeleteOriginal"
          :label="t('delete_original')"
          color="primary"
          disabled
          hide-details
        />
      </v-col>
    </v-row>
    <v-text-field
      v-model="outputPath"
      :label="t('output_path')"
      readonly
      :disabled="isSameDirectory"
    >
      <template #append>
        <v-btn variant="flat" icon="mdi-folder-open" @click="browse" />
      </template>
    </v-text-field>
  </v-container>
  <progress-dialog
    v-model:dialog="dialog"
    v-model:current-file="currentFile"
    v-model:progress="progress"
    v-model:in-progress="inProgress"
    :title="t('progress')"
  />
</template>

<i18n lang="yaml">
en:
  info: Drag and drop or Paste image files here to compress image.
  recursive: Include Subdirectories
  ignore_jpeg: Ignore JPEG
  lossless: Lossless
  quality: Quality
  overwrite: Overwrite Output
  select_files: Select Files
  select_folder: Select Folder
  same_directory: Same Directory
  delete_original: Delete Original
  output_path: Output Path
  progress: Compressing...
  scanning: Scanning images...
  error:
    no_images_found_dropped: No images found in the dropped items.
    no_images_found_selected: No images found in the selected items.
    no_images_found_in_folder: No images found in the selected folder.
ja:
  info: ここに画像ファイルをドラッグ＆ドロップするか、貼り付けることで画像圧縮できます。
  recursive: サブディレクトリを含む
  ignore_jpeg: JPEGを無視
  lossless: 可逆圧縮
  overwrite: 出力を上書き
  quality: 品質
  select_files: ファイルを選択
  select_folder: フォルダを選択
  same_directory: 同じディレクトリ
  delete_original: オリジナルを削除
  output_path: 出力先
  progress: 圧縮しています…
  scanning: 画像を走査しています…
  error:
    no_images_found_dropped: ドロップされたアイテムに画像が見つかりませんでした。
    no_images_found_selected: 選択されたアイテムに画像が見つかりませんでした。
    no_images_found_in_folder: フォルダ内に画像が見つかりませんでした。
</i18n>
