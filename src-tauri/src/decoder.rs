mod heif;
mod jpeg2k;

use crate::error::AppError;
use image::{DynamicImage, ImageFormat};

/// バイトデータから画像をデコードし、DynamicImageとして返す
/// サポートする形式: HEIC, JPEG 2000, そして imageクレートが対応する形式
/// # 引数
/// - `image_bytes`: 画像のバイトデータ
/// # 戻り値
/// - 成功した場合は `DynamicImage` を返します。
/// - 失敗した場合は `Box<dyn Error>` を返します。
/// # 注意
/// - EXR形式はこのバージョンではサポートされていません
/// - HEIC形式のデコードには `libheif-rs` クレートを使用しています。ビルド時に `libheif` ライブラリがシステムにインストールされている必要があります。
/// - JPEG 2000形式のデコードには `jpeg2k` クレートを使用しています。
///  ただし、このクレートはすべてのJPEG 2000ファイルに対応しているわけではないため、特定のファイルでエラーが発生する可能性があります。
#[allow(dead_code)]
pub fn decode(image_bytes: &[u8]) -> Result<DynamicImage, AppError> {
    // まず、バイトデータから画像形式を判別する
    let format = detect_format(image_bytes)
        .ok_or_else(|| AppError::Decode("Unsupported or unknown image format".to_string()))?;

    // 判別した形式に応じて、適切なデコーダーを呼び出す
    match format {
        DetectedFormat::Heic => {
            println!("Decoder: Using heif decoder...");
            heif::decode(image_bytes)
        }
        DetectedFormat::Exr => Err(AppError::Decode(
            "EXR format is not supported in this version".into(),
        )),
        DetectedFormat::Jpeg2000 => {
            println!("Decoder: Using Jpeg2k decoder...");
            jpeg2k::decode(image_bytes)
        }
        DetectedFormat::Standard(image_format) => {
            println!("Decoder: Using image decoder...");
            image::load_from_memory_with_format(image_bytes, image_format)
                .map_err(|e| AppError::Decode(e.to_string()))
        }
    }
}

// 独自の形式を定義するためのenum
enum DetectedFormat {
    Heic,
    Exr,
    Jpeg2000,
    // imageクレートがサポートするその他の形式
    Standard(ImageFormat),
}

/// バイトデータのマジックナンバーから画像形式を判別する
fn detect_format(bytes: &[u8]) -> Option<DetectedFormat> {
    // HEIC/AVIF (ISOBMFFコンテナ) のチェック
    // ftyp ボックスが "heic", "heix", "avif" などを含むか
    if bytes.len() > 12 && &bytes[4..8] == b"ftyp" {
        let ftyp = &bytes[8..12];
        if ftyp == b"heic" || ftyp == b"heix" || ftyp == b"hevc" || ftyp == b"heim" {
            return Some(DetectedFormat::Heic);
        }
        // AVIFの判別もここに追加できる
        if ftyp == b"avif" || ftyp == b"avis" {
            // AVIFの場合はimageクレートが扱えるのでStandardに流す
            if let Ok(format) = image::guess_format(bytes) {
                return Some(DetectedFormat::Standard(format));
            }
        }
    }
    // EXRのチェック
    if bytes.starts_with(&[0x76, 0x2f, 0x31, 0x01]) {
        return Some(DetectedFormat::Exr);
    }

    // JPEG 2000のチェック
    if bytes.starts_with(b"\x00\x00\x00\x0CjP  \r\n\x87\n") {
        return Some(DetectedFormat::Jpeg2000);
    }

    // 上記のいずれでもない場合、imageクレートの形式推測に任せる
    if let Ok(format) = image::guess_format(bytes) {
        return Some(DetectedFormat::Standard(format));
    }

    None
}
