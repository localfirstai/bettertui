# Architecture

This document describes the internal architecture of BetterTUI.

## Dependency Flow

```
┌─────────────────────┐
│   Framework Adapter  │  ← React, Vue, Solid, Svelte...
│  (packages/react/)   │
└─────────┬───────────┘
          │
┌─────────▼───────────┐
│     @bettertui/core  │  ← Shared types, tree diffing
│  (packages/core/)    │
└─────────┬───────────┘
          │
┌─────────▼───────────┐
│  Rust Native Engine  │  ← Rendering, layout, events
│ (native/engine/)     │
└─────────┬───────────┘
          │
┌─────────▼───────────┐
│      Terminal        │  ← crossterm I/O
└─────────────────────┘
```

## Principles

### Framework Agnosticism

The Rust engine has zero knowledge of React, Vue, or any UI framework. It exposes a generic tree-based rendering API. Framework adapters translate their component trees into this generic format.

### Separation of Concerns

- **Rust** handles all performance-critical operations (rendering, layout, input, scheduling)
- **TypeScript** handles API design, developer experience, and framework bindings
- **No business logic** lives in the engine — it is purely a rendering framework

### Zero-Config Performance

The engine automatically handles:
- Dirty rect diffing (only redraw changed regions)
- Frame scheduling (60fps target with vsync)
- Layout caching (incremental recalculation)
- Memory pooling (reuse allocations)

## Engine Modules

| Module | Responsibility |
|--------|---------------|
| `renderer` | Frame buffer composition and terminal output |
| `layout` | Flexbox/CSS layout via Taffy |
| `scheduler` | Async task scheduling and frame timing |
| `terminal` | Terminal size, capabilities, raw mode |
| `framebuffer` | Cell-based frame buffer |
| `events` | Event dispatch and routing |
| `keyboard` | Key parsing and modifier tracking |
| `mouse` | Mouse button, position, and drag tracking |
| `selection` | Text selection with range management |
| `clipboard` | System clipboard read/write |
| `editor` | Rope-based text buffer with cursor |
| `animation` | Tween engine with keyframes |
| `widgets` | Native widget implementations |
| `graphics` | Color, style, and border rendering |
| `screen` | Multi-layer compositing |
| `protocol` | ANSI escape sequence handling |
| `capabilities` | Terminal feature detection |
| `benchmark` | Performance measurement harness |
| `ffi` | Foreign function interface bridge |

## React Integration

BetterTUI uses a **custom React reconciler** (not Ink). The reconciler translates React's virtual DOM operations into BetterTUI's native tree format:

1. React calls reconciler host config methods (createInstance, appendChild, etc.)
2. Reconciler maintains an internal tree of BetterTUI nodes
3. On commit, dirty nodes are diffed against the previous frame
4. Changed nodes produce render commands
5. Commands are batched and sent to the Rust engine via napi-rs

This approach gives React full control over component lifecycle while the engine handles all rendering performance.

## Data Flow

```
User Code (React components)
    ↓ (React render)
Reconciler (host config)
    ↓ (tree diff)
BetterTUI Tree (@bettertui/core)
    ↓ (napi-rs bridge)
Rust Engine
    ↓ (layout + render)
Terminal Output
```
