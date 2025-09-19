mod heif;
mod jpeg2k;
mod jxl;

use crate::error::AppError;
use exif::{In, Reader as ExifReader, Tag};
use image::{self, DynamicImage, ImageFormat, RgbaImage, imageops::*};
use std::io::Cursor;

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
        DetectedFormat::Jxl => {
            println!("Decoder: Using JPEG XL decoder...");
            jxl::decode(image_bytes)
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
    Jxl,
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

    // JPEG XLのチェック
    if bytes.starts_with(b"\xFF\x0A") || bytes.starts_with(b"\x00\x00\x00\x0CJXL ") {
        return Some(DetectedFormat::Jxl);
    }

    // 上記のいずれでもない場合、imageクレートの形式推測に任せる
    if let Ok(format) = image::guess_format(bytes) {
        return Some(DetectedFormat::Standard(format));
    }

    None
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
