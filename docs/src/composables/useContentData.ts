/**
 * Content data composable for features and formats
 */
import avif from '@/assets/Avif-logo-rgb.svg';
import jxl from '@/assets/JPEG_XL_logo.svg';
import jpeg from '@/assets/Mozjpeg_logotype.svg';
import webp from '@/assets/WebPLogo.svg';
import png from '@/assets/zopfli-logo.png';

export const useContentData = () => {
  const { tm } = useI18n();

  // 機能リスト
  const features = [
    {
      icon: 'mdi-image-multiple',
      key: 'multiple_formats'
    },
    {
      icon: 'mdi-lightning-bolt',
      key: 'high_speed'
    },
    {
      icon: 'mdi-drag',
      key: 'drag_drop'
    },
    {
      icon: 'mdi-earth',
      key: 'i18n'
    },
    {
      icon: 'mdi-theme-light-dark',
      key: 'dark_mode'
    },
    {
      icon: 'mdi-clipboard-outline',
      key: 'paste'
    }
  ];

  // フォーマットリスト
  const formats = [
    { key: 'webp', logo: webp },
    { key: 'avif', logo: avif },
    { key: 'jxl', logo: jxl },
    { key: 'png', logo: png },
    { key: 'jpeg', logo: jpeg }
  ];

  // 型安全なdescriptionの取得
  const leadDescriptions = computed(() => {
    try {
      const descriptions = tm('lead.description') as unknown;
      return Array.isArray(descriptions) ? (descriptions as string[]) : [];
    } catch {
      return [];
    }
  });

  // フォーマット説明の取得
  const getFormatDescriptions = (key: string) => {
    try {
      const descriptions = tm(`format.${key}.description`) as unknown;
      return Array.isArray(descriptions) ? (descriptions as string[]) : [];
    } catch {
      return [];
    }
  };

  return {
    features,
    formats,
    leadDescriptions,
    getFormatDescriptions
  };
};
