export interface PathInfo {
  /** ファイル名（拡張子あり） */
  fileName: string;
  /** 拡張子 */
  extension: string;
  /** 親ディレクトリ（末尾にスラッシュあり） */
  parentDir: string;
  /** ファイルか */
  isFile: boolean;
  /** ディレクトリか */
  isDir: boolean;
  /** 存在するか */
  exists: boolean;
}
