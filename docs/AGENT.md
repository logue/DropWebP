# AGENT.md

This file defines docs-specific rules for AI coding agents.

---

## Inheritance

Common repository rules are defined in ../AGENT.md and apply here.
Before writing app-specific documentation content, read and follow the
"Downstream App Specification (Fill This Section)" in ../AGENT.md.

---

## Rule Priority

Apply rules in this order:

1. This file (docs-specific)
2. .github/copilot-instructions.md
3. ../AGENT.md

When rules conflict, prefer Nuxt/docs-specific rules.

---

## Scope

- Directory: docs/
- Stack: Nuxt 4, Vue 3, TypeScript, @nuxt/content, @nuxtjs/i18n, Pinia, Vuetify

---

## Commands

Use docs-scoped commands:

- pnpm -C docs dev
- pnpm -C docs lint
- pnpm -C docs build
- pnpm -C docs generate

Before finishing changes, run at least:

1. pnpm -C docs lint
2. pnpm -C docs build

---

## Nuxt / Content Rules

- Keep page and layout files thin; move reusable logic to composables.
- Prefer Nuxt runtime APIs (useRuntimeConfig, useAsyncData, useRoute).
- Keep localized content aligned across en, ja, fr, ko, zhHans, zhHant.
- Do not hardcode user-facing UI text in components.

---

## Docs Comment Rule

- Follow the mandatory English JSDoc rule from ../AGENT.md for TS/JS code.

---

## What Not To Do

- Do not bypass Nuxt conventions with framework-agnostic shortcuts.
- Do not use npm or yarn in docs/.
