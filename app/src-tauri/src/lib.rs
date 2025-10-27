// 1. encoder.rs と decoder.rs をプライベートなモジュールとして宣言
mod decoder;
mod encoder;
mod error; // errorモジュールなども同様
mod logging; // ログシステム
mod options; // optionsモジュールも同様

// 2. 各モジュールから、公開したい関数や型を "pub use" で再エクスポートする
pub use decoder::decode;
pub use encoder::encode;
pub use error::AppError;
pub use logging::{init_logging, send_log, AppErrorExt, LogLevel, ResultExt};

// オプションの型定義なども必要に応じて公開する
pub use options::EncodeOptions;
