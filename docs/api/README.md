# API Reference

This is the index for per-package API docs. Every entry below is generated from the actual `src/index.ts` exports — no invented APIs.

## TypeScript packages

| Package | API doc | Status |
|---------|---------|--------|
| `@bettertui/shared` | [shared.md](packages/shared.md) | Implemented (types only) |
| `@bettertui/core` | [core.md](packages/core.md) | Implemented |
| `@bettertui/react` | [react.md](packages/react.md) | Partial (renderer+hooks real; 69 components are thin wrappers) |
| `@bettertui/core` (native bridge) | [native.md](packages/native.md) | Implemented (requires native addon) |
| `@bettertui/themes` | [themes.md](packages/themes.md) | Implemented |
| `@bettertui/devtools` | [devtools.md](packages/devtools.md) | Stub (`createDevTools` → `null`) |
| `@bettertui/benchmark` | — | Implemented (Vitest bench) |
| `@bettertui/widgets` | [widgets.md](packages/widgets.md) | Proposed (not a package yet) |
| `@bettertui/icons` | [icons.md](packages/icons.md) | Proposed (not a package yet) |

## Rust crates

| Crate | Surface |
|-------|---------|
| `bettertui-engine` | Modules listed in [Architecture Overview](../architecture/Overview.md); per-module docs under `../architecture/` |
| `bettertui-bindings` | napi classes: `NapiEngine`, `NapiEventBus`, `NapiFocusManager`, `NapiTextEngine`, `NapiScheduler`, `NapiCapabilities`; free fns `getVersion`, `detectCapabilities` |

## Naming conventions

- All TS packages export from a single `src/index.ts`.
- Values are exported as runtime bindings; types via `export type`.
- ESM-only, built with `tsdown` (`dts: true`).

## Cross-cutting notes

- `@bettertui/core` re-exports `shared` types and adds `Command`, `CommandBuffer`, `Runtime`, `createReconciler`, and tree-op helpers.
- `@bettertui/react` `Theme` (hook-authored) differs from `@bettertui/shared` `Theme` (engine colors+borders) — do not conflate them.
- `@bettertui/core`'s native bridge depends on an **unbuilt** `bettertui_bindings` addon (not declared in package.json).
