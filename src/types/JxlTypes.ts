export type EncoderSpeed = (typeof EncoderSpeed)[keyof typeof EncoderSpeed];

export const EncoderSpeed = {
  Lightning: 'Lightning',
  Thunder: 'Thunder',
  Falcon: 'Falcon',
  Cheetah: 'Cheetah',
  Hare: 'Hare',
  Wombat: 'Wombat',
  Squirrel: 'Squirrel',
  Kitten: 'Kitten',
  Tortoise: 'Tortoise',
  Glacier: 'Glacier'
} as const;

export type ColorEncoding = (typeof ColorEncoding)[keyof typeof ColorEncoding];

export const ColorEncoding = {
  Srgb: 'Srgb',
  LinearSrgb: 'LinearSrgb',
  SrgbLuma: 'SrgbLuma',
  LinearSrgbLuma: 'LinearSrgbLuma'
} as const;
