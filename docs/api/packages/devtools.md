# DevTools

BetterTUI's developer tooling ships **inside `@bettertui/core`** at `src/devtools/`. The standalone `@bettertui/devtools` package has been retired; import from `@bettertui/core`.

## Exports

| Export | Type | Notes |
|--------|------|-------|
| `createDevTools(options?)` | function | Returns `DevTools` instance (no-op when `enabled: false`) |
| `DevTools` | interface | Inspector facade |
| `DevToolsOptions` | interface | `{ enabled?, maxEvents?, logging?, logLevel?, timeline?, performance?, snapshots? }` |
| `DebugPanel` | enum | `Performance`, `Tree`, `Layout`, `Events`, `DirtyRegions` |
| `OverlayHost` | class | Per-frame ANSI overlay compositor |
| `Panel`, `PanelContext` | interface | Panel contract |
| `DevToolsLogger` | class | Ring-buffer structured logger |
| `CommandInspector` | class | Records commands emitted through `CommandBuffer` |
| `EventInspector` | class | Keyboard/mouse/focus/resize event log |
| `PerformanceTracker` | class | Frame timing, FPS, dropped frames |
| `TreeInspector` | class | Render-tree capture and node lookup |
| `SchedulerInspector` | class | Frame budget and scheduler state |
| `FocusInspector` | class | Focused node, tab order, scopes |
| `CapabilityInspector` | class | Terminal capability snapshot |
| `Timeline` | class | Chronological event recording |
| `SnapshotManager` | class | Capture and compare tree snapshots |
| `createExport`, `exportToJson`, `createSummary` | function | DiagnosticExport builder |

## See also

- [DevTools doc](../../devtools.md) for usage and examples
- [Architecture: DevTools](../../architecture/devtools.md)
