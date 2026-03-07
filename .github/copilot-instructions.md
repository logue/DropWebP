# Tauri Vue3 App - AI Coding Agent Instructions

## Project Architecture

**Tauri v2 desktop app** with Vue 3 frontend + Rust backend for building cross-platform desktop applications.

### Key Components

- **Frontend**: Vue 3 + TypeScript + Vuetify + Pinia (located in `frontend/src/`)
- **Backend**: Rust with Tauri v2 (located in `backend/src/`)
- **Workspace**: pnpm monorepo

### Rust Backend Structure

```
backend/src/
├── lib.rs          # Public API exports (error, logging)
├── main.rs         # Tauri app entry point with plugin initialization
├── command.rs      # Tauri commands (sample commands for demonstration)
├── error.rs        # AppError type with common error variants
└── logging.rs      # Logging system with ResultExt trait
```

### Frontend Structure

```
frontend/src/
├── components/
│   └── MainContent.vue        # Main application component
├── composables/
│   ├── useFileSystem.ts       # File operations (open/save dialogs)
│   ├── useLogger.ts           # Frontend logging
│   └── useNotification.ts     # System notifications
├── store/
│   ├── ConfigStore.ts         # Theme, locale
│   └── GlobalStore.ts         # Loading, progress, messages
├── locales/                   # i18n translation files (6 languages)
└── plugins/
    ├── i18n.ts               # Vue I18n configuration
    └── vuetify.ts            # Material Design components
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
pnpm --filter frontend build:tauri:mac

# Linux via Docker
./scripts/docker/docker-build.sh x64    # or arm64

# Package managers
pnpm run package:chocolatey  # Windows
pnpm run package:homebrew    # macOS
```

### Testing Rust code

```bash
cd backend
cargo test
cargo build --release
```

## Project-Specific Conventions

### Rust Code Patterns

1. **Tauri Commands**: Async functions with `#[tauri::command]` attribute:

   ```rust
   #[tauri::command]
   async fn my_command(app: AppHandle, param: String) -> Result<String, String> {
       // Implementation
       Ok(result)
   }
   ```

   Register commands in `main.rs`:

   ```rust
   tauri::Builder::default()
       .invoke_handler(tauri::generate_handler![my_command])
       .run(tauri::generate_context!())
   ```

2. **Error Handling**: Use `ResultExt` trait from `logging.rs` to auto-log errors:

   ```rust
   some_operation()
       .log_error(Some("Operation name"))
       .map_err(|e| format!("Failed: {}", e))?
   ```

3. **Module Exports**: `lib.rs` re-exports public API. Keep modules private, expose only what's needed.

### Frontend Patterns

1. **Composables Architecture**: Each concern is isolated (file operations, logging, notifications). Reusable logic as Vue composables.

2. **State Management**:
   - `ConfigStore`: Theme and locale (persisted to localStorage via pinia-plugin-persistedstate)
   - `GlobalStore`: Transient UI state (loading, progress, messages)

3. **Tauri Commands**: Call via `@tauri-apps/api/core`:

   ```typescript
   import { invoke } from "@tauri-apps/api/core";
   const result = await invoke<string>("my_command", { param: "value" });
   ```

4. **Tauri Events**: Listen for events from Rust backend:

   ```typescript
   import { listen } from "@tauri-apps/api/event";
   await listen("my-event", (event) => {
     console.log("Received:", event.payload);
   });
   ```

5. **i18n**: Use `vue-i18n` composable in components. Translation files are in `src/locales/*.yml` (6 languages: en, ja, fr, ko, zhHans, zhHant).

### Build Configuration

1. **macOS Compatibility**: Uses `edition = "2024"`, `lto = "thin"`, `codegen-units = 16` for cross-Apple Silicon compatibility.

2. **Cargo Profile**: Release profile optimizes for speed (`opt-level = 3`) with thin LTO for compatibility.

3. **Vite Config**: Uses path aliases `@/` for `src/`, file-based routing, and Vuetify auto-import.

4. **Environment Variables**: All application metadata is configured via `.env` file (version, app name, author, URLs, etc.).

## Integration Points

### Tauri ↔ Frontend Communication

- **Commands** (`command.rs`): Sample commands: `echo_message()`, `get_app_version()`, `process_data()`
- **Plugins**: dialog, fs, notification, opener, os (via `@tauri-apps/plugin-*`)

### Platform-Specific Code

- **Windows**: Uses native APIs in `target.'cfg(windows)'.dependencies` for Windows-specific optimizations.
- **Linux**: Built via Docker with Debian Bookworm + WebKit2GTK.
- **macOS**: Supports Universal binaries (Apple Silicon + Intel).

## Key Files

- [backend/src/command.rs](backend/src/command.rs) - Tauri command implementations (add your commands here)
- [backend/src/main.rs](backend/src/main.rs) - Application entry point
- [frontend/src/components/MainContent.vue](frontend/src/components/MainContent.vue) - Main UI component
- [backend/Cargo.toml](backend/Cargo.toml) - Rust dependencies and build config
- [.env](.env) - Application configuration (version, name, author, URLs)

## Common Tasks

- **Add new Tauri command**: Add function to `backend/src/command.rs`, register in `main.rs`
- **Add UI string**: Edit `frontend/src/locales/*.yml` for all languages (en, ja, fr, ko, zhHans, zhHant)
- **Add composable**: Create new file in `frontend/src/composables/` following existing patterns
- **Update configuration**: Edit `.env` file with your app name, version, URLs, etc.
- **Platform-specific code**: Use `#[cfg(target_os = "macos")]` or `cfg(windows)` in Rust

## Important Notes

- **pnpm workspace**: Always use `pnpm --filter frontend <command>` for app-specific operations
- **Rust edition**: Uses edition 2024 for modern Rust features
- **Version management**: Version is in root `.env` file, synced to `Cargo.toml`, `package.json`, and `tauri.conf.json`
- **Package managers**: Build scripts generate `.nuspec` (Chocolatey) and `.rb` (Homebrew) files dynamically from `.env` configuration
- **Template system**: All app-specific values are in `.env` - edit this file when creating a new project from this template
