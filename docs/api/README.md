# API Reference

This is the index for per-package API docs. Every entry below is generated from the actual `src/index.ts` exports — no invented APIs. All TypeScript packages are currently `private` (not published to npm).

## TypeScript packages

| Package | API doc | Status |
|---------|---------|--------|
| `@bettertui/core` | [core.md](packages/core.md) | Framework package for vanilla / native TypeScript (command protocol, tree ops, `CommandRuntime`, native bridge) |
| `@bettertui/react` | [react.md](packages/react.md) | React adapter — host config, hooks, and **13 component functions**; install **only** this for React apps (depends on `@bettertui/core`) |
| `@bettertui/core` (native bridge) | [native.md](packages/native.md) | Implemented (requires the `bettertui_engine.node` addon); part of the `@bettertui/core` surface |
| `@bettertui/shared` | [shared.md](packages/shared.md) | **Internal — types only, re-exported via `@bettertui/core`/`@bettertui/react`** |
| `@bettertui/themes` | [themes.md](packages/themes.md) | **Removed** — absorbed into `@bettertui/shared` + Rust engine |
| `@bettertui/devtools` | [devtools.md](packages/devtools.md) | Implemented (`createDevTools` factory) |
| `@bettertui/benchmark` | — | Implemented (Vitest bench) |

## Rust crates

| Crate | Surface |
|-------|---------|
| `bettertui-engine` | Modules listed in [Architecture Overview](../architecture/overview.md); per-module docs under `../architecture/`. With the `napi` feature it builds as the `bettertui_engine.node` addon exposing `NapiEngine`, `NapiEventBus`, `NapiFocusManager`, `NapiTextEngine`, `NapiScheduler`, `NapiKeymap`, `NapiCapabilities`, plus `getVersion` / `detectCapabilities`. |

## Naming conventions

- All TS packages export from a single `src/index.ts`.
- Values are exported as runtime bindings; types via `export type`.
- ESM-only, built with `tsdown` (`dts: true`).

## Cross-cutting notes

- `@bettertui/core` is the framework package for vanilla / native TypeScript. It re-exports `@bettertui/shared` types (shared is internal — do not install directly) and adds `Command`, `CommandBuffer`, `CommandRuntime`, `createReconciler`, and tree-op helpers.
- `@bettertui/react` re-exports `@bettertui/shared` types and depends on `@bettertui/core`. React apps install **only** `@bettertui/react`; core resolves automatically. Consumers should import `Theme`, `ThemeColors`, and `ThemeSpacing` from `@bettertui/react` (or `@bettertui/core` for framework-agnostic use).
- `@bettertui/core`'s native bridge depends on an **unbuilt** `bettertui_engine.node` addon (not declared in package.json).
