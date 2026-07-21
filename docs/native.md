# Native Bridge

The native bridge is part of **`@bettertui/core`** — the framework package for vanilla / native TypeScript — at `packages/core/src/platform/`. It is the TypeScript side of the napi-rs FFI boundary. It loads the `bettertui_engine` addon and exposes engine factories, a runtime, and an event loop. Vanilla TypeScript apps use this surface directly (React apps get it transitively through `@bettertui/react`).

## Loading

`loadNativeAddon()` does `require("bettertui_engine")` (lazy, cached). Missing addon → throws `Failed to load native bindings. Run pnpm --filter @bettertui/core build:native first.` The addon is **not** in `package.json` — build it from `packages/core/crates/engine/` with `pnpm --filter @bettertui/core build:native` (which runs `napi build --manifest-path crates/engine/Cargo.toml --features napi`).

## Surface

| Export | Returns |
|--------|---------|
| `createEngine(w?, h?)` | `NapiEngine` |
| `createEventBus()` | `NapiEventBus` |
| `createFocusManager()` | `NapiFocusManager` |
| `createTextEngine()` | `NapiTextEngine` |
| `createScheduler()` | `NapiScheduler` |
| `detectCapabilities()` | `TerminalCapabilities` |
| `getVersion()` | `string` |
| `createRuntime(engine, eventBus, buffer)` | `Runtime` (`processCommands`, `renderFrame`, `resize`, `shutdown`) |
| `createEventLoop(eventBus)` | `EventLoop` (`start`, `stop`, `pushKey`, `pushMouse`, `drain`, `onEvent`) |

See the [API doc](api/packages/native.md) for full types.

## Rust side

The napi surface lives in the `napi` module of `bettertui-engine` (`packages/core/crates/engine`), compiled only with the `napi` feature. It exposes `NapiEngine`, `NapiEventBus`, `NapiFocusManager`, `NapiTextEngine`, `NapiScheduler`, `NapiKeymap`, `NapiCapabilities`, plus free fns `getVersion` / `detectCapabilities`. It decodes a `CommandJson` envelope into the engine `Command` enum and transmutes `NodeId` ↔ `u64`.

## Status

Implemented at `packages/core/src/platform/`. Depends on an unbuilt native addon (`bettertui_engine.node`) at runtime.
