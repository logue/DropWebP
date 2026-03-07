/**
 * Content data composable for features
 */
export const useContentData = () => {
  const { tm } = useI18n();

  // 機能リスト
  const features = [
    {
      icon: 'mdi-package-variant',
      key: 'multiple_formats'
    },
    {
      icon: 'mdi-lightning-bolt',
      key: 'high_speed'
    },
    {
      icon: 'mdi-application-outline',
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
      icon: 'mdi-shield-check',
      key: 'paste'
    }
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

  return {
    features,
    leadDescriptions
  };
};
