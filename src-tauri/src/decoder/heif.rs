use crate::error::AppError;
use image::{DynamicImage, ImageBuffer, Rgba};
use libheif_rs::{HeifContext, LibHeif};

/// HEIFファイルを読み込み、DynamicImageに変換する関数
pub fn decode(bytes: &[u8]) -> Result<DynamicImage, AppError> {
    let lib_heif = LibHeif::new();

    let ctx = HeifContext::read_from_bytes(bytes).map_err(|e| AppError::Decode(e.to_string()))?;
    let handle = ctx
        .primary_image_handle()
        .map_err(|e| AppError::Decode(e.to_string()))?;
    let img = lib_heif
        .decode(
            &handle,
            libheif_rs::ColorSpace::Rgb(libheif_rs::RgbChroma::Rgba),
            None,
        )
        .map_err(|e| AppError::Decode(e.to_string()))?;

    let width = handle.width();
    let height = handle.height();
    let planes = img.planes();
    let interleaved_plane = planes
        .interleaved
        .ok_or(AppError::Decode("Interleaved plane not found".to_string()))?;
    let pixel_data = interleaved_plane.data.to_vec();

    let image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, pixel_data).ok_or(AppError::Decode(
            "Failed to create ImageBuffer from raw data".to_string(),
        ))?;

    println!("Decoder: Finish decoding HEIC.");
    Ok(DynamicImage::ImageRgba8(image_buffer))
}
