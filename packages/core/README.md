# @bettertui/core

Framework-agnostic runtime, command protocol, tree operations, and the internal native bridge for BetterTUI. This package contains **no React** and no UI-framework code — it is the boundary between any UI framework and the Rust engine.

## What's inside

- `CommandBuffer` — ordered queue of render commands (taffy layout → ANSI).
- `Runtime` — frame loop and commit orchestration.
- Tree operations — `createInstance`, `createTextInstance`, `appendChild`, `insertBefore`, `removeChild`, `commitUpdate`, `commitTextUpdate`, and friends used by reconcilers.
- `createReconciler()` — framework-agnostic `react-reconciler`-style host config (no React import).
- Engine module (`src/engine/`) — loads the `bettertui_bindings` napi addon and exposes engine factories (`createEngine`, `createEventBus`, `createFocusManager`, `createKeymap`, `createTextEngine`, `createScheduler`, `createRuntime`, `createEventLoop`, `detectCapabilities`, `getVersion`).

## Building

```bash
pnpm build                # tsdown -> dist/
cargo build -p bettertui-bindings --manifest-path packages/core/Cargo.toml  # native addon
```

The native addon is **not** declared in `package.json`. `@bettertui/core` calls `require("bettertui_bindings")` at runtime and throws a clear error if the addon was not built first.

## Testing

```bash
pnpm test                 # vitest run (src/**/*.test.ts)
pnpm test:coverage        # with @vitest/coverage-v8
```

The package carries the largest TypeScript test suite in the repo (command buffer, runtime, tree ops, native bridge).

## Status

Implemented. The native bridge (merged from the former `@bettertui/native`) requires the Rust addon to be built before any native call executes.

See [`docs/api/packages/core.md`](../../docs/api/packages/core.md) and [`docs/architecture/overview.md`](../../docs/architecture/overview.md).
