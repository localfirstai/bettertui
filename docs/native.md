# Native Bridge

The native bridge is part of **`@bettertui/core`** at `packages/core/src/platform/`. It loads the `bettertui_engine` addon and exposes engine factories, a runtime, and an event loop.

`loadNativeAddon()` lazily `require("bettertui_engine")`. Missing addon throws `Failed to load native bindings. Run pnpm --filter @bettertui/core build:native first.`

Factories: `createEngine`, `createEventBus`, `createFocusManager`, `createTextEngine`, `createScheduler`, `detectCapabilities`, `getVersion`, `createRuntime(engine, eventBus, buffer)`, `createEventLoop(eventBus)`.

The napi surface lives in `bettertui-engine`'s `napi.rs` module (compiled with the `napi` feature). See [Architecture: Protocol](architecture/protocol.md) and [API: @bettertui/core](api/packages/core.md).
