use crate::error::AppError;
use crate::options::HighBitDepthImage;
use libheif_rs::{HeifContext, ImageHandle, LibHeif};

/// HEIFファイルを読み込み、HighBitDepthImageに変換する関数
pub fn decode(bytes: &[u8]) -> Result<(HighBitDepthImage, Option<Vec<u8>>), AppError> {
    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_bytes(bytes).map_err(|e| AppError::Decode(e.to_string()))?;
    let handle: ImageHandle = ctx
        .primary_image_handle()
        .map_err(|e| AppError::Decode(e.to_string()))?; // 型を明記すると分かりやすい

    // ★ 正しくは `color_profile_raw` という名前のメソッドです
    let icc_profile: Option<Vec<u8>> = handle.color_profile_raw().map(|p| p.data.to_vec());

    let bit_depth = handle.luma_bits_per_pixel();

    let img = lib_heif
        .decode(
            &handle, // handleはここで借用
            libheif_rs::ColorSpace::Rgb(libheif_rs::RgbChroma::Rgba),
            None,
        )
        .map_err(|e| AppError::Decode(e.to_string()))?;

    let width = handle.width();
    let height = handle.height();
    let interleaved_plane = img
        .planes()
        .interleaved
        .ok_or_else(|| AppError::Decode("Interleaved plane not found".to_string()))?;

    // ... f32バッファへの変換処理 (この部分は変更ありません) ...
    let high_bit_depth_image: HighBitDepthImage = if bit_depth > 8 {
        let data_u16: &[u16] = bytemuck::cast_slice(interleaved_plane.data);
        let pixels_f32: Vec<f32> = data_u16.iter().map(|&p| p as f32 / 65535.0).collect();
        let buffer = image::ImageBuffer::<image::Rgba<f32>, _>::from_raw(width, height, pixels_f32)
            .ok_or_else(|| AppError::Decode("Failed to create f32 ImageBuffer".to_string()))?;
        HighBitDepthImage::Rgba(buffer)
    } else {
        let pixels_f32: Vec<f32> = interleaved_plane
            .data
            .iter()
            .map(|&p| p as f32 / 255.0)
            .collect();
        let buffer = image::ImageBuffer::<image::Rgba<f32>, _>::from_raw(width, height, pixels_f32)
            .ok_or_else(|| AppError::Decode("Failed to create f32 ImageBuffer".to_string()))?;
        HighBitDepthImage::Rgba(buffer)
    };

    // ピクセルデータとICCプロファイルを両方返す
    Ok((high_bit_depth_image, icc_profile))
}
