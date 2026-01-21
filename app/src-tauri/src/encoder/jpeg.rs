use crate::error::AppError;
use crate::options::HighBitDepthImage;
use jpegli_rs::encoder::{ChromaSubsampling, EncoderConfig, PixelLayout, Unstoppable};
use serde::{Deserialize, Serialize};

/// デフォルトの画質
const DEFAULT_QUALITY: u8 = 95;

/// JPEG (jpegli) エンコードオプション
///
/// jpegliは、libjxlプロジェクトに含まれる高品質なJPEGエンコーダーです。
/// 標準のJPEGエンコーダーよりも優れた圧縮率と画質を提供します。
/// JPEG XLと同じ技術を活用し、高品質なJPEG画像を生成します。
///
/// # Ultra HDR サポート (将来実装予定)
/// `ultra_hdr`オプションは将来のリリースでUltra HDR (JPEG-R)形式の
/// エンコードをサポートする予定です。
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JpegOptions {
    /// 画質 (1-100の範囲、デフォルト: 95)
    /// 85以上を推奨（高品質）
    pub quality: u8,

    /// プログレッシブJPEG（推奨）
    /// Webでの読み込みパフォーマンスが向上
    #[serde(default = "default_progressive")]
    pub progressive: bool,

    /// 最適化（推奨）
    /// ファイルサイズをさらに削減
    #[serde(default = "default_optimize")]
    pub optimize: bool,

    /// Ultra HDR (JPEG-R with gainmap) エンコード (将来実装予定)
    /// 現在は無視されます
    #[serde(default = "default_ultra_hdr")]
    pub ultra_hdr: bool,

    /// Ultra HDR Gainmapの画質 (1-100の範囲、デフォルト: 85)
    /// 将来実装予定
    #[serde(default = "default_gainmap_quality")]
    pub gainmap_quality: u8,
}

fn default_progressive() -> bool {
    true
}

fn default_optimize() -> bool {
    true
}

fn default_ultra_hdr() -> bool {
    false
}

fn default_gainmap_quality() -> u8 {
    85
}

impl Default for JpegOptions {
    fn default() -> Self {
        Self {
            quality: DEFAULT_QUALITY,
            progressive: true,
            optimize: true,
            ultra_hdr: false,
            gainmap_quality: 85,
        }
    }
}

/// 画像をJPEG (jpegli) 形式でエンコードします
///
/// # 引数
/// - `img`: 変換対象の高ビット深度画像
/// - `icc_profile`: ICCプロファイル（オプション）
/// - `options`: JPEGエンコードオプション
///
/// # 戻り値
/// - 成功した場合はエンコードされたJPEGデータを `Vec<u8>` として返します
/// - 失敗した場合は `AppError` を返します
///
/// # 注意
/// - jpegliは標準JPEGより高品質でJPEG XL技術を活用
/// - 透明度はサポートされません（RGBAはRGBに変換されます）
/// - 推奨画質範囲は85-100です
pub fn encode(
    img: &HighBitDepthImage,
    icc_profile: Option<Vec<u8>>,
    options: &JpegOptions,
) -> Result<Vec<u8>, AppError> {
    println!("Starting jpegli encoding...");
    println!(
        "Quality: {}, Progressive: {}, Optimize: {}",
        options.quality, options.progressive, options.optimize
    );

    // 画質の検証
    if options.quality < 85 {
        println!(
            "Warning: Quality {} is below recommended minimum of 85 for high-quality JPEG",
            options.quality
        );
    }

    // 画像の次元とRGBデータを取得
    let (width, height, rgb_data) = match img {
        HighBitDepthImage::Rgb(buf) => {
            let (w, h) = buf.dimensions();
            // f32からu8への変換
            let data: Vec<u8> = buf
                .as_raw()
                .iter()
                .map(|&x| (x.clamp(0.0, 1.0) * 255.0) as u8)
                .collect();
            (w, h, data)
        }
        HighBitDepthImage::Rgba(buf) | HighBitDepthImage::Argb(buf) => {
            let (w, h) = buf.dimensions();
            // RGBAをRGBに変換（透明度を破棄）
            println!("Warning: Converting RGBA to RGB (alpha channel will be discarded)");
            let mut rgb_data = Vec::with_capacity((w * h * 3) as usize);

            for pixel in buf.pixels() {
                let r = (pixel[0].clamp(0.0, 1.0) * 255.0) as u8;
                let g = (pixel[1].clamp(0.0, 1.0) * 255.0) as u8;
                let b = (pixel[2].clamp(0.0, 1.0) * 255.0) as u8;
                rgb_data.push(r);
                rgb_data.push(g);
                rgb_data.push(b);
            }

            (w, h, rgb_data)
        }
    };

    println!(
        "Image dimensions: {}x{}, data size: {} bytes",
        width,
        height,
        rgb_data.len()
    );

    // jpegli-rs 0.8 新API: quality と chroma subsampling が必須パラメータ
    let mut config = EncoderConfig::new(options.quality, ChromaSubsampling::Quarter);

    if options.progressive {
        config = config.progressive(true);
    }

    if options.optimize {
        config = config.optimize_huffman(true);
    }

    // ICCプロファイルを設定に追加（jpegli-rs 0.8ではネイティブサポート）
    if let Some(ref icc) = icc_profile {
        config = config.icc_profile(icc.clone());
    }

    println!("Encoding image data...");
    let mut encoder = config
        .encode_from_bytes(width, height, PixelLayout::Rgb8Srgb)
        .map_err(|e| AppError::Encode(format!("Failed to create JPEG encoder: {:?}", e)))?;

    encoder
        .push_packed(&rgb_data, Unstoppable)
        .map_err(|e| AppError::Encode(format!("Failed to push image data: {:?}", e)))?;

    let jpeg_data = encoder
        .finish()
        .map_err(|e| AppError::Encode(format!("Failed to finish encoding: {:?}", e)))?;

    println!("jpegli encoding completed: {} bytes", jpeg_data.len());

    Ok(jpeg_data)
}

/// エンコード済みJPEGデータにICCプロファイルを追加
///
/// JPEG形式では、ICCプロファイルはAPP2マーカー内に埋め込まれます
///
/// # 注意
/// jpegli-rs 0.8以降はネイティブでICCプロファイルをサポートするため、
/// この関数は後方互換性のために残されています
#[allow(dead_code)]
fn add_icc_profile(jpeg_data: Vec<u8>, icc: &[u8]) -> Result<Vec<u8>, AppError> {
    // JPEGマーカー: SOI(0xFFD8)の直後にAPP2マーカーを挿入
    if jpeg_data.len() < 2 || jpeg_data[0] != 0xFF || jpeg_data[1] != 0xD8 {
        return Err(AppError::Encode(
            "Invalid JPEG: missing SOI marker".to_string(),
        ));
    }

    // APP2マーカー (0xFFE2) + "ICC_PROFILE\0" + シーケンス番号
    const MAX_CHUNK_SIZE: usize = 65533 - 14; // 65535 - marker(2) - length(2) - "ICC_PROFILE\0"(12) - seq(2)
    let chunk_count = (icc.len() + MAX_CHUNK_SIZE - 1) / MAX_CHUNK_SIZE;

    let mut result = Vec::with_capacity(jpeg_data.len() + icc.len() + chunk_count * 18);

    // SOIマーカーをコピー
    result.extend_from_slice(&jpeg_data[0..2]);

    // ICCプロファイルをチャンク化してAPP2マーカーとして追加
    for (i, chunk) in icc.chunks(MAX_CHUNK_SIZE).enumerate() {
        result.push(0xFF); // マーカー開始
        result.push(0xE2); // APP2

        let seg_len = 2 + 12 + 2 + chunk.len(); // length(2) + "ICC_PROFILE\0"(12) + seq(2) + data
        result.push((seg_len >> 8) as u8);
        result.push(seg_len as u8);

        result.extend_from_slice(b"ICC_PROFILE\0"); // 識別子
        result.push((i + 1) as u8); // 現在のチャンク番号 (1-based)
        result.push(chunk_count as u8); // 総チャンク数

        result.extend_from_slice(chunk);
    }

    // 残りのJPEGデータをコピー
    result.extend_from_slice(&jpeg_data[2..]);

    Ok(result)
}

/// JPEGファイルサイズを推定
///
/// # 引数
/// - `img`: 変換対象の高ビット深度画像
/// - `options`: JPEGエンコードオプション
///
/// # 戻り値
/// 推定されるJPEGファイルサイズ（バイト単位）
///
/// # 注意
/// これは推定値であり、実際のファイルサイズとは異なる場合があります
pub fn estimate_size(img: &HighBitDepthImage, options: &JpegOptions) -> usize {
    let (width, height) = match img {
        HighBitDepthImage::Rgb(buf) => buf.dimensions(),
        HighBitDepthImage::Rgba(buf) | HighBitDepthImage::Argb(buf) => buf.dimensions(),
    };

    // 基本的なピクセル数（JPEGは常にRGBなので3チャンネル）
    let pixel_count = (width * height) as usize;
    let uncompressed_size = pixel_count * 3; // RGB = 3 bytes per pixel

    // 品質に基づく圧縮率の推定
    // JPEGの圧縮率は品質設定に大きく依存する
    let compression_ratio = match options.quality {
        95..=100 => 0.15, // 高品質: 85%圧縮
        85..=94 => 0.10,  // 標準品質: 90%圧縮
        70..=84 => 0.08,  // 中品質: 92%圧縮
        50..=69 => 0.06,  // 低品質: 94%圧縮
        _ => 0.04,        // 非常に低品質: 96%圧縮
    };

    // 最適化とプログレッシブの影響を考慮
    let optimization_factor = if options.optimize { 0.95 } else { 1.0 }; // 最適化で5%削減
    let progressive_factor = if options.progressive { 1.02 } else { 1.0 }; // プログレッシブで2%増加

    // 基本推定サイズ
    let base_size = (uncompressed_size as f64 * compression_ratio) as usize;

    // ファクターを適用
    let estimated_size = (base_size as f64 * optimization_factor * progressive_factor) as usize;

    // 最小サイズを保証（ヘッダーやメタデータのサイズを考慮）
    let min_size = 2048; // 最低2KBは必要
    estimated_size.max(min_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let options = JpegOptions::default();
        assert_eq!(options.quality, DEFAULT_QUALITY);
        assert!(options.progressive);
        assert!(options.optimize);
    }

    #[test]
    fn test_estimate_size_quality_variations() {
        use image::ImageBuffer;

        // テスト用の画像を作成（100x100 RGB）
        let img_buffer: ImageBuffer<image::Rgb<f32>, Vec<f32>> =
            ImageBuffer::from_raw(100, 100, vec![0.5f32; 100 * 100 * 3]).unwrap();
        let img = HighBitDepthImage::Rgb(img_buffer);

        // 高品質設定
        let high_quality_options = JpegOptions {
            quality: 95,
            progressive: true,
            optimize: true,
            ultra_hdr: true,
            gainmap_quality: 95,
        };
        let high_quality_size = estimate_size(&img, &high_quality_options);

        // 低品質設定
        let low_quality_options = JpegOptions {
            quality: 50,
            progressive: false,
            optimize: false,
            ultra_hdr: false,
            gainmap_quality: 50,
        };
        let low_quality_size = estimate_size(&img, &low_quality_options);

        // 低品質の方がファイルサイズが小さいことを確認
        assert!(low_quality_size < high_quality_size);

        // 最小サイズが保証されていることを確認
        assert!(high_quality_size >= 2048);
        assert!(low_quality_size >= 2048);
    }
}
