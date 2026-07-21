# DevTools

BetterTUI's developer tooling ships **inside `@bettertui/core`** (module
`@bettertui/core` → `src/devtools/`). The standalone `@bettertui/devtools`
package has been retired; import from `@bettertui/core`.

## Exports

| Export | Type | Notes |
|--------|------|-------|
| `createDevTools(options?)` | `function` | Returns a `DevTools` instance. With `enabled: true` it is fully functional; otherwise a no-op instance with near-zero overhead. |
| `DevTools` | `interface` | The instance shape (inspectors, panel control, query methods, record methods, export methods). |
| `DevToolsOptions` / `CreateDevToolsOptions` | `interface` | `{ enabled?, maxEvents?, logging?, logLevel?, timeline?, performance?, snapshots? }` |
| `DebugPanel` | `enum` | `Performance`, `Tree`, `Layout`, `Events`, `DirtyRegions`. |
| `OverlayHost` | `class` | Per-frame ANSI overlay compositor used by `CliRenderer`. |
| `Panel`, `PanelContext` | `interface` | Panel contract (`(state) → string[] lines`). |
| `ansi` | `namespace` | ANSI helpers (`moveTo`, `drawBox`, `truncate`, `sparkline`, …). |
| `DevToolsLogger` | `class` | Structured logger with a bounded entry buffer (aliased to avoid colliding with the platform `Logger`). |
| `CommandInspector` | `class` | Records every command emitted, with `maxCommands` bound. |
| `EventInspector` | `class` | Tracks keyboard, mouse, focus, and resize events. |
| `PerformanceTracker` | `class` | Frame timing, FPS, and dropped-frame metrics. |
| `TreeInspector` | `class` | Render-tree capture and node lookup. |
| `SchedulerInspector` | `class` | Frame budget, drops, and callback state. |
| `FocusInspector` | `class` | Focused node, tab order, and scopes. |
| `CapabilityInspector` | `class` | Terminal capability snapshot. |
| `Timeline` | `class` | Chronological event recording. |
| `SnapshotManager` | `class` | Capture and compare tree snapshots. |
| `createExport` / `exportToJson` / `createSummary` | `function` | Build a `DiagnosticExport` and serialise it. |
| `LogEntry`, `RecordedCommand`, `RecordedEvent`, `FrameMetrics`, `TreeSnapshot`, `SnapshotDiff`, `DiagnosticExport`, `MemoryStats`, … | `type` | Supporting payload shapes. |

## Usage

```ts
import { createDevTools } from "@bettertui/core";

const devtools = createDevTools({ enabled: true });
devtools.recordFrame({ duration: 4.2, commandCount: 12 });
const summary = devtools.getSummary();
```

Or let the renderer own it:

```ts
import { createCliRenderer, DebugPanel } from "@bettertui/core";

const renderer = await createCliRenderer({ debug: true });
renderer.toggleDebugOverlay(DebugPanel.Performance);
```

## Status

Implemented and in-core. Panel 5 per-cell dirty highlighting and Panel 2
click-to-inspect are deferred to a later Rust phase (they need napi cell-buffer
access).
