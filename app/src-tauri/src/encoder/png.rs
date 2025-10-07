use crate::error::AppError;
use crate::options::HighBitDepthImage;
use oxipng::{optimize_from_memory, Deflaters, Options as OxiPngOptions};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

/// PNG最適化オプション（Zopfli専用）
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PngOptions {
    /// Zopfliの反復回数
    pub zopfli_iterations: u32,
    /// ICCプロファイルを含めるか
    pub embed_icc_profile: bool,
}

impl Default for PngOptions {
    fn default() -> Self {
        Self {
            zopfli_iterations: 15, // Zopfliのデフォルト
            embed_icc_profile: true,
        }
    }
}

/// PNG画像をZopfliで圧縮します
pub fn encode(
    img: &HighBitDepthImage,
    icc_profile: Option<Vec<u8>>,
    options: &PngOptions,
) -> Result<Vec<u8>, AppError> {
    let (width, height, color_type, bit_depth, data) = prepare_image_data(img)?;
    encode_with_zopfli(
        width,
        height,
        color_type,
        bit_depth,
        &data,
        icc_profile,
        options,
    )
}

/// Zopfli（OxiPNG）を使った高圧縮PNGエンコード
fn encode_with_zopfli(
    width: u32,
    height: u32,
    color_type: png::ColorType,
    bit_depth: png::BitDepth,
    data: &[u8],
    icc_profile: Option<Vec<u8>>,
    options: &PngOptions,
) -> Result<Vec<u8>, AppError> {
    // まず標準的なPNGを生成
    let mut temp_buffer = Vec::new();
    let cursor = Cursor::new(&mut temp_buffer);

    let mut encoder = png::Encoder::new(cursor, width, height);
    encoder.set_color(color_type);
    encoder.set_depth(bit_depth);
    encoder.set_compression(png::Compression::Fast); // 一時的に高速圧縮

    let mut writer = encoder
        .write_header()
        .map_err(|e| AppError::Encode(format!("PNG header write error: {}", e)))?;

    // ICCプロファイル埋め込み
    if options.embed_icc_profile {
        if let Some(profile) = icc_profile {
            writer
                .write_chunk(png::chunk::iCCP, &create_iccp_chunk(&profile)?)
                .map_err(|e| AppError::Encode(format!("ICC profile embedding error: {}", e)))?;
        }
    }

    writer
        .write_image_data(data)
        .map_err(|e| AppError::Encode(format!("PNG image data write error: {}", e)))?;

    writer
        .finish()
        .map_err(|e| AppError::Encode(format!("PNG encoding finish error: {}", e)))?;

    // OxiPNGでZopfli最適化
    let mut oxipng_options = OxiPngOptions::default();
    oxipng_options.deflate = Deflaters::Zopfli {
        iterations: std::num::NonZero::new(options.zopfli_iterations as u8)
            .unwrap_or(std::num::NonZero::new(15).unwrap()),
    };
    oxipng_options.optimize_alpha = true;
    oxipng_options.strip = oxipng::StripChunks::Safe;

    println!(
        "PNG: Applying OxiPNG optimization with Zopfli ({} iterations)...",
        options.zopfli_iterations
    );

    let optimized = optimize_from_memory(&temp_buffer, &oxipng_options)
        .map_err(|e| AppError::Encode(format!("OxiPNG optimization error: {}", e)))?;

    let original_size = temp_buffer.len();
    let optimized_size = optimized.len();
    let reduction = ((original_size - optimized_size) as f64 / original_size as f64) * 100.0;

    println!("PNG: OxiPNG optimization completed");
    println!("     Original: {} bytes", original_size);
    println!("     Optimized: {} bytes", optimized_size);
    println!("     Reduction: {:.1}%", reduction);

    Ok(optimized)
}

/// 画像データをPNGエンコード用に準備
fn prepare_image_data(
    img: &HighBitDepthImage,
) -> Result<(u32, u32, png::ColorType, png::BitDepth, Vec<u8>), AppError> {
    match img {
        HighBitDepthImage::Rgb(buffer) => {
            let (width, height) = buffer.dimensions();
            let pixels = buffer.as_raw();

            // f32からu8に変換
            let data: Vec<u8> = pixels
                .iter()
                .map(|&pixel| (pixel.clamp(0.0, 1.0) * 255.0) as u8)
                .collect();

            Ok((
                width,
                height,
                png::ColorType::Rgb,
                png::BitDepth::Eight,
                data,
            ))
        }
        HighBitDepthImage::Rgba(buffer) => {
            let (width, height) = buffer.dimensions();
            let pixels = buffer.as_raw();

            // f32からu8に変換
            let data: Vec<u8> = pixels
                .iter()
                .map(|&pixel| (pixel.clamp(0.0, 1.0) * 255.0) as u8)
                .collect();

            Ok((
                width,
                height,
                png::ColorType::Rgba,
                png::BitDepth::Eight,
                data,
            ))
        }
        HighBitDepthImage::Argb(buffer) => {
            let (width, height) = buffer.dimensions();
            let argb_pixels = buffer.as_raw();

            // ARGBからRGBAに変換してu8に変換
            let mut rgba_data = Vec::with_capacity(argb_pixels.len());
            for chunk in argb_pixels.chunks_exact(4) {
                let a = (chunk[0].clamp(0.0, 1.0) * 255.0) as u8; // Alpha
                let r = (chunk[1].clamp(0.0, 1.0) * 255.0) as u8; // Red
                let g = (chunk[2].clamp(0.0, 1.0) * 255.0) as u8; // Green
                let b = (chunk[3].clamp(0.0, 1.0) * 255.0) as u8; // Blue

                rgba_data.extend_from_slice(&[r, g, b, a]);
            }

            Ok((
                width,
                height,
                png::ColorType::Rgba,
                png::BitDepth::Eight,
                rgba_data,
            ))
        }
    }
}

/// ICCプロファイルのiCCPチャンクを作成
fn create_iccp_chunk(profile: &[u8]) -> Result<Vec<u8>, AppError> {
    use flate2::{write::DeflateEncoder, Compression};
    use std::io::Write;

    println!(
        "PNG: Creating iCCP chunk with profile size: {} bytes",
        profile.len()
    );

    // プロファイル名（固定）
    let profile_name = b"Embedded Profile\0";

    // 圧縮方式（0 = deflate）
    let compression_method = [0u8];

    // プロファイルをDeflateで圧縮
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
    encoder
        .write_all(profile)
        .map_err(|e| AppError::Encode(format!("ICC profile compression write error: {}", e)))?;

    let compressed_profile = encoder
        .finish()
        .map_err(|e| AppError::Encode(format!("ICC profile compression finish error: {}", e)))?;

    let mut chunk_data = Vec::new();
    chunk_data.extend_from_slice(profile_name);
    chunk_data.extend_from_slice(&compression_method);
    chunk_data.extend_from_slice(&compressed_profile);

    println!(
        "PNG: iCCP chunk created, compressed profile size: {} bytes",
        compressed_profile.len()
    );

    Ok(chunk_data)
}

/// PNGファイルサイズを推定
pub fn estimate_size(img: &HighBitDepthImage, options: &PngOptions) -> usize {
    let (width, height, color_type, _, _) = match prepare_image_data(img) {
        Ok(data) => data,
        Err(_) => return 0,
    };

    // 基本的なサイズ推定
    let channels = match color_type {
        png::ColorType::Rgb => 3,
        png::ColorType::Rgba => 4,
        _ => 3,
    };

    let uncompressed_size = (width * height * channels) as usize;

    // 圧縮率の推定（Zopfliのiteration数により調整）
    let compression_ratio = if options.zopfli_iterations >= 50 {
        0.3 // 70%圧縮（高iteration）
    } else if options.zopfli_iterations >= 20 {
        0.4 // 60%圧縮（中iteration）
    } else {
        0.5 // 50%圧縮（低iteration）
    };

    (uncompressed_size as f64 * compression_ratio) as usize
}
