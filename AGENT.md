# AGENT.md

This file defines repository-wide common rules for AI coding agents.

---

## Scope and Priority

Use rules in this order:

1. Directory-specific AGENT.md (frontend/, docs/, backend/) when present
2. Instructions under .github/instructions/\* and .github/copilot-instructions.md
3. This root AGENT.md

When rules conflict, prefer the rule with narrower scope.

---

## Repository Baseline

- Monorepo: frontend (Vue/Vite), backend (Rust/Tauri), docs (Nuxt)
- Package manager: pnpm only
- .env is the source of truth for app metadata/version; do not manually desync version fields

---

## Downstream App Specification (Fill This Section)

This repository is the Drop Compress Image application.

### App Profile

- App name: Drop Compress Image
- Domain/business context: Next-generation image converter (avif, webp, jpeg-xl, jpeg-li, zopfli png)
- Target users: Desktop users who need high-quality image conversion
- Supported platforms: Windows, macOS, Linux
- Release constraints: Offline-capable, cross-platform desktop app

### Core Features

- Feature 1: Drag-and-drop image conversion
- Feature 2: Multiple output format support (WebP, AVIF, JXL, JPEG, PNG)
- Feature 3: Batch processing with progress indication

### Project-Specific Decisions

- Build scripts use `scripts/run-tauri-dev.mjs` and `scripts/run-tauri-build.mjs` (not the inline sync+tauri approach)
- Windows builds require LLVM/Clang and NASM for AVIF/JXL encoding
- `scripts/sync-tauri-config-from-env.mjs` is called by `run-tauri-dev.mjs` to sync version/identifier

---

## Documentation Comment Rule (Mandatory)

For all generated code, documentation comments in English are mandatory.

### TypeScript / JavaScript

Add JSDoc-compliant comments for generated:

- functions (including exported arrow functions)
- constants
- classes

Use tags when applicable:

- @param
- @returns
- @throws
- @example

### Rust

Add Rustdoc comments for generated:

- functions
- constants
- types (struct, enum, trait)

Use sections when applicable:

- # Arguments
- # Returns
- # Errors
- # Panics
- # Examples

This applies to newly created symbols and symbols modified during refactoring.

---

## Directory Guides

- frontend/: frontend/AGENT.md
- docs/: docs/AGENT.md
- backend/: backend/AGENT.md
