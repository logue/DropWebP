use super::progress::ProgressCallback;
use crate::error::AppError;
use crate::options::HighBitDepthImage;
use indexmap::IndexSet;
use oxipng::optimize_from_memory;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::sync::Arc;

/// PNGフィルター戦略
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PngFilter {
    /// フィルターなし
    None,
    /// Subフィルター
    Sub,
    /// Upフィルター
    Up,
    /// Averageフィルター
    Average,
    /// Paethフィルター
    Paeth,
    /// 最小合計（すべてのフィルターを試して最小を選択）
    MinSum,
    /// エントロピー（最小エントロピーのフィルターを選択）
    Entropy,
    /// Bigrams（2グラム頻度分析）
    Bigrams,
    /// BigEnt（BigramsとEntropyの組み合わせ）
    BigEnt,
    /// Brute（すべての組み合わせを試行、最も遅いが最良の圧縮）
    Brute,
}

impl Default for PngFilter {
    fn default() -> Self {
        Self::MinSum // バランスの良いデフォルト
    }
}

impl From<PngFilter> for oxipng::RowFilter {
    fn from(filter: PngFilter) -> Self {
        match filter {
            PngFilter::None => oxipng::RowFilter::None,
            PngFilter::Sub => oxipng::RowFilter::Sub,
            PngFilter::Up => oxipng::RowFilter::Up,
            PngFilter::Average => oxipng::RowFilter::Average,
            PngFilter::Paeth => oxipng::RowFilter::Paeth,
            PngFilter::MinSum => oxipng::RowFilter::MinSum,
            PngFilter::Entropy => oxipng::RowFilter::Entropy,
            PngFilter::Bigrams => oxipng::RowFilter::Bigrams,
            PngFilter::BigEnt => oxipng::RowFilter::BigEnt,
            PngFilter::Brute => oxipng::RowFilter::Brute,
        }
    }
}

/// PNGインターレース設定
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PngInterlace {
    /// インターレースなし（最小ファイルサイズ）
    None,
    /// Adam7インターレース（プログレッシブ読み込み）
    Adam7,
}

impl Default for PngInterlace {
    fn default() -> Self {
        Self::None
    }
}

impl From<PngInterlace> for oxipng::Interlacing {
    fn from(interlace: PngInterlace) -> Self {
        match interlace {
            PngInterlace::None => oxipng::Interlacing::None,
            PngInterlace::Adam7 => oxipng::Interlacing::Adam7,
        }
    }
}

/// PNG最適化オプション（OxiPNG専用）
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PngOptions {
    /// Zopfliの反復回数（15-255、高いほど高圧縮だが遅い）
    pub zopfli_iterations: u32,
    /// ICCプロファイルを含めるか
    pub embed_icc_profile: bool,
    /// ビット深度削減を有効にする
    pub bit_depth_reduction: bool,
    /// カラータイプ削減を有効にする（RGBA→RGB、RGB→Grayscaleなど）
    pub color_type_reduction: bool,
    /// パレット削減を有効にする
    pub palette_reduction: bool,
    /// インターレース設定
    pub interlace: PngInterlace,
    /// フィルター戦略
    pub filter: PngFilter,
}

impl Default for PngOptions {
    fn default() -> Self {
        Self {
            zopfli_iterations: 15, // Zopfliのデフォルト
            embed_icc_profile: true,
            bit_depth_reduction: true,
            color_type_reduction: true,
            palette_reduction: true,
            interlace: PngInterlace::None,
            filter: PngFilter::MinSum,
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

    // OxiPNGで最適化
    let oxipng_options = oxipng::Options {
        deflate: oxipng::Deflaters::Zopfli {
            iterations: std::num::NonZeroU8::new(options.zopfli_iterations.min(255) as u8).unwrap(),
        },
        optimize_alpha: true,
        strip: oxipng::StripChunks::Safe,
        bit_depth_reduction: options.bit_depth_reduction,
        color_type_reduction: options.color_type_reduction,
        palette_reduction: options.palette_reduction,
        interlace: Some(options.interlace.into()),
        filter: IndexSet::from([options.filter.into()]),
        ..Default::default()
    };

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
    use flate2::{Compression, write::DeflateEncoder};
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

/// Encode image to PNG format with progress callback support
///
/// # Arguments
/// - `img`: Source image to encode (HighBitDepthImage)
/// - `icc_profile`: ICC profile for color management
/// - `options`: PNG encoding options (PngOptions)
/// - `progress_callback`: Progress callback implementation
///
/// # Returns
/// - Success: PNG format byte data as Vec<u8>
/// - Failure: AppError
///
/// # Notes
/// - Progress reporting is approximate as PNG encoding is multi-stage
/// - Zopfli optimization stage provides the most granular progress updates
pub fn encode_with_progress(
    img: &HighBitDepthImage,
    icc_profile: Option<Vec<u8>>,
    options: &PngOptions,
    progress_callback: Arc<dyn ProgressCallback>,
) -> Result<Vec<u8>, AppError> {
    progress_callback.on_progress(0.0, "Starting PNG encoding");
    progress_callback.on_progress(10.0, "Preparing image data");

    let (width, height, color_type, bit_depth, data) = prepare_image_data(img)?;

    progress_callback.on_progress(20.0, "Writing PNG header and data");

    // まず標準的なPNGを生成
    let mut temp_buffer = Vec::new();
    let cursor = Cursor::new(&mut temp_buffer);

    let mut encoder = png::Encoder::new(cursor, width, height);
    encoder.set_color(color_type);
    encoder.set_depth(bit_depth);
    encoder.set_compression(png::Compression::Fast);

    let mut writer = encoder
        .write_header()
        .map_err(|e| AppError::Encode(format!("PNG header write error: {}", e)))?;

    // ICCプロファイル埋め込み
    if options.embed_icc_profile {
        if let Some(profile) = icc_profile {
            progress_callback.on_progress(30.0, "Embedding ICC profile");
            writer
                .write_chunk(png::chunk::iCCP, &create_iccp_chunk(&profile)?)
                .map_err(|e| AppError::Encode(format!("ICC profile embedding error: {}", e)))?;
        }
    }

    progress_callback.on_progress(40.0, "Writing image data");

    writer
        .write_image_data(&data)
        .map_err(|e| AppError::Encode(format!("PNG image data write error: {}", e)))?;

    writer
        .finish()
        .map_err(|e| AppError::Encode(format!("PNG encoding finish error: {}", e)))?;

    progress_callback.on_progress(60.0, "Optimizing with OxiPNG");

    // OxiPNGで最適化
    let oxipng_options = oxipng::Options {
        deflate: oxipng::Deflaters::Zopfli {
            iterations: std::num::NonZeroU8::new(options.zopfli_iterations.min(255) as u8).unwrap(),
        },
        filter: {
            let mut filters = IndexSet::new();
            filters.insert(options.filter.into());
            filters
        },
        interlace: Some(options.interlace.into()),
        bit_depth_reduction: options.bit_depth_reduction,
        color_type_reduction: options.color_type_reduction,
        palette_reduction: options.palette_reduction,
        ..Default::default()
    };

    progress_callback.on_progress(80.0, "Applying Zopfli compression");

    let optimized = optimize_from_memory(&temp_buffer, &oxipng_options).map_err(|e| {
        progress_callback.on_error(&e.to_string());
        AppError::Encode(format!("OxiPNG optimization error: {}", e))
    })?;

    progress_callback.on_progress(95.0, "Finalizing");

    println!(
        "PNG: Encoding complete - Original: {} bytes, Optimized: {} bytes",
        temp_buffer.len(),
        optimized.len()
    );

    progress_callback.on_complete();
    Ok(optimized)
}
