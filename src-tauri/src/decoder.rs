use image::{DynamicImage, ImageBuffer, Rgba};
use libheif_rs::{HeifContext, LibHeif};
use std::error::Error;
use std::path::Path;

/// HEIFファイルを読み込み、DynamicImageに変換する関数
pub fn heif_to_dynamic_image<P: AsRef<Path>>(
    path: P,
) -> std::result::Result<DynamicImage, Box<dyn Error>> {
    let lib_heif = LibHeif::new();

    // --- 修正点 1: unwrap() を安全なエラーハンドリングに変更 ---
    // path.as_ref().to_str() は、パスがUTF-8でない場合にNoneを返すため、unwrap()は危険です。
    let path_str = path
        .as_ref()
        .to_str()
        .ok_or("Path contains invalid UTF-8 characters")?;

    let ctx = HeifContext::read_from_file(path_str)?;
    let handle = ctx.primary_image_handle()?;
    let img = lib_heif.decode(
        &handle,
        libheif_rs::ColorSpace::Rgb(libheif_rs::RgbChroma::Rgba),
        None,
    )?;

    let width = handle.width();
    let height = handle.height();
    let planes = img.planes();
    let interleaved_plane = planes.interleaved.ok_or("Interleaved plane not found")?;
    let pixel_data = interleaved_plane.data.to_vec();

    let image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, pixel_data)
            .ok_or("Failed to create ImageBuffer from raw data")?;

    Ok(DynamicImage::ImageRgba8(image_buffer))
}

/// JPEG 2000 ファイルを読み込み、DynamicImageに変換する
pub fn jpeg2000_to_dynamic_image<P: AsRef<Path>>(
    path: P,
) -> std::result::Result<DynamicImage, Box<dyn Error>> {
    // Load jpeg 2000 file from file.
    let jp2_image = jpeg2k::Image::from_file(path).expect("Failed to load j2k file.");

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
        _ => return Err("Unsupported number of components in JPEG 2000 file".into()),
    };

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
