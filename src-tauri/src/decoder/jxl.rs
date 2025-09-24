use crate::error::AppError;
use crate::options::HighBitDepthImage;

use ::image::{ImageBuffer, Rgb, Rgba};
use jpegxl_rs::decode::*;

/// JPEG XL 画像をデコードします。
pub fn decode(data: &[u8]) -> Result<(HighBitDepthImage, Option<Vec<u8>>), AppError> {
    // jpegxl-rsクレートを使用してJPEG XLをデコード
    let decoder = decoder_builder()
        .build()
        .map_err(|e| AppError::Decode(format!("Failed to create JXL decoder: {:?}", e)))?;

    // デコードを実行
    let (metadata, pixels) = decoder
        .decode(data)
        .map_err(|e| AppError::Decode(format!("Failed to decode JXL: {:?}", e)))?;

    let icc_profile = metadata.icc_profile;

    let width = metadata.width;
    let height = metadata.height;

    // ピクセルデータをf32形式で取得し、HighBitDepthImageに変換
    let image_buffer = match pixels {
        Pixels::Float(buffer_f32) => {
            // チャンネル数を計算してRGBまたはRGBAを判定
            let channels = buffer_f32.len() / (width as usize * height as usize);

            match channels {
                3 => {
                    // RGB
                    let buffer =
                        ImageBuffer::<Rgb<f32>, Vec<f32>>::from_raw(width, height, buffer_f32)
                            .ok_or_else(|| {
                                AppError::Decode("Failed to create f32 RGB ImageBuffer".to_string())
                            })?;
                    HighBitDepthImage::Rgb(buffer)
                }
                4 => {
                    // RGBA
                    let buffer =
                        ImageBuffer::<Rgba<f32>, Vec<f32>>::from_raw(width, height, buffer_f32)
                            .ok_or_else(|| {
                                AppError::Decode(
                                    "Failed to create f32 RGBA ImageBuffer".to_string(),
                                )
                            })?;
                    HighBitDepthImage::Rgba(buffer)
                }
                _ => {
                    return Err(AppError::Decode(format!(
                        "Unsupported channel count: {}",
                        channels
                    )));
                }
            }
        }
        Pixels::Uint8(buffer_u8) => {
            // u8データをf32に変換
            let buffer_f32: Vec<f32> = buffer_u8
                .iter()
                .map(|&pixel| pixel as f32 / 255.0)
                .collect();

            let channels = buffer_f32.len() / (width as usize * height as usize);

            match channels {
                3 => {
                    let buffer =
                        ImageBuffer::<Rgb<f32>, Vec<f32>>::from_raw(width, height, buffer_f32)
                            .ok_or_else(|| {
                                AppError::Decode(
                                    "Failed to create f32 RGB ImageBuffer from u8".to_string(),
                                )
                            })?;
                    HighBitDepthImage::Rgb(buffer)
                }
                4 => {
                    let buffer =
                        ImageBuffer::<Rgba<f32>, Vec<f32>>::from_raw(width, height, buffer_f32)
                            .ok_or_else(|| {
                                AppError::Decode(
                                    "Failed to create f32 RGBA ImageBuffer from u8".to_string(),
                                )
                            })?;
                    HighBitDepthImage::Rgba(buffer)
                }
                _ => {
                    return Err(AppError::Decode(format!(
                        "Unsupported channel count: {}",
                        channels
                    )));
                }
            }
        }
        Pixels::Uint16(buffer_u16) => {
            // u16データをf32に変換
            let buffer_f32: Vec<f32> = buffer_u16
                .iter()
                .map(|&pixel| pixel as f32 / 65535.0)
                .collect();

            let channels = buffer_f32.len() / (width as usize * height as usize);

            match channels {
                3 => {
                    let buffer =
                        ImageBuffer::<Rgb<f32>, Vec<f32>>::from_raw(width, height, buffer_f32)
                            .ok_or_else(|| {
                                AppError::Decode(
                                    "Failed to create f32 RGB ImageBuffer from u16".to_string(),
                                )
                            })?;
                    HighBitDepthImage::Rgb(buffer)
                }
                4 => {
                    let buffer =
                        ImageBuffer::<Rgba<f32>, Vec<f32>>::from_raw(width, height, buffer_f32)
                            .ok_or_else(|| {
                                AppError::Decode(
                                    "Failed to create f32 RGBA ImageBuffer from u16".to_string(),
                                )
                            })?;
                    HighBitDepthImage::Rgba(buffer)
                }
                _ => {
                    return Err(AppError::Decode(format!(
                        "Unsupported channel count: {}",
                        channels
                    )));
                }
            }
        }
        Pixels::Float16(buffer_f16) => {
            // f16データをf32に変換
            let buffer_f32: Vec<f32> = buffer_f16.iter().map(|&pixel| pixel.to_f32()).collect();

            let channels = buffer_f32.len() / (width as usize * height as usize);

            match channels {
                3 => {
                    let buffer =
                        ImageBuffer::<Rgb<f32>, Vec<f32>>::from_raw(width, height, buffer_f32)
                            .ok_or_else(|| {
                                AppError::Decode(
                                    "Failed to create f32 RGB ImageBuffer from f16".to_string(),
                                )
                            })?;
                    HighBitDepthImage::Rgb(buffer)
                }
                4 => {
                    let buffer =
                        ImageBuffer::<Rgba<f32>, Vec<f32>>::from_raw(width, height, buffer_f32)
                            .ok_or_else(|| {
                                AppError::Decode(
                                    "Failed to create f32 RGBA ImageBuffer from f16".to_string(),
                                )
                            })?;
                    HighBitDepthImage::Rgba(buffer)
                }
                _ => {
                    return Err(AppError::Decode(format!(
                        "Unsupported channel count: {}",
                        channels
                    )));
                }
            }
        }
    };

    Ok((image_buffer, icc_profile))
}
