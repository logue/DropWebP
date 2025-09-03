mod decoder;

use std::{
    error::Error, ffi::c_void, io::Cursor, path::Path, ptr::null_mut, slice::from_raw_parts,
};

use exif::{In, Reader as ExifReader, Tag};
use image::{DynamicImage, RgbaImage, imageops::*};
use libwebp_sys::{
    WebPEncodeLosslessRGB, WebPEncodeLosslessRGBA, WebPEncodeRGB, WebPEncodeRGBA, WebPFree,
};

/// 画像を読み込んでDynamicImageに変換する
/// # 引数
/// - `path`: 画像ファイルのパス
/// # 戻り値
/// - 成功した場合はDynamicImageを返します。
/// - 失敗した場合はエラーを返します。
pub fn load_image(path: &Path) -> Result<DynamicImage, Box<dyn Error>> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tif" | "tiff" => Ok(image::open(path)?),
        "heic" | "heif" => decoder::heif_to_dynamic_image(path),
        "jp2" | "j2k" => decoder::jpeg2000_to_dynamic_image(path),
        _ => Err("Unsupported format".into()),
    }
}

/// 画像を WebP にエンコードします。
/// # 引数
/// - `img`: 変換対象の画像 (DynamicImage)
/// - `quality`: 品質 (0〜100)。100 の場合はロスレスになります。
/// # 戻り値
/// - 成功した場合は WebP のバイト列を `Vec<u8>` として返します。
/// - 失敗した場合は `Box<dyn Error>` を返します。
/// # 注意
/// - `libwebp-sys` クレートを使用して WebP エンコードを行います。事前に `libwebp` ライブラリがシステムにインストールされている必要があります。
pub fn encode_webp(img: &DynamicImage, quality: i32) -> Result<Vec<u8>, Box<dyn Error>> {
    if quality < 0 || quality > 100 {
        return Err("Quality must be between 0 and 100".into());
    }

    let width = img.width() as i32;
    let height = img.height() as i32;

    // RGBA または RGB の判定と格納処理
    let (raw, is_rgba) = match img {
        DynamicImage::ImageRgba8(img) => (img.clone().into_raw(), true),
        DynamicImage::ImageRgb8(img) => (img.clone().into_raw(), false),
        _ => {
            let img_buf = img.to_rgba8();
            (img_buf.into_raw(), true)
        }
    };

    unsafe {
        // 出力バッファのポインタ
        let mut out_buf: *mut u8 = null_mut();
        // ストライドの計算
        let stride = if is_rgba {
            width
                .checked_mul(4)
                .ok_or("Stride calculation overflowed")?
        } else {
            width
                .checked_mul(3)
                .ok_or("Stride calculation overflowed")?
        };

        // WebP にエンコード
        // qualityが100の場合はロスレスエンコードを使用
        let len = if is_rgba {
            // RGBA圧縮
            if quality == 100 {
                WebPEncodeLosslessRGBA(raw.as_ptr(), width, height, stride, &mut out_buf)
            } else {
                WebPEncodeRGBA(
                    raw.as_ptr(),
                    width,
                    height,
                    stride,
                    quality as f32,
                    &mut out_buf,
                )
            }
        } else {
            // RGB圧縮
            if quality == 100 {
                WebPEncodeLosslessRGB(raw.as_ptr(), width, height, stride, &mut out_buf)
            } else {
                WebPEncodeRGB(
                    raw.as_ptr(),
                    width,
                    height,
                    stride,
                    quality as f32,
                    &mut out_buf,
                )
            }
        };

        if out_buf.is_null() || len == 0 {
            return Err("WebP encoding failed".into());
        }

        // Rust Vec にコピー
        let slice = from_raw_parts(out_buf, len as usize);
        let result = slice.to_vec();

        // C 側で確保されたメモリを解放
        WebPFree(out_buf as *mut c_void);

        Ok(result)
    }
}

/// EXIF Orientation をもとに画像を回転・反転
pub fn correct_orientation(img: &RgbaImage, data: &[u8]) -> RgbaImage {
    if let Ok(exif) = ExifReader::new().read_from_container(&mut Cursor::new(data)) {
        if let Some(field) = exif.get_field(Tag::Orientation, In::PRIMARY) {
            match field.value.get_uint(0) {
                Some(2) => flip_horizontal(img),
                Some(3) => rotate180(img),
                Some(4) => flip_vertical(img),
                Some(5) => rotate90(&flip_horizontal(img)),
                Some(6) => rotate90(img),
                Some(7) => rotate270(&flip_horizontal(img)),
                Some(8) => rotate270(img),
                _ => img.clone(),
            }
        } else {
            img.clone()
        }
    } else {
        img.clone()
    }
}

// End of file src-tauri/src/lib.rs
