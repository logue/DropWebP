<script setup lang="ts">
import { useSettingsStore } from '@/store';
import { computed, ref, type ComputedRef } from 'vue';
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

// ラジオボタンの選択肢
const formats: ComputedRef<
  Record<OutputFormat, { label: string; color: string; description: string; badge?: string }>
> = computed(() => ({
  [OutputFormat.WebP]: {
    label: t('formats.webp.label'),
    color: 'orange',
    description: t('formats.webp.description')
  },
  [OutputFormat.AVIF]: {
    label: t('formats.avif.label'),
    color: 'red',
    description: t('formats.avif.description')
  },
  [OutputFormat.JXL]: {
    label: t('formats.jxl.label'),
    color: 'blue',
    description: t('formats.jxl.description'),
    badge: t('formats.jxl.badge')
  },
  [OutputFormat.JPEG]: {
    label: t('formats.jpeg.label'),
    color: 'green',
    description: t('formats.jpeg.description'),
    badge: t('formats.jpeg.badge')
  },
  [OutputFormat.PNG]: {
    label: t('formats.png.label'),
    color: 'pink',
    description: t('formats.png.description'),
    badge: t('formats.png.badge')
  }
}));

const highlightColor = computed(() => {
  return isDragging.value
    ? `bg-${formats.value[settingsStore.commonOptions.format].color}-lighten-5`
    : '';
});

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
      :class="highlightColor"
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

    <v-card flat>
      <v-card-actions>
        <v-radio-group
          v-model="settingsStore.commonOptions.format"
          :label="t('convert_to')"
          class="d-flex justify-end"
          inline
        >
          <v-tooltip
            v-for="(format, key) in formats"
            :key="key"
            :text="format.description"
            location="top"
          >
            <template #activator="{ props }">
              <v-radio v-bind="props" :value="key" :color="format.color">
                <template #label>
                  {{ format.label }}
                  <template v-if="format.badge">
                    &nbsp;
                    <v-chip size="x-small">{{ format.badge }}</v-chip>
                  </template>
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
  formats:
    png:
      label: PNG
      badge: Zopfli Comp.
      description:
        PNG is a widely used lossless image format that supports transparency and is ideal for images with sharp edges and text.
        Zopfli, used in this program, is a more advanced compression algorithm that requires more processing power but significantly reduces the size of the same PNG file while maintaining quality.
        This makes it an ideal choice for preserving original data or recompressing textures for VRC.
    jpeg:
      label: JPEG
      badge: MozJPEG Comp.
      description:
        JPEG is a format primarily used for photos, emphasizing "small size, even at the expense of some image quality loss."
        MozJPEG is a technology developed by Mozilla to make JPEG even smaller while minimizing visual degradation.
        It is recommended for general photos and large photos on websites.
    webp:
      label: WebP
      description:
        WebP is a new image format developed by Google that combines the best features of JPEG and PNG.
        It achieves higher compression rates than PNG when using lossless compression, and maintains image quality comparable to or better than JPEG when using lossy compression.
        It's ideal as a general-purpose image format, such as for general website images.
    avif:
      label: AVIF
      description:
        AVIF is a next-generation lossy image format based on the AV1 Image Codec, offering superior compression and quality characteristics compared to older formats like JPEG and PNG.
        It supports features like HDR, wide color gamut, and transparency, making it recommended for web content that demands high image quality or for websites that require faster performance.
        It achieves higher compression rates while maintaining better image quality than WebP, but requires more processing power.
    jxl:
      label: JPEG XL
      badge: Experimental
      description:
        JPEG XL is a new format developed to overcome the shortcomings of the previous JPEG standard and become the ultimate image format.
        It supports features like lossless and lossy compression, wide color gamut, and high dynamic range (HDR), making it ideal for modern web and mobile applications.
        It's expected to become the standard for this image format in the future, and is recommended for image archiving.
        However, it's still a relatively new format, and support may be limited on some platforms and applications.
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
  formats:
    png:
      label: PNG
      badge: Zopfli Comp.
      description:
        PNG est un format d'image sans perte largement utilisé qui prend en charge la transparence et est idéal pour les images aux contours nets et le texte.
        Zopfli, utilisé dans ce programme, est un algorithme de compression plus avancé qui nécessite plus de puissance de traitement, mais réduit considérablement la taille du fichier PNG tout en préservant la qualité.
        Il est donc idéal pour préserver les données d'origine ou recompresser les textures pour les applications VRC.
    jpeg:
      label: JPEG
      badge: MozJPEG Comp.
      description:
        JPEG est un format principalement utilisé pour les photos, privilégiant la «petite taille, même au prix d'une certaine perte de qualité d'image».
        MozJPEG est une technologie développée par Mozilla pour réduire la taille des fichiers JPEG tout en minimisant la dégradation visuelle.
        Il est recommandé pour les photos courantes et les photos de grande taille sur les sites web.
    webp:
      label: WebP
      description:
        WebP est un nouveau format d'image développé par Google qui combine les meilleures fonctionnalités du JPEG et du PNG.
        Il atteint des taux de compression plus élevés que le PNG avec la compression sans perte et maintient une qualité d'image comparable, voire supérieure, à celle du JPEG avec la compression avec perte.
        C'est un format d'image idéal pour un usage général, notamment pour les images de sites web.
    avif:
      label: AVIF
      description:
        AVIF est un format d'image de nouvelle génération basé sur le codec d'image AV1, offrant des caractéristiques de compression et de qualité supérieures à celles des formats plus anciens comme JPEG et PNG.
        Il prend en charge des fonctionnalités telles que le HDR, une large gamme de couleurs et la transparence, ce qui le rend recommandé pour les contenus web exigeant une qualité d'image élevée ou pour les sites web nécessitant des performances plus rapides.
        Il atteint des taux de compression plus élevés tout en conservant une meilleure qualité d'image que WebP, mais nécessite une puissance de traitement plus importante.
    jxl:
      label: JPEG XL
      badge: Expérimental
      description:
        JPEG XL est un nouveau format développé pour pallier les lacunes de l'ancienne norme JPEG et devenir le format d'image ultime.
        Il prend en charge des fonctionnalités telles que la compression avec et sans perte, une large gamme de couleurs et une plage dynamique élevée (HDR), ce qui le rend idéal pour les applications web et mobiles modernes.
        Il devrait devenir la norme pour ce format d'image à l'avenir et est recommandé pour l'archivage des images.
        Cependant, il s'agit encore d'un format relativement nouveau et la prise en charge peut être limitée sur certaines plates-formes et applications.
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
  formats:
    png:
      label: PNG
      badge: Zopfli圧縮
      description:
        PNGは、広く使用されているロスレス画像形式で、透明性をサポートし、シャープなエッジやテキストを含む画像に最適です。
        本プログラムで使用されているZopfliは、より高度な圧縮アルゴリズムで、変換負荷は高めなものの、品質を維持しながら同じPNGファイルでもサイズを大幅に削減できるため、オリジナルデータの保存やVRC向けのテクスチャの再圧縮に最適な選択肢です。
    jpeg:
      label: JPEG
      badge: MozJPEG圧縮
      description:
        JPEGは、主に写真に使われる形式で、「ある程度画質が落ちてもいいから、とにかく小さく」を重視した形式です。
        MozJPEG（モズジェイペグ）は、このJPEGを「見た目の劣化を抑えつつ、もっと小さく」するためにMozillaが開発した技術です。
        一般的な写真や、Webサイト上の大きな写真の利用におすすめです。
    webp:
      label: WebP
      description:
        WebP（ウェッピー）は、Googleが開発した新しい画像形式で、JPEGとPNGの良いところを合わせたような存在です。
        ロスレス圧縮時はPNGよりも高い圧縮率を実現し、ロッシー圧縮時もJPEGと比較しても同等以上の画質を維持できます。
        Webサイトの画像全般など、汎用的な画像形式として最適です。
    avif:
      label: AVIF
      description:
        AVIFは、AV1 Image Codecをベースにした次世代のロッシーな画像形式で、JPEGやPNGなどの古い形式と比較して優れた圧縮と品質特性を提供します。
        HDR、広色域、透明性などの機能をサポートしており、高画質を求められるWebコンテンツや、Webサイトの更なる高速化を図りたい場合におすすめの形式です。
        WebPよりも高画質を保ったまま高圧縮率を実現できますが、処理負荷が高めです。
    jxl:
      label: JPEG XL
      badge: 実験的
      description:
        JPEG XL（ジェイペグ エックスエル）は、これまでのJPEG規格の欠点を克服し、究極の画像形式を目指して開発された新しい形式です。
        ロスレスおよびロス圧縮、広色域、高ダイナミックレンジ（HDR）などの機能をサポートしており、最新のWebおよびモバイルアプリケーションに最適です。
        将来的にはこの画像形式に統一されることが期待されており、画像のアーカイブとしておすすめの形式です。
        ただし、まだ比較的新しい形式であり、一部のプラットフォームやアプリケーションでのサポートが限られている場合があります。
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
  formats:
    png:
      label: PNG
      badge: Zopfli 압축
      description:
        PNG는 널리 사용되는 무손실 이미지 형식으로, 투명도를 지원하며 선명한 가장자리와 텍스트가 포함된 이미지에 적합합니다.
        이 프로그램에서 사용되는 Zopfli는 더 발전된 압축 알고리즘으로, 처리 부하는 높지만 품질을 유지하면서 동일한 PNG 파일의 크기를 크게 줄일 수 있어 원본 데이터를 보존하거나 VRC용 텍스처를 재압축하는 데 이상적인 선택입니다.
    jpeg:
      label: JPEG
      badge: MozJPEG 압축
      description:
        JPEG는 주로 사진에 사용되는 형식으로, '일부 이미지 품질 손실이 있더라도 가능한 한 작게'를 중시하는 형식입니다.
        MozJPEG는 Mozilla에서 개발한 기술로, JPEG 파일 크기를 더욱 작게 만들면서 시각적 저하를 최소화합니다.
        일반 사진 및 웹사이트의 대형 사진에 권장됩니다.
    webp:
      label: WebP
      description:
        WebP는 Google에서 개발한 새로운 이미지 형식으로, JPEG와 PNG의 장점을 결합한 형식입니다.
        무손실 압축 시 PNG보다 높은 압축률을 달성하며, 손실 압축 시 JPEG와 비교하여 동등하거나 더 나은 이미지 품질을 유지합니다.
        일반 웹사이트 이미지 등 범용 이미지 형식으로 이상적입니다.
    avif:
      label: AVIF
      description:
        AVIF는 AV1 이미지 코덱을 기반으로 한 차세대 손실 이미지 형식으로, JPEG 및 PNG와 같은 이전 형식에 비해 우수한 압축 및 품질 특성을 제공합니다.
        HDR, 광색역 및 투명성과 같은 기능을 지원하여 고품질 이미지가 필요한 웹 콘텐츠나 더 빠른 성능이 필요한 웹사이트에 권장됩니다.
        WebP보다 더 나은 이미지 품질을 유지하면서 더 높은 압축률을 달성하지만 처리 부하가 높습니다.
    jxl:
      label: JPEG XL
      badge: 실험적
      description:
        JPEG XL은 이전 JPEG 표준의 단점을 극복하고 궁극적인 이미지 형식을 목표로 개발된 새로운 형식입니다.
        무손실 및 손실 압축, 광색역 및 고동적 범위(HDR)와 같은 기능을 지원하여 최신 웹 및 모바일 애플리케이션에 이상적입니다.
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
  formats:
    png:
      label: PNG
      badge: Zopfli压缩
      description: PNG 是一种广泛使用的无损影像格式，支持透明度，非常适合包含锐利边缘和文字的影像。
        本程序中使用的 Zopfli 是一种更先进的压缩算法，需要更高的处理能力，但在保持品质的同时显著减小了相同 PNG 文件的大小。
        这使得它成为保留原始数据或为 VRC 重新压缩纹理的理想选择。
    jpeg:
      label: JPEG
      badge: MozJPEG压缩
      description: JPEG 是一种主要用于照片的格式，强调“尺寸小，即使以牺牲一些影像品质为代价”。
        MozJPEG是由 Mozilla 开发的技术，旨在使 JPEG 尺寸更小，同时最大限度地减少视觉效果的下降。
        建议用于网站上的普通照片和大型照片。
    webp:
      label: WebP
      description: WebP 是 Google 开发的一种新型影像格式，它结合了 JPEG 和 PNG 的最佳特性。
        它在无损压缩时可实现比 PNG 更高的压缩率，并在有损压缩时保持与 JPEG 相当甚至更优的影像品质。
        它是理想的通用图像格式，例如用于一般网站图像。
    avif:
      label: AVIF
      description:
        AVIF 是基于 AV1 影像编解码器的下一代有损影像格式，与 JPEG 和 PNG 等旧格式相比，它具有更卓越的压缩率和品质特性。
        它支持 HDR、广色域和透明度等功能，因此推荐用于对影像品质要求较高的网页内容或需要更快效能的网站。
        它在保持比 WebP 更好的影像品质的同时实现了更高的压缩率，但需要更高的处理能力。
    jxl:
      label: JPEG XL
      badge: 实验性功能
      description: JPEG XL是一种新格式，旨在克服先前 JPEG 标准的缺陷，使其成为终极影像格式。
        它支持无损和有损压缩、广色域和高动态范围 (HDR) 等特性，使其成为现代 Web 和行动应用的理想选择。
        预计未来该影像格式将会统一，并且是建议的影像归档格式。
        然而，它仍然是一种相对较新的格式，在某些平台和应用程序上的支持可能有限。
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
  formats:
    png:
      label: PNG
      badge: Zopfli 壓縮
      description: PNG 是一種廣泛使用的無損影像格式，支援透明度，非常適合包含銳利邊緣和文字的影像。
        本程式中使用的 Zopfli 是一種更先進的壓縮演算法，需要更高的處理能力，但在保持品質的同時顯著減小了相同 PNG 檔案的大小。
        這使得它成為保留原始資料或為 VRC 重新壓縮紋理的理想選擇。
    jpeg:
      label: JPEG
      badge: MozJPEG 壓縮
      description: JPEG 是一種主要用於照片的格式，強調「小尺寸，即使以犧牲一些影像品質為代價」。
        MozJPEG是由 Mozilla 開發的技術，旨在使 JPEG 尺寸更小，同時最大限度地減少視覺效果的下降。
        建議用於網站上的普通照片和大型照片。
    webp:
      label: WebP
      description: WebP 是 Google 開發的一種新型影像格式，它結合了 JPEG 和 PNG 的最佳特性。
        它在無損壓縮時可實現比 PNG 更高的壓縮率，並在有損壓縮時保持與 JPEG 相當甚至更優的影像品質。
        它是理想的通用圖像格式，例如用於一般網站圖像。
    avif:
      label: AVIF
      description:
        AVIF 是基於 AV1 影像編解碼器的下一代有損影像格式，與 JPEG 和 PNG 等舊格式相比，它具有更卓越的壓縮率和品質特性。
        它支援 HDR、廣色網域和透明度等功能，因此推薦用於對影像品質要求較高的網頁內容或需要更快效能的網站。
        它在保持比 WebP 更好的影像品質的同時實現了更高的壓縮率，但需要更高的處理能力。
    jxl:
      label: JPEG XL
      badge: 實驗性功能
      description: JPEG XL是一種新格式，旨在克服先前 JPEG 標準的缺陷，使其成為終極影像格式。
        它支援無損和有損壓縮、廣色網域和高動態範圍 (HDR) 等特性，使其成為現代 Web 和行動應用的理想選擇。
        預計未來該影像格式將會統一，並且是建議的影像歸檔格式。
        然而，它仍然是一種相對較新的格式，在某些平台和應用程式上的支援可能有限。
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
