use crate::{encoder::extract_pixel_data, error::AppError};
use image::DynamicImage;
use libwebp_sys::{
    WebPEncodeLosslessRGB, WebPEncodeLosslessRGBA, WebPEncodeRGB, WebPEncodeRGBA, WebPFree,
};
use std::{ffi::c_void, ptr::null_mut, slice::from_raw_parts};

/// 画像を WebP にエンコードします。
/// # 引数
/// - `img`: 変換対象の画像 (DynamicImage)
/// - `quality`: 品質 (0〜100)
/// - `lossless`: ロスレス
/// # 戻り値
/// - 成功した場合は WebP のバイト列を `Vec<u8>` として返します。
/// - 失敗した場合は `AppError` を返します。
/// # 注意
/// - `libwebp-sys` クレートを使用して WebP エンコードを行います。ビルド時に `libwebp` ライブラリがシステムにインストールされている必要があります。
pub fn encode(img: &DynamicImage, quality: f32, lossless: bool) -> Result<Vec<u8>, AppError> {
    if quality < 0.0 || quality > 100.0 {
        return Err(AppError::Encode("Quality must be between 0 and 100".into()));
    }

    let width = img.width() as i32;
    let height = img.height() as i32;

    // 1. データ準備
    let (raw, is_rgba) = extract_pixel_data(img);

    unsafe {
        // 出力バッファのポインタ
        let mut out_buf: *mut u8 = null_mut();
        // ストライドの計算
        let stride = if is_rgba {
            width.checked_mul(4).ok_or(AppError::Encode(
                "Stride calculation overflowed".to_string(),
            ))?
        } else {
            width.checked_mul(3).ok_or(AppError::Encode(
                "Stride calculation overflowed".to_string(),
            ))?
        };

        // WebP にエンコード
        // qualityが100の場合はロスレスエンコードを使用
        let len = if is_rgba {
            println!("Optimized path: Encoding as RGBA...");
            // RGBA圧縮
            if lossless == true {
                WebPEncodeLosslessRGBA(raw.as_ptr(), width, height, stride, &mut out_buf)
            } else {
                WebPEncodeRGBA(raw.as_ptr(), width, height, stride, quality, &mut out_buf)
            }
        } else {
            println!("Optimized path: Encoding as RGB...");
            // RGB圧縮
            if lossless == true {
                WebPEncodeLosslessRGB(raw.as_ptr(), width, height, stride, &mut out_buf)
            } else {
                WebPEncodeRGB(raw.as_ptr(), width, height, stride, quality, &mut out_buf)
            }
        };

        if out_buf.is_null() || len == 0 {
            return Err(AppError::Encode("WebP encoding failed".into()));
        }

        // Rust Vec にコピー
        let slice = from_raw_parts(out_buf, len as usize);
        let result = slice.to_vec();

        // C 側で確保されたメモリを解放
        WebPFree(out_buf as *mut c_void);

        println!("Finished encoding WebP.");

        Ok(result)
    }
}
