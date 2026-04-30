//! Library crate for the Drop Compress Image backend.
//!
//! Internal modules are kept private; only the items intentionally re-exported
//! below form the public API consumed by the binary crate and tests.

mod decoder;
mod encoder;
mod error;
mod logging;
mod options;

// Re-export the public API surface.
pub use decoder::IccProfileInfo;
pub use decoder::decode;
pub use encoder::encode;
pub use encoder::estimate_size;
pub use error::AppError;
pub use logging::{LogLevel, ResultExt, init_logging, send_log};

// Encode option types are exposed for cross-module use.
pub use options::EncodeOptions;
