use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Image decoding failed: {0}")]
    Decode(String),

    #[error("Image encoding failed: {0}")]
    Encode(String),

    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError), // image::ImageErrorから自動

    // libavif::Errorを保持するためのバリアントを追加
    #[error("AVIF encoding error: {0}")]
    Avif(String), // libavif::Errorは複雑なのでStringに変換

    #[error("JPEG XL encoding error: {0}")]
    Jxr(#[from] jpegxl_rs::EncodeError),

    #[error("Filesystem error: {0}")]
    Io(#[from] std::io::Error), // std::io::Errorから自動変換

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("System dependency not found: {0}")]
    DependencyNotFound(String),

    #[error("Command execution failed: {0}")]
    CommandFailed(String),

    #[error("Path conversion error")]
    PathConversion,

    #[error("Image conversion error")]
    ImageConversion,

    #[error("Image decoding error")]
    ImageDecoding,

    #[error("Image processing Error: {0}")] // TODO: image::ImageErrorと混同
    ImageProcessing(String),

    #[error("IO error: {0}")]
    IoError(#[source] std::io::Error),

    #[error("Image error: {0}")]
    ImageError(#[source] image::ImageError),

    #[cfg(target_os = "windows")]
    #[error("Windows API error: {0}")]
    WindowsError(String),
}

#[cfg(target_os = "windows")]
impl From<windows::core::Error> for AppError {
    fn from(error: windows::core::Error) -> Self {
        AppError::WindowsError(error.to_string())
    }
}

/// Tauriコマンドは String を返す必要があるため、変換を実装
impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}
