export type WebPImageHint = (typeof WebPImageHint)[keyof typeof WebPImageHint];

export const WebPImageHint = {
  Default: 'Default',
  Picture: 'Picture',
  Photo: 'Photo',
  Graph: 'Graph'
} as const;

export type WebPPreset = (typeof WebPPreset)[keyof typeof WebPPreset];

export const WebPPreset = {
  Default: 'Default',
  Drawing: 'Drawing',
  Icon: 'Icon',
  Photo: 'Photo',
  Picture: 'Picture'
} as const;
