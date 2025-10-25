use crate::encoder::{
    avif::AvifOptions, jpeg::JpegOptions, jxl::JxlOptions, png::PngOptions, webp::WebpOptions,
};
use serde::{Deserialize, Serialize};

/// 高ビット深度画像を表す列挙型
/// RGB、RGBA、ARGBの3種類をサポート
/// f32型のピクセルデータを使用
/// 例: 16ビットや32ビットの画像データを扱う場合に使用
#[derive(Debug)]
#[allow(dead_code)]
pub enum HighBitDepthImage {
    Rgb(image::ImageBuffer<image::Rgb<f32>, Vec<f32>>),
    Rgba(image::ImageBuffer<image::Rgba<f32>, Vec<f32>>),
    Argb(image::ImageBuffer<image::Rgba<f32>, Vec<f32>>), // ARGB形式（内部的にはRgbaBufferとして保存、ピクセル順序はARGB）
}

/// ファイルパス情報
/// file_name: ファイル名 (拡張子含む)
/// extension: 拡張子 (ドット無し)
/// parent_dir: 親ディレクトリのパス
/// exists: パスが存在するか
/// is_file: ファイルであるか
/// is_dir: ディレクトリであるか
#[derive(serde::Serialize)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")] // JS側でキャメルケースになるように設定
pub struct PathInfo {
    pub(crate) file_name: Option<String>,
    pub(crate) extension: Option<String>,
    pub(crate) parent_dir: Option<String>,
    pub(crate) exists: Option<String>,
    pub(crate) is_file: Option<bool>,
    pub(crate) is_dir: Option<bool>,
}

/// 全てのエンコードオプションをまとめる親構造体
/// エンコード形式を一つだけ指定するための列挙型
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum EncodeOptions {
    Avif(AvifOptions),
    Webp(WebpOptions),
    Jxl(JxlOptions),
    Png(PngOptions),
    Jpeg(JpegOptions),
}
