# Runtime

"Runtime" refers to the frame loop and command drain that tie the TypeScript command buffer to the Rust engine.

## Core Runtime (`@bettertui/core`)

`CommandRuntime` (exported from `@bettertui/core`) owns a `CommandBuffer` and:

- `subscribe(consumer)` — register a sink for drained commands
- `drain()` — pull accumulated commands
- `flush()` — notify subscribers
- `startFrameLoop(intervalMs = 16)` / `stopFrameLoop()` — timed drain+flush
- `render()` — if an engine is provided, calls `beginFrame` → `renderFrame` → `commitFrame`
- `resize(width, height)` — update terminal dimensions
- `dispose()` — tear down

## React Runtime (`@bettertui/react`)

`createRoot(element)` creates a reconciler root and a runtime. `RuntimeContext` + `useRuntimeContext()` expose the runtime to components.

## Native bridge (`@bettertui/core`)

The native bridge (`packages/core/src/platform/`) loads the `bettertui_engine` addon and exposes `createEngine`, `createEventBus`, `createFocusManager`, `createTextEngine`, `createScheduler`, `createKeymap`, `detectCapabilities`, `getVersion`, plus `CliRenderer` / `KeyInput` for CLI rendering. `createRuntime(engine, eventBus, buffer)` ties these together.

## See also

- [Architecture: Scheduler](architecture/scheduler.md)
- [API: @bettertui/core](api/packages/core.md)
