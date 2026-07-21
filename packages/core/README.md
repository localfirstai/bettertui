# @bettertui/core

**The framework package for native / vanilla TypeScript.** Use it directly when you don't need React. It is fully framework-agnostic — no React, no UI-framework code — and is the boundary between any TypeScript app (vanilla or a custom framework adapter) and the Rust engine. (All packages are currently `private`.)

> **React apps:** install **only** `@bettertui/react`. It depends on `@bettertui/core` and pulls it in automatically — you never install core by hand for a React project.

## What's inside

- `CommandBuffer` — ordered queue of render commands (taffy layout → ANSI).
- `CommandRuntime` — frame loop and commit orchestration.
- Tree operations — `createInstance`, `createTextInstance`, `appendChild`, `insertBefore`, `removeChild`, `commitUpdate`, `commitTextUpdate`, and friends used by reconcilers.
- `createReconciler()` — framework-agnostic `react-reconciler`-style host config (no React import).
- Engine module (`src/platform/`) — loads the `bettertui_engine` napi addon and exposes engine factories (`createEngine`, `createEventBus`, `createFocusManager`, `createKeymap`, `createTextEngine`, `createScheduler`, `detectCapabilities`, `getVersion`) plus `CliRenderer` / `KeyInput` for CLI rendering.

## Building

```bash
pnpm build                # tsdown -> dist/
pnpm build:native         # builds bettertui_engine.node via napi (--features napi)
```

The native addon is **not** declared in `package.json`. `@bettertui/core` calls `require("bettertui_engine")` at runtime and throws a clear error if the addon was not built first.

## Testing

```bash
pnpm test                 # vitest run (src/**/*.test.ts)
pnpm test:coverage        # with @vitest/coverage-v8
```

The package carries the largest TypeScript test suite in the repo (command buffer, runtime, tree ops, native bridge).

## Status

Implemented. The native bridge (merged from the former `@bettertui/native`) requires the Rust addon to be built before any native call executes.

See [`docs/api/packages/core.md`](../../docs/api/packages/core.md) and [`docs/architecture/overview.md`](../../docs/architecture/overview.md).
