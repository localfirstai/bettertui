# @bettertui/devtools — Architecture

## Overview

`@bettertui/devtools` is a production-ready developer toolkit for BetterTUI applications. It provides debugging, profiling, inspection, and diagnostics capabilities comparable to React DevTools, Flutter DevTools, and Chrome DevTools — while remaining fully integrated with BetterTUI's Rust + TypeScript architecture.

## Design Principles

1. **Framework-agnostic**: Operates at the BetterTUI runtime level. No React dependency.
2. **Near-zero overhead when disabled**: Instrumentation only activates when requested.
3. **Composable inspectors**: Each inspector is an independent module that can be used standalone or composed together.
4. **Event-driven**: Inspectors subscribe to runtime events and record them for analysis.
5. **Exportable**: All diagnostic data can be serialized and exported.

## Architecture

```
┌─────────────────────────────────────────────────┐
│                  DevTools Instance               │
│                                                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐      │
│  │ Logger   │  │ Timeline │  │ Snapshot │      │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘      │
│       │              │              │             │
│  ┌────┴──────────────┴──────────────┴────┐      │
│  │           Event Bus (internal)        │      │
│  └────┬──────┬──────┬──────┬──────┬──────┘      │
│       │      │      │      │      │              │
│  ┌────┴─┐ ┌──┴──┐ ┌─┴──┐ ┌─┴──┐ ┌─┴──────┐    │
│  │Cmd   │ │Evt  │ │Perf│ │Tree│ │Focus   │    │
│  │Insp  │ │Insp │ │Trk │ │Insp│ │Insp    │    │
│  └──────┘ └─────┘ └────┘ └────┘ └────────┘    │
│                                                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐      │
│  │Scheduler │  │Terminal  │  │ Export   │      │
│  │Inspector │  │Caps Insp │  │          │      │
│  └──────────┘  └──────────┘  └──────────┘      │
└─────────────────────────────────────────────────┘
```

## Modules

| Module | Purpose |
|--------|---------|
| `types.ts` | Core type definitions shared across all modules |
| `logger.ts` | Structured logging with levels, filtering, and search |
| `command-inspector.ts` | Records every command emitted through the CommandBuffer |
| `event-inspector.ts` | Tracks keyboard, mouse, focus, resize, and lifecycle events |
| `performance.ts` | Frame timing, FPS calculation, memory usage, render cost |
| `tree-inspector.ts` | Render tree, component hierarchy, props, styles, layout |
| `scheduler-inspector.ts` | Frame budget, dropped frames, idle callbacks, animations |
| `focus-inspector.ts` | Current focus, tab order, focus scopes, traversal |
| `capability-inspector.ts` | Terminal capability detection and reporting |
| `timeline.ts` | Chronological event recording with filtering |
| `snapshot.ts` | Capture and compare tree/layout states |
| `export.ts` | Serialize and export diagnostic data |

## Data Flow

1. **Instrumentation**: The host application calls `devtools.record*()` methods or the DevTools hooks into the CommandBuffer/EventBus.
2. **Collection**: Each inspector records events into its internal buffer.
3. **Analysis**: Inspectors provide query methods for real-time analysis.
4. **Export**: The `export` module serializes all collected data.

## Performance

- All inspectors use ring buffers with configurable max sizes.
- When `enabled: false`, `createDevTools()` returns a no-op implementation.
- Recording methods are synchronous and non-allocating in the hot path.
