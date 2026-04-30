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
   * Get format information for a given output format.
   *
   * @param format - Target output format.
   * @returns Format metadata such as label, color, and description.
   */
  const getFormatInfo = (format: OutputFormat): FormatInfo => {
    // eslint-disable-next-line security/detect-object-injection -- format is a typed enum key, not user input
    return formats.value[format];
  };

  /**
   * Get the localized label for a format.
   *
   * @param format - Target output format.
   * @returns Localized format label.
   */
  const getFormatLabel = (format: OutputFormat): string => {
    // eslint-disable-next-line security/detect-object-injection -- format is a typed enum key, not user input
    return formats.value[format].label;
  };

  /**
   * Get the theme color associated with a format.
   *
   * @param format - Target output format.
   * @returns Color name string.
   */
  const getFormatColor = (format: OutputFormat): string => {
    // eslint-disable-next-line security/detect-object-injection -- format is a typed enum key, not user input
    return formats.value[format].color;
  };

  return {
    formats,
    getFormatInfo,
    getFormatLabel,
    getFormatColor
  };
}
