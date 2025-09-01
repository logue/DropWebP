import { join } from '@tauri-apps/api/path';
import { readDir, type DirEntry } from '@tauri-apps/plugin-fs';

/**
 * ディレクトリからファイルを収集する
 * @param path 入力ファイル
 * @param extension 拡張子正規表現
 * @param recursive 再起的に探索するか
 * @returns ファイルパスの配列
 */
async function collectFilesFromDir(
  path: string,
  extension: RegExp,
  recursive = false
): Promise<string[]> {
  const entries: DirEntry[] = await readDir(path);
  let files: string[] = [];

  for (const entry of entries) {
    const fullPath = await join(path, entry.name); // フルパス生成
    if (entry.isFile && extension.exec(entry.name)) {
      files.push(fullPath);
    } else if (recursive && entry.isDirectory) {
      const sub = await collectFilesFromDir(fullPath, extension, recursive);
      files = files.concat(sub);
    }
  }

  return files;
}

/**
 * ファイル or フォルダのパス配列を受け取ってファイル一覧に正規化
 * @param paths 入力パス配列
 * @param extension 拡張子正規表現
 * @param recursive 再起的に探索するか
 * @returns ファイルパスの配列
 */
export async function collectFiles(
  paths: string[],
  extension: RegExp,
  recursive = false
): Promise<string[]> {
  let results: string[] = [];

  for (const p of paths) {
    if (extension.test(p)) {
      results.push(p);
    } else {
      const subFiles = await collectFilesFromDir(p, extension, recursive);
      results = results.concat(subFiles);
    }
  }

  return results;
}
