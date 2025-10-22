import { defineStore } from 'pinia';
import { ref, type Ref } from 'vue';

import { documentDir } from '@tauri-apps/api/path';
import { open } from '@tauri-apps/plugin-dialog';

import { BitDepth, ColorModel, type AvifOptions } from '@/interfaces/AvifOptions';
import { OutputFormat, type CommonOptions } from '@/interfaces/CommonOptions';
import { ColorEncoding, EncoderSpeed, type JxlOptions } from '@/interfaces/JxlOptions';
import { PngFilter, PngInterlace, type PngOptions } from '@/interfaces/PngOptions';
import { WebPImageHint, type WebpOptions } from '@/interfaces/WebpOptions';

// デフォルト設定を定義
const defaultAvifOptions: AvifOptions = {
  quality: 80.0,
  bitDepth: BitDepth.Auto,
  alphaQuality: 80.0,
  speed: 5,
  colorModel: ColorModel.YCbCr,
  threads: undefined,
  alphaColorMode: 'Premultiplied'
} as const;

const defaultWebpOptions: WebpOptions = {
  quality: 80,
  lossless: true,
  hint: WebPImageHint.Default,
  method: 6,
  autofilter: false
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
  zopfliIterations: 15,
  embedIccProfile: true,
  bitDepthReduction: true,
  colorTypeReduction: true,
  paletteReduction: true,
  interlace: PngInterlace.None,
  filter: PngFilter.MinSum
} as const;

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
  outputPath: ''
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

    /** 設定を初期化 */
    const reset = async () => {
      commonOptions.value = { ...defaultCommonOptions };
      avifOptions.value = { ...defaultAvifOptions };
      webpOptions.value = { ...defaultWebpOptions };
      jxlOptions.value = { ...defaultJxlOptions };
      pngOptions.value = { ...defaultPngOptions };
      commonOptions.value.outputPath = await documentDir();
    };

    const resetCommonOptions = () => (commonOptions.value = { ...defaultCommonOptions });
    const resetAvifOptions = () => (avifOptions.value = { ...defaultAvifOptions });
    const resetWebpOptions = () => (webpOptions.value = { ...defaultWebpOptions });
    const resetJxlOptions = () => (jxlOptions.value = { ...defaultJxlOptions });
    const resetPngOptions = () => (pngOptions.value = { ...defaultPngOptions });

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

    return {
      avifOptions,
      webpOptions,
      jxlOptions,
      pngOptions,
      commonOptions,
      reset,
      resetAvifOptions,
      resetWebpOptions,
      resetCommonOptions,
      resetJxlOptions,
      resetPngOptions,
      browseOutputPath
    };
  },
  {
    persist: {
      storage: window.localStorage
    }
  }
);
