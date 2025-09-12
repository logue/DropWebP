<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

import ProgressDialog from './modals/ProgressDialog.vue';

import { useImageConversionController } from '@/composables/useImageConversionController';

const settingsStore = useSettingsStore();
const { t } = useI18n();

const { dialog, inProgress, currentFile, progress, convertByDialog } =
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
      class="d-flex align-center justify-center mb-4"
      height="300"
      rounded="xl"
    >
      <h2
        class="text-center"
        style="pointer-events: none; user-select: none; opacity: 0.6; max-width: 80%"
      >
        {{ t('hero_text') }}
      </h2>
    </v-sheet>
    <v-row>
      <v-col>
        <v-btn prepend-icon="mdi-file-multiple" class="mr-2" @click="convertByDialog">
          {{ t('select_files') }}
        </v-btn>
      </v-col>
      <!--v-col>
        <v-btn prepend-icon="mdi-folder-open" @click="convertByDirDialog">
          {{ t('select_folder') }}
        </v-btn>
      </!v-col-->
      <v-col>
        <p>{{ t('convert_to') }}</p>
        <v-radio-group
          v-model="settingsStore.commonOptions.format"
          class="d-flex justify-end"
          inline
        >
          <v-radio label="WebP" value="webp" color="green" />
          <v-radio label="Avif" value="avif" color="red" />
          <v-radio label="JPEG XL" value="jxl" color="blue" />
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
  hero_text: Drag and drop image files or directories here, or paste images to compress them in the format selected by the radio buttons below.
  select_files: Select Files
  select_folder: Select Folder
  progress: Compressing...
  scanning: Scanning images...
  completed: Completed!
  save_as_title: Save As...
  select_files_title: Select image files to compress
  select_directory_title: Select a directory containing images to batch compress
  convert_to: 'Convert to:'
  type:
    webp: WebP Image
    avif: Avif Image
    jxl: JPEG XL Image
  error:
    no_images_found_dropped: No images found in the dropped items.
    no_images_found_selected: No images found in the selected items.
    no_images_found_in_folder: No images found in the selected folder.
ja:
  hero_text: ここに画像ファイルやディレクトリをドラッグ＆ドロップするか、画像をペーストすることで下のラジオボタンの形式で画像圧縮できます。
  select_files: ファイルを選択
  select_folder: フォルダを選択
  progress: 圧縮しています…
  scanning: 画像を走査しています…
  completed: 完了しました！
  save_as_title: 名前を付けて保存…
  select_files_title: 圧縮したい画像ファイルを選択
  select_directory_title: 一括圧縮したい画像の入ったディレクトリを選択
  convert_to: 変換先：
  type:
    webp: WebP画像
    avif: Avif画像
    jxl: JPEG XL画像
  error:
    no_images_found_dropped: ドロップされたアイテムに画像が見つかりませんでした。
    no_images_found_selected: 選択されたアイテムに画像が見つかりませんでした。
    no_images_found_in_folder: フォルダ内に画像が見つかりませんでした。
kr:
  hero_text: 여기에 이미지 파일이나 디렉토리를 드래그 앤 드롭하거나 이미지를 붙여넣기하여 아래의 라디오 버튼 형식으로 이미지 압축할 수 있습니다.
  select_files: 파일 선택
  select_folder: 폴더 선택
  progress: 압축 중...
  scanning: 이미지 검색 중...
  completed: 완료!
  save_as_title: 다른 이름으로 저장...
  select_files_title: 압축할 이미지 파일 선택
  select_directory_title: 일괄 압축할 이미지가 들어 있는 디렉터리 선택
  convert_to: '변환 대상:'
  type:
    webp: WebP 이미지
    avif: Avif 이미지
    jxl: JPEG XL 이미지
  error:
    no_images_found_dropped: 드롭된 항목에서 이미지를 찾을 수 없습니다
    no_images_found_selected: 선택한 항목에서 이미지를 찾을 수 없습니다.
    no_images_found_in_folder: 폴더에서 이미지를 찾을 수 없습니다.
zh:
  hero_text: 將圖像檔案或目錄拖放到此處，或貼上圖像以按照下面單選按鈕選擇的格式進行壓縮。
  select_files: 選擇文件
  select_folder: 選擇文件夾
  progress: 壓縮中...
  scanning: 掃描圖像中...
  completed: 完成！
  save_as_title: 另存為...
  select_files_title: 選擇要壓縮的圖像文件
  select_directory_title: 選擇包含圖像以進行批量壓縮的目錄
  convert_to: 轉換為：
  type:
    webp: WebP 圖像
    avif: Avif 圖像
    jxl: JPEG XL 圖像
  error:
    no_images_found_dropped: 在拖放的項目中未找到圖像。
    no_images_found_selected: 在所選項目中未找到圖像。
    no_images_found_in_folder: 在所選文件夾中未找到圖像。
</i18n>
