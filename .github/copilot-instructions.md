# Drop Compress Image - AI Coding Agent Instructions

## Project Architecture

**Tauri v2 desktop app** with Vue 3 frontend + Rust backend for high-performance image conversion to modern formats (AVIF, JPEG XL, WebP, PNG, JPEG).

### Current Project Goal

**HDR Support**: Preserve HDR (High Dynamic Range) metadata and color information when converting to JPEG XL and AVIF formats. This includes:

- Maintaining ICC color profiles during decode/encode pipeline
- Supporting high bit-depth images (10-bit, 12-bit)
- Preserving HDR metadata (PQ, HLG transfer functions)
- Ensuring color space accuracy for wide gamut images
- _(Lower priority)_ JPEG HDR output using JPEG-LI with embedded HDR metadata

### Key Components

- **Frontend**: Vue 3 + TypeScript + Vuetify + Pinia (located in `app/src/`)
- **Backend**: Rust with Tauri v2 (located in `app/src-tauri/src/`)
- **Documentation site**: Nuxt 3 (located in `docs/`)
- **Workspace**: pnpm monorepo with 2 packages (`app/`, `docs/`)

### Rust Backend Structure

```
app/src-tauri/src/
├── lib.rs          # Public API exports (decode, encode, options, error, logging)
├── command.rs      # Tauri commands: convert(), convert_with_progress()
├── decoder.rs      # Image decoding (handles 15+ formats)
├── encoder.rs      # Image encoding (avif, jxl, webp, png, jpeg)
├── encoder/
│   ├── progress.rs # Progress callback trait + Tauri event emitter
│   ├── avif.rs     # AVIF encoding (libavif via codec-aom)
│   ├── jxl.rs      # JPEG XL encoding + JPEG transcode
│   ├── webp.rs     # WebP encoding with progress (lossy/lossless)
│   ├── png.rs      # PNG encoding via oxipng with progress
│   └── jpeg.rs     # JPEG encoding via jpegli
├── options.rs      # EncodeOptions enum + PathInfo
├── error.rs        # AppError type
└── logging.rs      # Logging system with ResultExt trait
```

### Frontend Structure

```
app/src/
├── composables/
│   ├── useImageConversionController.ts  # Main orchestrator
│   ├── useImageConverter.ts             # Invokes Tauri commands
│   ├── useDragAndDrop.ts                # Drag-and-drop handling
│   ├── usePaste.ts                      # Clipboard paste handling
│   ├── useFileSystem.ts                 # File operations
│   └── useConversionState.ts            # Progress state management
├── store/
│   ├── ConfigStore.ts    # Theme, locale
│   ├── GlobalStore.ts    # Loading, progress, messages
│   └── SettingsStore.ts  # Format options (persisted)
├── interfaces/           # TypeScript option interfaces
└── plugins/
    ├── i18n.ts          # Vue I18n with 5 languages
    └── vuetify.ts       # Material Design components
```

## Critical Workflows

### Development

```bash
# Install dependencies (first time)
pnpm install

# Run Tauri dev server (hot reload for both frontend and Rust)
pnpm run dev:tauri

# Run only frontend
pnpm run dev

# Type checking
pnpm run type-check

# Lint all
pnpm run lint
pnpm run lint:style
```

### Building

```bash
# Build Tauri app for current platform
pnpm run build:tauri

# macOS build (Universal binary for both Apple Silicon and Intel)
pnpm --filter app build:tauri:mac

# Linux via Docker
cd scripts/docker
./docker-build.sh x64    # or arm64

# Or from project root
./scripts/docker/docker-build.sh x64
./scripts/docker/docker-build.sh arm64

# Package managers
pnpm run package:chocolatey  # Windows
pnpm run package:homebrew    # macOS
```

### Testing Rust code

```bash
cd app/src-tauri
cargo test
cargo build --release
```

## Project-Specific Conventions

### Rust Code Patterns

1. **Image Decoding Architecture**: The decoder (`decoder.rs`) handles 15+ formats with specialized decoders in `decoder/` subdirectory:
   - **Standard formats**: PNG, JPEG, TIFF, BMP, etc. via `image` crate with ICC profile extraction
   - **AVIF**: Custom decoder in `decoder/avif.rs` using libavif, supports HDR/10-bit
   - **JPEG XL**: Custom decoder in `decoder/jxl.rs` using jpegxl-rs
   - **JPEG 2000**: Custom decoder in `decoder/jpeg2k.rs` using openjpeg-sys

   All decoders return `(HighBitDepthImage, Option<Vec<u8>>)` where the second value is ICC profile.

2. **HDR Color Management**: ICC color profiles are extracted during decoding and passed through to encoders. Both AVIF and JPEG XL encoders support embedding ICC profiles:

   ```rust
   let (img, icc_profile) = decoder::decode(&data)?;
   encoder::encode(img, icc_profile, &options)?;
   ```

   The `icc_profile` is `Option<Vec<u8>>` and must be preserved through the pipeline. For HDR images, this ensures color space information (Rec.2020, PQ/HLG transfer) is maintained.

3. **Progress Reporting**: Encoders implement `ProgressCallback` trait. WebP and PNG support progress via callbacks. Check `encoder/progress.rs` for the pattern.

4. **Error Handling**: Use `ResultExt` trait from `logging.rs` to auto-log errors:

   ```rust
   decoder::decode(&data)
       .log_error(Some("Image decoding"))
       .map_err(|e| format!("Failed: {}", e))?
   ```

5. **JPEG XL Transcode**: Fast path for JPEG→JXL without decode/encode. Check format first in `command.rs` convert functions.

6. **Module Exports**: `lib.rs` re-exports public API. Keep modules private, expose only what's needed.

### Frontend Patterns

1. **Composables Architecture**: Each concern is isolated (conversion, drag-drop, paste, filesystem). `useImageConversionController` orchestrates them.

2. **State Management**:
   - `SettingsStore`: User preferences (persisted to localStorage via pinia-plugin-persistedstate)
   - `ConfigStore`: Theme and locale
   - `GlobalStore`: Transient UI state (loading, progress, messages)

3. **Tauri Commands**: Call via `@tauri-apps/api/core`:

   ```typescript
   import { invoke } from '@tauri-apps/api/core';
   await invoke<Uint8Array>('convert', { data, options });
   ```

4. **Progress Events**: Listen via Tauri events:

   ```typescript
   import { listen } from '@tauri-apps/api/event';
   await listen('encoding-progress', event => {
     const { percent, stage, status } = event.payload;
   });
   ```

5. **i18n**: Use `vue-i18n` composable in components. Translation files are in `src/locales/*.yml`.

### Build Configuration

1. **macOS Compatibility**: Uses `edition = "2021"`, `lto = "thin"`, `codegen-units = 16` for cross-M1/M2/M3 compatibility. See `MACOS_COMPATIBILITY.md`.

2. **Static Linking**: All image libraries are statically linked. See `DEPENDENCY_ANALYSIS.md`.

3. **Cargo Profile**: Release profile optimizes for speed (`opt-level = 3`) with thin LTO for compatibility.

4. **Vite Config**: Uses path aliases `@/` for `src/`, file-based routing, and Vuetify auto-import.

## Integration Points

### Tauri ↔ Frontend Communication

- **Commands** (`command.rs`): `convert()`, `convert_with_progress()`
- **Events**: `encoding-progress` (emitted from Rust via `TauriProgressCallback`)
- **Plugins**: dialog, fs, notification, opener, os (via `@tauri-apps/plugin-*`)

### Image Processing Pipeline

```
Input File → Decoder (decoder.rs)
          ↓
   DynamicImage + ICC Profile
          ↓
   Format-specific encoder (encoder/*.rs)
          ↓
   Output bytes (Vec<u8>)
```

Special case: JPEG → JPEG XL uses `jxl::transcode()` to skip decode/encode.

### Platform-Specific Code

- **Windows**: Uses native APIs in `target.'cfg(windows)'.dependencies` for Windows-specific optimizations.
- **Linux**: Built via Docker with Debian Bookworm + WebKit2GTK. See [docs/content/ja/docker-build.md](../docs/content/ja/docker-build.md).
- **macOS Cross-compilation**: Uses `vendored` features for jpegxl-rs to enable x86_64 builds from Apple Silicon. See `INTEL_MAC_BUILD.md`.

## Key Files

- [app/src-tauri/src/command.rs](app/src-tauri/src/command.rs) - Tauri command implementations
- [app/src-tauri/src/encoder.rs](app/src-tauri/src/encoder.rs) - Main encode() function
- [app/src-tauri/src/decoder.rs](app/src-tauri/src/decoder.rs) - Image decoding logic
- [app/src/composables/useImageConversionController.ts](app/src/composables/useImageConversionController.ts) - Frontend orchestration
- [app/src-tauri/Cargo.toml](app/src-tauri/Cargo.toml) - Rust dependencies and build config
- [MACOS_COMPATIBILITY.md](MACOS_COMPATIBILITY.md) - Apple Silicon compatibility notes
- [INTEL_MAC_BUILD.md](INTEL_MAC_BUILD.md) - Intel Mac cross-compilation from Apple Silicon
- [DEPENDENCY_ANALYSIS.md](DEPENDENCY_ANALYSIS.md) - Static vs dynamic linking decisions

## Common Tasks

- **Add new image format**: Create encoder in `app/src-tauri/src/encoder/newformat.rs`, add to `EncodeOptions` enum in `options.rs`, implement in `encoder.rs` encode function, add TypeScript interface in `app/src/interfaces/`
- **Add progress support**: Implement `ProgressCallback` in encoder, check `webp.rs` or `png.rs` for reference pattern
- **Update UI strings**: Edit `app/src/locales/*.yml` for all languages (en, fr, ja, ko, zhHans, zhHant)
- **Fix build issues**:
  - Check `MACOS_COMPATIBILITY.md` for M1/M2/M3 compatibility
  - Check `INTEL_MAC_BUILD.md` for Intel Mac cross-compilation (requires YASM and jpegxl-rs vendored feature)
  - Check [docs/content/ja/docker-build.md](../docs/content/ja/docker-build.md) for Linux builds
- **Platform-specific code**: Use `#[cfg(target_os = "macos")]` or `cfg(windows)` in Rust, feature flags in Cargo.toml

## Important Notes

- **pnpm workspace**: Always use `pnpm --filter app <command>` or `pnpm --filter docs <command>` for package-specific operations
- **Rust edition**: Stick to edition 2024 (was 2021) for modern Rust features
- **Version management**: Version is in root `.env` file, synced to `Cargo.toml`, `package.json`, and `tauri.conf.json`
- **Cross-compilation**: jpegxl-rs uses `vendored` feature to enable cross-compilation without system library dependencies. This allows building x86_64 Mac binaries from Apple Silicon without installing x86_64 Homebrew.
- **No HEIC/HEIF support**: HEIC/HEIF format removed due to libheif-rs licensing constraints (LGPL-3.0). Modern iPhones capture in JPEG XL format (fully supported). For legacy HEIC files, users should convert via macOS Preview app (File → Export → JPEG) first. See `HEIC_REMOVAL_SUMMARY.md`.
