use crate::encoder::progress::ProgressCallback;
use crate::error::AppError;
use crate::options::HighBitDepthImage;
use oxipng::Options;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PngOptions {
    #[serde(default = "default_optimization_level")]
    pub optimization_level: u8, // 0-6
    #[serde(default)]
    pub use_zopfli: bool, // Use Zopfli compression (slower but smaller).
    #[serde(default)]
    pub strip_metadata: bool, // Strip metadata.
    #[serde(default)]
    pub bit_depth_reduction: bool, // Try reducing bit depth.
    #[serde(default)]
    pub color_type_reduction: bool, // Try reducing color type.
    #[serde(default)]
    pub palette_reduction: bool, // Try reducing palette.
    #[serde(default)]
    pub grayscale_reduction: bool, // Try grayscale conversion.
    #[serde(default)]
    pub interlace: Option<bool>, // Interlace (None = unchanged, Some(true) = on, Some(false) = off).
    #[serde(default)]
    pub optimize_alpha: bool, // Optimize transparent pixels.
    #[serde(default)]
    pub fast_evaluation: bool, // Fast evaluation mode (default true).
    #[serde(default)]
    pub scale_16: bool, // Force 16-bit images down to 8-bit.
}

/// Default optimization level (balanced).
fn default_optimization_level() -> u8 {
    2
}

/// PNG encoder backed by oxipng 10.0.
///
/// oxipng only supports lossless compression.
/// optimization_level: 0 (fastest / least compression) - 6 (slowest / most compression).
pub fn encode(
    pixel_data: &HighBitDepthImage,
    _icc_profile: Option<Vec<u8>>,
    options: &PngOptions,
) -> Result<Vec<u8>, AppError> {
    let encode_start = std::time::Instant::now();
    println!("PNG: Starting PNG optimization with oxipng 10.0...");

    let opt_level = options.optimization_level.min(6);

    println!(
        "PNG: Optimization level {} (Zopfli: {})",
        opt_level, options.use_zopfli
    );

    // Encode as PNG using the `image` crate.
    let mut png_buffer = Vec::new();
    {
        let mut cursor = Cursor::new(&mut png_buffer);
        match pixel_data {
            HighBitDepthImage::Rgb(img) => {
                // RGB32F → RGB8
                let rgb8 = image::DynamicImage::ImageRgb32F(img.clone()).to_rgb8();
                rgb8.write_to(&mut cursor, image::ImageFormat::Png)
                    .map_err(|e| AppError::Encode(format!("Failed to encode PNG: {}", e)))?;
            }
            HighBitDepthImage::Rgba(img) => {
                // RGBA32F → RGBA8
                let rgba8 = image::DynamicImage::ImageRgba32F(img.clone()).to_rgba8();
                rgba8
                    .write_to(&mut cursor, image::ImageFormat::Png)
                    .map_err(|e| AppError::Encode(format!("Failed to encode PNG: {}", e)))?;
            }
            HighBitDepthImage::Argb(img) => {
                // ARGB32F → RGBA8 (channel swap)
                let rgba8 = image::DynamicImage::ImageRgba32F(img.clone()).to_rgba8();
                rgba8
                    .write_to(&mut cursor, image::ImageFormat::Png)
                    .map_err(|e| AppError::Encode(format!("Failed to encode PNG: {}", e)))?;
            }
        }
    }

    println!("PNG: Initial PNG size: {} bytes", png_buffer.len());

    // oxipng 10.0: configure options.
    let mut opts = Options::from_preset(opt_level);

    // Apply detailed options.
    if options.use_zopfli {
        use std::num::NonZero;
        let zopfli_opts = oxipng::ZopfliOptions {
            iteration_count: NonZero::new(15).unwrap(),
            iterations_without_improvement: NonZero::new(u64::MAX).unwrap(),
            maximum_block_splits: 15,
        };
        opts.deflater = oxipng::Deflater::Zopfli(zopfli_opts);
        println!("PNG: Using Zopfli compression (15 iterations)");
    }

    if options.strip_metadata {
        opts.strip = oxipng::StripChunks::Safe;
        println!("PNG: Stripping metadata");
    }

    // Interlace setting.
    if let Some(interlace) = options.interlace {
        opts.interlace = Some(interlace);
        println!("PNG: Interlace: {}", interlace);
    }

    // Alpha optimization.
    opts.optimize_alpha = options.optimize_alpha;
    if options.optimize_alpha {
        println!("PNG: Alpha optimization enabled");
    }

    // Fast evaluation mode.
    opts.fast_evaluation = options.fast_evaluation;

    // Force 16-bit -> 8-bit scaling.
    opts.scale_16 = options.scale_16;
    if options.scale_16 {
        println!("PNG: 16-bit to 8-bit scaling enabled");
    }

    // Reduction settings.
    opts.bit_depth_reduction = options.bit_depth_reduction;
    opts.color_type_reduction = options.color_type_reduction;
    opts.palette_reduction = options.palette_reduction;
    opts.grayscale_reduction = options.grayscale_reduction;

    if options.bit_depth_reduction
        || options.color_type_reduction
        || options.palette_reduction
        || options.grayscale_reduction
    {
        println!(
            "PNG: Enabled reductions - bit_depth: {}, color_type: {}, palette: {}, grayscale: {}",
            options.bit_depth_reduction,
            options.color_type_reduction,
            options.palette_reduction,
            options.grayscale_reduction
        );
    }

    match oxipng::optimize_from_memory(&png_buffer, &opts) {
        Ok(optimized) => {
            println!(
                "PNG: Optimization completed in {:.2}s, final size: {} bytes (saved {} bytes)",
                encode_start.elapsed().as_secs_f64(),
                optimized.len(),
                png_buffer.len().saturating_sub(optimized.len())
            );
            Ok(optimized)
        }
        Err(e) => Err(AppError::Encode(format!(
            "oxipng optimization failed: {}",
            e
        ))),
    }
}

/// Estimate the encoded PNG size.
///
/// PNG is lossless, so size estimation is inherently rough; this function
/// returns roughly 60% of the uncompressed size as a heuristic.
pub fn estimate_size(pixel_data: &HighBitDepthImage, _options: &PngOptions) -> usize {
    let (width, height) = match pixel_data {
        HighBitDepthImage::Rgb(img) => (img.width(), img.height()),
        HighBitDepthImage::Rgba(img) => (img.width(), img.height()),
        HighBitDepthImage::Argb(img) => (img.width(), img.height()),
    };

    let bytes_per_pixel = match pixel_data {
        HighBitDepthImage::Rgb(_) => 3,
        HighBitDepthImage::Rgba(_) | HighBitDepthImage::Argb(_) => 4,
    };

    // Uncompressed size.
    let raw_size = (width * height * bytes_per_pixel) as usize;

    // Assume a PNG compression ratio of 60% (varies with optimization level).
    // Zopfli may yield smaller files; we estimate conservatively here.
    (raw_size as f64 * 0.6) as usize
}

/// PNG encoder with progress reporting.
///
/// Wraps `encode` with progress callbacks. oxipng itself does not expose progress
/// callbacks, so we report progress at coarse-grained stages.
// Used by the binary crate via `crate::encoder::png::encode_with_progress`.
#[allow(dead_code)]
pub fn encode_with_progress(
    pixel_data: &HighBitDepthImage,
    _icc_profile: Option<Vec<u8>>,
    options: &PngOptions,
    progress_callback: Arc<dyn ProgressCallback>,
) -> Result<Vec<u8>, AppError> {
    progress_callback.on_progress(0.0, "Starting PNG encoding");

    let encode_start = std::time::Instant::now();
    println!("PNG: Starting PNG optimization with oxipng 10.0...");

    let opt_level = options.optimization_level.min(6);

    println!(
        "PNG: Optimization level {} (Zopfli: {})",
        opt_level, options.use_zopfli
    );

    progress_callback.on_progress(10.0, "Encoding to PNG format");

    // Encode to PNG using the `image` crate.
    let mut png_buffer = Vec::new();
    {
        let mut cursor = Cursor::new(&mut png_buffer);
        match pixel_data {
            HighBitDepthImage::Rgb(img) => {
                // RGB32F → RGB8
                let rgb8 = image::DynamicImage::ImageRgb32F(img.clone()).to_rgb8();
                rgb8.write_to(&mut cursor, image::ImageFormat::Png)
                    .map_err(|e| {
                        progress_callback.on_error(&format!("Failed to encode PNG: {}", e));
                        AppError::Encode(format!("Failed to encode PNG: {}", e))
                    })?;
            }
            HighBitDepthImage::Rgba(img) => {
                // RGBA32F → RGBA8
                let rgba8 = image::DynamicImage::ImageRgba32F(img.clone()).to_rgba8();
                rgba8
                    .write_to(&mut cursor, image::ImageFormat::Png)
                    .map_err(|e| {
                        progress_callback.on_error(&format!("Failed to encode PNG: {}", e));
                        AppError::Encode(format!("Failed to encode PNG: {}", e))
                    })?;
            }
            HighBitDepthImage::Argb(img) => {
                // ARGB32F → RGBA8 (channel swap)
                let rgba8 = image::DynamicImage::ImageRgba32F(img.clone()).to_rgba8();
                rgba8
                    .write_to(&mut cursor, image::ImageFormat::Png)
                    .map_err(|e| {
                        progress_callback.on_error(&format!("Failed to encode PNG: {}", e));
                        AppError::Encode(format!("Failed to encode PNG: {}", e))
                    })?;
            }
        }
    }

    println!("PNG: Initial PNG size: {} bytes", png_buffer.len());
    progress_callback.on_progress(30.0, "Initial encoding complete");

    // oxipng 10.0: configure options.
    progress_callback.on_progress(40.0, "Configuring optimizer");
    let mut opts = Options::from_preset(opt_level);

    // Apply detailed options.
    if options.use_zopfli {
        use std::num::NonZero;
        let zopfli_opts = oxipng::ZopfliOptions {
            iteration_count: NonZero::new(15).unwrap(),
            iterations_without_improvement: NonZero::new(u64::MAX).unwrap(),
            maximum_block_splits: 15,
        };
        opts.deflater = oxipng::Deflater::Zopfli(zopfli_opts);
        println!("PNG: Using Zopfli compression (15 iterations)");
        progress_callback.on_progress(50.0, "Using Zopfli compression (slow but better)");
    } else {
        progress_callback.on_progress(50.0, "Using standard compression");
    }

    if options.strip_metadata {
        opts.strip = oxipng::StripChunks::Safe;
        println!("PNG: Stripping metadata");
    }

    // Interlace setting.
    if let Some(interlace) = options.interlace {
        opts.interlace = Some(interlace);
        println!("PNG: Interlace: {}", interlace);
    }

    // Alpha optimization.
    opts.optimize_alpha = options.optimize_alpha;
    if options.optimize_alpha {
        println!("PNG: Alpha optimization enabled");
    }

    // Fast evaluation mode.
    opts.fast_evaluation = options.fast_evaluation;

    // Force 16-bit -> 8-bit scaling.
    opts.scale_16 = options.scale_16;
    if options.scale_16 {
        println!("PNG: 16-bit to 8-bit scaling enabled");
    }

    // Reduction settings.
    opts.bit_depth_reduction = options.bit_depth_reduction;
    opts.color_type_reduction = options.color_type_reduction;
    opts.palette_reduction = options.palette_reduction;
    opts.grayscale_reduction = options.grayscale_reduction;

    if options.bit_depth_reduction
        || options.color_type_reduction
        || options.palette_reduction
        || options.grayscale_reduction
    {
        println!(
            "PNG: Enabled reductions - bit_depth: {}, color_type: {}, palette: {}, grayscale: {}",
            options.bit_depth_reduction,
            options.color_type_reduction,
            options.palette_reduction,
            options.grayscale_reduction
        );
    }

    progress_callback.on_progress(60.0, "Optimizing PNG");

    match oxipng::optimize_from_memory(&png_buffer, &opts) {
        Ok(optimized) => {
            println!(
                "PNG: Optimization completed in {:.2}s, final size: {} bytes (saved {} bytes)",
                encode_start.elapsed().as_secs_f64(),
                optimized.len(),
                png_buffer.len().saturating_sub(optimized.len())
            );
            progress_callback.on_progress(90.0, "Optimization complete");
            progress_callback.on_complete();
            Ok(optimized)
        }
        Err(e) => {
            let error_msg = format!("oxipng optimization failed: {}", e);
            progress_callback.on_error(&error_msg);
            Err(AppError::Encode(error_msg))
        }
    }
}
