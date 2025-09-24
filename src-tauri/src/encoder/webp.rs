use crate::{
    encoder::extract_pixel_data,
    error::AppError,
    options::HighBitDepthImage, // WebpOptionsはそのまま使う
};
use serde::{Deserialize, Serialize};
use webp::{Encoder, WebPMemory};

/// WebP形式のオプション
/// quality: 0-100 (0は最低品質、100は最高品質)
/// lossless: true/false (可逆圧縮を使うかどうか
/// method: 0-6 (0は高速、6は高品質)
/// autofilter: true/false (自動フィルタリングを使うかどうか)
/// hint: 画像のヒント (WebPImageHint列挙型)
/// 注意: losslessがtrueの場合、qualityは無視される)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WebpOptions {
    pub quality: f32,
    pub lossless: bool,
    // pub method: u8,
    // pub autofilter: bool,
    // pub hint: WebPImageHint,
    // pub preset: WebPPreset,
}

/*
/// WebPの画像ヒント
/// - Default: 標準的な用途
/// - Picture: 写真やリアルな画像向け
/// - Photo: 写真向け
/// - Graph: 図やイラスト向け
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum WebPImageHint {
    Default = libwebp_sys::WEBP_HINT_DEFAULT as isize,
    Picture = libwebp_sys::WEBP_HINT_PICTURE as isize,
    Photo = libwebp_sys::WEBP_HINT_PHOTO as isize,
    Graph = libwebp_sys::WEBP_HINT_GRAPH as isize,
    Last = libwebp_sys::WEBP_HINT_LAST as isize,
}
*/

/// 画像を WebP にエンコードします。
/// # 引数
/// - `img`: 変換対象の画像 (DynamicImage)
/// - `options`: WebPエンコードオプション (WebpOptions)
/// # 戻り値
/// - 成功した場合は WebP のバイト列を `Vec<u8>` として返します。
/// - 失敗した場合は `AppError` を返します。
/// # 注意
/// - `libwebp-sys` クレートを使用して WebP エンコードを行います。ビルド時に `libwebp` ライブラリがシステムにインストールされている必要があります。
pub fn encode(
    pixel_data: &HighBitDepthImage,
    icc_profile: Option<Vec<u8>>,
    options: &WebpOptions,
) -> Result<Vec<u8>, AppError> {
    // ★ 1. 画像サイズとf32ピクセルデータを取得
    let (width, height) = match pixel_data {
        HighBitDepthImage::Rgb(buf) => buf.dimensions(),
        HighBitDepthImage::Rgba(buf) => buf.dimensions(),
        HighBitDepthImage::Argb(buf) => buf.dimensions(),
    };
    let (pixels_f32, is_rgba) = extract_pixel_data(pixel_data);

    // ★ 2. f32 -> u8 にデノーマライズ
    // WebPの標準的なエンコーダーは8bit入力を基本とするため
    let pixels_u8: Vec<u8> = pixels_f32
        .iter()
        .map(|&p| (p * 255.0).round().clamp(0.0, 255.0) as u8)
        .collect();

    // ★ 3. RGB/RGBAに応じてエンコーダーを生成
    let encoder = if is_rgba {
        Encoder::from_rgba(&pixels_u8, width, height)
    } else {
        Encoder::from_rgb(&pixels_u8, width, height)
    };

    // ★ 4. オプションに応じてエンコード処理を呼び出し
    let webp_memory: WebPMemory = if options.lossless {
        // ロスレスエンコード
        encoder.encode_lossless()
    } else {
        // 非可逆エンコード (品質指定)
        encoder.encode(options.quality)
    };

    println!("Finished encoding WebP.");
    // ★ 2. ICCプロファイルがなければ、エンコードしたピクセルデータのみを返す
    if icc_profile.is_none() {
        return Ok(webp_memory.to_vec());
    };
    let profile = icc_profile;

    /*

    // ★ 3. Muxerを使ってICCプロファイルを結合
    let mut mux = WebPMux::new();
    let frame = WebPData {
        bytes: webp_memory.as_ptr(),
        size: webp_memory.len(),
    };
    mux.push_frame(frame, None)?;
    mux.set_chunk("ICCP", &profile)?; // "ICCP"チャンクとして設定

    let final_data = mux.encode()?;
    */

    Ok(webp_memory.to_vec())
}
