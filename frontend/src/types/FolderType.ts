import { documentDir, desktopDir, pictureDir, homeDir } from '@tauri-apps/api/path';

export type FolderType = (typeof FolderType)[keyof typeof FolderType];

export const FolderType = {
  Home: await homeDir(),
  Document: await documentDir(),
  Desktop: await desktopDir(),
  Picture: await pictureDir()
  // ほかのフォルダタイプがあればここに追加
} as const;
