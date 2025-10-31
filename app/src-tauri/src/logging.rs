use crate::error::AppError;
use std::sync::OnceLock;
use tauri::{AppHandle, Emitter};

// グローバルなAppHandleの保存
static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

/// ログレベル定義
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        }
    }
}

/// Initialize the logging system with AppHandle
pub fn init_logging(app_handle: AppHandle) {
    if APP_HANDLE.set(app_handle).is_err() {
        eprintln!("Warning: Logging system already initialized");
    }
}

/// Send log message to frontend
pub fn send_log(level: LogLevel, message: &str) {
    if let Some(app_handle) = APP_HANDLE.get() {
        let log_data = serde_json::json!({
            "level": level.as_str(),
            "message": message,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        if let Err(e) = app_handle.emit("log-message", &log_data) {
            eprintln!("Failed to send log message: {}", e);
        }
    } else {
        // Fallback to console if logging not initialized
        eprintln!("[{}] {}", level.as_str().to_uppercase(), message);
    }
}

/// Send log message with AppHandle (legacy compatibility)
pub fn send_log_with_handle(app_handle: &AppHandle, level: LogLevel, message: &str) {
    let log_data = serde_json::json!({
        "level": level.as_str(),
        "message": message,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    if let Err(e) = app_handle.emit("log-message", &log_data) {
        eprintln!("Failed to send log message: {}", e);
    }
}

/// Log AppError with automatic error-level logging
pub fn log_app_error(error: &AppError, context: Option<&str>) {
    let message = match context {
        Some(ctx) => format!("{}: {}", ctx, error),
        None => error.to_string(),
    };

    send_log(LogLevel::Error, &message);

    // Also log to console for debugging
    eprintln!("AppError: {}", message);
}

/// Log AppError with custom log level
#[allow(dead_code)]
pub fn log_app_error_with_level(error: &AppError, level: LogLevel, context: Option<&str>) {
    let message = match context {
        Some(ctx) => format!("{}: {}", ctx, error),
        None => error.to_string(),
    };

    send_log(level, &message);

    // Also log to console for debugging
    match level {
        LogLevel::Error => eprintln!("AppError: {}", message),
        LogLevel::Warn => println!("AppWarning: {}", message),
        LogLevel::Info => println!("AppInfo: {}", message),
        LogLevel::Debug => println!("AppDebug: {}", message),
    }
}

/// Convenience macros for logging
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::logging::send_log($crate::logging::LogLevel::Debug, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::logging::send_log($crate::logging::LogLevel::Info, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::logging::send_log($crate::logging::LogLevel::Warn, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::logging::send_log($crate::logging::LogLevel::Error, &format!($($arg)*))
    };
}

/// Extension trait for AppError to add logging capabilities
pub trait AppErrorExt {
    fn log(&self, context: Option<&str>) -> &Self;
    fn log_with_level(&self, level: LogLevel, context: Option<&str>) -> &Self;
}

impl AppErrorExt for AppError {
    fn log(&self, context: Option<&str>) -> &Self {
        log_app_error(self, context);
        self
    }

    fn log_with_level(&self, level: LogLevel, context: Option<&str>) -> &Self {
        log_app_error_with_level(self, level, context);
        self
    }
}

/// Result extension for convenient error logging
pub trait ResultExt<T, E> {
    fn log_error(self, context: Option<&str>) -> Self;
    fn log_error_with_level(self, level: LogLevel, context: Option<&str>) -> Self;
}

impl<T> ResultExt<T, AppError> for Result<T, AppError> {
    fn log_error(self, context: Option<&str>) -> Self {
        if let Err(ref error) = self {
            log_app_error(error, context);
        }
        self
    }

    fn log_error_with_level(self, level: LogLevel, context: Option<&str>) -> Self {
        if let Err(ref error) = self {
            log_app_error_with_level(error, level, context);
        }
        self
    }
}
