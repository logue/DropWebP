use image::DynamicImage;
use libwebp_sys::*;
use std::error::Error;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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

    // RGBA or RGB に揃える
    let (raw, is_rgba) = match img {
        DynamicImage::ImageRgba8(img) => (img.clone().into_raw(), true),
        DynamicImage::ImageRgb8(img) => (img.clone().into_raw(), false),
        _ => {
            let img_buf = img.to_rgba8();
            (img_buf.into_raw(), true)
        }
    };

    unsafe {
        let mut out_buf: *mut u8 = std::ptr::null_mut();
        let stride = if is_rgba { width * 4 } else { width * 3 };

        // WebP にエンコード
        let len = if is_rgba {
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
        let slice = std::slice::from_raw_parts(out_buf, len as usize);
        let result = slice.to_vec();

        // C 側で確保されたメモリを解放
        WebPFree(out_buf as *mut std::ffi::c_void);

        Ok(result)
    }
}
// End of file src-tauri/src/lib.rs
