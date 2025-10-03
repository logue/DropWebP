import { invoke } from '@tauri-apps/api/core';
import { join, sep } from '@tauri-apps/api/path';
import { readDir, readFile, writeFile, type DirEntry } from '@tauri-apps/plugin-fs';

import type { PathInfo } from '@/interfaces/PathInfo';

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
      console.info(`Successfully read ${contents.length} bytes from ${filePath}`);
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
      console.info(`Successfully saved file to ${path}`);
    } catch (error) {
      console.error('Failed to save file:', error);
    }
  }

  /**
   * パスを削除する
   * @param path 削除するパス
   */
  async function del(path: string): Promise<void> {
    try {
      await invoke('delete_path', { pathStr: path }); // Rust側でdelete_pathコマンドを呼び出す
      console.info(`Successfully deleted: ${path}`);
    } catch (error) {
      console.error(`Failed to delete path: ${path}`, error);
      throw error;
    }
  }

  /**
   * ディレクトリからファイルを収集する
   * @param path ディレクトリ
   * @param recursive 再起的に探索するか
   * @returns ファイルパスの配列
   */
  async function collectFilesFromDir(path: string, recursive = false): Promise<string[]> {
    const entries: DirEntry[] = await readDir(path);
    let files: string[] = [];

    for (const entry of entries) {
      const fullPath = await join(path, entry.name); // フルパス生成
      if (entry.isFile) {
        files.push(fullPath);
      } else if (recursive && entry.isDirectory) {
        const sub = await collectFilesFromDir(fullPath, recursive);
        files = files.concat(sub);
      }
    }

    return files;
  }

  /**
   * ファイル or フォルダのパス配列を受け取ってファイル一覧に正規化
   * @param paths 入力パス配列
   * @param recursive 再起的に探索するか
   * @returns ファイルパスの配列
   */
  async function collectFiles(paths: string[], recursive = false): Promise<string[]> {
    let results: string[] = [];

    for (const path of paths) {
      if ((await pathInfo(path)).isDir) {
        // ディレクトリだった場合
        const subFiles = await collectFilesFromDir(path, recursive);
        results = results.concat(subFiles);
        continue;
      }
      results = results.concat(path);
    }

    return results;
  }

  /**
   * パスからファイル名などを取得
   * @param path パス文字列
   * @returns ファイル名、拡張子、親ディレクトリ名
   */
  async function pathInfo(path: string): Promise<PathInfo> {
    try {
      const ret = await invoke<PathInfo>('get_path_info', { pathStr: path });

      // 出力前にミューテーションする
      return {
        fileName: ret.fileName,
        extension: ret.extension ? ret.extension.toLowerCase() : ret.extension, // 拡張子は常に小文字にする
        parentDir: ret.parentDir + sep(), // 親ディレクトリの末尾の/がつかないのでここで追記
        isFile: ret.isFile,
        isDir: ret.isDir,
        exists: ret.exists
      };
    } catch (error) {
      console.error('Failed to parse path:', error);
      throw error;
    }
  }

  return { read, save, del, collectFiles, pathInfo };
}
