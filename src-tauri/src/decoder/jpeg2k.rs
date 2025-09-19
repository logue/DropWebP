use crate::error::AppError;
use crate::options::HighBitDepthImage;
use image::{ImageBuffer, Rgb, Rgba};
use jpeg2k::Image as Jp2Image;

// JPEG 2000 ファイルを読み込み、HighBitDepthImageに変換する
pub fn decode(bytes: &[u8]) -> Result<HighBitDepthImage, AppError> {
    let jp2_image = Jp2Image::from_bytes(bytes).map_err(|e| AppError::Decode(e.to_string()))?;

    let width = jp2_image.width();
    let height = jp2_image.height();
    let components = jp2_image.components();

    // ★ 1. ビット深度を取得し、正規化のための最大値を計算
    let bit_depth = components[0].precision();
    // 符号なし整数の最大値を計算 (例: 10bitなら (1 << 10) - 1 = 1023)
    let max_val = ((1 << bit_depth) - 1) as f32;

    let result: HighBitDepthImage = match components.len() {
        3 => {
            let r = components[0].data();
            let g = components[1].data();
            let b = components[2].data();

            // ★ 2. f32のピクセルデータを格納するVecを直接作成
            let mut pixels_f32 = Vec::with_capacity((width * height * 3) as usize);
            for i in 0..(width * height) as usize {
                // ★ 3. i32の値をf32に正規化
                pixels_f32.push(r[i] as f32 / max_val);
                pixels_f32.push(g[i] as f32 / max_val);
                pixels_f32.push(b[i] as f32 / max_val);
            }

            let buffer = ImageBuffer::<Rgb<f32>, Vec<f32>>::from_raw(width, height, pixels_f32)
                .ok_or_else(|| {
                    AppError::Decode("Failed to create f32 RGB ImageBuffer".to_string())
                })?;
            HighBitDepthImage::Rgb(buffer)
        }
        4 => {
            let r = components[0].data();
            let g = components[1].data();
            let b = components[2].data();
            let a = components[3].data();

            let mut pixels_f32 = Vec::with_capacity((width * height * 4) as usize);
            for i in 0..(width * height) as usize {
                pixels_f32.push(r[i] as f32 / max_val);
                pixels_f32.push(g[i] as f32 / max_val);
                pixels_f32.push(b[i] as f32 / max_val);
                pixels_f32.push(a[i] as f32 / max_val); // アルファも同様に正規化
            }

            let buffer = ImageBuffer::<Rgba<f32>, Vec<f32>>::from_raw(width, height, pixels_f32)
                .ok_or_else(|| {
                    AppError::Decode("Failed to create f32 RGBA ImageBuffer".to_string())
                })?;
            HighBitDepthImage::Rgba(buffer)
        }
        _ => {
            return Err(AppError::Decode(
                "Unsupported number of components in JPEG 2000 file".into(),
            ));
        }
    };

    println!("Decoder: Finish decoding JPEG 2000 to f32 buffer.");
    Ok(result)
}
