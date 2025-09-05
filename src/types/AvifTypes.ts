// Rustの `enum BitDepth` に対応
export type BitDepth = (typeof BitDepth)[keyof typeof BitDepth];

export const BitDepth = {
  Auto: 'Auto',
  Eight: 'Eight',
  Ten: 'Ten'
} as const;

// Rustの `enum ColorModel` に対応
export type ColorModel = (typeof ColorModel)[keyof typeof ColorModel];

export const ColorModel = {
  YCbCr: 'YCbCr',
  RGB: 'RGB'
} as const;

// Rustの `enum AlphaColorMode` に対応
export type AlphaColorMode = (typeof AlphaColorMode)[keyof typeof AlphaColorMode];

export const AlphaColorMode = {
  UnassociatedDirty: 'UnassociatedDirty',
  UnassociatedClean: 'UnassociatedClean',
  Premultiplied: 'Premultiplied'
} as const;
