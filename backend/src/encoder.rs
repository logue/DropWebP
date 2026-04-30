pub mod avif;
pub mod common;
pub mod jpeg;
pub mod jxl;
pub mod png;
pub mod progress;
pub mod webp;

use crate::error::AppError;
use crate::options::{EncodeOptions, HighBitDepthImage};
use std::borrow::Cow;

/// Encode an image into the requested format.
///
/// # Arguments
/// - `img`: source image (`HighBitDepthImage`).
/// - `icc_profile`: optional ICC profile bytes to embed.
/// - `options`: encoding options (`EncodeOptions`).
///
/// # Returns
/// Encoded bytes on success.
///
/// # Errors
/// Returns `AppError` when encoding fails for the selected format.
///
/// # Notes
/// - AVIF encoding uses the `libavif-sys` crate; `libavif` must be available at build time.
/// - WebP encoding uses the `libwebp-sys` crate; `libwebp` must be available at build time.
/// - JPEG XL encoding uses `jxl-sys`; `libjxl` must be available at build time.
pub fn encode(
    img: HighBitDepthImage,
    icc_profile: Option<Vec<u8>>,
    options: &EncodeOptions,
) -> Result<Vec<u8>, AppError> {
    // Bind the result of the match expression to a local variable.
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
            println!("Adapter: Converting JxlOptions for jxl-sys encoder...");
            jxl::encode(&img, icc_profile, opts)
        }
        EncodeOptions::Png(opts) => {
            println!("Adapter: Converting PngOptions for oxipng encoder...");
            png::encode(&img, icc_profile, opts)
        }
        EncodeOptions::Jpeg(opts) => {
            println!("Adapter: Converting JpegOptions for jpegli encoder...");
            jpeg::encode(&img, icc_profile, opts)
        }
    };

    // Return the result captured from the match expression.
    result
}

/// Estimate the encoded file size for the requested format.
///
/// # Arguments
/// - `img`: source image (`HighBitDepthImage`).
/// - `options`: encoding options (`EncodeOptions`).
///
/// # Returns
/// Estimated size in bytes.
pub fn estimate_size(img: &HighBitDepthImage, options: &EncodeOptions) -> usize {
    let (width, height, channels) = match img {
        HighBitDepthImage::Rgb(buf) => (buf.width(), buf.height(), 3),
        HighBitDepthImage::Rgba(buf) | HighBitDepthImage::Argb(buf) => {
            (buf.width(), buf.height(), 4)
        }
    };

    match options {
        EncodeOptions::Avif(opts) => avif::estimate_size(img, opts),
        EncodeOptions::Webp(opts) => webp::estimate_size(img, opts),
        EncodeOptions::Jxl(opts) => jxl::estimate_size(width, height, channels, opts.quality),
        EncodeOptions::Png(opts) => png::estimate_size(img, opts),
        EncodeOptions::Jpeg(opts) => jpeg::estimate_size(img, opts),
    }
}

/// Extract pixel data from a `HighBitDepthImage` for encoding.
///
/// - For RGB/RGBA images, the underlying buffer is borrowed to avoid copies.
/// - For ARGB images, the data is converted to RGBA in an owned buffer.
///
/// # Arguments
/// * `img` - Reference to the source `HighBitDepthImage`.
///
/// # Returns
/// Tuple `(Cow<'a, [f32]>, bool)` of the pixel data and a flag indicating whether
/// an alpha channel is present (`true` means RGBA).
pub fn extract_pixel_data(img: &HighBitDepthImage) -> (Cow<'_, [f32]>, bool) {
    match img {
        HighBitDepthImage::Rgba(buffer) => (Cow::Borrowed(buffer.as_raw()), true),
        HighBitDepthImage::Rgb(buffer) => (Cow::Borrowed(buffer.as_raw()), false),
        HighBitDepthImage::Argb(buffer) => {
            // Convert ARGB to RGBA.
            let argb_pixels = buffer.as_raw();
            let mut rgba_pixels = Vec::with_capacity(argb_pixels.len());

            // Convert each ARGB pixel (A, R, G, B) to RGBA pixel (R, G, B, A).
            for chunk in argb_pixels.chunks_exact(4) {
                let a = chunk[0]; // Alpha
                let r = chunk[1]; // Red
                let g = chunk[2]; // Green
                let b = chunk[3]; // Blue

                // Store in RGBA order.
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
