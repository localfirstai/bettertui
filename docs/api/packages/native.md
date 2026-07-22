# @bettertui/core (native bridge)

The native bridge at `packages/core/src/platform/` loads the Rust engine addon (`bettertui_engine.node`). Previously a separate `@bettertui/native` package; now part of `@bettertui/core`.

## Loading

`loadNativeAddon()` does `require("bettertui_engine")` lazily. If missing: `Failed to load native bindings. Run pnpm --filter @bettertui/core build:native first.`

## Factories

| Export | Returns | Rust type |
|--------|---------|-----------|
| `createEngine(width?, height?)` | `NapiEngine` | `NapiEngine` |
| `createEventBus()` | `NapiEventBus` | `NapiEventBus` |
| `createFocusManager()` | `NapiFocusManager` | `NapiFocusManager` |
| `createTextEngine()` | `NapiTextEngine` | `NapiTextEngine` |
| `createScheduler()` | `NapiScheduler` | `NapiScheduler` |
| `createKeymap()` | `NapiKeymap` | `NapiKeymap` |
| `detectCapabilities()` | `TerminalCapabilities` | JSON from `detectCapabilities` |
| `getVersion()` | `string` | `getVersion` |

## Runtime & event loop

| Export | Notes |
|--------|-------|
| `createRuntime(engine, eventBus, buffer)` | `{ processCommands(), renderFrame(), resize(), shutdown() }` |
| `createEventLoop(eventBus)` | `{ start, stop, pushKey, pushMouse, drain, onEvent }` |

## Re-exported types

`NapiEngine`, `NapiEventBus`, `NapiFocusManager`, `NapiTextEngine`, `NapiScheduler`, `ProcessResult`, `TerminalCapabilities`, `SchedulerStats`, `Runtime`, `RuntimeOptions`, `EventLoop`, `EventCallback`, `KeyEvent`, `MouseEvent`

## Status

Implemented at `packages/core/src/platform/`. All native factories throw at runtime unless the Rust addon is built first.
