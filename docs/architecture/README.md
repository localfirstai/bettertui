# BetterTUI Architecture Summary

> Executive summary of the complete BetterTUI architecture.
> This document links to all detailed design documents.

## System Architecture

BetterTUI is a **framework-agnostic, Rust-powered terminal UI rendering engine** with JavaScript bindings. It consists of three layers:

1. **Framework Adapters** (TypeScript) — Translate framework-specific component trees into generic node operations.
2. **Core TypeScript Layer** — Node model, protocol codec, event bridge.
3. **Rust Engine** — Arena allocation, layout, rendering, events, terminal I/O.

## Design Documents

| Document | Description | Key Decision |
|----------|-------------|--------------|
| [Architecture.md](Architecture.md) | Overall system architecture | Three-layer model with framework adapters |
| [NodeModel.md](NodeModel.md) | Internal UI node model | Arena-allocated, generational-indexed nodes |
| [Protocol.md](Protocol.md) | Rust ↔ TypeScript protocol | Command-based, batch-oriented, synchronous |
| [RenderingPipeline.md](RenderingPipeline.md) | Rendering stages | 11-stage pipeline from React to terminal |
| [Layout.md](Layout.md) | Layout system | Taffy-based flexbox adapted for terminal cells |
| [FrameBuffer.md](FrameBuffer.md) | Frame buffer design | Double-buffered, cell-based, dirty tracking |
| [EventSystem.md](EventSystem.md) | Event dispatch | DOM-style capture → target → bubble |
| [InputSystem.md](InputSystem.md) | Input handling | Keyboard, mouse, paste, Kitty protocol |
| [Animation.md](Animation.md) | Animation system | Tween engine with keyframes and springs |
| [WidgetModel.md](WidgetModel.md) | Widget abstraction | Composable factories that produce node trees |
| [PluginAPI.md](PluginAPI.md) | Plugin system | Extension points at every level |
| [Threading.md](Threading.md) | Threading model | Single-threaded first, parallel-ready |
| [MemoryModel.md](MemoryModel.md) | Memory management | Arena allocation, pooling, zero leaks |
| [Performance.md](Performance.md) | Performance strategy | 60fps target, dirty tracking, caching |
| [Roadmap.md](Roadmap.md) | Implementation roadmap | 15 phases over ~6 months |

## Key Architecture Decisions

### 1. Framework Agnosticism

The Rust engine has zero knowledge of React, Vue, Solid, or any UI framework. It exposes a generic node tree protocol. Framework adapters translate their component trees into this protocol.

**Why:** If we couple to React, we cannot support Vue or Solid without rewriting the engine. If we couple to any specific reactivity model, we limit which frameworks can use us.

### 2. Arena-Based Node Storage

Nodes are stored in a `slotmap::SlotMap` with generational indices.

**Why:** Arena allocation provides O(1) access, cache-friendly iteration, and automatic cleanup. Generational indices prevent use-after-free bugs.

### 3. Command-Based Protocol

TypeScript and Rust communicate via batched commands.

**Why:** Batching reduces FFI overhead. A single FFI call with 100 commands is cheaper than 100 individual calls. Commands can be logged, replayed, and inspected.

### 4. Taffy for Layout

We use Facebook's Taffy library for flexbox layout.

**Why:** Taffy is the same engine used by Leptos and Dioxus. It implements CSS flexbox correctly, has layout caching, and is well-tested.

### 5. Double-Buffered Rendering

We use double buffering with dirty-region diffing.

**Why:** Terminal I/O is slow. Writing only changed cells dramatically reduces output volume. Double buffering prevents flickering.

### 6. DOM-Style Events

Events propagate through capture → target → bubble phases.

**Why:** DOM events are a well-understood, battle-tested model. Every web developer knows how `onClick` and `stopPropagation` work.

## Package Responsibilities

### Current Packages

| Package | Role | Status |
|---------|------|--------|
| `@bettertui/shared` | Framework-agnostic type definitions | Types complete |
| `@bettertui/core` | Node model, protocol, tree operations | Types only |
| `@bettertui/reconciler` | React reconciler host config | Type stubs |
| `@bettertui/react` | React component API | Stub components |
| `@bettertui/widgets` | Widget library | Interface only |
| `@bettertui/themes` | Theme system | Partially implemented |
| `@bettertui/icons` | Icon registry | Working, empty |
| `@bettertui/devtools` | Developer tools | Stub |

### Proposed Packages

| Package | Reason |
|---------|--------|
| `@bettertui/protocol` | Protocol codec for alternative transports |
| `@bettertui/renderer` | TypeScript renderer wrapper |
| `@bettertui/hooks` | Framework-agnostic hook utilities |
| `@bettertui/testing` | Headless renderer, mock input, snapshots |
| `@bettertui/animations` | Animation API backed by Rust engine |
| `@bettertui/editor` | Text editor component backed by Rust rope |
| `@bettertui/graphics` | Canvas-like API backed by Rust framebuffer |

## Rust Module Structure

```
bettertui-engine
├── tree          — Arena-based node storage
├── layout        — Taffy integration
├── render        — Frame production, dirty diffing
├── framebuffer   — Cell grid, double buffering
├── terminal      — Size detection, capabilities
├── input         — Keyboard, mouse, paste parsing
├── events        — Event dispatch, bubbling
├── animation     — Tween engine, keyframes
├── scheduler     — Frame timing, async tasks
├── graphics      — Color, style, border rendering
├── protocol      — ANSI escape sequence generation
├── selection     — Text selection management
├── clipboard     — System clipboard integration
├── editor        — Rope-based text buffer
├── plugin        — Plugin host, extension points
├── capabilities  — Terminal feature detection
├── ffi           — napi-rs binding surface
├── benchmark     — Performance measurement
└── error         — Error types and recovery
```

## Performance Targets

| Metric | Target |
|--------|--------|
| Frame rate | 60fps (16.67ms) |
| Input latency | <1ms |
| Layout time | <5ms (1000 nodes) |
| Render time | <10ms (10000 cells) |
| Memory per node | <256 bytes |
| Total engine memory | <10MB |

## Implementation Timeline

| Phase | Duration | Deliverable |
|-------|----------|-------------|
| 0: Architecture | 1 week | Design documents (this) |
| 1: Engine Core | 2-3 weeks | Terminal I/O, frame buffer |
| 2: Node Tree | 2-3 weeks | Arena-based node tree |
| 3: Layout Engine | 2-3 weeks | Taffy flexbox layout |
| 4: Event System | 2 weeks | Keyboard, mouse input |
| 5: Protocol Layer | 2-3 weeks | TypeScript ↔ Rust communication |
| 6: React Integration | 3-4 weeks | Working React components |
| 7: Widgets | 3-4 weeks | Reusable widget library |
| 8: Rendering Pipeline | 2-3 weeks | Diffing, optimization |
| 9: Theming | 1-2 weeks | Theme system |
| 10: Animation | 2-3 weeks | Tween engine |
| 11: DevTools | 2-3 weeks | Inspector, profiler |
| 12: Advanced Features | 3-4 weeks | Clipboard, selection, editing |
| 13: Plugin System | 2-3 weeks | Extensibility |
| 14: Documentation | 2-3 weeks | Docs and examples |
| 15: Multi-Framework | 4-6 weeks | Vue, Solid, Svelte adapters |
| **Total** | **~6 months** | **Complete framework** |

## Next Steps

1. **Review this architecture** with the team.
2. **Prioritize Phase 1** — Engine Core.
3. **Set up CI/CD** for automated testing.
4. **Begin implementation** of terminal module and frame buffer.
