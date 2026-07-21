# In-Core Debug Tooling — Architecture

## Overview

BetterTUI's debug tooling lives **inside `@bettertui/core`** (under `src/devtools/`). It
provides debugging, profiling, inspection, and diagnostics comparable to React
DevTools, Flutter DevTools, and Chrome DevTools — integrated directly with the
renderer so an overlay can composite onto the frame and read engine internals.

It was previously the standalone `@bettertui/devtools` package; that package has
been retired and its surface re-exported from `@bettertui/core` (see the
[task plan](../../../../tasks/devtools-in-core-plan.md)).

## Design Principles

1. **Built-in, not bolt-on**: `CliRenderer` creates DevTools when `debug: true`.
2. **Near-zero overhead when disabled**: `createDevTools()` returns a no-op
   implementation unless `enabled: true`.
3. **Composable inspectors**: each inspector is an independent module usable
   standalone or via the `DevTools` facade.
4. **Pure-TS overlay**: the engine returns finished base64 ANSI bytes (there is
   no TS-accessible cell buffer), so the overlay is absolute-positioned ANSI
   written *after* the engine output, via `CliRenderer.writeFrame()`.
5. **Exportable**: all diagnostic data can be serialized and exported.

## Layout

```
packages/core/src/devtools/
  index.ts                 # createDevTools() facade + DevTools interface
  devtools.types.ts        # shared payload types
  logger.ts timeline.ts snapshot.ts export.ts
  commandInspector.ts eventInspector.ts performance.ts
  treeInspector.ts schedulerInspector.ts focusInspector.ts
  capabilityInspector.ts
  overlay/
    overlayHost.ts         # per-frame ANSI compositing + region-tracking clear
    ansi.utils.ts          # cursor/box/truncate/sparkline helpers
    panel.types.ts         # DebugPanel enum, Panel interface
    panels/
      performance.panel.ts # Panel 1 + 6 (performance + render stats)
      tree.panel.ts        # Panel 2 (display-only)
      layout.panel.ts      # Panel 3
      events.panel.ts      # Panel 4
      dirtyRegions.panel.ts # Panel 5 (stats-level)
```

## Modules

| Module | Purpose |
|--------|---------|
| `devtools.types.ts` | Core type definitions shared across all modules |
| `logger.ts` | Structured logging with levels, filtering, and search |
| `commandInspector.ts` | Records every command emitted through the CommandBuffer |
| `eventInspector.ts` | Tracks keyboard, mouse, focus, resize, and lifecycle events |
| `performance.ts` | Frame timing, FPS calculation, memory usage, render cost |
| `treeInspector.ts` | Render tree, component hierarchy, props, styles, layout |
| `schedulerInspector.ts` | Frame budget, dropped frames, idle callbacks, animations |
| `focusInspector.ts` | Current focus, tab order, focus scopes, traversal |
| `capabilityInspector.ts` | Terminal capability detection and reporting |
| `timeline.ts` | Chronological event recording with filtering |
| `snapshot.ts` | Capture and compare tree/layout states |
| `export.ts` | Serialize and export diagnostic data |
| `overlay/*` | Per-frame ANSI overlay compositing and the debug panels |

## Overlay data flow

1. `CliRenderer` constructs `createDevTools({ enabled: true })` and an
   `OverlayHost` when `debug` is truthy (or `BTUI_DEBUG`/`BTUI_SHOW_STATS` is set).
2. Each frame, `writeFrame()` writes the engine's ANSI output, then calls
   `devtools.recordFrame(...)` (timing + dirty-region count) and
   `overlay.paint()`.
3. `OverlayHost.paint()` saves the cursor, draws the visible panels
   (absolute-positioned), clears any rows vacated since the previous frame to
   avoid trails, and restores the cursor.
4. Toggling the last panel off forces a `renderFull()` so the vacated region
   repaints cleanly.

## Deferred to a later Rust phase

- Panel 5 per-cell dirty highlighting and putting the overlay inside the
  engine's dirty-diff need napi cell-buffer exposure.
- Panel 2 click-to-inspect hit routing (uses the existing `hitGridCheck`).
- A full interactive console overlay (mouse selection/copy); the first pass is a
  log-backed `console` surface.

## Performance

- All inspectors use ring buffers with configurable max sizes.
- When `enabled: false`, `createDevTools()` returns a no-op implementation and
  the overlay is never constructed.
- Recording methods are synchronous and non-allocating in the hot path.
