# Native Bridge

`@bettertui/native` is the TypeScript side of the napi-rs FFI boundary. It loads the `bettertui_bindings` addon and exposes factories, a runtime, and an event loop.

## Loading

`loadNativeAddon()` does `require("bettertui_bindings")` (lazy, cached). Missing addon → throws `Failed to load native bindings. Run cargo build -p bettertui-bindings first.` The addon is **not** in `package.json` — build it with `cargo build -p bettertui-bindings`.

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

`bettertui-bindings` (`cdylib`) exposes `NapiEngine`, `NapiEventBus`, `NapiFocusManager`, `NapiTextEngine`, `NapiScheduler`, `NapiCapabilities`, plus free fns `getVersion` / `detectCapabilities`. It decodes a `CommandJson` envelope into the engine `Command` enum and transmutes `NodeId` ↔ `u64`.

## Status

Implemented. Depends on an unbuilt native addon at runtime.
