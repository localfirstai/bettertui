# @bettertui/core

> **Website:** [bettertui.dev](https://bettertui.dev) | **Docs:** [bettertui.dev/docs](https://bettertui.dev/docs)

**Framework-agnostic command protocol, tree manipulation, reconciler wrapper, and runtime.** Framework package for vanilla / native TypeScript. No React dependency.

## Overview

`@bettertui/core` is the public entry point for building terminal UIs without React. It provides:

- **Command protocol** — typed `Command` union and `CommandBuffer` for batching
- **Reconciler** — `createReconciler(buffer)` wraps tree ops with command emission
- **Runtime** — `CommandRuntime` owns the buffer, frame loop, and subscriber dispatch
- **Native bridge** — loads `bettertui_engine.node` addon, exposes engine factories
- **DevTools** — in-core debug tooling (`createDevTools`, debug overlay)
- **Testing utilities** — `createTestRenderer`, `createMockKeys`, mock streams, spies
- **Keymap** — framework-agnostic input binding engine
- **Widgets** — TypeScript widget option types
- **Validation** — `validate`, `warnIfInvalid`, layout/style validation

## Installation

```bash
npm install @bettertui/core
```

Requires building the native Rust addon for native bridge features:

```bash
pnpm --filter @bettertui/core build:native
```

## Quick start

```ts
import { createEngine, detectCapabilities, CliRenderer } from "@bettertui/core";

const engine = createEngine();
const caps = detectCapabilities();
```

## Re-exports

Re-exports all types from `@bettertui/shared` (internal — don't install separately).

## Features

- Framework-agnostic (no React dependency)
- Native Rust engine via napi-rs FFI
- Command-batched rendering (one FFI call per frame)
- In-core DevTools with debug overlay
- Testing utilities for headless rendering
- Keymap with layered bindings, chord sequences, and modes

## Related Documentation

- [Website](https://bettertui.dev)
- [Architecture overview](../../docs/architecture/overview.md)
- [API reference](../../docs/api/packages/core.md)
- [Guides](../../docs/guides/getting-started.md)
