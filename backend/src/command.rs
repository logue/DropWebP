use crate::encoder::progress::{EncoderCapabilities, ProgressCallback, TauriProgressCallback};
use crate::error::AppError;
use crate::logging::{LogLevel, ResultExt, send_log_with_handle};
use crate::options::{EncodeOptions, PathInfo};
use image::{ImageFormat, guess_format};
use std::sync::Arc;
use std::{self, path::Path};
use tauri::AppHandle;

/// Compress a `Uint8Array` payload of image bytes and return the encoded result.
///
/// # Arguments
/// - `data`: raw bytes of the source image to convert
/// - `options`: encoder selection and parameters
/// - `app`: Tauri application handle (used for progress and logging events)
///
/// # Returns
/// The encoded bytes on success, or a string describing the failure on error.
///
/// # Errors
/// Returns an error string when decoding, size estimation, or encoding fails,
/// or when the spawn-blocking task itself panics.
#[tauri::command]
pub async fn convert(
    data: Vec<u8>,
    options: EncodeOptions,
    app: AppHandle,
) -> Result<Vec<u8>, String> {
    send_log_with_handle(&app, LogLevel::Info, "Starting compression...");

    let app_clone = app.clone();
    // Run heavy work on a blocking thread so the UI stays responsive.
    let converted_data = tauri::async_runtime::spawn_blocking(move || {
        // Check for the JPEG -> JPEG XL transcode fast path before decoding.
        if let EncodeOptions::Jxl(_jxl_opts) = &options {
            // Only enter when the input is detected as JPEG.
            if guess_format(&data).is_ok_and(|format| format == ImageFormat::Jpeg) {
                send_log_with_handle(
                    &app_clone,
                    LogLevel::Info,
                    "JPEG detected for JPEG XL target. Using transcode path...",
                );

                // Run the transcode and return immediately on success.
                return crate::encoder::jxl::transcode(&data)
                    .log_error(Some("JPEG to JPEG XL transcode"))
                    .map_err(|e| format!("Failed to transcode JPEG to JPEG XL: {}", e));
            }
        }
        // Fall through to the regular decode -> encode pipeline.

        // Remember the input size for ratio reporting.
        let input_size = data.len();

        // HEIC needs a real file path, so detect it via the magic bytes here.
        let is_heic = data.len() >= 12
            && &data[4..8] == b"ftyp"
            && (&data[8..12] == b"heic"
                || &data[8..12] == b"heix"
                || &data[8..12] == b"hevc"
                || &data[8..12] == b"heim");

        // Decode the input image.
        let decoded_data = if is_heic {
            // For HEIC, persist the bytes to a temp file and use `decode_from_path`.
            use std::io::Write;
            let mut temp_file = tempfile::NamedTempFile::new()
                .map_err(|e| format!("Failed to create temp file: {}", e))?;
            temp_file
                .write_all(&data)
                .map_err(|e| format!("Failed to write temp file: {}", e))?;

            send_log_with_handle(
                &app_clone,
                LogLevel::Info,
                "Decoding HEIC image using OS-native decoder...",
            );
            crate::decoder::decode_from_path(temp_file.path())
                .log_error(Some("HEIC decoding"))
                .map_err(|e| format!("Failed to decode HEIC image: {}", e))?
        } else {
            crate::decoder::decode(&data)
                .log_error(Some("Image decoding"))
                .map_err(|e| format!("Failed to decode image: {}", e))?
        };

        let (img, icc_profile) = decoded_data;

        // Estimate the output size before encoding.
        send_log_with_handle(&app_clone, LogLevel::Info, "Estimating output size...");
        let estimated_size = crate::encoder::estimate_size(&img, &options);

        // Encode the image.
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

        // Log the actual encoded size and ratio.
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

/// Compress a `Uint8Array` payload with progress events emitted to the frontend.
///
/// # Arguments
/// - `data`: raw bytes of the source image to convert
/// - `options`: encoder selection and parameters
/// - `app`: Tauri application handle used to emit progress events
///
/// # Returns
/// The encoded bytes on success, or a string describing the failure on error.
///
/// # Errors
/// Returns an error string when decoding, size estimation, or encoding fails.
///
/// # Events
/// Emits `"encoding-progress"` with payload
/// `{ percent: number, stage: string, status: "progress" | "complete" | "error" }`.
///
/// # Notes
/// Progress monitoring is currently supported only for WebP (lossy) and PNG.
/// Other formats fall back to the regular [`convert`] command.
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

    // Identify the target format name.
    let format_name = match &options {
        EncodeOptions::Webp(_) => "webp",
        EncodeOptions::Png(_) => "png",
        EncodeOptions::Avif(_) => "avif",
        EncodeOptions::Jxl(_) => "jxl",
        EncodeOptions::Jpeg(_) => "jpeg",
    };

    // Check whether progress monitoring is supported.
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
        // Build the progress callback that emits Tauri events.
        let progress_callback = Arc::new(TauriProgressCallback::new(
            app_clone.clone(),
            "encoding-progress".to_string(),
        ));

        // JPEG -> JPEG XL transcode fast path (no progress reporting).
        if let EncodeOptions::Jxl(_jxl_opts) = &options
            && guess_format(&data).is_ok_and(|format| format == ImageFormat::Jpeg) {
                send_log_with_handle(
                    &app_clone,
                    LogLevel::Info,
                    "JPEG detected for JPEG XL target. Using transcode path (no progress)...",
                );

                return crate::encoder::jxl::transcode(&data)
                    .log_error(Some("JPEG to JPEG XL transcode"))
                    .map_err(|e| format!("Failed to transcode JPEG to JPEG XL: {}", e));
            }

        let input_size = data.len();

        // HEIC magic-byte detection.
        let is_heic = data.len() >= 12
            && &data[4..8] == b"ftyp"
            && (&data[8..12] == b"heic"
                || &data[8..12] == b"heix"
                || &data[8..12] == b"hevc"
                || &data[8..12] == b"heim");

        // Decode the input image.
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

        // Estimate the output size.
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

        // Encode with progress reporting where supported.
        let encoded_data = match &options {
            EncodeOptions::Webp(webp_opts) => crate::encoder::webp::encode_with_progress(
                &img,
                icc_profile,
                webp_opts,
                progress_callback.clone(),
            ),
            EncodeOptions::Png(png_opts) => crate::encoder::png::encode_with_progress(
                &img,
                icc_profile,
                png_opts,
                progress_callback.clone(),
            ),
            // Other formats fall through to the standard encoder (this branch
            // is unreachable in practice because of the supports_progress gate above).
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

/// Estimate the size of the encoded output without performing the full encode.
///
/// # Arguments
/// - `data`: raw bytes of the source image
/// - `options`: encoder selection and parameters
///
/// # Returns
/// The estimated output size in bytes on success, or a string describing the
/// failure on error.
///
/// # Errors
/// Returns an error string when decoding fails or the spawn-blocking task panics.
#[tauri::command]
pub async fn estimate_size(
    data: Vec<u8>,
    options: EncodeOptions,
    app: AppHandle,
) -> Result<usize, String> {
    send_log_with_handle(&app, LogLevel::Info, "Estimating output size...");

    let app_clone = app.clone();
    let size = tauri::async_runtime::spawn_blocking(move || {
        // Decode the source image first.
        send_log_with_handle(
            &app_clone,
            LogLevel::Info,
            "Decoding image for size estimation...",
        );
        let (img, _) = crate::decoder::decode(&data)
            .log_error(Some("Image decoding for estimation"))
            .map_err(|e| format!("Failed to decode image: {}", e))?;

        // Estimate based on the chosen options.
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

/// Inspect a filesystem path and return name, extension, parent directory and
/// presence flags as a [`PathInfo`] structure.
///
/// # Arguments
/// - `path_str`: filesystem path to inspect
///
/// # Returns
/// A [`PathInfo`] describing the path components and metadata.
///
/// # Errors
/// Currently always succeeds; the result type is preserved for forward
/// compatibility.
#[tauri::command]
pub fn get_path_info(path_str: String) -> Result<PathInfo, String> {
    let path = Path::new(&path_str);

    // Check existence once up front.
    let exists = path.exists();

    // Distinguish file vs. directory only when the path actually exists.
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

/// Delete a file or directory at the given path.
///
/// # Arguments
/// - `path_str`: target path to remove (file or directory)
///
/// # Returns
/// `Ok(())` on success.
///
/// # Errors
/// Returns an error string when the underlying filesystem call fails.
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
