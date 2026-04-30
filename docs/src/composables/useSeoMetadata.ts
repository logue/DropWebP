/**
 * SEO metadata and structured data composable
 */
import ogp from '@/assets/ogp.png';

export const useSeoMetadata = () => {
  const { locale, t } = useI18n();
  const { version } = useDownloads();

  // サイトのベースURL
  const baseUrl = 'https://logue.dev';
  const sitePath = useRuntimeConfig().app.baseURL;

  const currentUrl = computed(() => {
    const path = locale.value === 'en' ? '' : `/${locale.value}`;
    return `${baseUrl}${sitePath}${path}`;
  });

  // OGP画像
  const ogImage = computed(() => {
    if (ogp.startsWith('/DropWebP/')) {
      return `${baseUrl}${ogp}`;
    }
    return `${baseUrl}${sitePath}${ogp}`;
  });

  // 言語リスト定義
  const languages = [
    { code: 'en', name: '🇺🇸 English' },
    { code: 'ja', name: '🇯🇵 日本語' },
    { code: 'fr', name: '🇫🇷 Français' },
    { code: 'ko', name: '🇰🇷 한국어' },
    { code: 'zhHans', name: '🇨🇳 简体中文' },
    { code: 'zhHant', name: '🇹🇼 繁體中文' }
  ];

  // hreflangタグ
  const hreflangLinks = computed(() => {
    const links = [];

    // x-default（英語）
    links.push({
      rel: 'alternate',
      hreflang: 'x-default',
      href: `${baseUrl}${sitePath}`
    });

    // 各言語
    languages.forEach(lang => {
      if (lang.code === 'en') {
        links.push({
          rel: 'alternate',
          hreflang: 'en',
          href: `${baseUrl}${sitePath}`
        });
      } else {
        links.push({
          rel: 'alternate',
          hreflang: lang.code,
          href: `${baseUrl}${sitePath}${lang.code}/`
        });
      }
    });

    return links;
  });

  // JSON-LD 構造化データ
  const jsonLdData = computed(() => {
    const languageMap: Record<string, string> = {
      ja: 'ja',
      en: 'en',
      fr: 'fr',
      ko: 'ko',
      zhHans: 'zh-CN',
      zhHant: 'zh-TW'
    };

    return {
      '@context': 'https://schema.org',
      '@type': 'SoftwareApplication',
      name: 'Tauri Vue3 App',
      applicationCategory: 'DeveloperApplication',
      operatingSystem: 'Windows, macOS, Linux',
      offers: {
        '@type': 'Offer',
        price: '0',
        priceCurrency: 'USD'
      },
      description: unref(t('lead.description[0]')),
      url: `${unref(baseUrl)}${sitePath}`,
      image: unref(ogImage),
      softwareVersion: unref(version),
      releaseNotes: `https://github.com/logue/tauri-vuetify-starter/releases/tag/${unref(version)}`,
      downloadUrl: `https://github.com/logue/tauri-vuetify-starter/releases/download/${unref(version)}/`,
      author: {
        '@type': 'Person',
        name: 'Logue',
        url: 'https://github.com/logue'
      },
      featureList: [
        unref(t('features.multiple_formats.title')),
        unref(t('features.high_speed.title')),
        unref(t('features.drag_drop.title')),
        unref(t('features.i18n.title')),
        unref(t('features.dark_mode.title')),
        unref(t('features.paste.title'))
      ],
      screenshot: unref(ogImage),
      inLanguage: languageMap[locale.value] || 'en'
    };
  });

  // SEO メタデータの設定
  /**
   * Apply SEO meta tags and structured data to the current page,
   * binding reactive translations and locale-aware OpenGraph fields.
   */
  const setupSeoMeta = () => {
    useSeoMeta({
      title: computed(() => `Tauri Vue3 App - ${t('lead.subtitle')}`),
      ogSiteName: 'Tauri Vue3 App',
      description: computed(() => t('lead.description[0]')),
      ogTitle: 'Tauri Vue3 App',
      ogDescription: computed(() => t('lead.subtitle')),
      ogImage: ogImage,
      ogUrl: currentUrl,
      ogType: 'website',
      ogLocale: computed(() => {
        const localeMap: Record<string, string> = {
          ja: 'ja_JP',
          en: 'en_US',
          fr: 'fr_FR',
          ko: 'ko_KR',
          zhHans: 'zh_CN',
          zhHant: 'zh_TW'
        };
        return localeMap[locale.value] || 'en_US';
      }),
      twitterCard: 'summary_large_image',
      twitterTitle: 'Tauri Vue3 App',
      twitterDescription: computed(() => t('lead.subtitle')),
      twitterImage: ogImage
    });

    useHead({
      link: hreflangLinks,
      script: [
        {
          type: 'application/ld+json',
          innerHTML: () => JSON.stringify(jsonLdData.value)
        }
      ]
    });
  };

  return {
    baseUrl,
    sitePath,
    currentUrl,
    ogImage,
    languages,
    hreflangLinks,
    jsonLdData,
    setupSeoMeta
  };
};
