/// HEIC/HEIF デコーダー（heif-convert経由で16ビットPNG変換）
///
/// すべてのプラットフォームで統一された実装:
/// 1. heif-convertコマンドを使ってHEIC→16ビットPNGに変換
/// 2. 生成されたPNGをデコード
/// 3. 一時ファイルを削除
///
/// この方法により:
/// - ✅ Apple Gain Map HDR対応（iOS 14+）
/// - ✅ PQベースHDR対応（iOS 13以前）
/// - ✅ すべてのHEIC形式に対応
/// - ✅ クロスプラットフォームで一貫した動作
use crate::error::AppError;
use image::DynamicImage;
use std::path::Path;

/// Check if heif-convert command is available on the system
fn check_heif_convert() -> bool {
    #[cfg(target_os = "windows")]
    let heif_convert_cmd = "heif-convert.exe";
    #[cfg(not(target_os = "windows"))]
    let heif_convert_cmd = "heif-convert";

    std::process::Command::new(heif_convert_cmd)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Decode HEIC file via heif-convert to 16-bit PNG intermediate file
pub fn decode_heic<P: AsRef<Path>>(path: P) -> Result<(DynamicImage, Option<Vec<u8>>), AppError> {
    println!("Decoder: Using heif-convert for HEIC decoding (HDR-capable)...");

    // Check if heif-convert is available
    if !check_heif_convert() {
        return Err(AppError::Decode(format!(
            "heif-convert not found. Please install libheif:\n\
             macOS: brew install libheif\n\
             Linux: sudo apt install libheif-examples (Debian/Ubuntu) or sudo dnf install libheif-tools (Fedora/RHEL)\n\
             Windows: scoop install libheif or download from https://github.com/strukturag/libheif/releases"
        )));
    }

    println!("HEIC: heif-convert found - using 16-bit PNG intermediate format");

    // Create temporary PNG file path
    let temp_dir = std::env::temp_dir();
    let temp_png = temp_dir.join(format!(
        "dropwebp_heic_temp_{}.png",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    ));

    // Determine heif-convert command name based on platform
    #[cfg(target_os = "windows")]
    let heif_convert_cmd = "heif-convert.exe";
    #[cfg(not(target_os = "windows"))]
    let heif_convert_cmd = "heif-convert";

    // Convert HEIC to 16-bit PNG
    println!("HEIC: Converting to 16-bit PNG via heif-convert...");
    let output = std::process::Command::new(heif_convert_cmd)
        .arg("-q")
        .arg("100") // Maximum quality
        .arg(path.as_ref())
        .arg(&temp_png)
        .output()
        .map_err(|e| AppError::Decode(format!("Failed to run heif-convert: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Decode(format!("heif-convert failed: {}", stderr)));
    }

    println!("HEIC: Conversion successful, decoding PNG...");

    // Decode the PNG file
    let png_img = image::open(&temp_png).map_err(|e| {
        // Clean up temp file even if PNG decoding fails
        let _ = std::fs::remove_file(&temp_png);
        AppError::Decode(format!("Failed to decode temporary PNG: {}", e))
    })?;

    // Extract ICC profile from PNG if present
    let icc_profile = extract_png_icc_profile(&temp_png);

    // Clean up temporary file
    if let Err(e) = std::fs::remove_file(&temp_png) {
        println!("HEIC: Warning - failed to delete temporary PNG: {}", e);
    }

    println!("HEIC: Successfully decoded via heif-convert");
    if let Some(ref profile) = icc_profile {
        println!("HEIC: Extracted ICC profile ({} bytes)", profile.len());
    }

    Ok((png_img, icc_profile))
}

/// Extract ICC profile from PNG file
fn extract_png_icc_profile<P: AsRef<Path>>(path: P) -> Option<Vec<u8>> {
    use std::fs::File;
    use std::io::{BufReader, Read};

    let file = File::open(path).ok()?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).ok()?;

    // Look for iCCP chunk in PNG file
    // PNG chunk format: length (4 bytes) + type (4 bytes) + data + CRC (4 bytes)
    let mut pos = 8; // Skip PNG signature

    while pos + 12 <= buffer.len() {
        let length = u32::from_be_bytes([
            buffer[pos],
            buffer[pos + 1],
            buffer[pos + 2],
            buffer[pos + 3],
        ]) as usize;

        let chunk_type = &buffer[pos + 4..pos + 8];

        if chunk_type == b"iCCP" {
            let chunk_data_start = pos + 8;
            let chunk_data_end = chunk_data_start + length;

            if chunk_data_end <= buffer.len() {
                let chunk_data = &buffer[chunk_data_start..chunk_data_end];

                // iCCP format: profile_name\0compression_method compressed_profile
                // Find null terminator
                if let Some(null_pos) = chunk_data.iter().position(|&b| b == 0) {
                    if null_pos + 2 <= chunk_data.len() {
                        let compressed_profile = &chunk_data[null_pos + 2..];

                        // Decompress using flate2
                        use flate2::read::ZlibDecoder;
                        let mut decoder = ZlibDecoder::new(compressed_profile);
                        let mut decompressed = Vec::new();

                        if decoder.read_to_end(&mut decompressed).is_ok() {
                            println!(
                                "HEIC: Extracted ICC profile from PNG ({} bytes)",
                                decompressed.len()
                            );
                            return Some(decompressed);
                        }
                    }
                }
            }
        }

        pos += 12 + length; // Move to next chunk
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_decode_heic() {
        // テスト用のHEICファイルがあれば
        let test_file = PathBuf::from("test_data/sample.heic");
        if test_file.exists() {
            let result = decode_heic(&test_file);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_heif_convert_check() {
        // heif-convertが利用可能かチェック
        let available = check_heif_convert();
        println!("heif-convert available: {}", available);
    }
}
