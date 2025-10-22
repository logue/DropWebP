export type PngFilter = (typeof PngFilter)[keyof typeof PngFilter];

/**
 * PNGフィルター戦略
 */
export const PngFilter = {
  /** フィルターなし */
  None: 'none',
  /** Subフィルター（左のピクセルとの差分）*/
  Sub: 'sub',
  /** Upフィルター（上のピクセルとの差分） */
  Up: 'up',
  /** Averageフィルター（左と上の平均との差分） */
  Average: 'average',
  /** Paethフィルター（予測値と実際の値の差分） */
  Paeth: 'paeth',
  /** 最小合計（すべてのフィルターを試して最小を選択） */
  MinSum: 'minSum',
  /** エントロピー（最小エントロピーのフィルターを選択） */
  Entropy: 'entropy',
  /** Bigrams（2グラム頻度分析） */
  Bigrams: 'bigrams',
  /** BigEnt（BigramsとEntropyの組み合わせ） */
  BigEnt: 'bigEnt',
  /** Brute（すべての組み合わせを試行、最も遅いが最良の圧縮） */
  Brute: 'brute'
} as const;

export type PngInterlace = (typeof PngInterlace)[keyof typeof PngInterlace];

/**
 * PNGインターレース設定
 */
export const PngInterlace = {
  /** インターレースなし（最小ファイルサイズ） */
  None: 'none',
  /** Adam7インターレース（プログレッシブ表示対応、ファイルサイズ増加） */
  Adam7: 'adam7'
} as const;
