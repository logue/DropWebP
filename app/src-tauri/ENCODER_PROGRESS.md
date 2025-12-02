# Encoder Progress Monitoring

This document describes the progress monitoring functionality for image encoders.

## Overview

The encoder module now supports progress callbacks for monitoring encoding operations. This allows applications to display encoding progress to users in real-time.

## Supported Encoders

| Encoder | Progress Support | Notes |
|---------|-----------------|-------|
| **WebP** | ✅ Full Support | Uses libwebp's native progress hook API for lossy encoding. Lossless encoding provides coarse-grained updates only. |
| **PNG** | ✅ Approximate | Progress is estimated based on encoding stages (header, data, optimization). No fine-grained updates available. |
| AVIF (ravif) | ❌ Not Supported | The `ravif` crate does not provide progress callback APIs. |
| JPEG (mozjpeg) | ❌ Not Supported | The `mozjpeg` crate does not expose progress callbacks. |
| JPEG XL (jpegxl-rs) | ❌ Not Supported | The `jpegxl-rs` crate does not provide progress callback APIs. |
| JPEG 2000 | ❌ Not Supported | No progress API available. |

## Usage

### Progress Callback Trait

Implement the `ProgressCallback` trait to receive progress updates:

```rust
use std::sync::Arc;
use crate::encoder::progress::ProgressCallback;

struct MyProgressCallback;

impl ProgressCallback for MyProgressCallback {
    fn on_progress(&self, percent: f32, stage: &str) {
        println!("Progress: {:.1}% - {}", percent, stage);
    }

    fn on_complete(&self) {
        println!("Encoding completed!");
    }

    fn on_error(&self, error: &str) {
        eprintln!("Encoding failed: {}", error);
    }
}
```

### WebP with Progress

```rust
use crate::encoder::webp::encode_with_progress;
use crate::encoder::progress::ConsoleProgressCallback;

let callback = Arc::new(ConsoleProgressCallback::new("WebP"));
let result = encode_with_progress(
    &pixel_data,
    icc_profile,
    &webp_options,
    callback
)?;
```

### PNG with Progress

```rust
use crate::encoder::png::encode_with_progress;
use crate::encoder::progress::ConsoleProgressCallback;

let callback = Arc::new(ConsoleProgressCallback::new("PNG"));
let result = encode_with_progress(
    &img,
    icc_profile,
    &png_options,
    callback
)?;
```

## Progress Stages

### WebP Lossy Encoding
- 0-10%: Analysis
- 10-20%: Pixel data extraction
- 20-40%: Tone mapping (if HDR detected)
- 40-60%: Encoder configuration
- 60-90%: Encoding (with fine-grained updates from libwebp)
- 90-100%: Finalization and ICC profile embedding

### WebP Lossless Encoding
- 0-20%: Analysis and preparation
- 20-50%: Encoding (single-step, no intermediate progress)
- 50-90%: Complete
- 90-100%: Finalization

### PNG Encoding
- 0-10%: Starting
- 10-20%: Image data preparation
- 20-40%: PNG header and image data writing
- 40-60%: ICC profile embedding (if requested)
- 60-80%: OxiPNG optimization
- 80-95%: Zopfli compression
- 95-100%: Finalization

## Built-in Callbacks

### ConsoleProgressCallback
Logs progress to stdout/stderr:

```rust
let callback = Arc::new(ConsoleProgressCallback::new("WebP"));
```

### NoOpProgressCallback
No-op callback for testing or when progress monitoring is not needed:

```rust
let callback = Arc::new(NoOpProgressCallback);
```

## Checking Support

Use `EncoderCapabilities` to check if an encoder supports progress:

```rust
use crate::encoder::progress::EncoderCapabilities;

if EncoderCapabilities::supports_progress("webp") {
    // Use progress-enabled encoding
} else {
    // Fall back to standard encoding
}
```

## Implementation Details

### WebP Progress Hook

WebP's progress hook is called periodically during lossy encoding:

```c
// libwebp callback signature
int (*progress_hook)(int percent, const WebPPicture* picture);
```

We wrap this in a Rust-safe callback that extracts the user-provided `ProgressCallback` from the `WebPPicture.user_data` field.

### PNG Approximate Progress

PNG encoding with OxiPNG doesn't provide fine-grained progress updates. We estimate progress based on encoding stages:

1. Prepare image data (0-20%)
2. Write PNG with fast compression (20-60%)
3. Optimize with OxiPNG/Zopfli (60-95%)
4. Finalize (95-100%)

## Thread Safety

All `ProgressCallback` implementations must be `Send + Sync` since encoding may occur on background threads. Use `Arc` to share callbacks across threads:

```rust
let callback: Arc<dyn ProgressCallback> = Arc::new(MyCallback);
```

## Future Enhancements

Potential improvements for unsupported encoders:

- **AVIF**: Monitor if `ravif` adds progress callback support
- **JPEG XL**: Check if `jpegxl-rs` exposes progress APIs
- **JPEG**: Consider switching to `libjpeg-turbo` bindings with progress support

## Performance Impact

Progress callbacks add minimal overhead:

- **WebP**: Native libwebp hooks, negligible impact
- **PNG**: Stage-based updates, no encoding performance impact
- Callback execution time should be minimal to avoid blocking encoding

## Example: Tauri Integration

```rust
use tauri::Window;
use std::sync::Arc;

struct TauriProgressCallback {
    window: Window,
    event_name: String,
}

impl ProgressCallback for TauriProgressCallback {
    fn on_progress(&self, percent: f32, stage: &str) {
        let _ = self.window.emit(
            &self.event_name,
            serde_json::json!({
                "percent": percent,
                "stage": stage
            })
        );
    }

    fn on_complete(&self) {
        let _ = self.window.emit(&self.event_name, serde_json::json!({
            "percent": 100.0,
            "stage": "complete"
        }));
    }

    fn on_error(&self, error: &str) {
        let _ = self.window.emit("encoding-error", error);
    }
}
```
