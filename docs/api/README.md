# API Reference

This is the index for per-package API docs. Every entry below is generated from the actual `src/index.ts` exports — no invented APIs.

## TypeScript packages

| Package | API doc | Status |
|---------|---------|--------|
| `@bettertui/shared` | [shared.md](packages/shared.md) | Implemented (types only) |
| `@bettertui/core` | [core.md](packages/core.md) | Implemented |
| `@bettertui/react` | [react.md](packages/react.md) | Partial (renderer+hooks+keymap real; 53 components are thin wrappers) |
| `@bettertui/core` (native bridge) | [native.md](packages/native.md) | Implemented (requires native addon) |
| `@bettertui/themes` | [themes.md](packages/themes.md) | **Removed** — absorbed into `@bettertui/shared` + Rust engine |
| `@bettertui/devtools` | [devtools.md](packages/devtools.md) | Stub (`createDevTools` → `null`) |
| `@bettertui/benchmark` | — | Implemented (Vitest bench) |

## Rust crates

| Crate | Surface |
|-------|---------|
| `bettertui-engine` | Modules listed in [Architecture Overview](../architecture/overview.md); per-module docs under `../architecture/` |
| `bettertui-bindings` | napi classes: `NapiEngine`, `NapiEventBus`, `NapiFocusManager`, `NapiTextEngine`, `NapiScheduler`, `NapiCapabilities`; free fns `getVersion`, `detectCapabilities` |

## Naming conventions

- All TS packages export from a single `src/index.ts`.
- Values are exported as runtime bindings; types via `export type`.
- ESM-only, built with `tsdown` (`dts: true`).

## Cross-cutting notes

- `@bettertui/core` re-exports `shared` types and adds `Command`, `CommandBuffer`, `Runtime`, `createReconciler`, and tree-op helpers.
- `@bettertui/react`'s Theme, ThemeColors, and ThemeSpacing are the same types as @bettertui/shared's — re-exported for consumer convenience.
- `@bettertui/core`'s native bridge depends on an **unbuilt** `bettertui_bindings` addon (not declared in package.json).
