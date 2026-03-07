use crate::encoder::progress::{EncoderCapabilities, ProgressCallback, TauriProgressCallback};
use crate::error::AppError;
use crate::logging::{LogLevel, ResultExt, send_log_with_handle};
use crate::options::{EncodeOptions, PathInfo};
use image::{ImageFormat, guess_format};
use std::sync::Arc;
use std::{self, path::Path};
use tauri::AppHandle;

/// Uint8Arrayバイナリデータを圧縮してUint8Arrayで返します。
/// # 引数
/// - `data`: 変換対象の画像データのバイト列
/// - `options`: エンコードオプション
/// # 戻り値
/// - 成功した場合は WebP のバイト列を `Vec<u8>` として返します。
/// - 失敗した場合は `Box<dyn Error>` を返します。
#[tauri::command]
pub async fn convert(
    data: Vec<u8>,
    options: EncodeOptions,
    app: AppHandle,
) -> Result<Vec<u8>, String> {
    send_log_with_handle(&app, LogLevel::Info, "Starting compression...");

    let app_clone = app.clone();
    // spawn_blocking でUIをフリーズさせずに重い処理を実行
    let converted_data = tauri::async_runtime::spawn_blocking(move || {
        // まず、デコードする前にJPEGトランスコードの条件をチェックする
        // `if let` を使って、JXLオプションの場合のみ中身を取り出す
        if let EncodeOptions::Jxl(jxl_opts) = &options {
            // `guess_format`がJPEGを返した場合にのみ、このブロックに入る
            if guess_format(&data).map_or(false, |format| format == ImageFormat::Jpeg) {
                send_log_with_handle(
                    &app_clone,
                    LogLevel::Info,
                    "JPEG detected for JPEG XL target. Using transcode path...",
                );

                // トランスコードを実行し、成功したら`return`で即座に関数を抜ける
                return crate::encoder::jxl::transcode(&data)
                    .log_error(Some("JPEG to JPEG XL transcode"))
                    .map_err(|e| format!("Failed to transcode JPEG to JPEG XL: {}", e));
            }
        }
        // --- 上記のif条件に当てはまらなかった場合、通常のデコード→エンコード処理に進む ---

        // 入力データのサイズを保存
        let input_size = data.len();

        // HEICフォーマットチェック - ファイルパスが必要なため特別処理
        let is_heic = data.len() >= 12 &&
                      &data[4..8] == b"ftyp" &&
                      (&data[8..12] == b"heic" || &data[8..12] == b"heix" ||
                       &data[8..12] == b"hevc" || &data[8..12] == b"heim");

        // 画像デコード
        let decoded_data = if is_heic {
            // HEICの場合は一時ファイルに保存してdecode_from_pathを使用
            use std::io::Write;
            let mut temp_file = tempfile::NamedTempFile::new()
                .map_err(|e| format!("Failed to create temp file: {}", e))?;
            temp_file.write_all(&data)
                .map_err(|e| format!("Failed to write temp file: {}", e))?;

            send_log_with_handle(&app_clone, LogLevel::Info, "Decoding HEIC image using OS-native decoder...");
            crate::decoder::decode_from_path(temp_file.path())
                .log_error(Some("HEIC decoding"))
                .map_err(|e| format!("Failed to decode HEIC image: {}", e))?
        } else {
            crate::decoder::decode(&data)
                .log_error(Some("Image decoding"))
                .map_err(|e| format!("Failed to decode image: {}", e))?
        };

        let (img, icc_profile) = decoded_data;

        // 推計サイズの算出
        send_log_with_handle(&app_clone, LogLevel::Info, "Estimating output size...");
        let estimated_size = crate::encoder::estimate_size(&img, &options);

        // 画像エンコード
        send_log_with_handle(
            &app_clone,
            LogLevel::Info,
            &format!(
                "Encoding image... (Input size: {} bytes / Estimated output size: {} bytes)",
                input_size,
                estimated_size
            ),
        );

        let encoded_data = crate::encoder::encode(img, icc_profile, &options)
            .log_error(Some("Image encoding"))
            .map_err(|e| format!("Failed to encode image: {}", e))?;

        // 実際のエンコード結果のサイズをログ出力
        send_log_with_handle(
            &app_clone,
            LogLevel::Info,
            &format!(
                "Encoding completed. Input: {} bytes -> Output: {} bytes (Estimated: {} bytes, Ratio: {:.2}%)",
                input_size,
                encoded_data.len(),
                estimated_size,
                (encoded_data.len() as f64 / input_size as f64) * 100.0
            ),
        );

        Ok(encoded_data)
    })
    .await
    .map_err(|e| e.to_string())?;

    send_log_with_handle(&app, LogLevel::Info, "Compression completed successfully");
    converted_data
}

/// Uint8Arrayバイナリデータを圧縮してUint8Arrayで返します（進捗付き）
/// # 引数
/// - `data`: 変換対象の画像データのバイト列
/// - `options`: エンコードオプション
/// - `app`: Tauriアプリケーションハンドル
/// # 戻り値
/// - 成功した場合は圧縮されたバイト列を `Vec<u8>` として返します。
/// - 失敗した場合はエラーメッセージを `String` として返します。
/// # 進捗イベント
/// - イベント名: "encoding-progress"
/// - ペイロード: { percent: number, stage: string, status: "progress" | "complete" | "error" }
/// # 注意
/// - 進捗監視は WebP (lossy) と PNG でのみサポートされています
/// - その他のフォーマットは通常の `convert` コマンドと同じ動作になります
#[tauri::command]
pub async fn convert_with_progress(
    data: Vec<u8>,
    options: EncodeOptions,
    app: AppHandle,
) -> Result<Vec<u8>, String> {
    send_log_with_handle(
        &app,
        LogLevel::Info,
        "Starting compression with progress monitoring...",
    );

    // フォーマット名を取得
    let format_name = match &options {
        EncodeOptions::Webp(_) => "webp",
        EncodeOptions::Png(_) => "png",
        EncodeOptions::Avif(_) => "avif",
        EncodeOptions::Jxl(_) => "jxl",
        EncodeOptions::Jpeg(_) => "jpeg",
    };

    // 進捗監視サポート確認
    let supports_progress = EncoderCapabilities::supports_progress(format_name);

    if !supports_progress {
        send_log_with_handle(
            &app,
            LogLevel::Info,
            &format!(
                "Progress monitoring not supported for {}. Using standard conversion.",
                format_name
            ),
        );
        return convert(data, options, app).await;
    }

    send_log_with_handle(
        &app,
        LogLevel::Info,
        &format!("Progress monitoring enabled for {}", format_name),
    );

    let app_clone = app.clone();
    let converted_data = tauri::async_runtime::spawn_blocking(move || {
        // 進捗コールバックを作成
        let progress_callback = Arc::new(TauriProgressCallback::new(
            app_clone.clone(),
            "encoding-progress".to_string(),
        ));

        // JXLトランスコードチェック（進捗なし）
        if let EncodeOptions::Jxl(jxl_opts) = &options {
            if guess_format(&data).map_or(false, |format| format == ImageFormat::Jpeg) {
                send_log_with_handle(
                    &app_clone,
                    LogLevel::Info,
                    "JPEG detected for JPEG XL target. Using transcode path (no progress)...",
                );

                return crate::encoder::jxl::transcode(&data)
                    .log_error(Some("JPEG to JPEG XL transcode"))
                    .map_err(|e| format!("Failed to transcode JPEG to JPEG XL: {}", e));
            }
        }

        let input_size = data.len();

        // HEICフォーマットチェック
        let is_heic = data.len() >= 12 &&
                      &data[4..8] == b"ftyp" &&
                      (&data[8..12] == b"heic" || &data[8..12] == b"heix" ||
                       &data[8..12] == b"hevc" || &data[8..12] == b"heim");

        // 画像デコード
        progress_callback.on_progress(0.0, "Decoding image");
        let decoded_data = if is_heic {
            use std::io::Write;
            let mut temp_file = tempfile::NamedTempFile::new()
                .map_err(|e| {
                    progress_callback.on_error(&format!("Failed to create temp file: {}", e));
                    format!("Failed to create temp file: {}", e)
                })?;
            temp_file.write_all(&data)
                .map_err(|e| {
                    progress_callback.on_error(&format!("Failed to write temp file: {}", e));
                    format!("Failed to write temp file: {}", e)
                })?;

            progress_callback.on_progress(5.0, "Decoding HEIC using OS-native decoder");
            crate::decoder::decode_from_path(temp_file.path())
                .log_error(Some("HEIC decoding"))
                .map_err(|e| {
                    progress_callback.on_error(&format!("Failed to decode HEIC: {}", e));
                    format!("Failed to decode HEIC: {}", e)
                })?
        } else {
            crate::decoder::decode(&data)
                .log_error(Some("Image decoding"))
                .map_err(|e| {
                    progress_callback.on_error(&format!("Failed to decode image: {}", e));
                    format!("Failed to decode image: {}", e)
                })?
        };

        let (img, icc_profile) = decoded_data;

        // 推計サイズの算出
        progress_callback.on_progress(10.0, "Estimating output size");
        let estimated_size = crate::encoder::estimate_size(&img, &options);

        send_log_with_handle(
            &app_clone,
            LogLevel::Info,
            &format!(
                "Encoding image with progress... (Input: {} bytes / Estimated: {} bytes)",
                input_size, estimated_size
            ),
        );

        // 進捗付きエンコード
        let encoded_data = match &options {
            EncodeOptions::Webp(webp_opts) => {
                crate::encoder::webp::encode_with_progress(
                    &img,
                    icc_profile,
                    webp_opts,
                    progress_callback.clone(),
                )
            }
            EncodeOptions::Png(png_opts) => {
                crate::encoder::png::encode_with_progress(
                    &img,
                    icc_profile,
                    png_opts,
                    progress_callback.clone(),
                )
            }
            // その他のフォーマットは通常のエンコード（ここには到達しないはず）
            _ => crate::encoder::encode(img, icc_profile, &options),
        }
        .log_error(Some("Image encoding"))
        .map_err(|e| {
            progress_callback.on_error(&format!("Failed to encode image: {}", e));
            format!("Failed to encode image: {}", e)
        })?;

        send_log_with_handle(
            &app_clone,
            LogLevel::Info,
            &format!(
                "Encoding completed. Input: {} bytes -> Output: {} bytes (Estimated: {} bytes, Ratio: {:.2}%)",
                input_size,
                encoded_data.len(),
                estimated_size,
                (encoded_data.len() as f64 / input_size as f64) * 100.0
            ),
        );

        Ok(encoded_data)
    })
    .await
    .map_err(|e| e.to_string())?;

    send_log_with_handle(
        &app,
        LogLevel::Info,
        "Compression with progress completed successfully",
    );
    converted_data
}

/// 圧縮後のファイルサイズを推定
/// # 引数
/// - `data`: 変換対象の画像データのバイト列
/// - `options`: エンコードオプション
/// # 戻り値
/// - 成功した場合は推定サイズを `usize` として返します。
/// - 失敗した場合はエラーメッセージを `String` として返します。
#[tauri::command]
pub async fn estimate_size(
    data: Vec<u8>,
    options: EncodeOptions,
    app: AppHandle,
) -> Result<usize, String> {
    send_log_with_handle(&app, LogLevel::Info, "Estimating output size...");

    let app_clone = app.clone();
    let size = tauri::async_runtime::spawn_blocking(move || {
        // まず画像をデコード
        send_log_with_handle(
            &app_clone,
            LogLevel::Info,
            "Decoding image for size estimation...",
        );
        let (img, _) = crate::decoder::decode(&data)
            .log_error(Some("Image decoding for estimation"))
            .map_err(|e| format!("Failed to decode image: {}", e))?;

        // オプションに応じてサイズ推定
        let size = crate::encoder::estimate_size(&img, &options);

        Ok::<usize, String>(size)
    })
    .await
    .map_err(|e| e.to_string())??;
    send_log_with_handle(
        &app,
        LogLevel::Info,
        &format!("Estimated size: {} bytes", size),
    );
    Ok(size)
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
    let path = Path::new(&path_str);
    if path.exists() {
        if path.is_file() {
            std::fs::remove_file(path).map_err(|e| AppError::Io(e).to_string())?;
        } else if path.is_dir() {
            std::fs::remove_dir_all(path).map_err(|e| AppError::Io(e).to_string())?;
        }
    }
    Ok(())
}
