pub mod avif;
pub mod jxl;
pub mod webp;

use crate::error::AppError;
use crate::options::EncodeOptions;
use image::DynamicImage;
use std::borrow::Cow;

/// 画像を指定された形式でエンコードします。
/// # 引数
/// - `img`: 変換対象の画像 (DynamicImage)
/// - `options`: エンコードオプション (EncodeOptions)
/// # 戻り値
/// - 成功した場合はエンコードされたバイト列を `Vec<u8>` として返します。
/// - 失敗した場合は `Box<dyn Error>` を返します。
/// # 注意
/// - AVIF形式のエンコードには `ravif` クレートを使用しています。ビルド時に `libavif` ライブラリがシステムにインストールされている必要があります。
/// - WebP形式のエンコードには `libwebp-sys` クレートを使用しています。ビルド時に `libwebp` ライブラリがシステムにインストールされている必要があります。
/// - JPEG XL形式のエンコードには `jpegxl-rs` クレートを使用しています。ビルド時に `libjxl` ライブラリがシステムにインストールされている必要があります。
pub fn encode(img: &DynamicImage, options: &EncodeOptions) -> Result<Vec<u8>, AppError> {
    // match式の結果を変数に束縛する
    let result = match options {
        EncodeOptions::Avif(opts) => {
            println!("Adapter: Converting AvifOptions for ravif encoder...");
            avif::encode(img, opts)
        }
        EncodeOptions::Webp(opts) => {
            println!("Adapter: Converting WebpOptions for libwebp_sys encoder...");
            webp::encode(img, opts)
        }
        EncodeOptions::Jxl(opts) => {
            println!("Adapter: Converting JxlOptions for jpegxl_rs encoder...");
            jxl::encode(img, opts)
        }
    };

    // match式から受け取った結果を返す
    result
}

/// DynamicImageからエンコード用のピクセルデータを効率的に抽出します。
///
/// - 元の画像がRGB8/RGBA8形式の場合、データを借用して不要なコピーを避けます。
/// - それ以外の形式の場合は、RGBA8に変換して所有権を持つデータを生成します。
///
/// # Arguments
/// * `img` - 処理対象の`DynamicImage`への参照。
///
/// # Returns
/// * `(Cow<'a, [u8]>, bool)` - ピクセルデータと、アルファチャンネルの有無 (`true`ならRGBA) のタプル。
pub fn extract_pixel_data(img: &DynamicImage) -> (Cow<'_, [u8]>, bool) {
    match img {
        DynamicImage::ImageRgba8(buffer) => (Cow::Borrowed(buffer.as_raw()), true),
        DynamicImage::ImageRgb8(buffer) => (Cow::Borrowed(buffer.as_raw()), false),
        // 16ビット画像(Rgba16)や他の形式が来た場合...
        _ => {
            // ...img.to_rgba8() を使って8ビットのRGBA形式に変換する！
            let buffer = img.to_rgba8();
            // これで、どんな入力でも必ず8ビットRGBAデータになる
            (Cow::Owned(buffer.into_raw()), true)
        }
    }
}
