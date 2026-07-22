# API Reference

Per-package API docs generated from `src/index.ts` exports. All packages are currently `private` (not published to npm).

## TypeScript packages

| Package | API doc | Status |
|---------|---------|--------|
| `@bettertui/core` | [core.md](packages/core.md) | Framework-agnostic public API — **implemented** |
| `@bettertui/react` | [react.md](packages/react.md) | React 19 adapter — **implemented** |
| `@bettertui/solid` | — | **Placeholder** — not implemented |
| `@bettertui/shared` | [shared.md](packages/shared.md) | Internal — types only, re-exported via core |
| `@bettertui/core` (native bridge) | [native.md](packages/native.md) | Implemented (requires `bettertui_engine.node`) |
| `@bettertui/core` (devtools) | [devtools.md](packages/devtools.md) | Implemented — in-core `createDevTools` |
| `@bettertui/performance` | — | Vitest benchmark suite |

## Naming conventions

- All TS packages export from a single `src/index.ts`
- Values as runtime bindings; types via `export type`
- ESM-only, built with `tsdown` (`dts: true`)
