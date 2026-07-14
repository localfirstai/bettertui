# DevTools

`@bettertui/devtools` ships a working developer-tools package. It is **not** a stub.

## Exports

| Export | Type | Notes |
|--------|------|-------|
| `createDevTools(options?)` | `function` | Returns a `DevTools` instance. With `enabled: true` it is fully functional; otherwise a no-op instance with near-zero overhead. |
| `DevTools` | `interface` | The instance shape (inspectors, record methods, export methods). |
| `CreateDevToolsOptions` | `interface` | `{ enabled?, maxEvents?, logging?, logLevel?, timeline?, performance?, snapshots? }` |
| `Logger` | `class` | Structured logger with a bounded entry buffer. |
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
| `LogEntry`, `RecordedCommand`, `RecordedEvent`, `FrameMetrics`, `TreeSnapshot`, `SnapshotDiff`, `DiagnosticExport`, … | `type` | Supporting payload shapes. |

## Usage

```ts
import { createDevTools } from "@bettertui/devtools";

const devtools = createDevTools({ enabled: true });
devtools.recordFrame({ duration: 4.2, commandCount: 12 });
const summary = devtools.getSummary();
```

## Status

Implemented. The `Scheduler` already produces `SchedulerStats` (`frame_count`, `dropped_frames`, `avg_frame_time`, `frame_budget`) that the `PerformanceTracker` consumes.
