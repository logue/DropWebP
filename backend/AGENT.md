# AGENT.md

This file defines backend-specific rules for AI coding agents.

---

## Inheritance

Common repository rules are defined in ../AGENT.md and apply here.
Before implementing app-specific backend commands/logic, read and follow the
"Downstream App Specification (Fill This Section)" in ../AGENT.md.

---

## Rule Priority

Apply rules in this order:

1. This file (backend-specific)
2. .github/instructions/backend.instructions.md
3. .github/copilot-instructions.md
4. ../AGENT.md

When rules conflict, prefer Rust/Tauri backend rules.

---

## Scope

- Directory: backend/
- Stack: Rust 2024 edition, Tauri v2

---

## Commands

Use backend-scoped commands:

- cd backend && cargo test
- cd backend && cargo build --release

---

## Rust / Tauri Rules

- Add Tauri commands in backend/src/command.rs.
- Register commands in backend/src/main.rs.
- Tauri command signatures must return Result<T, String>.
- Never use println! for app logging.
- Use send_log_with_handle and LogLevel from backend/src/lib.rs.
- Use jiff for date/time; do not introduce chrono.
- Keep modules private by default and re-export only intentional APIs from lib.rs.

---

## Backend Comment Rule

- Follow the mandatory English Rustdoc rule from ../AGENT.md.

---

## What Not To Do

- Do not edit version fields directly in Cargo.toml.
- Do not expose unnecessary public APIs.
- Do not introduce chrono or println!-based logging.
