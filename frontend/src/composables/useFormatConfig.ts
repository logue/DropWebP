import { computed, type ComputedRef } from 'vue';
import type { ComposerTranslation } from 'vue-i18n';

import { OutputFormat } from '@/types/SettingsTypes';

/**
 * フォーマット情報の型定義
 */
export interface FormatInfo {
  /** ラジオボタンのラベル */
  label: string;
  /** ラジオボタンの色（マテリアルカラー） */
  color: string;
  /** 項目の説明文（ツールチップ） */
  description: string;
  /** 項目のバッジ（オプション） */
  badge?: string;
}

/**
 * フォーマット設定を管理するComposable
 */
export function useFormatConfig(t: ComposerTranslation) {
  /**
   * 利用可能なフォーマット情報
   */
  const formats: ComputedRef<Record<OutputFormat, FormatInfo>> = computed(() => ({
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

  /**
   * フォーマット情報を取得
   */
  const getFormatInfo = (format: OutputFormat): FormatInfo => {
    return formats.value[format];
  };

  /**
   * フォーマットのラベルを取得
   */
  const getFormatLabel = (format: OutputFormat): string => {
    return formats.value[format].label;
  };

  /**
   * フォーマットの色を取得
   */
  const getFormatColor = (format: OutputFormat): string => {
    return formats.value[format].color;
  };

  return {
    formats,
    getFormatInfo,
    getFormatLabel,
    getFormatColor
  };
}
