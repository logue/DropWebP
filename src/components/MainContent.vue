<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

import ProgressDialog from './modals/ProgressDialog.vue';

import { useImageConversionController } from '@/composables/useImageConversionController';

const settingsStore = useSettingsStore();
const { t } = useI18n();

const { dialog, inProgress, currentFile, progress, convertByDialog, convertByDirDialog } =
  useImageConversionController(t);

const isEnter = ref(false);
</script>

<template>
  <v-container
    @dragenter="isEnter = true"
    @dragleave="isEnter = false"
    @drop.prevent="isEnter = false"
  >
    <v-sheet
      :class="isEnter ? 'bg-green-lighten-5' : ''"
      class="d-flex align-center justify-center"
      height="300"
      rounded="xl"
    >
      <h2
        class="text-center"
        style="pointer-events: none; user-select: none; opacity: 0.6; max-width: 80%"
      >
        {{ t('info') }}
      </h2>
    </v-sheet>
    <v-row>
      <v-col>
        <v-btn prepend-icon="mdi-file-multiple" class="mr-2" @click="convertByDialog">
          {{ t('select_files') }}
        </v-btn>
      </v-col>
      <v-col>
        <v-btn prepend-icon="mdi-folder-open" @click="convertByDirDialog">
          {{ t('select_folder') }}
        </v-btn>
      </v-col>
      <v-col>
        <v-radio-group v-model="settingsStore.commonOptions.format" inline>
          <v-radio label="WebP" value="webp" color="green" />
          <v-radio label="Avif" value="avif" color="red" />
        </v-radio-group>
      </v-col>
    </v-row>
  </v-container>
  <progress-dialog
    v-model:current-file="currentFile"
    v-model:dialog="dialog"
    v-model:in-progress="inProgress"
    v-model:progress="progress"
    :title="t('progress')"
  />
</template>

<i18n lang="yaml">
en:
  info: Drag and drop image files or Paste image here to compress image.
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
  info: ここに画像ファイルをドラッグ＆ドロップするか、画像を貼り付けることで画像圧縮できます。
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
