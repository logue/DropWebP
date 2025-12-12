import { defineStore } from 'pinia';
import { ref, type Ref } from 'vue';

import { open } from '@tauri-apps/plugin-dialog';

import { BitDepth, ColorModel, type AvifOptions } from '@/interfaces/AvifOptions';
import { OutputFormat, type CommonOptions } from '@/interfaces/CommonOptions';
import { type JpegOptions } from '@/interfaces/JpegOptions';
import { ColorEncoding, EncoderSpeed, type JxlOptions } from '@/interfaces/JxlOptions';
import { type PngOptions } from '@/interfaces/PngOptions';
import { WebPImageHint, type WebpOptions } from '@/interfaces/WebpOptions';
import { FolderType } from '@/types/FolderType';
import { WebPPreset } from '@/types/WebpTypes';

// デフォルト設定を定義
const defaultAvifOptions: AvifOptions = {
  quality: 80.0,
  bitDepth: BitDepth.Auto,
  alphaQuality: 80.0,
  speed: 6,
  colorModel: ColorModel.YCbCr,
  threads: undefined
} as const;

const defaultWebpOptions: WebpOptions = {
  quality: 80,
  lossless: true,
  method: 4,
  autofilter: true,
  hint: WebPImageHint.Default,
  preset: WebPPreset.Default,
  filterStrength: 50, // 中程度のフィルタ
  filterSharpness: 4, // シャープネス
  snsStrength: 80, // ノイズシェーピング
  alphaQuality: 90
} as const;

const defaultJxlOptions: JxlOptions = {
  lossless: true,
  speed: EncoderSpeed.Squirrel,
  quality: 1,
  useContainer: false,
  usesOriginalProfile: false,
  decodingSpeed: 0,
  initBufferSize: 512,
  colorEncoding: ColorEncoding.Srgb
};

const defaultPngOptions: PngOptions = {
  optimizationLevel: 2,
  useZopfli: true,
  stripMetadata: false,
  bitDepthReduction: false,
  colorTypeReduction: false,
  paletteReduction: false,
  grayscaleReduction: false,
  interlace: null,
  optimizeAlpha: false,
  fastEvaluation: true,
  scale16: false
} as const;

const defaultJpegOptions: JpegOptions = { quality: 95, progressive: true, optimize: true };

const defaultCommonOptions: CommonOptions = {
  sound: true,
  volume: 1.0,
  notify: true,
  format: OutputFormat.WebP,
  overwrite: true,
  deleteOriginal: false,
  recursive: false,
  sameDirectory: true,
  ignoreJpeg: false,
  outputPath: FolderType.Picture
} as const;

/** Global Store */
export default defineStore(
  'settings',
  () => {
    /** 全般オプション */
    const commonOptions: Ref<CommonOptions> = ref({ ...defaultCommonOptions });
    /** AVIFオプション */
    const avifOptions: Ref<AvifOptions> = ref({ ...defaultAvifOptions });
    /** WebPオプション */
    const webpOptions: Ref<WebpOptions> = ref({ ...defaultWebpOptions });
    /** JPEG XLオプション */
    const jxlOptions: Ref<JxlOptions> = ref({ ...defaultJxlOptions });
    /** PNGオプション */
    const pngOptions: Ref<PngOptions> = ref({ ...defaultPngOptions });
    /** JPEGオプション */
    const jpegOptions: Ref<JpegOptions> = ref({ ...defaultJpegOptions });

    /** 設定を初期化 */
    const reset = async () => {
      commonOptions.value = { ...defaultCommonOptions };
      avifOptions.value = { ...defaultAvifOptions };
      webpOptions.value = { ...defaultWebpOptions };
      jxlOptions.value = { ...defaultJxlOptions };
      pngOptions.value = { ...defaultPngOptions };
      jpegOptions.value = { ...defaultJpegOptions };
      // デフォルトは書類フォルダ
      commonOptions.value.outputPath = FolderType.Picture;
    };

    const resetCommonOptions = () => (commonOptions.value = { ...defaultCommonOptions });
    const resetAvifOptions = () => (avifOptions.value = { ...defaultAvifOptions });
    const resetWebpOptions = () => (webpOptions.value = { ...defaultWebpOptions });
    const resetJxlOptions = () => (jxlOptions.value = { ...defaultJxlOptions });
    const resetPngOptions = () => (pngOptions.value = { ...defaultPngOptions });
    const resetJpegOptions = () => (jpegOptions.value = { ...defaultJpegOptions });

    /** 出力先ディレクトリ選択ダイアログ */
    const browseOutputPath = async () => {
      const path = await open({
        multiple: false,
        directory: true
      });
      if (path?.length) {
        commonOptions.value.outputPath = path;
      }
    };

    const setOutputPath = async (type?: FolderType) => {
      switch (type) {
        default:
          commonOptions.value.outputPath = FolderType.Home;
          break;
        case 'document':
          commonOptions.value.outputPath = FolderType.Document;
          break;
        case 'desktop':
          commonOptions.value.outputPath = FolderType.Desktop;
          break;
        case 'picture':
          commonOptions.value.outputPath = FolderType.Picture;
          break;
      }
    };

    return {
      avifOptions,
      webpOptions,
      jxlOptions,
      pngOptions,
      jpegOptions,
      commonOptions,
      reset,
      resetAvifOptions,
      resetWebpOptions,
      resetCommonOptions,
      resetJxlOptions,
      resetPngOptions,
      resetJpegOptions,
      setOutputPath,
      browseOutputPath
    };
  },
  {
    persist: {
      storage: window.localStorage
    }
  }
);
