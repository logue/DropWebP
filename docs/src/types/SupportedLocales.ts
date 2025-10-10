export const SupportedLocales = ['en', 'fr', 'ja', 'ko', 'zhHant', 'zhHans'] as const;
export type SupportedLocale = (typeof SupportedLocales)[number];
