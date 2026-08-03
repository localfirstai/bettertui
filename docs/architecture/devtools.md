# DevTools Architecture

> **See also:** [DevTools API & Usage](../devtools.md) · [API reference](../api/packages/devtools.md)

BetterTUI's debug tooling lives **inside `@bettertui/core`** (under
`src/devtools/`). It provides debugging, profiling, inspection, and
diagnostics comparable to React DevTools, Flutter DevTools, and Chrome
DevTools — integrated directly with the renderer so an overlay can composite
onto the frame and read engine internals without any external process.

The standalone `@bettertui/devtools` package has been retired; everything is
re-exported from `@bettertui/core`. See the
[task plan](../../tasks/devtools-in-core-plan.md) for the migration history.

---

## Design Principles

1. **Built-in, not bolt-on.** `CliRenderer` creates DevTools automatically when
   `debug: true`. No separate install, no IPC, no browser.
2. **Near-zero overhead when disabled.** `createDevTools()` without
   `enabled: true` returns a no-op instance. Every `record*` method is a
   no-op; the overlay is never constructed; ring buffers are never allocated.
3. **Composable inspectors.** Each inspector is an independent module. You can
   import and use `PerformanceTracker` or `Logger` standalone without the
   full `DevTools` facade.
4. **Pure-TS overlay.** The Rust engine returns finished base64 ANSI bytes —
   there is no TypeScript-accessible cell buffer. The debug overlay is
   absolute-positioned ANSI text written _after_ the engine output via
   `CliRenderer.writeFrame()`. No Rust changes are needed to add a new panel.
5. **Exportable.** All diagnostic data can be serialised to a `DiagnosticExport`
   JSON blob for offline analysis or issue reports.

---

## Source Layout

```
packages/core/src/devtools/
│
├── index.ts                  # createDevTools() facade + DevTools interface
├── devtools.types.ts         # all shared payload types
│
├── logger.ts                 # ring-buffer structured logger
├── commandInspector.ts       # records every Command emitted through CommandBuffer
├── eventInspector.ts         # keyboard / mouse / focus / resize event log
├── performance.ts            # frame-timing metrics, FPS, dropped frames
├── treeInspector.ts          # render-tree snapshot capture + node lookup
├── schedulerInspector.ts     # frame budget and scheduler state
├── focusInspector.ts         # current focus, tab order, traversal history
├── capabilityInspector.ts    # terminal capability snapshot (wraps TerminalCapabilities)
├── timeline.ts               # chronological event log with filtering
├── snapshot.ts               # named tree captures + two-snapshot diff
├── export.ts                 # DiagnosticExport builder + JSON serialiser
│
└── overlay/
    ├── overlayHost.ts        # per-frame ANSI compositor + region-tracking clear
    ├── ansi.utils.ts         # moveTo / drawBox / truncate / sparkline helpers
    ├── panel.types.ts        # DebugPanel enum, Panel interface, PanelContext
    └── panels/
        ├── performance.panel.ts    # Panel: Performance + Render Stats
        ├── tree.panel.ts           # Panel: Render Tree (display-only)
        ├── layout.panel.ts         # Panel: Layout (node geometry)
        ├── events.panel.ts         # Panel: Recent Events
        └── dirtyRegions.panel.ts   # Panel: Dirty-Region stats (count only)
```

---

## Module Responsibilities

| Module                   | Purpose                                                                                                                                                                                                                 |
| ------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `devtools.types.ts`      | Single source of truth for all payload types: `LogEntry`, `RecordedCommand`, `RecordedEvent`, `FrameMetrics`, `PerformanceSnapshot`, `TreeSnapshot`, `SnapshotDiff`, `DiagnosticExport`, `MemoryStats`, etc.            |
| `logger.ts`              | Ring-buffer structured logger. Levels: trace / debug / info / warn / error. Named categories, level filtering, `getEntriesByCategory()`, exportable.                                                                    |
| `commandInspector.ts`    | Wraps every flush of `CommandBuffer`. Records command type, payload size, and emit duration. Configurable `maxCommands` ring.                                                                                           |
| `eventInspector.ts`      | Tracks keyboard, mouse, focus, resize, and lifecycle events with timestamps. Configurable `maxEvents` ring.                                                                                                             |
| `performance.ts`         | Sliding-window frame-timing ring. Computes current FPS, min/max/avg frame time, and dropped-frame count. `startProfiling()` / `stopProfiling()` accumulate session stats.                                               |
| `treeInspector.ts`       | Records render-tree snapshots on demand. Stores node id, kind, props, layout geometry (x/y/width/height), and children recursively. `getNode(id)` for targeted lookup.                                                  |
| `schedulerInspector.ts`  | Mirrors `NativeScheduler` state: frame budget used, is-idle, pending callbacks, animation frame request queue depth.                                                                                                    |
| `focusInspector.ts`      | Wraps `NativeFocusManager`. Records focus/blur history, exposes current focused id and ordered tab list.                                                                                                                |
| `capabilityInspector.ts` | One-shot snapshot of `TerminalCapabilities` (true-color, Kitty, Sixel, OSC-52, mouse, etc.). Updated on resize/re-detect.                                                                                               |
| `timeline.ts`            | Chronological log of commands, renders, and events in insertion order. Filterable by type. Used by the export layer.                                                                                                    |
| `snapshot.ts`            | `captureSnapshot(name)` freezes the tree; `diffSnapshots(a, b)` returns a `SnapshotDiff` highlighting added/removed/changed nodes.                                                                                      |
| `export.ts`              | `createExport()` assembles a `DiagnosticExport` from all active inspectors. `exportToJson()` serialises it. `createSummary()` returns a human-readable text summary.                                                    |
| `overlay/overlayHost.ts` | Draws visible panels on each frame using absolute ANSI positioning. Tracks previously-painted regions; clears rows vacated since the last frame to prevent trails. Forces `renderFull()` when the last panel is hidden. |
| `overlay/ansi.utils.ts`  | Stateless helpers: `moveTo(row, col)`, `drawBox(...)`, `truncate(str, width)`, `sparkline(values)`, color escape builders.                                                                                              |
| `overlay/panels/`        | Each panel is a pure function `(PanelContext) → string[]` (one string per visible line). The `OverlayHost` calls each visible panel and writes the returned lines at their assigned position.                           |

---

## Overlay Data Flow

```
CliRenderer.writeFrame(ansiBytes)
    │
    ├─ 1. write engine ANSI output to stdout
    │
    ├─ 2. devtools.recordFrame({ duration, commandCount, dirtyRegionCount })
    │        → PerformanceTracker updates ring buffer
    │        → Timeline records a "render" entry
    │
    └─ 3. overlay.paint(ctx)
             │
             ├─ save cursor (ESC[s)
             ├─ for each visible DebugPanel:
             │      panel.render(ctx) → string[] lines
             │      write lines at absolute position (ESC[row;colH)
             ├─ clear rows vacated since previous frame (ESC[2K on each row)
             └─ restore cursor (ESC[u)
```

When `CliRenderer` is created with `debug: true`:

```ts
import { createCliRenderer, DebugPanel } from "@bettertui/core";

const renderer = await createCliRenderer({ debug: true });
// Toggle a panel with a key binding:
renderer.toggleDebugOverlay(DebugPanel.Performance);
// Access all inspectors:
const fps = renderer.devtools.performance.currentFps();
```

`BTUI_DEBUG=1` and `BTUI_SHOW_STATS=1` environment variables force the overlay
on at startup.

---

## Panels

| Panel (`DebugPanel` enum) | Content                                                                                       |
| ------------------------- | --------------------------------------------------------------------------------------------- |
| `Performance`             | Current FPS, avg/min/max frame time (ms), dropped frames, command count per frame, RSS memory |
| `Tree`                    | Render tree node hierarchy (display-only; click-to-inspect deferred, see §Deferred)           |
| `Layout`                  | Per-node layout geometry: id, kind, x/y, width × height                                       |
| `Events`                  | Last N keyboard / mouse / focus / resize events with timestamps                               |
| `DirtyRegions`            | Per-frame dirty-region count (per-cell highlighting deferred, see §Deferred)                  |

---

## Integration with CliRenderer

`CliRenderer` is the only type that constructs `DevTools` and `OverlayHost`
directly; application code always accesses devtools through
`renderer.devtools`. The renderer wires the inspectors into its frame loop:

```
beginFrame()
  └─ scheduler.beginFrame()

commitFrame()
  ├─ engine.commitFrame()           → Rust renders ANSI bytes
  ├─ writeFrame(ansiBytes)          → stdout + devtools.recordFrame(...)
  └─ overlay.paint()                → debug panels drawn over output
```

Calling `renderer.toggleDebugOverlay(panel)` sets the panel's visibility
in `OverlayHost`. If all panels are hidden, the host forces a `renderFull()`
to repaint any rows the overlay had occupied.

---

## Performance Characteristics

- All inspectors use **ring buffers** with configurable max sizes (`maxCommands`,
  `maxEvents`, `maxFrames`). Entries older than the ring size are dropped
  silently — no allocation growth over time.
- When `enabled: false`, `createDevTools()` returns a **no-op stub**. Every
  `record*` call is a single property check and immediate return. The
  `OverlayHost` is never constructed.
- `record*` methods are **synchronous and non-allocating** in the hot path
  (the ring slot is pre-allocated at construction time; the entry object is
  mutated in place).
- Panel rendering happens **after** the engine bytes hit stdout, so it never
  adds to the engine's frame deadline.

---

## Deferred to a Later Rust Phase

Three capabilities require napi cell-buffer exposure that does not yet exist:

1. **Per-cell dirty-region highlighting** (Panel 5 stats only today) — full
   highlighting needs the engine to expose the dirty cell bitmap per frame via
   napi so `OverlayHost` can draw coloured overlays.
2. **Panel 2 click-to-inspect** — routing a mouse click through the hit grid
   to the matching render-tree node (the `hitGridCheck` API exists in the
   engine; the TS glue and panel UI are missing).
3. **Interactive console overlay** — mouse-driven text selection, copy, and
   scroll within the log panel; the first pass is a plain log-backed `console`
   surface.

---

## Adding a New Panel

1. Create `overlay/panels/my-feature.panel.ts` exporting a function
   `MyFeaturePanel: Panel = (ctx: PanelContext) => string[]`.
2. Add the enum variant to `DebugPanel` in `overlay/panel.types.ts`.
3. Register the panel in `OverlayHost` alongside the others.
4. If the panel needs new data, add a corresponding inspector module and wire
   it through `DevTools` in `index.ts`.

No Rust changes are required for a new panel.
