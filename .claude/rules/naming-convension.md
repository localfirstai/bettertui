# Naming Conventions

These rules govern how we name files and symbols across the repo. Apply them whenever you create a new file or add new code.

## Rust

- Follow standard Rust nomenclature (the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/naming.html)) — no project-specific overrides.
  - Files & modules: `snake_case` (e.g. `command_buffer.rs`).
  - Types, traits, enums: `PascalCase` (e.g. `CommandBuffer`, `NodeId`).
  - Functions, methods, variables, fields: `snake_case`.
  - Constants & statics: `SCREAMING_SNAKE_CASE`.
  - Crates: `snake_case` (or `kebab-case` in `Cargo.toml`'s `name`, matching existing crates like `bettertui-engine`).

## TypeScript

Default identifier casing is **camelCase** (functions, variables, methods). Types, interfaces, enums, and classes use **PascalCase** as usual.

### File naming

Most files (services, utils, plain modules) use **camelCase** file names:

- Services: `*.service.ts` — e.g `user.service.ts`, `theme.service.ts`
- Utils: `*.utils.ts` — e.g `parseColor.utils.ts`, `string.utils.ts`
- Generic modules: `commandBuffer.ts`

Special suffixed files:

- **Types**: `*.types.ts` — e.g. `demo.types.ts`
- **Examples**: `*.example.ts` — e.g. `demo.example.ts`

### Widgets (TypeScript, non-React)

TypeScript widget files use **PascalCase** file names:

- `Button.ts`, `TextInput.ts`, `ScrollView.ts`

### React

- **Components**: `kebab-case.tsx` — e.g. `text-input.tsx`, `scroll-view.tsx`
  (the exported component itself is still `PascalCase`, e.g. `export function TextInput()`).
- **Hooks**: `useHookName.ts` (camelCase, `use` prefix) — e.g. `useFocus.ts`, `useKeyboard.ts`
  (the exported hook is `camelCase`, matching the file name).

## Quick reference

| Kind                       | File name        | Example             |
| -------------------------- | ---------------- | ------------------- |
| Rust module                | `snake_case.rs`  | `command_buffer.rs` |
| TS service / util / module | `camelCase.ts`   | `userService.ts`    |
| TS types                   | `*.types.ts`     | `demo.types.ts`     |
| TS examples                | `*.example.ts`   | `demo.example.ts`   |
| TS widget                  | `PascalCase.ts`  | `Button.ts`         |
| React component            | `kebab-case.tsx` | `text-input.tsx`    |
| React hook                 | `useHookName.ts` | `useFocus.ts`       |
