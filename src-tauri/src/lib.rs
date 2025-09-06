mod decoder;
pub mod error;

use exif::{In, Reader as ExifReader, Tag};
use image::{self, ImageFormat, RgbaImage, imageops::*};
use std::io::Cursor;

// 独自の形式を定義するためのenum
pub enum DetectedFormat {
    Heic,
    // Exr,
    Jpeg2000,
    // imageクレートがサポートするその他の形式
    Standard(ImageFormat),
}

/// バイトデータのマジックナンバーから画像形式を判別する
pub fn detect_format(bytes: &[u8]) -> Option<DetectedFormat> {
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
    /*
    // EXRのチェック
    if bytes.starts_with(&[0x76, 0x2f, 0x31, 0x01]) {
        return Some(DetectedFormat::Exr);
    }
    */

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
