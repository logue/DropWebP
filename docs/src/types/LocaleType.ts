export type Locale = (typeof Locale)[keyof typeof Locale];

export const Locale = {
  en: 'en',
  ja: 'ja',
  fr: 'fr',
  ko: 'ko',
  zhHans: 'zhHans',
  zhHant: 'zhHant'
} as const;
