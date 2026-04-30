use thiserror::Error;

/// Application-level error type covering decode/encode failures, IO errors,
/// platform-specific failures and conversions to and from upstream library errors.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Image decoding failed: {0}")]
    Decode(String),

    #[error("Image encoding failed: {0}")]
    Encode(String),

    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError), // Auto-converted from `image::ImageError`.

    /// Wraps `libavif::Error` (kept as `String` since the source type is complex).
    #[error("AVIF encoding error: {0}")]
    Avif(String),

    /// Wraps `jxl-sys` errors as a `String`.
    #[error("JPEG XL encoding error: {0}")]
    Jxr(String),

    #[error("Filesystem error: {0}")]
    Io(#[from] std::io::Error), // Auto-converted from `std::io::Error`.

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

    // TODO: collapse with `Image` variant; currently kept for callers that
    // produce a custom message string instead of the upstream error.
    #[error("Image processing Error: {0}")]
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

/// Tauri commands return `String` errors, so provide a conversion from
/// `AppError` into the canonical string representation.
impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}
