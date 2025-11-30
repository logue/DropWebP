/// Progress callback trait for encoding operations
///
/// This trait allows encoders to report progress during encoding.
/// Note: Not all encoders support progress reporting.
pub trait ProgressCallback: Send + Sync {
    /// Called when encoding progress updates
    ///
    /// # Arguments
    /// * `percent` - Progress percentage (0-100)
    /// * `stage` - Current encoding stage description
    fn on_progress(&self, percent: f32, stage: &str);

    /// Called when encoding completes successfully
    fn on_complete(&self);

    /// Called when encoding fails
    ///
    /// # Arguments
    /// * `error` - Error message
    fn on_error(&self, error: &str);
}

/// Progress event payload for Tauri frontend
#[derive(Clone, serde::Serialize)]
pub struct ProgressEvent {
    pub percent: f32,
    pub stage: String,
    pub status: ProgressStatus,
}

/// Progress status enumeration
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProgressStatus {
    Progress,
    Complete,
    Error,
}

/// Tauri progress callback that emits events to frontend
pub struct TauriProgressCallback {
    app: tauri::AppHandle,
    event_name: String,
}

impl TauriProgressCallback {
    pub fn new(app: tauri::AppHandle, event_name: String) -> Self {
        Self { app, event_name }
    }

    fn emit_event(&self, event: ProgressEvent) {
        // AppHandleからイベントを送信
        use tauri::Emitter;
        if let Err(e) = self.app.emit(&self.event_name, event) {
            eprintln!("Failed to emit progress event: {}", e);
        }
    }
}

impl ProgressCallback for TauriProgressCallback {
    fn on_progress(&self, percent: f32, stage: &str) {
        let event = ProgressEvent {
            percent,
            stage: stage.to_string(),
            status: ProgressStatus::Progress,
        };
        self.emit_event(event);
    }

    fn on_complete(&self) {
        let event = ProgressEvent {
            percent: 100.0,
            stage: "Complete".to_string(),
            status: ProgressStatus::Complete,
        };
        self.emit_event(event);
    }

    fn on_error(&self, error: &str) {
        let event = ProgressEvent {
            percent: 0.0,
            stage: error.to_string(),
            status: ProgressStatus::Error,
        };
        self.emit_event(event);
    }
}

/// Default no-op progress callback
pub struct NoOpProgressCallback;

impl ProgressCallback for NoOpProgressCallback {
    fn on_progress(&self, _percent: f32, _stage: &str) {}
    fn on_complete(&self) {}
    fn on_error(&self, _error: &str) {}
}

/// Console progress callback for debugging
pub struct ConsoleProgressCallback {
    format_name: String,
}

impl ConsoleProgressCallback {
    pub fn new(format_name: &str) -> Self {
        Self {
            format_name: format_name.to_string(),
        }
    }
}

impl ProgressCallback for ConsoleProgressCallback {
    fn on_progress(&self, percent: f32, stage: &str) {
        println!(
            "{}: Encoding progress - {:.1}% ({})",
            self.format_name, percent, stage
        );
    }

    fn on_complete(&self) {
        println!("{}: Encoding completed successfully", self.format_name);
    }

    fn on_error(&self, error: &str) {
        eprintln!("{}: Encoding failed - {}", self.format_name, error);
    }
}

/// Encoder support matrix for progress reporting
pub struct EncoderCapabilities;

impl EncoderCapabilities {
    /// Check if an encoder supports progress reporting
    pub fn supports_progress(format: &str) -> bool {
        matches!(format.to_lowercase().as_str(), "webp" | "png")
    }

    /// Get supported encoders list
    pub fn supported_encoders() -> Vec<&'static str> {
        vec!["webp", "png"]
    }

    /// Get unsupported encoders list
    pub fn unsupported_encoders() -> Vec<&'static str> {
        vec!["avif", "jpeg", "jxl", "jpeg2000"]
    }
}
