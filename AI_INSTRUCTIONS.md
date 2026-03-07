# AI Agent Instructions: Tauri v2 + Vue 3 Template

**Target Audience:** AI Coding Agents (GitHub Copilot, Claude, GPT-4, etc.)
**Purpose:** Generic Tauri v2 + Vue 3 desktop application template for building cross-platform desktop applications.

---

## 🎯 Mission Overview

Transform this specialized image conversion application into a reusable template by:

1. Removing all image processing logic (Rust decoders/encoders)
2. Removing image-specific UI components and composables
3. Replacing with generic sample code
4. Updating documentation and metadata

---

## 📋 Prerequisites Check

Before starting, verify:

- [ ] Current working directory is the project root
- [ ] `pnpm` is installed (version >= 10)
- [ ] `cargo` (Rust) is installed (version >= 1.93.1)
- [ ] Bash shell is available
- [ ] Write permissions in parent directory (for output)

---

## 🚀 Execution Methods

### Method 1: Automated Script (Recommended)

```bash
# Execute the template creation script
./scripts/create-template.sh <output-directory>

# Example:
./scripts/create-template.sh ../tauri-vue3-template
```

**Script behavior:**

- Copies entire project to output directory
- Removes all image processing files
- Generates generic sample code
- Updates configuration files
- Cleans build artifacts

**After script completion:**

```bash
cd <output-directory>
pnpm install
pnpm run dev:tauri  # Verify template works
```

### Method 2: Manual Execution (For Granular Control)

Follow the step-by-step instructions in sections below.

---

## 🗑️ DELETION TARGET LIST

### Rust Backend Files (backend/)

**Complete deletion:**

```
src/decoder.rs
src/encoder.rs
src/options.rs
src/encoder/avif.rs
src/encoder/jxl.rs
src/encoder/webp.rs
src/encoder/png.rs
src/encoder/jpeg.rs
src/encoder/progress.rs
src/decoder/avif.rs      (if exists)
src/decoder/jxl.rs       (if exists)
src/decoder/jpeg2k.rs    (if exists)
ENCODER_PROGRESS.md
```

**Cargo.toml - Remove these dependencies:**

```toml
image = "0.25.9"
imgref = "1.12.0"
jpeg2k = "0.10.1"
jpegli_rs = { ... }
jxl-sys = "0.1.5"
libavif-sys = { ... }
libwebp-sys = "0.14.2"
openjpeg-sys = "1.0.12"
oxipng = "10.1.0"
png = "0.18.1"
rgb = "0.8.52"
lcms2 = "6.1.1"
kamadak-exif = "0.6.1"
bytemuck = "1.25.0"
flate2 = "1.1.9"
```

**Keep these dependencies:**

```toml
serde = "1.0.228"
serde_json = "1.0.149"
thiserror = "2.0.18"
jiff = { version = "0.2.20", features = ["serde"] }
tempfile = "3.25.0"
tauri = { version = "2.10.2", features = [] }
tauri-plugin-dialog = "2.6.0"
tauri-plugin-fs = "2.4.5"
tauri-plugin-log = "2.8.0"
tauri-plugin-notification = "2.3.3"
tauri-plugin-opener = "2.5.3"
tauri-plugin-os = "2.3.2"
```

### Vue Frontend Files (frontend/src/)

**Complete deletion:**

```
composables/useImageConverter.ts
composables/useImageConversionController.ts
composables/useConversionState.ts
composables/useDragAndDrop.ts
composables/usePaste.ts
composables/useFormatConfig.ts

interfaces/AvifOptions.ts
interfaces/JpegOptions.ts
interfaces/JxlOptions.ts
interfaces/PngOptions.ts
interfaces/WebpOptions.ts
interfaces/CommonOptions.ts
interfaces/EncodeOptions.ts
interfaces/PathInfo.ts

types/AvifTypes.ts
types/JxlTypes.ts
types/ProgressEvent.ts
types/SettingsTypes.ts
types/FolderType.ts

store/SettingsStore.ts

assets/sounds/
```

**Keep these files (but may need modification):**

```
components/AppBarMenuComponent.vue
components/LocaleSelector.vue
components/MainContent.vue          (REPLACE CONTENT)

composables/useFileSystem.ts        (SIMPLIFY)
composables/useLogger.ts
composables/useNotification.ts

interfaces/MetaInterface.ts

store/ConfigStore.ts
store/GlobalStore.ts
store/index.ts

locales/*.yml                       (UPDATE CONTENT)

plugins/i18n.ts
plugins/vuetify.ts

App.vue
main.ts
```

### Project Root Files

**Complete deletion:**

```
ARM64_SIGNING_ISSUE.md
HDR_SUPPORT_STATUS.md
DEV_AUTORELOAD_FIX.md
backend/ENCODER_PROGRESS.md
```

**Keep but update:**

```
ReadMe.md              (REPLACE)
package.json           (UPDATE metadata)
.env                   (SET VERSION=1.0.0)
```

---

## 📝 FILE GENERATION TASKS

### 1. Rust: src-tauri/Cargo.toml

**REPLACE entire file with:**

```toml
[package]
name = "tauri-vue3-app"
version = "1.0.0"
authors = ["Your Name <your@email.com>"]
edition = "2024"
rust-version = "1.93.1"
description = "A modern desktop application built with Tauri v2 and Vue 3"
readme = "../../ReadMe.md"
repository = "https://github.com/yourname/your-app-name"
homepage = "https://yourdomain.com/your-app-name"
license-file = "../../LICENSE"
keywords = ["application", "desktop", "tauri", "vue"]
categories = ["gui"]

[lib]
name = "tauri_vue3_app_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2.5.3", features = [] }

[dependencies]
serde = "1.0.228"
serde_json = "1.0.149"
thiserror = "2.0.18"
jiff = { version = "0.2.20", features = ["serde"] }
tempfile = "3.25.0"

tauri = { version = "2.10.2", features = [] }
tauri-plugin-dialog = "2.6.0"
tauri-plugin-fs = "2.4.5"
tauri-plugin-log = "2.8.0"
tauri-plugin-notification = "2.3.3"
tauri-plugin-opener = "2.5.3"
tauri-plugin-os = "2.3.2"

[profile.release]
lto = "thin"
codegen-units = 16
panic = "abort"
strip = true
opt-level = 3
```

### 2. Rust: src-tauri/src/lib.rs

**REPLACE entire file with:**

```rust
//! Tauri Vue3 App Library
//!
//! A modern desktop application built with Tauri v2 and Vue 3.

mod error;
mod logging;

pub use error::AppError;
pub use logging::{LogLevel, ResultExt, init_logging, send_log};
```

### 3. Rust: src-tauri/src/command.rs

**REPLACE entire file with:**

```rust
//! Tauri Command Handlers
//!
//! This module contains all Tauri commands that can be invoked from the frontend.

use crate::error::AppError;
use crate::logging::{LogLevel, send_log_with_handle};
use tauri::AppHandle;

/// Example command: Echo back the input message
///
/// # Arguments
/// * `message` - The message to echo
/// * `app` - Tauri application handle
///
/// # Returns
/// * `Ok(String)` - The echoed message
/// * `Err(String)` - Error message if something goes wrong
#[tauri::command]
pub async fn echo_message(
    message: String,
    app: AppHandle,
) -> Result<String, String> {
    send_log_with_handle(&app, LogLevel::Info, &format!("Received message: {}", message));

    // Simulate some async work
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let result = format!("Echo: {}", message);
    send_log_with_handle(&app, LogLevel::Info, "Message echoed successfully");

    Ok(result)
}

/// Get the application version
///
/// # Returns
/// * `Ok(String)` - The application version
#[tauri::command]
pub async fn get_app_version() -> Result<String, String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}

/// Example command: Process some data
///
/// # Arguments
/// * `data` - Input data to process
/// * `options` - Processing options (example)
/// * `app` - Tauri application handle
///
/// # Returns
/// * `Ok(String)` - Processed result
/// * `Err(String)` - Error message
#[tauri::command]
pub async fn process_data(
    data: String,
    options: Option<serde_json::Value>,
    app: AppHandle,
) -> Result<String, String> {
    send_log_with_handle(&app, LogLevel::Info, "Processing data...");

    // Example processing logic
    let result = if let Some(opts) = options {
        format!("Processed '{}' with options: {}", data, opts)
    } else {
        format!("Processed '{}' with default options", data)
    };

    send_log_with_handle(&app, LogLevel::Info, "Processing complete");
    Ok(result)
}
```

### 4. Rust: src-tauri/src/main.rs

**REPLACE entire file with:**

```rust
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;

use tauri_vue3_app_lib::{init_logging, LogLevel};

fn main() {
    // Initialize logging system
    init_logging();

    tauri::Builder::default()
        // Initialize Tauri plugins
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        // Register command handlers
        .invoke_handler(tauri::generate_handler![
            command::echo_message,
            command::get_app_version,
            command::process_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 5. Rust: src-tauri/tauri.conf.json

**REPLACE entire file with:**

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "tauri-vue3-app",
  "mainBinaryName": "Tauri Vue3 App",
  "version": "1.0.0",
  "identifier": "com.example.tauri-vue3-app",
  "build": {
    "beforeDevCommand": "pnpm dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "pnpm build-only",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "Tauri Vue3 App",
        "width": 1000,
        "height": 700,
        "minWidth": 800,
        "minHeight": 600,
        "resizable": true,
        "fullscreen": false,
        "transparent": false
      }
    ],
    "security": {
      "csp": null
    },
    "macOSPrivateApi": false
  },
  "bundle": {
    "publisher": "Your Name",
    "active": true,
    "category": "Utility",
    "shortDescription": "A modern desktop application template",
    "longDescription": "A modern desktop application template built with Tauri v2 and Vue 3, featuring TypeScript, Vuetify, Pinia, and i18n support.",
    "targets": ["dmg", "msi", "deb", "rpm", "appimage"],
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "macOS": {
      "bundleName": "Tauri Vue3 App",
      "minimumSystemVersion": "11.0"
    },
    "windows": {
      "webviewInstallMode": {
        "type": "downloadBootstrapper"
      },
      "nsis": {
        "displayLanguageSelector": true
      }
    }
  }
}
```

### 6. Vue: src/components/MainContent.vue

**REPLACE entire file with:**

```vue
<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useGlobalStore } from "@/store";
import { useNotification } from "@/composables/useNotification";

const globalStore = useGlobalStore();
const notification = useNotification();

const inputText = ref("");
const outputText = ref("");
const appVersion = ref("");

const handleProcess = async () => {
  if (!inputText.value) {
    notification.error("Please enter some text");
    return;
  }

  globalStore.setLoading(true);

  try {
    const result = await invoke<string>("echo_message", {
      message: inputText.value,
    });
    outputText.value = result;
    notification.success("Processed successfully");
  } catch (error) {
    notification.error(`Error: ${error}`);
  } finally {
    globalStore.setLoading(false);
  }
};

const getVersion = async () => {
  try {
    appVersion.value = await invoke<string>("get_app_version");
  } catch (error) {
    console.error("Failed to get version:", error);
  }
};

// Get version on mount
getVersion();
</script>

<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12">
        <v-card>
          <v-card-title class="text-h5">
            <v-icon icon="mdi-application" class="mr-2" />
            Sample Application
          </v-card-title>

          <v-card-subtitle v-if="appVersion"
            >Version: {{ appVersion }}</v-card-subtitle
          >

          <v-card-text>
            <p class="text-body-1 mb-4">
              This is a template application built with Tauri v2 and Vue 3.
              Replace this content with your own application logic.
            </p>

            <v-divider class="mb-4" />

            <v-textarea
              v-model="inputText"
              label="Input Text"
              placeholder="Enter some text to echo..."
              rows="4"
              variant="outlined"
              class="mb-4"
            />

            <v-btn
              color="primary"
              size="large"
              prepend-icon="mdi-send"
              @click="handleProcess"
            >
              Process
            </v-btn>

            <v-textarea
              v-model="outputText"
              label="Output"
              rows="4"
              readonly
              variant="outlined"
              class="mt-4"
            />
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>

    <v-row class="mt-4">
      <v-col cols="12" md="6">
        <v-card>
          <v-card-title>
            <v-icon icon="mdi-information" class="mr-2" />
            Features
          </v-card-title>
          <v-card-text>
            <v-list dense>
              <v-list-item prepend-icon="mdi-vuejs">
                <v-list-item-title>Vue 3 Composition API</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-language-typescript">
                <v-list-item-title>TypeScript Support</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-material-design">
                <v-list-item-title>Vuetify 3 Material Design</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-database">
                <v-list-item-title>Pinia State Management</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-translate">
                <v-list-item-title>Vue I18n Multi-language</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-lightning-bolt">
                <v-list-item-title>Vite Fast Build</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-card-text>
        </v-card>
      </v-col>

      <v-col cols="12" md="6">
        <v-card>
          <v-card-title>
            <v-icon icon="mdi-cog" class="mr-2" />
            Quick Start
          </v-card-title>
          <v-card-text>
            <v-list dense>
              <v-list-item>
                <v-list-item-title class="text-body-2">
                  1. Customize
                  <code>MainContent.vue</code>
                </v-list-item-title>
              </v-list-item>
              <v-list-item>
                <v-list-item-title class="text-body-2">
                  2. Add Tauri commands in
                  <code>command.rs</code>
                </v-list-item-title>
              </v-list-item>
              <v-list-item>
                <v-list-item-title class="text-body-2">
                  3. Create composables for your logic
                </v-list-item-title>
              </v-list-item>
              <v-list-item>
                <v-list-item-title class="text-body-2">
                  4. Add stores for state management
                </v-list-item-title>
              </v-list-item>
              <v-list-item>
                <v-list-item-title class="text-body-2">
                  5. Update i18n translations
                </v-list-item-title>
              </v-list-item>
            </v-list>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<style scoped>
code {
  background-color: rgba(0, 0, 0, 0.05);
  padding: 2px 6px;
  border-radius: 4px;
  font-family: "Courier New", monospace;
}
</style>
```

### 7. Vue: src/composables/useFileSystem.ts

**UPDATE to generic file operations:**

```typescript
import { open, save } from "@tauri-apps/plugin-dialog";
import { readFile, writeFile, exists } from "@tauri-apps/plugin-fs";

export function useFileSystem() {
  /**
   * Open file selection dialog
   */
  const selectFiles = async (options?: {
    multiple?: boolean;
    filters?: Array<{ name: string; extensions: string[] }>;
  }) => {
    return await open({
      multiple: options?.multiple ?? false,
      filters: options?.filters,
    });
  };

  /**
   * Open folder selection dialog
   */
  const selectFolder = async () => {
    return await open({
      directory: true,
    });
  };

  /**
   * Open save file dialog
   */
  const saveFile = async (options?: {
    defaultPath?: string;
    filters?: Array<{ name: string; extensions: string[] }>;
  }) => {
    return await save({
      defaultPath: options?.defaultPath,
      filters: options?.filters,
    });
  };

  /**
   * Read file contents
   */
  const readFileContents = async (path: string) => {
    return await readFile(path);
  };

  /**
   * Write file contents
   */
  const writeFileContents = async (path: string, data: Uint8Array | string) => {
    return await writeFile(path, data);
  };

  /**
   * Check if file exists
   */
  const fileExists = async (path: string) => {
    return await exists(path);
  };

  return {
    selectFiles,
    selectFolder,
    saveFile,
    readFileContents,
    writeFileContents,
    fileExists,
  };
}
```

### 8. Root: package.json

**UPDATE metadata sections:**

```json
{
  "name": "tauri-vue3-template",
  "description": "A modern desktop application template built with Tauri v2 and Vue 3",
  "license": "MIT",
  "type": "module",
  "private": true,
  "author": {
    "name": "Your Name",
    "email": "your@email.com",
    "url": "https://yourdomain.com/"
  },
  "homepage": "https://yourdomain.com/your-app-name",
  "repository": {
    "type": "git",
    "url": "git@github.com:yourname/your-app-name.git"
  }
}
```

### 9. Root: .env

**SET version:**

```
VERSION=1.0.0
```

### 10. Vue: src/locales/en.yml

**REPLACE with generic messages:**

```yaml
app:
  title: "Tauri Vue3 App"
  description: "A modern desktop application"

menu:
  file: "File"
  open: "Open"
  save: "Save"
  quit: "Quit"
  edit: "Edit"
  preferences: "Preferences"
  view: "View"
  help: "Help"
  about: "About"
  documentation: "Documentation"

message:
  success: "Operation successful"
  error: "An error occurred"
  processing: "Processing..."
  loading: "Loading..."
  saved: "Saved successfully"
  cancelled: "Operation cancelled"

button:
  ok: "OK"
  cancel: "Cancel"
  save: "Save"
  close: "Close"
  apply: "Apply"

label:
  input: "Input"
  output: "Output"
  settings: "Settings"
  language: "Language"
  theme: "Theme"

theme:
  light: "Light"
  dark: "Dark"
  auto: "Auto"
```

**REPLICATE for other languages:**

- `fr.yml` (French)
- `ja.yml` (Japanese)
- `ko.yml` (Korean)
- `zhHans.yml` (Simplified Chinese)
- `zhHant.yml` (Traditional Chinese)

### 11. Root: ReadMe.md

**Use content from TEMPLATE_GUIDE.md section "新しい`ReadMe.md`の例"**

---

## ✅ VERIFICATION CHECKLIST

After completing all tasks, verify:

### Build Verification

```bash
# Rust compilation
cd backend
cargo build
# Should succeed without image processing crate errors

# Frontend type check
cd ../..
pnpm run type-check
# Should pass without errors

# Development server
pnpm run dev:tauri
# Should launch application with sample UI
```

### File System Verification

```bash
# Ensure all image processing files are deleted
! test -f backend/src/decoder.rs
! test -f backend/src/encoder.rs
! test -f frontend/src/composables/useImageConverter.ts

# Ensure generic files exist
test -f backend/src/command.rs
test -f frontend/src/components/MainContent.vue
test -f frontend/src/composables/useFileSystem.ts
```

### Functionality Verification

1. Application launches successfully
2. Theme toggle works (dark/light)
3. Language selector works
4. Sample "Echo" command works
5. No console errors related to missing files

### Metadata Verification

```bash
# Check version is updated
grep "VERSION=1.0.0" .env
grep '"version": "1.0.0"' backend/Cargo.toml
grep '"version": "1.0.0"' backend/tauri.conf.json
```

---

## 🔧 TROUBLESHOOTING

### Build Errors

**Error: "cannot find crate `image`"**

- Solution: Ensure `Cargo.toml` has been updated to remove image processing crates

**Error: "cannot find module `decoder`"**

- Solution: Ensure `src/lib.rs` has been updated to remove decoder/encoder imports

**Error: "unresolved import `useImageConverter`"**

- Solution: Ensure Vue files importing deleted composables have been updated

### Runtime Errors

**Error: "command not found: convert"**

- Solution: Ensure `src/main.rs` has been updated with new command handlers

**Error: "Module not found: @/composables/useImageConverter"**

- Solution: Check all Vue component imports and remove references to deleted composables

---

## 📊 SUCCESS CRITERIA

The template creation is successful when:

1. ✅ All image processing files deleted
2. ✅ All image processing dependencies removed from Cargo.toml
3. ✅ Generic sample code generated and working
4. ✅ Application builds without errors (`cargo build`)
5. ✅ TypeScript compiles without errors (`pnpm run type-check`)
6. ✅ Development server runs (`pnpm run dev:tauri`)
7. ✅ Sample UI displays and functions correctly
8. ✅ Documentation updated (ReadMe.md, package.json)
9. ✅ Version set to 1.0.0 in all relevant files

---

## 🎯 FINAL OUTPUT

After successful completion, the template should:

- Be a fully functional Tauri v2 + Vue 3 starter project
- Have NO image processing business logic
- Include working sample code demonstrating:
  - Tauri command invocation
  - State management with Pinia
  - Multi-language support
  - Theme switching
  - File system operations (generic)
- Be ready for customization with new business logic

---

## 📝 NOTES FOR AI AGENTS

- **Preserve file structure**: Keep the original directory layout
- **Don't delete infrastructure**: Keep build scripts, configs, .gitignore, etc.
- **Update imports**: When deleting files, search for imports and remove them
- **Test incrementally**: After major deletions, run `cargo build` to catch errors early
- **Keep composables clean**: Generic composables (useLogger, useNotification) should remain
- **i18n consistency**: Update all language files with same structure
- **Error handling**: Maintain existing error handling patterns in Rust

---

**Document Version:** 1.0.0
**Last Updated:** 2026-02-18
**Companion Documents:** TEMPLATE_GUIDE.md (detailed human guide), TEMPLATE_QUICKSTART.md (quick reference)
