use crate::error::AppError;
use image::DynamicImage;
use jpegxl_rs::{decoder_builder, image::ToDynamic};

/// JPEG XL 画像をデコードします。
/// # 引数
/// - `data`: JPEG XL 形式のバイト列
/// # 戻り値
/// - 成功した場合は `DynamicImage` を返します。
/// - 失敗した場合は `AppError` を返します。
pub fn decode(data: &[u8]) -> Result<DynamicImage, AppError> {
    // 1. ビルダー経由でデコーダーを構築し、直接 `decode_to_image` を呼び出す
    let image = decoder_builder()
        .build()
        .map_err(|e| AppError::Decode(format!("JXL decoder build failed: {}", e)))?
        .decode_to_image(&data)
        .map_err(|e| AppError::Decode(format!("JXL decode failed: {}", e)))?
        .ok_or_else(|| AppError::Decode("JXL decode failed: no image found".to_string()))?;

    // 2. 成功した `DynamicImage` を返す
    Ok(image)
}
