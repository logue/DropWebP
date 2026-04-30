use crate::encoder::{
    avif::AvifOptions, jpeg::JpegOptions, jxl::JxlOptions, png::PngOptions, webp::WebpOptions,
};
use serde::{Deserialize, Serialize};

/// Enumeration of high bit-depth image variants.
///
/// Supports RGB, RGBA, and ARGB layouts using `f32` pixel data, allowing
/// callers to handle 16-bit or 32-bit source images uniformly.
#[derive(Debug)]
#[allow(dead_code)]
pub enum HighBitDepthImage {
    Rgb(image::ImageBuffer<image::Rgb<f32>, Vec<f32>>),
    Rgba(image::ImageBuffer<image::Rgba<f32>, Vec<f32>>),
    /// ARGB layout. Internally stored as an RGBA buffer with the channel order
    /// reinterpreted as ARGB.
    Argb(image::ImageBuffer<image::Rgba<f32>, Vec<f32>>),
}

/// File path metadata returned by the `get_path_info` command.
///
/// Fields:
/// - `file_name`: file name including extension
/// - `extension`: extension without the leading dot
/// - `parent_dir`: parent directory path
/// - `exists`: whether the path exists (stringified for JS interop)
/// - `is_file`: whether the path is a regular file
/// - `is_dir`: whether the path is a directory
#[derive(serde::Serialize)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")] // Render as camelCase on the JS side.
pub struct PathInfo {
    pub(crate) file_name: Option<String>,
    pub(crate) extension: Option<String>,
    pub(crate) parent_dir: Option<String>,
    pub(crate) exists: Option<String>,
    pub(crate) is_file: Option<bool>,
    pub(crate) is_dir: Option<bool>,
}

/// Tagged union of every supported encoder option set.
///
/// Exactly one variant is selected per encode invocation.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum EncodeOptions {
    Avif(AvifOptions),
    Webp(WebpOptions),
    Jxl(JxlOptions),
    Png(PngOptions),
    Jpeg(JpegOptions),
}
