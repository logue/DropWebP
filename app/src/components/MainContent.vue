<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

import ProgressDialog from './modals/ProgressDialog.vue';

import { useImageConversionController } from '@/composables/useImageConversionController';
import { useLogger } from '@/composables/useLogger';

const settingsStore = useSettingsStore();
const { t } = useI18n();
useLogger();

const { dialog, inProgress, currentFile, progress, message, convertByDialog } =
  useImageConversionController(t);

const isEnter = ref(false);
</script>

<template>
  <v-container
    class="fill-height pa-0 d-flex flex-column justify-center"
    @dragenter="isEnter = true"
    @dragleave="isEnter = false"
    @drop.prevent="isEnter = false"
  >
    <v-sheet
      :class="isEnter ? 'bg-green-lighten-5' : ''"
      class="d-flex flex-grow-1 align-center justify-center my-4 px-15"
      rounded="xl"
    >
      <h2 class="text-center text-medium-emphasis">
        {{ t('hero_text') }}
      </h2>
    </v-sheet>
    <v-sheet class="d-flex bg-transparent">
      <v-btn prepend-icon="mdi-file-multiple" class="mr-2" @click="convertByDialog">
        {{ t('select_files') }}
      </v-btn>
      <v-radio-group
        v-model="settingsStore.commonOptions.format"
        :label="t('convert_to')"
        class="d-flex justify-end"
        inline
      >
        <v-tooltip :text="t('type.png_description')" location="top">
          <template #activator="{ props }">
            <v-radio v-bind="props" :label="t('type.png')" value="png" color="purple" />
          </template>
        </v-tooltip>
        <v-tooltip :text="t('type.webp_description')" location="top">
          <template #activator="{ props }">
            <v-radio v-bind="props" :label="t('type.webp')" value="webp" color="green" />
          </template>
        </v-tooltip>
        <v-tooltip :text="t('type.avif_description')" location="top">
          <template #activator="{ props }">
            <v-radio v-bind="props" :label="t('type.avif')" value="avif" color="red" />
          </template>
        </v-tooltip>
        <v-tooltip :text="t('type.jxl_description')" location="top">
          <template #activator="{ props }">
            <v-radio v-bind="props" :label="t('type.jxl')" value="jxl" color="blue">
              <template #label>
                {{ t('type.jxl') }}&nbsp;
                <small class="text-grey">({{ t('experimental') }})</small>
              </template>
            </v-radio>
          </template>
        </v-tooltip>
      </v-radio-group>
    </v-sheet>
  </v-container>
  <progress-dialog
    v-model:current-file="currentFile"
    v-model:dialog="dialog"
    v-model:in-progress="inProgress"
    v-model:progress="progress"
    :title="message"
  />
</template>

<i18n lang="yaml">
en:
  hero_text: Drag and drop images here or paste to compress
  select_files: Select Files
  select_folder: Select Folder
  progress: Compressing {type} format...
  scanning: Scanning images...
  completed: Completed!
  save_as_title: Save As...
  select_files_title: Select image files to compress
  select_directory_title: Select a directory containing images to batch compress
  convert_to: 'Convert to:'
  image: Image
  type:
    png: PNG(Zopfli)
    png_description: PNG is a widely used lossless image format that supports transparency and is ideal for images with sharp edges and text. Zopfli is an advanced compression algorithm that can significantly reduce PNG file sizes while maintaining image quality, making it an excellent choice for web use and re-compressing textures for VRC.
    webp: WebP
    webp_description: WebP is a modern image format that provides superior lossless and lossy compression for images on the web. Using WebP, webmasters and web developers can create smaller, richer images that make the web faster.
    avif: AVIF
    avif_description: AVIF is a next-generation image format that provides superior compression and quality characteristics compared to older formats like JPEG and PNG. It supports features like HDR, wide color gamut, and transparency, making it ideal for modern web and mobile applications.
    jxl: JPEG XL
    jxl_description: JPEG XL is a next-generation image format that provides superior compression and quality characteristics compared to older formats like JPEG and PNG. It supports features like lossless and lossy compression, wide color gamut, and high dynamic range (HDR), making it ideal for modern web and mobile applications.
  experimental: Experimental
  error:
    no_images_found_dropped: No images found in the dropped items.
    no_images_found_selected: No images found in the selected items.
    no_images_found_in_folder: No images found in the selected folder.
ja:
  hero_text: 画像をここにドラッグ＆ドロップするかペースト
  select_files: ファイルを選択
  select_folder: フォルダを選択
  progress: '{type}形式で圧縮しています…'
  scanning: 画像を走査しています…
  completed: 完了しました！
  save_as_title: 名前を付けて保存…
  select_files_title: 圧縮したい画像ファイルを選択
  select_directory_title: 一括圧縮したい画像の入ったディレクトリを選択
  convert_to: 変換先の形式：
  image: 画像
  type:
    png: PNG(Zopfli)
    png_description: PNGは、広く使用されているロスレス画像形式で、透明性をサポートし、シャープなエッジやテキストを含む画像に最適です。本プログラムで使用されているZopfliとは、より高度な圧縮アルゴリズムで、画像品質を維持しながら同じPNGファイルでもサイズを大幅に削減できるため、Webでの使用やVRC向けのテクスチャの再圧縮に最適な選択肢です。
    webp: WebP
    webp_description: WebPは、Web上の画像に対して優れたロスレスおよびロス圧縮を提供する最新の画像形式です。WebPを使用すると、WebマスターやWeb開発者は、より小さく、より豊かな画像を作成して、Webを高速化できます。
    avif: AVIF
    avif_description: AVIFは、JPEGやPNGなどの古い形式と比較して優れた圧縮と品質特性を提供する次世代の画像形式です。HDR、広色域、透明性などの機能をサポートしており、最新のWebおよびモバイルアプリケーションに最適です。
    jxl: JPEG XL
    jxl_description: JPEG XLは、JPEGやPNGなどの古い形式と比較して優れた圧縮と品質特性を提供する次世代の画像形式です。ロスレスおよびロス圧縮、広色域、高ダイナミックレンジ（HDR）などの機能をサポートしており、最新のWebおよびモバイルアプリケーションに最適です。
  experimental: 実験的
  error:
    no_images_found_dropped: ドロップされたアイテムに画像が見つかりませんでした。
    no_images_found_selected: 選択されたアイテムに画像が見つかりませんでした。
    no_images_found_in_folder: フォルダ内に画像が見つかりませんでした。
kr:
  hero_text: 이미지를 여기에 끌어다 놓거나 붙여넣기하여 압축합니다.
  select_files: 파일 선택
  select_folder: 폴더 선택
  progress: '{type} 형식으로 압축 중...'
  scanning: 이미지 검색 중...
  completed: 완료!
  save_as_title: 다른 이름으로 저장...
  select_files_title: 압축할 이미지 파일 선택
  select_directory_title: 일괄 압축할 이미지가 들어 있는 디렉터리 선택
  convert_to: '변환 형식:'
  image: 이미지
  type:
    png: PNG(Zopfli)
    png_description: PNG는 널리 사용되는 무손실 이미지 형식으로 투명도를 지원하며, 선명한 가장자리와 텍스트가 포함된 이미지에 적합합니다. Zopfli는 이미지 품질을 유지하면서 동일한 PNG 파일 크기를 크게 줄일 수 있는 고급 압축 알고리즘으로, 웹 사용 및 VRC용 텍스처 재압축에 탁월한 선택입니다.
    webp: WebP
    webp_description: WebP는 웹의 이미지에 대해 우수한 무손실 및 손실 압축을 제공하는 최신 이미지 형식입니다. WebP를 사용하면 웹마스터와 웹 개발자가 더 작고 풍부한 이미지를 만들어 웹을 더 빠르게 만들 수 있습니다.
    avif: AVIF
    avif_description: AVIF는 JPEG 및 PNG와 같은 이전 형식에 비해 우수한 압축 및 품질 특성을 제공하는 차세대 이미지 형식입니다. HDR, 광색역 및 투명성과 같은 기능을 지원하여 최신 웹 및 모바일 애플리케이션에 적합합니다.
    jxl: JPEG XL
    jxl_description: JPEG XL은 JPEG 및 PNG와 같은 이전 형식에 비해 우수한 압축 및 품질 특성을 제공하는 차세대 이미지 형식입니다. 무손실 및 손실 압축, 광색역 및 고동적 범위(HDR)와 같은 기능을 지원하여 최신 웹 및 모바일 애플리케이션에 적합합니다.
  experimental: 실험적인
  error:
    no_images_found_dropped: 드롭된 항목에서 이미지를 찾을 수 없습니다
    no_images_found_selected: 선택한 항목에서 이미지를 찾을 수 없습니다.
    no_images_found_in_folder: 폴더에서 이미지를 찾을 수 없습니다.
zh:
  hero_text: 將圖像檔案或目錄拖放到此處，或貼上圖像以按照下面單選按鈕選擇的格式進行壓縮。
  select_files: 選擇文件
  select_folder: 選擇文件夾
  progress: 正在壓縮為 {type} 格式...
  scanning: 掃描圖像中...
  completed: 完成！
  save_as_title: 另存為...
  select_files_title: 選擇要壓縮的圖像文件
  select_directory_title: 選擇包含圖像以進行批量壓縮的目錄
  convert_to: 轉換為：
  image: 圖像
  type:
    png: PNG(Zopfli)
    png_description: PNG 是一種廣泛使用的無損圖像格式，支持透明度，非常適合包含銳利邊緣和文本的圖像。本程序中使用的 Zopfli 是一種更先進的壓縮算法，可以在保持圖像質量的同時顯著減小相同 PNG 文件的大小，是網絡使用和 VRC 紋理重新壓縮的絕佳選擇。
    webp: WebP
    webp_description: WebP 是一種現代圖像格式，為網絡上的圖像提供了優越的無損和有損壓縮。使用 WebP，網站管理員和網頁開發人員可以創建更小、更豐富的圖像，從而加快網絡速度。
    avif: AVIF
    avif_description: AVIF 是一種下一代圖像格式，與 JPEG 和 PNG 等較舊格式相比，提供了優越的壓縮和質量特性。它支持 HDR、寬色域和透明度等功能，非常適合現代網絡和移動應用程序。
    jxl: JPEG XL
    jxl_description: JPEG XL 是一種下一代圖像格式，與 JPEG 和 PNG 等較舊格式相比，提供了優越的壓縮和質量特性。它支持無損和有損壓縮、寬色域和高動態範圍 (HDR) 等功能，非常適合現代網絡和移動應用程序。
  experimental: 實驗性
  error:
    no_images_found_dropped: 在拖放的項目中未找到圖像。
    no_images_found_selected: 在所選項目中未找到圖像。
    no_images_found_in_folder: 在所選文件夾中未找到圖像。
</i18n>
