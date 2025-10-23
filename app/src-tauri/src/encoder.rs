pub mod avif;
pub mod common;
// pub mod jpeg;
pub mod jxl;
pub mod png;
pub mod webp;

use crate::error::AppError;
use crate::options::{EncodeOptions, HighBitDepthImage};
use std::borrow::Cow;

/// 画像を指定された形式でエンコードします。
/// # 引数
/// - `img`: 変換対象の画像 (HighBitDepthImage)
/// - `options`: エンコードオプション (EncodeOptions)
/// # 戻り値
/// - 成功した場合はエンコードされたバイト列を `Vec<u8>` として返します。
/// - 失敗した場合は `Box<dyn Error>` を返します。
/// # 注意
/// - AVIF形式のエンコードには `ravif` クレートを使用しています。ビルド時に `libavif` ライブラリがシステムにインストールされている必要があります。
/// - WebP形式のエンコードには `libwebp-sys` クレートを使用しています。ビルド時に `libwebp` ライブラリがシステムにインストールされている必要があります。
/// - JPEG XL形式のエンコードには `jpegxl-rs` クレートを使用しています。ビルド時に `libjxl` ライブラリがシステムにインストールされている必要があります。
pub fn encode(
    img: HighBitDepthImage,
    icc_profile: Option<Vec<u8>>,
    options: &EncodeOptions,
) -> Result<Vec<u8>, AppError> {
    // match式の結果を変数に束縛する
    let result = match options {
        EncodeOptions::Avif(opts) => {
            println!("Adapter: Converting AvifOptions for ravif encoder...");
            avif::encode(&img, icc_profile, opts)
        }
        EncodeOptions::Webp(opts) => {
            println!("Adapter: Converting WebpOptions for libwebp_sys encoder...");
            webp::encode(&img, icc_profile, opts)
        }
        EncodeOptions::Jxl(opts) => {
            println!("Adapter: Converting JxlOptions for jpegxl_rs encoder...");
            jxl::encode(&img, icc_profile, opts)
        }
        EncodeOptions::Png(opts) => {
            println!("Adapter: Converting PngOptions for zopfli encoder...");
            png::encode(&img, icc_profile, opts)
        }
        EncodeOptions::Jpeg(_opts) => {
            println!("Adapter: JPEG encoder is currently disabled");
            Err(AppError::Encode(
                "JPEG encoder is temporarily disabled".to_string(),
            ))
        }
    };

    // match式から受け取った結果を返す
    result
}

/// HighBitDepthImageからエンコード用のピクセルデータを効率的に抽出します。
///
/// - 元の画像がRGB8/RGBA8形式の場合、データを借用して不要なコピーを避けます。
/// - ARGB形式の場合は、ARGBからRGBAに変換して所有権を持つデータを生成します。
/// - それ以外の形式の場合は、RGBA8に変換して所有権を持つデータを生成します。
///
/// # Arguments
/// * `img` - 処理対象の`HighBitDepthImage`への参照。
///
/// # Returns
/// * `(Cow<'a, [f32]>, bool)` - ピクセルデータと、アルファチャンネルの有無 (`true`ならRGBA) のタプル。
pub fn extract_pixel_data(img: &HighBitDepthImage) -> (Cow<'_, [f32]>, bool) {
    match img {
        HighBitDepthImage::Rgba(buffer) => (Cow::Borrowed(buffer.as_raw()), true),
        HighBitDepthImage::Rgb(buffer) => (Cow::Borrowed(buffer.as_raw()), false),
        HighBitDepthImage::Argb(buffer) => {
            // ARGBからRGBAに変換
            let argb_pixels = buffer.as_raw();
            let mut rgba_pixels = Vec::with_capacity(argb_pixels.len());

            // ARGBピクセル（A, R, G, B）をRGBAピクセル（R, G, B, A）に変換
            for chunk in argb_pixels.chunks_exact(4) {
                let a = chunk[0]; // Alpha
                let r = chunk[1]; // Red
                let g = chunk[2]; // Green
                let b = chunk[3]; // Blue

                // RGBAの順序で格納
                rgba_pixels.extend_from_slice(&[r, g, b, a]);
            }

            println!(
                "ARGB to RGBA conversion: {} pixels converted",
                rgba_pixels.len() / 4
            );
            (Cow::Owned(rgba_pixels), true)
        }
    }
}
