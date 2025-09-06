use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Image decoding failed: {0}")]
    Decode(String),

    #[error("Image encoding failed: {0}")]
    Encode(String),

    // ravif::Errorを保持するためのバリアントを追加
    #[error("AVIF encoding error: {0}")]
    Ravif(#[from] ravif::Error), //

    #[error("Filesystem error: {0}")]
    Io(#[from] std::io::Error), // std::io::Errorから自動変換

    #[error("Unsupported format")]
    UnsupportedFormat,
}

/// Tauriコマンドは String を返す必要があるため、変換を実装
impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}
