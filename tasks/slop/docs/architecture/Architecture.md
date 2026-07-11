# BetterTUI Architecture

> The definitive architectural reference for BetterTUI.
> Every decision documented. Every trade-off explained.

## 1. System Overview

BetterTUI is a **framework-agnostic, Rust-powered terminal UI rendering engine** with JavaScript bindings. It is not an application. It is infrastructure.

The system has three layers:

```
┌─────────────────────────────────────────────────────┐
│            Framework Adapters (TypeScript)           │
│   React · Vue · Solid · Svelte · Vanilla TS         │
│   Each adapter translates its component model       │
│   into BetterTUI's generic node protocol            │
└───────────────────────┬─────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────┐
│          Core TypeScript Layer (@bettertui/core)     │
│   Protocol codec · Node model types · Event bridge   │
│   Framework-agnostic · Zero runtime dependencies     │
└───────────────────────┬─────────────────────────────┘
                        │ napi-rs FFI boundary
┌───────────────────────▼─────────────────────────────┐
│           Rust Native Engine (bettertui-engine)      │
│   Arena allocator · Layout (Taffy) · FrameBuffer     │
│   Renderer · Event dispatch · Terminal I/O           │
│   Animation · Scheduler · Plugin host                │
└───────────────────────┬─────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────┐
│                    Terminal                          │
│   crossterm · ANSI sequences · stdin/stdout          │
└─────────────────────────────────────────────────────┘
```

### 1.1 Why Three Layers

**Framework adapters** exist because every UI framework has a different component model, lifecycle, and reactivity system. React has virtual DOM diffing. Solid has fine-grained signals. Vue has proxy-based reactivity. These cannot share a reconciler.

**The core TypeScript layer** exists because the FFI boundary must be thin and well-typed. It provides the protocol codec, node model types, and event bridge that all adapters share. It has zero runtime dependencies beyond TypeScript itself.

**The Rust engine** exists because terminal rendering is performance-critical. Layout calculation, frame diffing, ANSI encoding, and input parsing must happen at native speed. Rust gives us memory safety without garbage collection overhead.

### 1.2 What BetterTUI Is NOT

- **Not an application** — it is a framework that applications are built with.
- **Not React-specific** — React is the first adapter, not the only one.
- **Not a terminal emulator** — it renders to an existing terminal.
- **Not a widget toolkit** — widgets are built on top of the rendering primitives.
- **Not a game engine** — though it supports animations and a framebuffer.

## 2. Core Principles

### 2.1 Framework Agnosticism

The Rust engine has zero knowledge of React, Vue, Solid, or any UI framework. It exposes a generic node tree protocol. Framework adapters translate their component trees into this protocol.

**Why this matters:** If we couple to React, we cannot support Vue or Solid without rewriting the engine. If we couple to any specific reactivity model, we限制 which frameworks can use us.

**Enforcement:** The Rust crate `bettertui-engine` must never import any JavaScript framework. The TypeScript `@bettertui/core` package must never import React. Only `@bettertui/react` imports React.

### 2.2 Separation of Concerns

- **Rust** owns all performance-critical operations: rendering, layout, input parsing, scheduling, frame diffing.
- **TypeScript** owns API design, developer experience, framework bindings, and type safety.
- **No business logic** lives in the engine — it is purely a rendering and layout framework.

**Trade-off:** This means some logic that could be in Rust (like node tree management) lives in TypeScript. This is intentional — TypeScript's garbage collector handles complex object graphs better than manual Rust ownership for the parts that frequently cross the FFI boundary.

### 2.3 Zero-Config Performance

The engine automatically handles:
- Dirty rect diffing (only redraw changed regions)
- Frame scheduling (60fps target with adaptive quality)
- Layout caching (incremental recalculation)
- Memory pooling (reuse allocations)

Developers should not need to optimize manually. The engine does it for them.

### 2.4 Explicit Over Implicit

Every behavior must be observable and debuggable. The engine exposes:
- Frame timing metrics
- Layout calculation counts
- Memory allocation statistics
- Dirty region visualization
- Event propagation traces

### 2.5 Forward-Compatible Design

Every protocol message, every API surface, every data structure must be designed for the next five years. We optimise for:
- 100+ widget types
- Multiple framework adapters
- Thousands of concurrent users
- Plugin ecosystem
- DevTools integration
- Animation engine
- Canvas and graphics
- Accessibility (screen readers)
- Multiple render backends (future GPU acceleration)

## 3. Layer Architecture

### 3.1 Framework Adapter Layer

Each adapter is a separate npm package:

| Package | Framework | Peer Dependency |
|---------|-----------|-----------------|
| `@bettertui/react` | React 19+ | `react` |
| `@bettertui/vue` | Vue 3+ | `vue` |
| `@bettertui/solid` | Solid 1+ | `solid-js` |
| `@bettertui/svelte` | Svelte 4+ | `svelte` |
| `@bettertui/preact` | Preact 10+ | `preact` |
| `@bettertui/vanilla` | None | None |

**Adapter responsibilities:**
1. Translate the framework's component tree into BetterTUI node operations.
2. Manage component lifecycle (mount, update, unmount).
3. Bridge framework-specific reactivity (signals, proxies, refs) to BetterTUI's mutation protocol.
4. Provide framework-idiomatic hooks/composables.

**Adapter does NOT:**
1. Perform layout.
2. Render to terminal.
3. Handle input.
4. Manage animations.

All of those are handled by the Rust engine.

### 3.2 Core TypeScript Layer

| Package | Responsibility |
|---------|---------------|
| `@bettertui/shared` | Framework-agnostic type definitions |
| `@bettertui/core` | Node model, protocol codec, tree operations |
| `@bettertui/protocol` | Binary protocol encoder/decoder (proposed) |
| `@bettertui/renderer` | TypeScript renderer wrapper (proposed) |

**`@bettertui/shared`** contains pure type definitions with zero runtime code. Geometry types, style types, event types, layout types. This package is imported by everything.

**`@bettertui/core`** contains the node model, tree operations, and protocol codec. It is the bridge between framework adapters and the Rust engine. It owns:
- `NodeId` generation and management
- `NodeTree` — the in-memory tree representation
- `ProtocolCodec` — encodes/decodes commands for the FFI boundary
- `EventBridge` — routes events from Rust to TypeScript

**`@bettertui/protocol`** (proposed) would contain the binary protocol definition and codec. Separated from core to allow alternative transports (WebSocket for remote devtools, IPC for multi-process).

**`@bettertui/renderer`** (proposed) would contain the TypeScript-side renderer that manages the connection to the Rust engine, handles frame scheduling, and provides the public API.

### 3.3 Rust Engine Layer

The Rust engine is a single crate (`bettertui-engine`) with well-defined internal modules:

```
bettertui-engine
├── tree          — Arena-based node storage, tree operations
├── layout        — Taffy integration, constraint solving
├── render        — Frame production, dirty diffing
├── framebuffer   — Cell grid, double buffering
├── terminal      — Size detection, capabilities, raw mode
├── input         — Keyboard, mouse, paste, focus
├── events        — Event dispatch, bubbling, capture
├── animation     — Tween engine, keyframes, scheduling
├── scheduler     — Frame timing, async task orchestration
├── graphics      — Color, style, border rendering
├── protocol      — ANSI escape sequence generation
├── selection     — Text selection with range management
├── clipboard     — System clipboard integration
├── editor        — Rope-based text buffer with cursor
├── plugin        — Plugin host, extension points
├── capabilities  — Terminal feature detection
├── ffi           — napi-rs binding surface
├── benchmark     — Performance measurement
└── error         — Error types and recovery
```

### 3.4 napi-rs Bindings

`bettertui-bindings` is a thin `cdylib` crate that:
1. Exposes Rust structs and functions to Node.js via napi-rs.
2. Handles thread synchronization between Node.js event loop and Rust threads.
3. Manages memory ownership across the FFI boundary.

**Design rule:** The bindings crate contains no business logic. It is purely a translation layer. All logic lives in `bettertui-engine`.

## 4. Data Flow

### 4.1 Render Flow (TypeScript → Terminal)

```
1. User code calls React render (or Vue mount, etc.)
2. Framework reconciler diffs virtual DOM
3. Reconciler calls adapter's host config methods
4. Adapter translates to BetterTUI node operations:
   - createNode(id, kind, props)
   - appendChild(parentId, childId)
   - setStyle(nodeId, style)
   - setLayout(nodeId, layout)
   - setText(nodeId, text)
5. Core TypeScript batches operations into protocol commands
6. Commands encoded via ProtocolCodec
7. FFI call to Rust engine (napi-rs)
8. Rust engine processes commands:
   - Updates arena-based node tree
   - Invalidates affected layout subtrees
   - Recalculates layout via Taffy
   - Produces new frame via renderer
   - Diffs against previous frame
   - Encodes dirty cells as ANSI
   - Writes to terminal stdout
```

### 4.2 Event Flow (Terminal → User Code)

```
1. Terminal produces input bytes on stdin
2. Rust input parser decodes bytes
   - Keyboard: crossterm key events
   - Mouse: SGR mouse protocol events
   - Paste: bracketed paste events
   - Resize: SIGWINCH signal
3. Parsed event dispatched through event system
4. Event hits focused node (or hit-tested node for mouse)
5. Event bubbles up the tree (capture → target → bubble)
6. At each node, handlers are invoked
7. Handlers may call back into Rust (e.g., scroll, focus change)
8. If state changes, new render cycle begins
```

### 4.3 Layout Flow

```
1. Node tree mutated (add/remove/update)
2. Affected subtrees marked dirty
3. Layout engine (Taffy) receives dirty subtrees
4. Top-down constraint propagation:
   - Root receives terminal dimensions as constraints
   - Each node computes its size based on constraints + layout props
   - Children receive remaining space after parent padding/gap
5. Bottom-up measurement:
   - Leaf nodes measure their content (text length, etc.)
   - Parent nodes aggregate child sizes
   - Fixed sizes override flex sizing
6. Final positions computed:
   - Flex grow/shrink distributes remaining space
   - Absolute positioning applied
   - Scroll offsets applied
7. Layout results stored on each node
8. Renderer uses layout results for cell placement
```

## 5. Package Responsibilities

### 5.1 Current Packages

| Package | Exists | Role | Status |
|---------|--------|------|--------|
| `@bettertui/shared` | Yes | Type definitions | Complete types, no logic |
| `@bettertui/core` | Yes | Node model, protocol | Types only, no implementation |
| `@bettertui/reconciler` | Yes | React reconciler | Type stubs only |
| `@bettertui/react` | Yes | React components | Stub components |
| `@bettertui/widgets` | Yes | Widget library | Interface only |
| `@bettertui/themes` | Yes | Theme system | Partially implemented |
| `@bettertui/icons` | Yes | Icon registry | Working but empty |
| `@bettertui/devtools` | Yes | Developer tools | Stub |

### 5.2 Proposed Additional Packages

| Package | Reason |
|---------|--------|
| `@bettertui/protocol` | Protocol codec separated from core for alternative transports |
| `@bettertui/renderer` | TypeScript renderer wrapper for the Rust engine connection |
| `@bettertui/hooks` | Framework-agnostic hook utilities (shared across adapters) |
| `@bettertui/testing` | Headless renderer, mock input, snapshot testing |
| `@bettertui/animations` | Animation API exposed to TypeScript (backed by Rust engine) |
| `@bettertui/editor` | Text editor component (backed by Rust rope buffer) |
| `@bettertui/graphics` | Canvas-like API for custom rendering (backed by Rust framebuffer) |

### 5.3 Proposed Package Removals

| Package | Reason |
|---------|--------|
| `@bettertui/reconciler` | Should be merged into `@bettertui/react` — the reconciler is React-specific |

**Justification:** The reconciler package currently exists as a generic abstraction, but React's reconciler API is fundamentally React-specific. A Vue adapter would use Vue's reactivity system, not a reconciler. A Solid adapter would use signals. The reconciler concept only applies to React. Keeping it separate adds unnecessary indirection.

## 6. Rust Module Design

### 6.1 Module Responsibility Matrix

| Module | Owns | Does NOT Own |
|--------|------|-------------|
| `tree` | Arena storage, node CRUD, parent-child relationships | Layout calculation, rendering |
| `layout` | Taffy integration, constraint solving, caching | Node storage, rendering |
| `render` | Frame production, dirty diffing, style resolution | Terminal I/O, layout calculation |
| `framebuffer` | Cell grid, double buffering, dirty tracking | ANSI encoding, layout |
| `terminal` | Size detection, capabilities, raw mode, alternate screen | Rendering, layout |
| `input` | Keyboard parsing, mouse parsing, paste detection | Event dispatch, focus management |
| `events` | Event dispatch, bubbling, capture, cancellation | Input parsing, rendering |
| `animation` | Tween engine, keyframes, easing functions | Rendering, event dispatch |
| `scheduler` | Frame timing, async task orchestration | Rendering, layout |
| `graphics` | Color resolution, style application, border drawing | Cell storage, layout |
| `protocol` | ANSI escape sequence generation | Terminal I/O (just generates strings) |
| `selection` | Text selection ranges, selection rendering | Text editing, input handling |
| `clipboard` | System clipboard read/write | Text editing, selection |
| `editor` | Rope-based text buffer, cursor management | Rendering, input handling |
| `plugin` | Plugin registration, extension point management | Widget implementations |
| `capabilities` | Terminal feature detection, protocol negotiation | Terminal I/O |
| `ffi` | napi-rs binding surface | All business logic |
| `benchmark` | Performance measurement, profiling | All business logic |
| `error` | Error types, recovery strategies | All business logic |

### 6.2 Module Dependency Graph

```
                    ┌─────────┐
                    │   ffi   │
                    └────┬────┘
                         │
                    ┌────▼────┐
                    │  tree   │
                    └────┬────┘
                         │
              ┌──────────┼──────────┐
              │          │          │
         ┌────▼───┐ ┌───▼────┐ ┌──▼──────┐
         │ layout │ │ events │ │ render  │
         └────┬───┘ └───┬────┘ └────┬────┘
              │          │          │
              │     ┌────▼────┐     │
              │     │  input  │     │
              │     └────┬────┘     │
              │          │          │
         ┌────▼──────────▼──────────▼────┐
         │         framebuffer           │
         └───────────────┬───────────────┘
                         │
              ┌──────────┼──────────┐
              │          │          │
         ┌────▼───┐ ┌───▼────┐ ┌──▼──────┐
         │protocol│ │graphics│ │terminal │
         └────────┘ └────────┘ └─────────┘
```

### 6.3 Module Visibility Rules

- Each module exposes a public API through its `mod.rs`.
- Internal implementation details are private.
- Cross-module communication happens through well-defined interfaces, not direct field access.
- The `tree` module is the foundation — most other modules depend on it.
- The `ffi` module is the only module that depends on `napi` and `napi-derive`.

## 7. Key Design Decisions

### 7.1 Arena Allocation for Nodes

**Decision:** Use `slotmap` for arena-based node storage.

**Why:** Arena allocation provides O(1) node access, cache-friendly iteration, and automatic cleanup when the arena is dropped. `slotmap` specifically provides generational indices that prevent use-after-free bugs.

**Trade-off:** Arena allocation means nodes cannot be individually freed. The entire arena is freed at once. This is acceptable because UI trees are typically short-lived (rebuilt on each render cycle) and the arena is cleared between frames.

**Alternative considered:** Reference-counted nodes (`Rc<Node>`). Rejected because it introduces cycles (parent-child references), requires cycle detection, and has higher per-node overhead.

### 7.2 Command-Based Protocol

**Decision:** Use a command-based protocol between TypeScript and Rust, not direct FFI calls per operation.

**Why:** Batching commands reduces FFI overhead. A single FFI call with 100 commands is cheaper than 100 individual FFI calls. Commands can be serialized, logged, and replayed for debugging.

**Trade-off:** Commands add serialization overhead. For small trees, this overhead may exceed the benefit of batching. We mitigate this by having a fast path for single-command batches.

**Alternative considered:** Direct struct sharing via napi-rs `#[napi(object)]`. Rejected because it couples TypeScript and Rust struct layouts, making evolution difficult.

### 7.3 Layout via Taffy

**Decision:** Use Facebook's Taffy library for flexbox layout.

**Why:** Taffy is the same layout engine used by Leptos, Dioxus, and other Rust UI frameworks. It implements CSS flexbox and grid correctly. It has layout caching built in. It is well-tested and maintained.

**Trade-off:** Taffy is designed for pixel-based layouts, not terminal cells. We must adapt its output (pixel values) to terminal grid positions. This is a well-understood mapping (1 cell = 1 character width × 1 line height).

**Alternative considered:** Writing a custom layout engine. Rejected because flexbox is complex (the CSS spec is 200+ pages) and Taffy already solves this correctly.

### 7.4 Double-Buffered Frame Output

**Decision:** Use double buffering with dirty-region diffing.

**Why:** Terminal I/O is slow. Writing only changed cells dramatically reduces output volume. Double buffering prevents flickering by computing the entire frame before writing.

**Trade-off:** Double buffering doubles memory usage for the frame buffer. For a 200×50 terminal, this is 10,000 cells × ~64 bytes/cell = ~640KB. This is negligible.

**Alternative considered:** Single-buffered direct writes. Rejected because it causes visible flickering during frame updates.

### 7.5 Framework Adapters as Separate Packages

**Decision:** Each framework gets its own package, not a single adapter with framework detection.

**Why:** Each framework has fundamentally different reactivity models. React uses virtual DOM diffing. Solid uses fine-grained signals. Vue uses proxy-based reactivity. Trying to unify these into a single adapter would create a leaky abstraction.

**Trade-off:** More packages to maintain. Each adapter must be updated when the core API changes.

**Alternative considered:** A single `@bettertui/framework` package with adapter plugins. Rejected because the adapter logic is framework-specific and should not be bundled with other frameworks' concerns.

### 7.6 Rust as the Performance Layer

**Decision:** All performance-critical code in Rust, not C++ or Zig.

**Why:** Rust provides memory safety without garbage collection. It has excellent FFI support via napi-rs. The ecosystem includes Taffy (layout), crossterm (terminal), ropey (text), and unicode-width (text measurement). The community is strong and growing.

**Trade-off:** Rust has a steeper learning curve than Zig or C. Compile times are slower. napi-rs is younger than Zig's FFI story.

**Alternative considered:** Zig (like OpenTUI). Rejected because Zig's FFI story with Node.js is less mature than napi-rs, and Rust's ecosystem for the specific libraries we need (Taffy, ropey) is stronger.

## 8. Future Considerations

### 8.1 GPU Acceleration

The rendering pipeline is designed to support GPU-accelerated rendering in the future. The framebuffer abstraction is backend-agnostic — a GPU backend would replace the ANSI encoder with a shader-based renderer. The layout and event systems remain unchanged.

### 8.2 Remote Rendering

The command-based protocol can be transmitted over WebSocket, enabling remote development tools. The DevTools UI could run in a browser while the application runs on a remote server.

### 8.3 Multi-Process Architecture

The scheduler can be extended to support multi-process rendering, where different parts of the UI run in separate processes. This is useful for fault isolation in long-running applications.

### 8.4 WebAssembly

The Rust engine could be compiled to WebAssembly for browser-based terminal emulators. The terminal backend would be replaced with a canvas-based renderer. The layout and event systems remain unchanged.

## 9. Document Index

| Document | Description |
|----------|-------------|
| [NodeModel.md](NodeModel.md) | Internal UI node model and arena allocation |
| [Protocol.md](Protocol.md) | Rust ↔ TypeScript command protocol |
| [RenderingPipeline.md](RenderingPipeline.md) | From framework to terminal output |
| [Layout.md](Layout.md) | Layout system using Taffy |
| [FrameBuffer.md](FrameBuffer.md) | Cell-based frame buffer |
| [EventSystem.md](EventSystem.md) | Event dispatch, bubbling, capture |
| [InputSystem.md](InputSystem.md) | Keyboard, mouse, clipboard handling |
| [Animation.md](Animation.md) | Animation system design |
| [WidgetModel.md](WidgetModel.md) | Widget abstraction and composition |
| [PluginAPI.md](PluginAPI.md) | Plugin system and extension points |
| [Threading.md](Threading.md) | Threading model and synchronization |
| [MemoryModel.md](MemoryModel.md) | Memory management and allocation strategy |
| [Performance.md](Performance.md) | Performance strategy and budgets |
| [Roadmap.md](Roadmap.md) | Phased implementation roadmap |
