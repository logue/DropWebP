import { invoke } from '@tauri-apps/api/core';
import { join } from '@tauri-apps/api/path';
import { readDir, readFile, writeFile, type DirEntry } from '@tauri-apps/plugin-fs';

/** ファイルシステムコンポーサブル */
export function useFileSystem() {
  /**
   * 指定されたファイルパスからファイルを読み込み、Uint8Arrayとして返す
   * @param filePath 読み込むファイルのフルパス
   * @returns ファイルのバイナリデータ
   */
  async function read(filePath: string): Promise<Uint8Array> {
    try {
      // fs.readBinaryFile は Uint8Array を Promise で返す
      const contents = await readFile(filePath);
      console.log(`Successfully read ${contents.length} bytes from ${filePath}`);
      return contents;
    } catch (error) {
      console.error(`Failed to read file: ${filePath}`, error);
      throw error;
    }
  }

  /**
   * Uint8Arrayデータをファイルパスに保存する
   * @param path 保存先のフルパス
   * @param data 保存するバイナリデータ
   * @param isOverwrite 上書きするか
   */
  async function save(path: string, data: Uint8Array): Promise<void> {
    try {
      await writeFile(path, data);
      console.log(`Successfully saved file to ${path}`);
    } catch (error) {
      console.error('Failed to save file:', error);
      throw error;
    }
  }

  /**
   * ディレクトリからファイルを収集する
   * @param path 入力ファイル
   * @param pattern 拡張子のマッチパターン
   * @param recursive 再起的に探索するか
   * @returns ファイルパスの配列
   */
  async function collectFilesFromDir(
    path: string,
    pattern: RegExp,
    recursive = false
  ): Promise<string[]> {
    const entries: DirEntry[] = await readDir(path);
    let files: string[] = [];

    for (const entry of entries) {
      const fullPath = await join(path, entry.name); // フルパス生成
      if (entry.isFile && pattern.exec(entry.name)) {
        files.push(fullPath);
      } else if (recursive && entry.isDirectory) {
        const sub = await collectFilesFromDir(fullPath, pattern, recursive);
        files = files.concat(sub);
      }
    }

    return files;
  }

  /**
   * ファイル or フォルダのパス配列を受け取ってファイル一覧に正規化
   * @param paths 入力パス配列
   * @param pattern 拡張子のマッチパターン
   * @param recursive 再起的に探索するか
   * @returns ファイルパスの配列
   */
  async function collectFiles(
    paths: string[],
    pattern: RegExp,
    recursive = false
  ): Promise<string[]> {
    let results: string[] = [];

    for (const p of paths) {
      if (pattern.test(p)) {
        results.push(p);
      } else {
        const subFiles = await collectFilesFromDir(p, pattern, recursive);
        results = results.concat(subFiles);
      }
    }

    return results;
  }

  /** パスからファイル名などを取得
   * @param path パス文字列
   * @returns ファイル名、拡張子、親ディレクトリ名
   */
  async function parsePath(path: string): Promise<{
    fileName: string;
    extension: string;
    parentDir: string;
  }> {
    try {
      return await invoke('parse_path', { path_str: path });
    } catch (error) {
      console.error('Failed to parse path:', error);
      throw error;
    }
  }

  /**
   * 親ディレクトリパスを取得
   * @param path パス文字列
   * @returns 親ディレクトリパス（ルートの場合は空文字）
   */
  async function getDir(path: string) {
    return (await parsePath(path)).parentDir;
  }

  /**
   * ファイル名を取得
   * @param path パス文字列
   * @returns ファイル名（拡張子含む）
   */
  async function getFileName(path: string) {
    return (await parsePath(path)).fileName;
  }

  /**
   * 拡張子を取得
   * @param path パス文字列
   * @returns 拡張子（ドットなし、存在しない場合は空文字）
   */
  async function getExtension(path: string) {
    return (await parsePath(path)).extension;
  }

  return { read, save, collectFiles, getDir, getFileName, getExtension };
}
