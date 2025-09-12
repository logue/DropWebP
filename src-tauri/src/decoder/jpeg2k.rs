use crate::error::AppError;
use image::DynamicImage;

/// JPEG 2000 ファイルを読み込み、DynamicImageに変換する
pub fn decode(bytes: &[u8]) -> Result<DynamicImage, AppError> {
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
