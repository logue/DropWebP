<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { useI18n } from 'vue-i18n';

import ProgressDialog from './modals/ProgressDialog.vue';

import { useImageConversionController } from '@/composables/useImageConversionController';
import { useLogger } from '@/composables/useLogger';

const settingsStore = useSettingsStore();
const { t } = useI18n();
useLogger();

const { dialog, inProgress, currentFile, progress, message, isDragging, convertByDialog } =
  useImageConversionController(t);
</script>

<template>
  <v-container class="fill-height pa-0 d-flex flex-column justify-center">
    <v-sheet
      :class="isDragging ? 'bg-green-lighten-5' : ''"
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
        <v-tooltip :text="t('type.jpeg_description')" location="top">
          <template #activator="{ props }">
            <v-radio v-bind="props" :label="t('type.jpeg')" value="jpeg" color="purple" />
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
    png: PNG (Zopfli)
    png_description: PNG is a widely used lossless image format that supports transparency and is ideal for images with sharp edges and text. Zopfli is an advanced compression algorithm that can significantly reduce PNG file sizes while maintaining image quality, making it an excellent choice for web use and re-compressing textures for VRC.
    jpeg: JPEG (Guetzli)
    jpeg_description: JPEG is a commonly used lossy image format that is ideal for photographs and  complex images. Guetzli is an advanced compression algorithm developed by Google that can produce high-quality JPEG images with smaller file sizes compared to traditional JPEG encoders, making it a great choice for web use and reducing bandwidth usage.
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
fr:
  hero_text: Faites glisser et déposez des images ici ou collez-les pour les compresser
  select_files: Sélectionner des fichiers
  select_folder: Sélectionner un dossier
  progress: Compression au format {type}...
  scanning: Analyse des images...
  completed: Terminé !
  save_as_title: Enregistrer sous...
  select_files_title: Sélectionnez les fichiers image à compresser
  select_directory_title: Sélectionnez un répertoire contenant des images à compresser en lot
  convert_to: 'Convertir en :'
  image: Image
  type:
    png: PNG (Zopfli)
    png_description: PNG est un format d'image sans perte largement utilisé qui prend en charge la transparence et est idéal pour les images avec des bords nets et du texte. Zopfli est un algorithme de compression avancé qui peut réduire considérablement la taille des fichiers PNG tout en maintenant la qualité de l'image, ce qui en fait un excellent choix pour une utilisation sur le web et pour la recompression des textures pour VRC.
    jpeg: JPEG (Guetzli)
    jpeg_description: JPEG est un format d'image avec perte couramment utilisé, idéal pour les photographies et les images complexes. Guetzli est un algorithme de compression avancé développé par Google qui peut produire des images JPEG de haute qualité avec des tailles de fichiers plus petites par rapport aux encodeurs JPEG traditionnels, ce qui en fait un excellent choix pour une utilisation sur le web et pour réduire l'utilisation de la bande passante.
    webp: WebP
    webp_description: WebP est un format d'image moderne qui offre une compression sans perte et avec perte supérieure pour les images sur le web. En utilisant WebP, les webmasters et les développeurs web peuvent créer des images plus petites et plus riches qui rendent le web plus rapide.
    avif: AVIF
    avif_description: AVIF est un format d'image de nouvelle génération qui offre des caractéristiques de compression et de qualité supérieures par rapport aux formats plus anciens comme JPEG et PNG. Il prend en charge des fonctionnalités telles que HDR, la large gamme de couleurs et la transparence, ce qui le rend idéal pour les applications web et mobiles modernes.
    jxl: JPEG XL
    jxl_description: JPEG XL est un format d'image de nouvelle génération qui offre des caractéristiques de compression et de qualité supérieures par rapport aux formats plus anciens comme JPEG et PNG. Il prend en charge des fonctionnalités telles que la compression sans perte et avec perte, la large gamme de couleurs et la plage dynamique élevée (HDR), ce qui le rend idéal pour les applications web et mobiles modernes.
  experimental: Expérimental
  error:
    no_images_found_dropped: Aucune image trouvée dans les éléments déposés.
    no_images_found_selected: Aucune image trouvée dans les éléments sélectionnés.
    no_images_found_in_folder: Aucune image trouvée dans le dossier sélectionné
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
    png: PNG (Zopfli)
    png_description: PNGは、広く使用されているロスレス画像形式で、透明性をサポートし、シャープなエッジやテキストを含む画像に最適です。本プログラムで使用されているZopfliとは、より高度な圧縮アルゴリズムで、画像品質を維持しながら同じPNGファイルでもサイズを大幅に削減できるため、Webでの使用やVRC向けのテクスチャの再圧縮に最適な選択肢です。
    jpeg: JPEG (Guetzli)
    jpeg_description: JPEGは、一般的に使用されているロッシー画像形式で、写真や複雑な画像に最適です。Guetzliは、Googleが開発した高度な圧縮アルゴリズムで、従来のJPEGエンコーダーと比較して、より小さなファイルサイズで高品質のJPEG画像を生成できるため、Webでの使用や帯域幅の使用量の削減に最適な選択肢です。
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
ko:
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
    png: PNG (Zopfli)
    png_description: PNG는 널리 사용되는 무손실 이미지 형식으로 투명도를 지원하며, 선명한 가장자리와 텍스트가 포함된 이미지에 적합합니다. Zopfli는 이미지 품질을 유지하면서 동일한 PNG 파일 크기를 크게 줄일 수 있는 고급 압축 알고리즘으로, 웹 사용 및 VRC용 텍스처 재압축에 탁월한 선택입니다.
    jpeg: JPEG (Guetzli)
    jpeg_description: JPEG는 일반적으로 사용되는 손실 이미지 형식으로 사진 및 복잡한 이미지에 적합합니다. Guetzli는 Google에서 개발한 고급 압축 알고리즘으로, 기존 JPEG 인코더에 비해 더 작은 파일 크기로 고품질 JPEG 이미지를 생성할 수 있어 웹 사용 및 대역폭 사용량 감소에 탁월한 선택입니다.
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
zhHant:
  hero_text: 將圖片拖放到此處或貼上以進行壓縮
  select_files: 選擇文件
  select_folder: 選擇文件夾
  progress: 正在壓縮為 {type} 格式...
  scanning: 正在掃描圖片...
  completed: 完成！
  save_as_title: 另存為...
  select_files_title: 選擇要壓縮的圖片文件
  select_directory_title: 選擇包含圖片以進行批量壓縮的目錄
  convert_to: '轉換為：'
  image: 圖片
  type:
    png: PNG (Zopfli)
    png_description: PNG 是一種廣泛使用的無損圖片格式，支持透明度，非常適合包含銳利邊緣和文字的圖片。本程序中使用的 Zopfli 是一種更先進的壓縮算法，可以在保持圖片質量的同時大幅減小相同 PNG 文件的大小，是網頁使用和 VRC 紋理重新壓縮的絕佳選擇。
    jpeg: JPEG (Guetzli)
    jpeg_description: JPEG 是一種常用的有損圖片格式，非常適合照片和複雜圖片。Guetzli 是 Google 開發的一種先進壓縮算法，與傳統 JPEG 編碼器相比，可以生成高質量且文件大小更小的 JPEG 圖片，是網頁使用和減少帶寬使用的絕佳選擇。
    webp: WebP
    webp_description: WebP 是一種現代圖片格式，為網頁上的圖片提供優異的無損和有損壓縮。使用 WebP，網站管理員和網頁開發人員可以創建更小、更豐富的圖片，使網頁加載更快。
    avif: AVIF
    avif_description: AVIF 是一種新一代圖片格式，與 JPEG 和 PNG 等舊格式相比，提供了優異的壓縮和質量特性。它支持 HDR、寬色域和透明度等功能，非常適合現代網頁和移動應用。
    jxl: JPEG XL
    jxl_description: JPEG XL 是一種新一代圖片格式，與 JPEG 和 PNG 等舊格式相比，提供了優異的壓縮和質量特性。它支持無損和有損壓縮、寬色域和高動態範圍（HDR）等功能，非常適合現代網頁和移動應用。
  experimental: 實驗性功能
  error:
    no_images_found_dropped: 在拖放的項目中未找到圖片。
    no_images_found_selected: 在選定的項目中未找到圖片。
    no_images_found_in_folder: 在所選文件夾中未找到圖片。
zhHans:
  hero_text: 将图片拖放到此处或粘贴以进行压缩
  select_files: 选择文件
  select_folder: 选择文件夹
  progress: 正在压缩为 {type} 格式...
  scanning: 正在扫描图片...
  completed: 完成！
  save_as_title: 另存为...
  select_files_title: 选择要压缩的图片文件
  select_directory_title: 选择包含图片以进行批量压缩的目录
  convert_to: '转换为：'
  image: 图片
  type:
    png: PNG(Zopfli)
    png_description: PNG 是一种广泛使用的无损图片格式，支持透明度，非常适合包含锐利边缘和文字的图片。本程序中使用的 Zopfli 是一种更先进的压缩算法，可以在保持图片质量的同时大幅减小相同 PNG 文件的大小，是网页使用和 VRC 纹理重新压缩的绝佳选择。
    jpeg: JPEG (Guetzli)
    jpeg_description: JPEG 是一种常用的有损图片格式，非常适合照片和复杂图片。Guetzli 是 Google 开发的一种先进压缩算法，与传统 JPEG 编码器相比，可以生成高质量且文件大小更小的 JPEG 图片，是网页使用和减少带宽使用的绝佳选择。
    webp: WebP
    webp_description: WebP 是一种现代图片格式，为网页上的图片提供优异的无损和有损压缩。使用 WebP，网站管理员和网页开发人员可以创建更小、更丰富的图片，使网页加载更快。
    avif: AVIF
    avif_description: AVIF 是一种新一代图片格式，与 JPEG 和 PNG 等旧格式相比，提供了优异的压缩和质量特性。它支持 HDR、宽色域和透明度等功能，非常适合现代网页和移动应用。
    jxl: JPEG XL
    jxl_description: JPEG XL 是一种新一代图片格式，与 JPEG 和 PNG 等旧格式相比，提供了优异的压缩和质量特性。它支持无损和有损压缩、宽色域和高动态范围（HDR）等功能，非常适合现代网页和移动应用。
  experimental: 实验性功能
  error:
    no_images_found_dropped: 在拖放的项目中未找到图片。
    no_images_found_selected: 在选定的项目中未找到图片。
    no_images_found_in_folder: 在所选文件夹中未找到图片。
</i18n>
