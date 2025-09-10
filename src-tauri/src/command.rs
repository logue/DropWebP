use crate::decoder::decode;
use crate::encoder::encode;
use crate::error::AppError;
use crate::options::EncodeOptions;
use crate::options::PathInfo;
use std::path;
use std::path::Path;

/// Uint8Arrayバイナリデータを圧縮してUint8Arrayで返します。
/// # 引数
/// - `data`: 変換対象の画像データのバイト列
/// - `options`: エンコードオプション
/// # 戻り値
/// - 成功した場合は WebP のバイト列を `Vec<u8>` として返します。
/// - 失敗した場合は `Box<dyn Error>` を返します。
#[tauri::command]
pub async fn convert(data: Vec<u8>, options: EncodeOptions) -> Result<Vec<u8>, String> {
    // spawn_blocking でUIをフリーズさせずに重い処理を実行
    let converted_data = tauri::async_runtime::spawn_blocking(move || {
        println!("Decoding...");
        // 画像デコード
        let img = decode(&data).map_err(|e| format!("Failed to decode image: {}", e))?;
        println!("Encoding...");
        // 画像エンコード
        let data = encode(&img, options).map_err(|e| format!("Failed to encode image: {}", e))?;

        Ok(data)
    })
    .await
    .map_err(|e| e.to_string())?;
    converted_data
}

/// ファイルパスを解析して、ファイル名、拡張子、親ディレクトリを抽出します。
/// # 引数
/// - `path_str`: 解析対象のファイルパス文字列
/// # 戻り値
/// - 成功した場合は `PathInfo` 構造体を返します。
/// - 失敗した場合はエラーメッセージを `String` として返します。
#[tauri::command]
pub fn get_path_info(path_str: String) -> Result<PathInfo, String> {
    let path = Path::new(&path_str);

    // 最初に一度だけ存在確認を行う
    let exists = path.exists();

    // パスが存在する場合のみ、ファイルかディレクトリかを判定する
    // 存在しない場合は、どちらも false になる
    let is_file = if exists { path.is_file() } else { false };
    let is_dir = if exists { path.is_dir() } else { false };

    let info = PathInfo {
        file_name: path
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string()),

        extension: path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string()),

        parent_dir: path
            .parent()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string()),

        exists: Some(exists.to_string()),
        is_file: Some(is_file),
        is_dir: Some(is_dir),
    };

    Ok(info)
}

/// 指定されたパスのファイルまたはディレクトリを削除します。
/// # 引数
/// - `path_str`: 削除対象のファイルまたはディレクトリのパス文字列
/// # 戻り値
/// - 成功した場合は `Ok(())` を返します。
/// - 失敗した場合はエラーメッセージを `String` として返します。
#[tauri::command]
pub async fn delete_path(path_str: String) -> Result<(), String> {
    let path = path::Path::new(&path_str);
    if path.exists() {
        if path.is_file() {
            std::fs::remove_file(path).map_err(|e| AppError::Io(e))?;
        } else if path.is_dir() {
            std::fs::remove_dir_all(path).map_err(|e| AppError::Io(e))?;
        }
    }
    Ok(())
}
