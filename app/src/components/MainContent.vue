<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

import ContextMenu from './ContextMenu.vue';
import ProgressDialog from './modals/ProgressDialog.vue';

import { useImageConversionController } from '@/composables/useImageConversionController';
import { useLogger } from '@/composables/useLogger';
import { OutputFormat } from '@/types/SettingsTypes';

const settingsStore = useSettingsStore();
const { t } = useI18n();
useLogger();

const {
  dialog,
  inProgress,
  currentFile,
  progress,
  message,
  isDragging,
  convertByDialog,
  handlePaste
} = useImageConversionController(t);

// コンテキストメニューの状態
const menuVisibility = ref(false);
const contextMenuX = ref(0);
const contextMenuY = ref(0);

// 右クリックハンドラー
const onRightClick = (e: MouseEvent) => {
  e.preventDefault();
  contextMenuX.value = e.clientX;
  contextMenuY.value = e.clientY;
  menuVisibility.value = true;
};

// ペースト処理
const onPasteFromContextMenu = async () => {
  try {
    const items = await navigator.clipboard.read();
    for (const item of items) {
      for (const type of item.types) {
        if (type.startsWith('image/')) {
          const blob = await item.getType(type);
          // ペーストイベントを模擬
          const fakeEvent = new ClipboardEvent('paste', {
            clipboardData: new DataTransfer()
          });
          // DataTransferにファイルを追加
          const file = new File([blob], 'pasted-image.' + type.split('/')[1], { type });
          fakeEvent.clipboardData?.items.add(file);
          handlePaste?.(fakeEvent);
          break;
        }
      }
    }
  } catch (error) {
    console.error('クリップボードからの読み取りに失敗しました:', error);
  }
};
</script>

<template>
  <v-container
    class="fill-height pa-0 d-flex flex-column justify-center"
    @contextmenu="onRightClick"
  >
    <v-sheet
      :class="isDragging ? 'bg-green-lighten-5' : ''"
      class="d-flex flex-grow-1 align-center justify-center my-4 px-15"
      rounded="xl"
    >
      <div class="text-center text-medium-emphasis">
        <h2>
          {{ t('hero_text') }}
        </h2>
        <v-btn
          :text="t('select_files')"
          class="mt-4"
          prepend-icon="mdi-file-multiple"
          variant="elevated"
          @click="convertByDialog"
        />
      </div>
    </v-sheet>

    <v-card class="d-flex bg-transparent" flat>
      <v-card-actions>
        <v-radio-group
          v-model="settingsStore.commonOptions.format"
          :label="t('convert_to')"
          class="d-flex justify-end"
          inline
        >
          <v-tooltip :text="t('type.webp_description')" location="top">
            <template #activator="{ props }">
              <v-radio
                v-bind="props"
                :label="t('type.webp')"
                :value="OutputFormat.WebP"
                color="green"
              />
            </template>
          </v-tooltip>
          <v-tooltip :text="t('type.avif_description')" location="top">
            <template #activator="{ props }">
              <v-radio
                v-bind="props"
                :label="t('type.avif')"
                :value="OutputFormat.AVIF"
                color="red"
              />
            </template>
          </v-tooltip>
          <v-tooltip :text="t('type.jxl_description')" location="top">
            <template #activator="{ props }">
              <v-radio v-bind="props" :label="t('type.jxl')" :value="OutputFormat.JXL" color="blue">
                <template #label>
                  {{ t('type.jxl') }}&nbsp;
                  <v-chip size="x-small">{{ t('experimental') }}</v-chip>
                </template>
              </v-radio>
            </template>
          </v-tooltip>
          <v-tooltip :text="t('type.png_description')" location="top">
            <template #activator="{ props }">
              <v-radio
                v-bind="props"
                :label="t('type.png')"
                :value="OutputFormat.PNG"
                color="purple"
              >
                <template #label>
                  {{ t('type.png') }}&nbsp;
                  <small class="text-grey">({{ t('zopfli') }})</small>
                </template>
              </v-radio>
            </template>
          </v-tooltip>
          <v-tooltip :text="t('type.jpeg_description')" location="top">
            <template #activator="{ props }">
              <v-radio
                v-bind="props"
                :label="t('type.jpeg')"
                :value="OutputFormat.JPEG"
                color="orange"
              >
                <template #label>
                  {{ t('type.jpeg') }}&nbsp;
                  <small class="text-grey">({{ t('mozjpeg') }})</small>
                </template>
              </v-radio>
            </template>
          </v-tooltip>
        </v-radio-group>
      </v-card-actions>
    </v-card>
  </v-container>

  <!-- コンテキストメニュー -->
  <context-menu
    v-model:show="menuVisibility"
    :x="contextMenuX"
    :y="contextMenuY"
    @paste="onPasteFromContextMenu"
  />

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
  hero_text: Drag and drop images here or paste to compress.
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
    png: PNG
    png_description:
      PNG is a widely used lossless image format that supports transparency and is ideal for images with sharp edges and text.
      Zopfli, used in this program, is a more advanced compression algorithm that requires more processing power but significantly reduces the size of the same PNG file while maintaining quality.
      This makes it an ideal choice for preserving original data or recompressing textures for VRC.
    jpeg: JPEG
    jpeg_description:
      JPEG is a format primarily used for photos, emphasizing "small size, even at the expense of some image quality loss."
      MozJPEG is a technology developed by Mozilla to make JPEG even smaller while minimizing visual degradation.
      It is recommended for general photos and large photos on websites.
    webp: WebP
    webp_description:
      WebP is a new image format developed by Google that combines the best features of JPEG and PNG.
      It achieves higher compression rates than PNG when using lossless compression, and maintains image quality comparable to or better than JPEG when using lossy compression.
      It's ideal as a general-purpose image format, such as for general website images.
    avif: AVIF
    avif_description:
      AVIF is a next-generation lossy image format based on the AV1 Image Codec, offering superior compression and quality characteristics compared to older formats like JPEG and PNG.
      It supports features like HDR, wide color gamut, and transparency, making it recommended for web content that demands high image quality or for websites that require faster performance.
      It achieves higher compression rates while maintaining better image quality than WebP, but requires more processing power.
    jxl: JPEG XL
    jxl_description:
      JPEG XL is a new format developed to overcome the shortcomings of the previous JPEG standard and become the ultimate image format.
      It supports features like lossless and lossy compression, wide color gamut, and high dynamic range (HDR), making it ideal for modern web and mobile applications.
      It's expected to become the standard for this image format in the future, and is recommended for image archiving.
      However, it's still a relatively new format, and support may be limited on some platforms and applications.
  experimental: Experimental
  zopfli: Zopfli
  mozjpeg: MozJPEG
  error:
    no_images_found_dropped: No images found in the dropped items.
    no_images_found_selected: No images found in the selected items.
    no_images_found_in_folder: No images found in the selected folder.
  notification:
    complete:
      title: Image Conversion Complete
      message: Conversion of {file} to {format} format is complete.
    batch_complete:
      title: Image Batch Conversion Complete
      message: Conversion of {count} images to {format} format is complete.
    error:
      title: Image Conversion Error
      message: '{message}'
fr:
  hero_text: Faites glisser et déposez des images ici ou collez-les pour les compresser.
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
    png: PNG
    png_description:
      PNG est un format d'image sans perte largement utilisé qui prend en charge la transparence et est idéal pour les images aux contours nets et le texte.
      Zopfli, utilisé dans ce programme, est un algorithme de compression plus avancé qui nécessite plus de puissance de traitement, mais réduit considérablement la taille du fichier PNG tout en préservant la qualité.
      Il est donc idéal pour préserver les données d'origine ou recompresser les textures pour les applications VRC.
    jpeg: JPEG
    jpeg_description:
      JPEG est un format principalement utilisé pour les photos, privilégiant la «petite taille, même au prix d'une certaine perte de qualité d'image».
      MozJPEG est une technologie développée par Mozilla pour réduire la taille des fichiers JPEG tout en minimisant la dégradation visuelle.
      Il est recommandé pour les photos courantes et les photos de grande taille sur les sites web.
    webp: WebP
    webp_description:
      WebP est un nouveau format d'image développé par Google qui combine les meilleures fonctionnalités du JPEG et du PNG.
      Il atteint des taux de compression plus élevés que le PNG avec la compression sans perte et maintient une qualité d'image comparable, voire supérieure, à celle du JPEG avec la compression avec perte.
      C'est un format d'image idéal pour un usage général, notamment pour les images de sites web.
    avif: AVIF
    avif_description:
      AVIF est un format d'image avec perte de nouvelle génération basé sur le codec d'image AV1, offrant des caractéristiques de compression et de qualité supérieures à celles des formats plus anciens comme JPEG et PNG.
      Il prend en charge des fonctionnalités telles que le HDR, une large gamme de couleurs et la transparence, ce qui le rend recommandé pour les contenus web exigeant une qualité d'image élevée ou pour les sites web nécessitant des performances plus rapides.
      Il atteint des taux de compression plus élevés tout en conservant une meilleure qualité d'image que WebP, mais nécessite une puissance de traitement plus importante.
    jxl: JPEG XL
    jxl_description:
      JPEG XL  est un nouveau format développé pour pallier les lacunes de l'ancienne norme JPEG et devenir le format d'image ultime.
      Il prend en charge des fonctionnalités telles que la compression avec et sans perte, une large gamme de couleurs et une plage dynamique élevée (HDR), ce qui le rend idéal pour les applications web et mobiles modernes.
      Il devrait devenir la norme pour ce format d'image à l'avenir et est recommandé pour l'archivage des images.
      Cependant, il s'agit encore d'un format relativement nouveau et la prise en charge peut être limitée sur certaines plates-formes et applications.
  experimental: Expérimental
  zopfli: Zopfli
  mozjpeg: MozJPEG
  error:
    no_images_found_dropped: Aucune image trouvée dans les éléments déposés.
    no_images_found_selected: Aucune image trouvée dans les éléments sélectionnés.
    no_images_found_in_folder: Aucune image trouvée dans le dossier sélectionné.
  notification:
    complete:
      title: Conversion d'image terminée
      message: La conversion de {file} au format {format} est terminée.
    batch_complete:
      title: Conversion par lot d'images terminée
      message: La conversion de {count} images au format {format} est terminée.
    error:
      title: Erreur de conversion d'image
      message: '{message}'
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
    png: PNG
    png_description:
      PNGは、広く使用されているロスレス画像形式で、透明性をサポートし、シャープなエッジやテキストを含む画像に最適です。
      本プログラムで使用されているZopfliは、より高度な圧縮アルゴリズムで、変換負荷は高めなものの、品質を維持しながら同じPNGファイルでもサイズを大幅に削減できるため、オリジナルデータの保存やVRC向けのテクスチャの再圧縮に最適な選択肢です。
    jpeg: JPEG
    jpeg_description:
      JPEGは、主に写真に使われる形式で、「ある程度画質が落ちてもいいから、とにかく小さく」を重視した形式です。
      MozJPEG（モズジェイペグ）は、このJPEGを「見た目の劣化を抑えつつ、もっと小さく」するためにMozillaが開発した技術です。
      一般的な写真や、Webサイト上の大きな写真の利用におすすめです。
    webp: WebP
    webp_description:
      WebP（ウェッピー）は、Googleが開発した新しい画像形式で、JPEGとPNGの良いところを合わせたような存在です。
      ロスレス圧縮時はPNGよりも高い圧縮率を実現し、ロッシー圧縮時もJPEGと比較しても同等以上の画質を維持できます。
      Webサイトの画像全般など、汎用的な画像形式として最適です。
    avif: AVIF
    avif_description:
      AVIFは、AV1 Image Codecをベースにした次世代のロッシーな画像形式で、JPEGやPNGなどの古い形式と比較して優れた圧縮と品質特性を提供します。
      HDR、広色域、透明性などの機能をサポートしており、高画質を求められるWebコンテンツや、Webサイトの更なる高速化を図りたい場合におすすめの形式です。
      WebPよりも高画質を保ったまま高圧縮率を実現できますが、処理負荷が高めです。
    jxl: JPEG XL
    jxl_description:
      JPEG XL（ジェイペグ エックスエル）は、これまでのJPEG規格の欠点を克服し、究極の画像形式を目指して開発された新しい形式です。
      ロスレスおよびロス圧縮、広色域、高ダイナミックレンジ（HDR）などの機能をサポートしており、最新のWebおよびモバイルアプリケーションに最適です。
      将来的にはこの画像形式に統一されることが期待されており、画像のアーカイブとしておすすめの形式です。
      ただし、まだ比較的新しい形式であり、一部のプラットフォームやアプリケーションでのサポートが限られている場合があります。
  experimental: 実験的
  zopfli: Zopfli
  mozjpeg: MozJPEG
  error:
    no_images_found_dropped: ドロップされたアイテムに画像が見つかりませんでした。
    no_images_found_selected: 選択されたアイテムに画像が見つかりませんでした。
    no_images_found_in_folder: フォルダ内に画像が見つかりませんでした。
  notification:
    complete:
      title: 画像変換完了
      message: '{file}の{format}形式への変換が完了しました。'
    batch_complete:
      title: 画像バッチ変換完了
      message: '{count}個の画像の{format}形式への変換が完了しました。'
    error:
      title: 画像変換エラー
      message: '{message}'
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
    png: PNG
    png_description:
      PNG는 널리 사용되는 무손실 이미지 형식으로 투명성을 지원하며 선명한 모서리와 텍스트가 포함된 이미지에 이상적입니다.
      본 프로그램에서 사용되는 Zoffli는 보다 고도의 압축 알고리즘으로 변환 부하는 높지만 품질을 유지하면서 같은 PNG 파일에서도 크기를 대폭 줄일 수 있기 때문에 원본 데이터의 저장이나 VRC용 텍스처의 재압축에 최적인 선택입니다.
    jpeg: JPEG
    jpeg_description:
      JPEG는 주로 사진에 사용되는 형식으로 "어느 정도 화질이 떨어질 수 있기 때문에 어쨌든 작다"를 중시한 형식입니다.
      MozJPEG(모즈제이페그)는, 이 JPEG를 「외형의 열화를 억제하면서, 보다 작게」하기 위해서 Mozilla가 개발한 기술입니다.
      일반적인 사진이나, 웹 사이트상의 큰 사진의 이용에 추천입니다.
    webp: WebP
    webp_description:
      WebP(웨이피)는 Google이 개발한 새로운 이미지 형식으로 JPEG와 PNG의 좋은 부분을 맞춘 것 같은 존재입니다.
      무손실 압축 시에는 PNG보다 높은 압축률을 실현하고, 로시 압축 시에도 JPEG와 비교해도 동등 이상의 화질을 유지할 수 있습니다.
      웹 사이트의 이미지 전반 등 범용적인 이미지 형식으로 최적입니다.
    avif: AVIF
    avif_description:
      AVIF는 AV1 Image Codec을 기반으로 한 차세대 로시 이미지 형식으로 JPEG 및 PNG와 같은 이전 형식과 비교하여 우수한 압축 및 품질 특성을 제공합니다.
      HDR, 광색역, 투명성 등의 기능을 서포트하고 있어 고화질을 요구하는 Web 컨텐츠나, Web 사이트의 한층 더 고속화를 도모하고 싶은 경우에 추천의 형식입니다.
      WebP보다 고화질을 유지한 채로 높은 압축률을 실현할 수 있습니다만, 처리 부하가 높습니다.
    jxl: JPEG XL
    jxl_description:
      JPEG XL(제이펙 엑스엘)은 지금까지의 JPEG 규격의 단점을 극복하고, 궁극의 화상 형식을 목표로 개발된 새로운 형식입니다.
      무손실 및 무손실 압축, 넓은 색 영역, 고 동적 범위(HDR) 등의 기능을 지원하며 최신 웹 및 모바일 애플리케이션에 이상적입니다.
      앞으로는 이 이미지 형식으로 통일될 것으로 기대되고 있어 이미지의 아카이브로서 추천하는 형식입니다.
      그러나 여전히 비교적 새로운 형식이므로 일부 플랫폼 및 응용 프로그램에 대한 지원이 제한되어 있을 수 있습니다.
  experimental: 실험적인
  zopfli: Zopfli
  mozjpeg: MozJPEG
  error:
    no_images_found_dropped: 드롭된 항목에서 이미지를 찾을 수 없습니다
    no_images_found_selected: 선택한 항목에서 이미지를 찾을 수 없습니다.
    no_images_found_in_folder: 폴더에서 이미지를 찾을 수 없습니다.
  notification:
    complete:
      title: 이미지 변환 완료
      message: '{file}의 {format} 형식 변환이 완료되었습니다.'
    batch_complete:
      title: 이미지 일괄 변환 완료
      message: '{count}개의 이미지의 {format} 형식 변환이 완료되었습니다.'
    error:
      title: 이미지 변환 오류
      message: '{message}'
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
    png: PNG
    png_description:
      PNG 是一种广泛使用的无损图像格式，支持透明度，非常适合包含锐利边缘和文字的图像。
      本程序中使用的 Zopfli 是一种更先进的压缩算法，需要更高的处理能力，但在保持质量的同时显著减小了相同 PNG 文件的大小。
      这使得它成为保留原始数据或为 VRC 重新压缩纹理的理想选择。
    jpeg: JPEG
    jpeg_description: JPEG 是一种主要用于照片的格式，强调“小尺寸，即使以牺牲一些图像质量为代价”。
      MozJPEG 是由 Mozilla 开发的一项技术，旨在使 JPEG 尺寸更小，同时最大限度地减少视觉效果的下降。
      建议用于网站上的普通照片和大型照片。
    webp: WebP
    webp_description: WebP 是 Google 开发的一种新型图像格式，它结合了 JPEG 和 PNG 的最佳特性。
      它在无损压缩时可实现比 PNG 更高的压缩率，并在有损压缩时保持与 JPEG 相当甚至更优的图像质量。
      它是理想的通用图像格式，例如用于一般网站图像。
    avif: AVIF
    avif_description:
      AVIF 是基于 AV1 图像编解码器的下一代有损图像格式，与 JPEG 和 PNG 等旧格式相比，它具有更卓越的压缩率和质量特性。
      它支持 HDR、广色域和透明度等功能，因此推荐用于对图像质量要求较高的网页内容或需要更快性能的网站。
      它在保持比 WebP 更好的图像质量的同时实现了更高的压缩率，但需要更高的处理能力。
    jxl: JPEG XL
    jxl_description:
      JPEG XL（Jpeg XL）是一种新格式，旨在克服先前 JPEG 标准的缺陷，使其成为终极图像格式。
      它支持无损和有损压缩、广色域和高动态范围 (HDR) 等特性，使其成为现代 Web 和移动应用的理想选择。
      预计未来该图像格式将会统一，并且是推荐的图像归档格式。
      然而，它仍然是一种相对较新的格式，在某些平台和应用程序上的支持可能有限。
  experimental: 实验性功能
  zopfli: Zopfli
  mozjpeg: MozJPEG
  error:
    no_images_found_dropped: 在拖放的项目中未找到图片。
    no_images_found_selected: 在选定的项目中未找到图片。
    no_images_found_in_folder: 在所选文件夹中未找到图片。
  notification:
    complete:
      title: 图片转换完成
      message: '{file} 的 {format} 格式转换已完成。'
    batch_complete:
      title: 图片批量转换完成
      message: '{count} 个图片的 {format} 格式转换已完成。'
    error:
      title: 图片转换错误
      message: '{message}'
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
    png: PNG
    png_description:
      PNG 是一種廣泛使用的無損影像格式，支援透明度，非常適合包含銳利邊緣和文字的影像。
      本程式中使用的 Zopfli 是一種更先進的壓縮演算法，需要更高的處理能力，但在保持品質的同時顯著減小了相同 PNG 檔案的大小。
      這使得它成為保留原始資料或為 VRC 重新壓縮紋理的理想選擇。
    jpeg: JPEG
    jpeg_description: JPEG 是一種主要用於照片的格式，強調「小尺寸，即使以犧牲一些影像品質為代價」。
      MozJPEG是由 Mozilla 開發的技術，旨在使 JPEG 尺寸更小，同時最大限度地減少視覺效果的下降。
      建議用於網站上的普通照片和大型照片。
    webp: WebP
    webp_description: WebP 是 Google 開發的一種新型影像格式，它結合了 JPEG 和 PNG 的最佳特性。
      它在無損壓縮時可實現比 PNG 更高的壓縮率，並在有損壓縮時保持與 JPEG 相當甚至更優的影像品質。
      它是理想的通用圖像格式，例如用於一般網站圖像。
    avif: AVIF
    avif_description:
      AVIF 是基於 AV1 影像編解碼器的下一代有損影像格式，與 JPEG 和 PNG 等舊格式相比，它具有更卓越的壓縮率和品質特性。
      它支援 HDR、廣色網域和透明度等功能，因此推薦用於對影像品質要求較高的網頁內容或需要更快效能的網站。
      它在保持比 WebP 更好的影像品質的同時實現了更高的壓縮率，但需要更高的處理能力。
    jxl: JPEG XL
    jxl_description: JPEG XL是一種新格式，旨在克服先前 JPEG 標準的缺陷，使其成為終極影像格式。
      它支援無損和有損壓縮、廣色域和高動態範圍 (HDR) 等特性，使其成為現代 Web 和行動應用的理想選擇。
      預計未來該影像格式將會統一，並且是建議的影像歸檔格式。
      然而，它仍然是一種相對較新的格式，在某些平台和應用程式上的支援可能有限。
  experimental: 实验性功能
  zopfli: Zopfli
  mozjpeg: MozJPEG
  error:
    no_images_found_dropped: 在拖放的项目中未找到图片。
    no_images_found_selected: 在选定的项目中未找到图片。
    no_images_found_in_folder: 在所选文件夹中未找到图片。
  notification:
    complete:
      title: 图片转换完成
      message: '{file} 的 {format} 格式转换已完成。'
    batch_complete:
      title: 图片批量转换完成
      message: '{count} 个图片的 {format} 格式转换已完成。'
    error:
      title: 图片转换错误
      message: '{message}'
</i18n>
