use crate::error::AppError;
use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};
use libheif_rs::{HeifContext, LibHeif};

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
            heif_to_dynamic_image(image_bytes)
        }
        DetectedFormat::Exr => Err(AppError::Decode(
            "EXR format is not supported in this version".into(),
        )),
        DetectedFormat::Jpeg2000 => {
            println!("Decoder: Using Jpeg2k decoder...");
            jpeg2k_to_dynamic_image(image_bytes)
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

/// HEIFファイルを読み込み、DynamicImageに変換する関数
fn heif_to_dynamic_image(bytes: &[u8]) -> Result<DynamicImage, AppError> {
    let lib_heif = LibHeif::new();

    let ctx = HeifContext::read_from_bytes(bytes).map_err(|e| AppError::Decode(e.to_string()))?;
    let handle = ctx
        .primary_image_handle()
        .map_err(|e| AppError::Decode(e.to_string()))?;
    let img = lib_heif
        .decode(
            &handle,
            libheif_rs::ColorSpace::Rgb(libheif_rs::RgbChroma::Rgba),
            None,
        )
        .map_err(|e| AppError::Decode(e.to_string()))?;

    let width = handle.width();
    let height = handle.height();
    let planes = img.planes();
    let interleaved_plane = planes
        .interleaved
        .ok_or(AppError::Decode("Interleaved plane not found".to_string()))?;
    let pixel_data = interleaved_plane.data.to_vec();

    let image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, pixel_data).ok_or(AppError::Decode(
            "Failed to create ImageBuffer from raw data".to_string(),
        ))?;

    println!("Decoder: Finish decoding HEIC.");
    Ok(DynamicImage::ImageRgba8(image_buffer))
}

/// JPEG 2000 ファイルを読み込み、DynamicImageに変換する
fn jpeg2k_to_dynamic_image(bytes: &[u8]) -> Result<DynamicImage, AppError> {
    // Use the `jpeg2k` crate to decode JPEG 2000 from bytes
    let jp2_image =
        jpeg2k::Image::from_bytes(bytes).map_err(|e| AppError::Decode(e.to_string()))?;

    let width = jp2_image.width();
    let height = jp2_image.height();
    let components = jp2_image.components();

    // Convert to a `image::DynamicImage`
    let dynamic_image: DynamicImage = match components.len() {
        3 => {
            let r = &components[0].data();
            let g = &components[1].data();
            let b = &components[2].data();
            let mut img_buf = image::RgbImage::new(width, height);
            for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
                let index = (y * width + x) as usize;
                *pixel = image::Rgb([r[index] as u8, g[index] as u8, b[index] as u8]);
            }
            DynamicImage::ImageRgb8(img_buf)
        }
        4 => {
            let r = &components[0].data();
            let g = &components[1].data();
            let b = &components[2].data();
            let a = &components[3].data();
            let mut img_buf = image::RgbaImage::new(width, height);
            for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
                let index = (y * width + x) as usize;
                *pixel = image::Rgba([
                    r[index] as u8,
                    g[index] as u8,
                    b[index] as u8,
                    a[index] as u8,
                ]);
            }
            DynamicImage::ImageRgba8(img_buf)
        }
        _ => {
            return Err(AppError::Decode(
                "Unsupported number of components in JPEG 2000 file".into(),
            ));
        }
    };

    println!("Decoder: Finish decoding JPEG 2000.");

    Ok(dynamic_image)
}

/*
/// ACESフィルミックトーンマッピング
fn aces_tonemap(x: f32) -> f32 {
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    ((x * (a * x + b)) / (x * (c * x + d) + e)).clamp(0.0, 1.0)
}

/// sRGBガンマ補正 (approx)
fn srgb_gamma(x: f32) -> f32 {
    if x <= 0.0031308 {
        12.92 * x
    } else {
        1.055 * x.powf(1.0 / 2.4) - 0.055
    }
}

/// f32 (0–∞, linear) → u8 (0–255, sRGB)
fn f32_to_u8(x: f32) -> u8 {
    (srgb_gamma(aces_tonemap(x)) * 255.0)
        .round()
        .clamp(0.0, 255.0) as u8
}

/// EXR → DynamicImage (Rgba8, ACESトーンマップ付き)
pub fn exr_to_dynamic_image(
    path: &str,
) -> std::result::Result<DynamicImage, Box<dyn std::error::Error>> {
    let mut rgba_image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(1, 1);

    exr::prelude::read()
        .no_deep_data()
        .largest_resolution_level()
        .rgba_channels(
            |resolution, _| {
                rgba_image = ImageBuffer::new(resolution.width(), resolution.height());
                &mut rgba_image
            },
            |img, pos, (r, g, b, a): (f32, f32, f32, f32)| {
                let pixel = Rgba([
                    f32_to_u8(r),
                    f32_to_u8(g),
                    f32_to_u8(b),
                    (a.clamp(0.0, 1.0) * 255.0).round() as u8,
                ]);
                img.put_pixel(pos.x(), pos.y(), pixel);
            },
        )
        .from_file(path)?;

    Ok(DynamicImage::ImageRgba8(rgba_image))
}
*/
