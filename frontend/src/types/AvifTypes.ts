// Rustの `enum BitDepth` に対応
export type BitDepth = (typeof BitDepth)[keyof typeof BitDepth];

export const BitDepth = {
  Auto: 'Auto',
  Eight: 'Eight',
  Ten: 'Ten',
  Twelve: 'Twelve'
} as const;

// Rustの `enum ColorModel` に対応
export type ColorModel = (typeof ColorModel)[keyof typeof ColorModel];

export const ColorModel = {
  YCbCr: 'YCbCr',
  RGB: 'RGB'
} as const;
