use crate::error::AppError;
use crate::options::HighBitDepthImage;
use image::{ImageBuffer, Rgba};
use libheif_rs::{HeifContext, LibHeif};

/// HEIFファイルを読み込み、HighBitDepthImageに変換する関数
pub fn decode(bytes: &[u8]) -> Result<HighBitDepthImage, AppError> {
    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_bytes(bytes)
        .map_err(|e| AppError::Decode(format!("HEIF context error: {}", e)))?;
    let handle = ctx
        .primary_image_handle()
        .map_err(|e| AppError::Decode(format!("HEIF primary image handle error: {}", e)))?;
    let bit_depth = handle.luma_bits_per_pixel();

    let img = lib_heif
        .decode(
            &handle,
            libheif_rs::ColorSpace::Rgb(libheif_rs::RgbChroma::Rgba),
            None,
        )
        .map_err(|e| AppError::Decode(format!("HEIF decode error: {}", e)))?;

    let width = handle.width();
    let height = handle.height();

    println!(
        "Decoder: HEIC image - width: {}, height: {}, bit depth: {}",
        width, height, bit_depth
    );

    let planes = img.planes();
    let interleaved_plane = planes
        .interleaved
        .ok_or_else(|| AppError::Decode("Interleaved plane not found".to_string()))?;

    // ★ 2. ビット深度に応じて f32 のバッファに変換する
    let buffer_f32: ImageBuffer<Rgba<f32>, Vec<f32>> = if bit_depth > 8 {
        // 10-bit, 12-bit の場合 (データは u16 として扱える)
        // bytemuckクレートなどを使うとより安全にキャストできます
        let data_u16: &[u16] = bytemuck::cast_slice(interleaved_plane.data);

        let pixels_f32: Vec<f32> = data_u16
            .iter()
            .map(|&p| p as f32 / 65535.0) // u16をf32に正規化
            .collect();

        ImageBuffer::from_raw(width, height, pixels_f32)
            .ok_or_else(|| AppError::Decode("Failed to create f32 ImageBuffer".to_string()))?
    } else {
        // 8-bit の場合
        let pixels_f32: Vec<f32> = interleaved_plane
            .data
            .iter()
            .map(|&p| p as f32 / 255.0) // u8をf32に正規化
            .collect();

        ImageBuffer::from_raw(width, height, pixels_f32)
            .ok_or_else(|| AppError::Decode("Failed to create f32 ImageBuffer".to_string()))?
    };

    println!("Decoder: Finish decoding HEIC to f32 buffer.");
    // ★ 3. f32 の ImageBuffer を HighBitDepthImage でラップして返す
    Ok(HighBitDepthImage::Rgba(buffer_f32))
}
