import { defineStore } from 'pinia';
import { computed, ref, type Ref } from 'vue';

import { documentDir } from '@tauri-apps/api/path';
import { open } from '@tauri-apps/plugin-dialog';

import type { AvifOptions } from '@/interfaces/AvifOptions';
import type { CommonOptions } from '@/interfaces/CommonOptions';
import type { WebpOptions } from '@/interfaces/WebpOptions';

// デフォルト設定を定義
const defaultAvifOptions: AvifOptions = {
  quality: 80.0,
  bitDepth: 'Auto',
  alphaQuality: 80.0,
  speed: 5,
  colorModel: 'YCbCr',
  threads: undefined,
  alphaColorMode: 'UnassociatedClean'
} as const;

const defaultWebpOptions: WebpOptions = {
  quality: 80,
  lossless: true
} as const;

const defaultCommonOptions: CommonOptions = {
  format: 'webp',
  overwrite: false,
  deleteOriginal: false,
  recursive: false,
  sameDirectory: true,
  ignoreJpeg: false,
  outputPath: await documentDir()
} as const;

/** Global Store */
export default defineStore(
  'encode_settings',
  () => {
    // State

    /** AVIF Options */
    const avifOptions: Ref<AvifOptions> = ref({ ...defaultAvifOptions });
    /** WebP Options */
    const webpOptions: Ref<WebpOptions> = ref({ ...defaultWebpOptions });
    /** Overwrite original file */
    const commonOptions: Ref<CommonOptions> = ref({ ...defaultCommonOptions });

    const extensionPattern: Ref<RegExp> = computed(() =>
      commonOptions.value.ignoreJpeg
        ? /\.(png|gif|tif?f|bmp|heic|heif|jp2|j2k)$/i
        : /\.(jpe?g|png|gif|tif?f|bmp|heic|heif|jp2|j2k)$/i
    );

    /** Reset to default settings */
    const reset = () => {
      avifOptions.value = { ...defaultAvifOptions };
      webpOptions.value = { ...defaultWebpOptions };
      commonOptions.value = { ...defaultCommonOptions };
    };

    const resetAvifOptions = () => {
      avifOptions.value = { ...defaultAvifOptions };
    };

    const resetWebpOptions = () => {
      webpOptions.value = { ...defaultWebpOptions };
    };

    const resetCommonOptions = () => {
      commonOptions.value = { ...defaultCommonOptions };
    };

    /** 出力先ディレクトリ選択ダイアログ */
    const browseOutputPath = async () => {
      const path = await open({
        multiple: false,
        directory: true
      });
      if (path && path.length) {
        commonOptions.value.outputPath = path as string;
      }
    };

    return {
      avifOptions,
      webpOptions,
      commonOptions,
      extensionPattern,
      reset,
      resetAvifOptions,
      resetWebpOptions,
      resetCommonOptions,
      browseOutputPath
    };
  },
  {
    persist: {
      storage: window.localStorage
    }
  }
);
