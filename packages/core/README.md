# @bettertui/core

## Purpose

Framework-agnostic foundation for BetterTUI. Provides the command protocol, tree manipulation, reconciler wrapper, and runtime — all with zero framework dependencies. This is the package that future adapters (Vue, Solid, Svelte) depend on.

## Responsibilities

- **CommandBuffer:** Queue of `Command` objects that represent tree mutations.
- **Tree operations:** `createInstance`, `createTextInstance`, `appendChild`, `removeChild`, `insertBefore`, `prepareUpdate`, `commitUpdate`, `commitTextUpdate`.
- **`createReconciler()`:** Wraps tree operations with command emission. Returns an object whose methods mirror the tree operation signatures.
- **`Runtime`:** Manages the `CommandBuffer`, provides a subscriber pattern for flushing commands, and runs a frame loop.

## Public API

### Types

```typescript
export type Command =
  | { type: "CreateNode"; id: string; kind: string }
  | { type: "RemoveNode"; id: string }
  | { type: "AppendChild"; parent: string; child: string }
  | { type: "InsertBefore"; reference: string; child: string }
  // ... 15 command types total

export interface CommandBufferConsumer {
  push(command: Command): void;
}

export interface Instance {
  id: string;
  type: string;
  props: Record<string, unknown>;
  style: Style;
  layout: LayoutConstraints;
  children: Instance[];
  parent: Instance | null;
}

export interface TextInstance {
  type: "#text";
  text: string;
  parent: Instance | null;
}
```

### Classes

```typescript
class CommandBuffer {
  push(command: Command): void;
  drain(): Command[];
  peek(): readonly Command[];
  clear(): void;
  get length(): number;
  get isEmpty(): boolean;
}

class Runtime {
  constructor(buffer?: CommandBuffer);
  get commandBuffer(): CommandBuffer;
  drain(): Command[];
  flush(): void;
  subscribe(fn: (commands: Command[]) => void): () => void;
  startFrameLoop(intervalMs?: number): void;
  stopFrameLoop(): void;
  dispose(): void;
}
```

### Functions

```typescript
function createReconciler(buffer: CommandBuffer): {
  createInstance: (type: string, props: Record<string, unknown>) => Instance;
  createTextInstance: (text: string) => TextInstance;
  appendChild: (parent: Instance, child: Instance | TextInstance) => void;
  removeChild: (parent: Instance, child: Instance | TextInstance) => void;
  insertBefore: (parent: Instance, child: Instance | TextInstance, reference: Instance | TextInstance) => void;
  prepareUpdate: (...) => Record<string, unknown> | null;
  commitUpdate: (instance: Instance, updatePayload: Record<string, unknown>) => void;
  commitTextUpdate: (textInstance: TextInstance, text: string) => void;
  finalizeInitialChildren: (instance: Instance) => boolean;
  resetAfterCommit: () => void;
};
```

## Dependencies

- `@bettertui/shared`

## Consumers

- `@bettertui/react` — uses `CommandBuffer`, `Instance`, `TextInstance`, `Runtime`, `createReconciler`
- `@bettertui/core` (native bridge) — uses `CommandBuffer`, `Command` internally
- `@bettertui/widgets` — depends on this package
- Examples — counter example demonstrates usage

## Internal Structure

```
src/
  index.ts            # Public exports
  command-buffer.ts   # CommandBuffer class, Command type, tree operations
  reconciler.ts       # createReconciler() wrapper
  runtime.ts          # Runtime class with frame loop and subscribers
```

## Design Principles

- **Zero framework imports.** No React, Vue, or Svelte. Future adapters implement their own reconciler/host config on top of these primitives.
- **Command is the API contract.** Framework adapters emit commands. The native bridge consumes them. This is the only coupling point.
- **`CommandBufferConsumer` is the adapter interface.** Any adapter that can `push(command)` can use the full tree operation API.

## Example Usage

```typescript
import { CommandBuffer, createReconciler, Runtime } from "@bettertui/core";

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);
const runtime = new Runtime(buffer);

// Create nodes
const root = reconciler.createInstance("box", {});
const text = reconciler.createTextInstance("Hello");
reconciler.appendChild(root, text);

// Flush commands to the engine
runtime.flush();
```

## Notes

- `createReconciler()` is the framework-agnostic version. For React, see `@bettertui/react` which implements a full `react-reconciler` host config.
- `Runtime.subscribe()` returns an unsubscribe function — always capture it for cleanup.
