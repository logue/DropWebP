import { defineStore } from 'pinia';
import { ref, type Ref } from 'vue';

import { documentDir } from '@tauri-apps/api/path';
import { open } from '@tauri-apps/plugin-dialog';

import type { AvifOptions } from '@/interfaces/AvifOptions';
import type { CommonOptions } from '@/interfaces/CommonOptions';
import type { JxlOptions } from '@/interfaces/JxlOptions';
import type { WebpOptions } from '@/interfaces/WebpOptions';

// デフォルト設定を定義
const defaultAvifOptions: AvifOptions = {
  quality: 80.0,
  bitDepth: 'Auto',
  alphaQuality: 80.0,
  speed: 5,
  colorModel: 'YCbCr',
  threads: undefined,
  alphaColorMode: 'Premultiplied'
} as const;

const defaultWebpOptions: WebpOptions = {
  quality: 80,
  lossless: true,
  hint: 'Default',
  method: 6,
  autofilter: false
} as const;

const defaultJxlOptions: JxlOptions = {
  lossless: true,
  speed: 'Squirrel',
  quality: 1,
  useContainer: false,
  usesOriginalProfile: false,
  decodingSpeed: 0,
  initBufferSize: 512,
  colorEncoding: 'Srgb'
};

const defaultCommonOptions: CommonOptions = {
  sound: true,
  volume: 1.0,
  notify: true,
  format: 'webp',
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

    /** 設定を初期化 */
    const reset = async () => {
      commonOptions.value = { ...defaultCommonOptions };
      avifOptions.value = { ...defaultAvifOptions };
      webpOptions.value = { ...defaultWebpOptions };
      jxlOptions.value = { ...defaultJxlOptions };
      commonOptions.value.outputPath = await documentDir();
    };

    const resetCommonOptions = () => (commonOptions.value = { ...defaultCommonOptions });
    const resetAvifOptions = () => (avifOptions.value = { ...defaultAvifOptions });
    const resetWebpOptions = () => (webpOptions.value = { ...defaultWebpOptions });
    const resetJxlOptions = () => (jxlOptions.value = { ...defaultJxlOptions });

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
      commonOptions,
      reset,
      resetAvifOptions,
      resetWebpOptions,
      resetCommonOptions,
      resetJxlOptions,
      browseOutputPath
    };
  },
  {
    persist: {
      storage: window.localStorage
    }
  }
);
