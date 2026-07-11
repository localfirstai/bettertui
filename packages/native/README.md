# @bettertui/native

## Purpose

TypeScript bridge between `@bettertui/core` and the Rust napi-rs bindings (`bettertui-bindings`). Provides factory functions to load native addons, a `Runtime` wrapper that serializes commands to JSON for the Rust engine, and an `EventLoop` wrapper for input handling.

## Responsibilities

- **Native addon loading:** `createEngine`, `createEventBus`, `createFocusManager`, `createTextEngine`, `createScheduler`, `detectCapabilities`, `getVersion`.
- **Runtime wrapper:** `createRuntime()` — drains the `CommandBuffer`, serializes commands to JSON, and sends them to the Rust engine. Handles render frame lifecycle.
- **Event loop:** `createEventLoop()` — wraps `NapiEventBus` for keyboard, mouse, paste, and resize event dispatch.
- **Type definitions:** TypeScript interfaces for all napi-rs exported classes (`NapiEngine`, `NapiEventBus`, `NapiFocusManager`, `NapiTextEngine`, `NapiScheduler`).

## Public API

### Factory functions

```typescript
function createEngine(width?: number, height?: number): NapiEngine;
function createEventBus(): NapiEventBus;
function createFocusManager(): NapiFocusManager;
function createTextEngine(): NapiTextEngine;
function createScheduler(): NapiScheduler;
function detectCapabilities(): TerminalCapabilities;
function getVersion(): string;
```

### Runtime

```typescript
function createRuntime(engine: NapiEngine, eventBus: NapiEventBus, buffer: CommandBuffer): Runtime;

interface Runtime {
  engine: NapiEngine;
  eventBus: NapiEventBus;
  buffer: CommandBuffer;
  processCommands(): ProcessResult;
  renderFrame(): RenderResult;
  resize(width: number, height: number): void;
  shutdown(): void;
}
```

### Event loop

```typescript
function createEventLoop(eventBus: NapiEventBus): EventLoop;

interface EventLoop {
  start(): void;
  stop(): void;
  pushKey(key: string, ctrl: boolean, shift: boolean, alt: boolean, targetId: number): void;
  pushMouse(button: string, x: number, y: number, targetId: number): void;
  drain(): string;
  onEvent(callback: EventCallback): void;
}
```

### Types

```typescript
interface NapiEngine { processCommands(json: string): string; render(): string; /* ... */ }
interface NapiEventBus { pushKey(...): void; drain(): string; /* ... */ }
interface NapiFocusManager { focus(id: number): boolean; traverse(dir: string): number; /* ... */ }
interface NapiTextEngine { insertText(text: string): void; /* ... */ }
interface NapiScheduler { beginFrame(): boolean; fps(): string; /* ... */ }
interface TerminalCapabilities { trueColor: boolean; mouse: boolean; /* ... */ }
```

## Dependencies

- `@bettertui/core` — imports `Command`, `CommandBuffer`
- Native addon: `bettertui_bindings` (Rust napi-rs crate, loaded via `require()`)

## Consumers

- Future application runtimes, examples (once connected)

## Internal Structure

```
src/
  index.ts     # Factory functions, native addon loader, type re-exports
  types.ts     # TypeScript interfaces for napi-rs classes
  runtime.ts   # createRuntime() — command serialization + render lifecycle
  events.ts    # createEventLoop() — event dispatch wrapper
```

## Design Principles

- **Thin bridge layer.** This package adds no rendering logic. It serializes commands and delegates to the Rust engine.
- **Lazy native loading.** The native addon is loaded on first use via `require("bettertui_bindings")`. This allows the TypeScript packages to be imported without the Rust binary present (e.g., for type-checking).
- **JSON command protocol.** All communication with Rust uses JSON-serialized commands. This keeps the FFI boundary simple and debuggable.

## Example Usage

```typescript
import { createEngine, createEventBus, createRuntime } from "@bettertui/native";
import { CommandBuffer } from "@bettertui/core";

const engine = createEngine(80, 24);
const eventBus = createEventBus();
const buffer = new CommandBuffer();
const runtime = createRuntime(engine, eventBus, buffer);

// Process commands
runtime.processCommands();

// Render a frame
const result = runtime.renderFrame();

// Shutdown
runtime.shutdown();
```

## Notes

- The native addon must be built before use: `cargo build -p bettertui-bindings`
- `EventLoop.start()` and `EventLoop.stop()` are currently empty — terminal input handling is not yet wired through this layer.
- `require("bettertui_bindings")` throws with a helpful error if the native addon is not built.
