# AGENT.md

This file defines frontend-specific rules for AI coding agents.

---

## Inheritance

Common repository rules are defined in ../AGENT.md and apply here.
Before implementing app-specific frontend behavior, read and follow the
"Downstream App Specification (Fill This Section)" in ../AGENT.md.

---

## Rule Priority

Apply rules in this order:

1. This file (frontend-specific)
2. .github/instructions/frontend.instructions.md
3. .github/copilot-instructions.md
4. ../AGENT.md

When rules conflict, prefer frontend-specific rules.

---

## Scope

- Directory: frontend/
- Stack: Vue 3, TypeScript, Vite, Vuetify, Pinia, vue-i18n

---

## Commands

Use frontend-scoped commands:

- pnpm --filter frontend dev
- pnpm --filter frontend type-check
- pnpm --filter frontend lint
- pnpm --filter frontend build-only

Before finishing changes, run at least:

1. pnpm --filter frontend lint
2. pnpm --filter frontend build-only

---

## Vue / State / Boundary Rules

- Use script setup with TypeScript.
- Keep components presentation-first; move reusable logic to composables.
- Manage shared UI state via Pinia stores.
- Do not import Tauri APIs directly in .vue components.
- Encapsulate Tauri command/event/dialog/fs/notification usage in composables or store actions.

Recommended flow:

1. Component emits intent.
2. Composable/store performs side effects.
3. Component renders reactive state.

---

## i18n Rules

- Do not hardcode user-facing text in templates.
- Keep translation keys namespaced by feature.
- Locale state is managed centrally (ConfigStore).
- Keep locale resources synchronized across en, ja, fr, ko, zhHans, zhHant.

---

## Frontend Comment Rule

- Follow the mandatory English JSDoc rule from ../AGENT.md.
- Add/maintain JSDoc when creating or modifying functions, constants, and classes.

---

## What Not To Do

- Do not use Options API.
- Do not write business logic-heavy components.
- Do not bypass composables/store for Tauri side effects.
- Do not use npm or yarn in frontend/.
