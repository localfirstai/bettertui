# DevTools

> **Architecture deep-dive:** [DevTools Architecture](architecture/devtools.md) · [API reference](api/packages/devtools.md)

BetterTUI's developer tooling lives **inside `@bettertui/core`** (module
`@bettertui/core` → `src/devtools/`). The standalone `@bettertui/devtools`
package has been retired; import everything from `@bettertui/core`.

## Surface

`createDevTools(options?)` returns a `DevTools` instance:

- With `enabled: true`, every record/export method is active and the inspectors collect data.
- With `enabled` omitted or `false`, it returns a no-op instance: the inspectors exist but their `record*` methods do nothing, so there is near-zero runtime cost.

```ts
import { createDevTools } from "@bettertui/core";

const devtools = createDevTools({ enabled: true });

devtools.recordFrame({ duration: 4.2, commandCount: 12, dirtyRegionCount: 3 });
devtools.recordKeyboard("a", { ctrl: false, shift: false, alt: false, meta: false });
const json: string = devtools.exportJson();
```

The instance exposes:

- `logger` — `Logger`, bounded structured logging.
- `commands` — `CommandInspector`, records every command emitted.
- `events` — `EventInspector`, keyboard/mouse/focus/resize events.
- `performance` — `PerformanceTracker`, frame timing, FPS, dropped frames.
- `tree` — `TreeInspector`, render-tree capture and lookup.
- `scheduler` — `SchedulerInspector`, frame budget and drops.
- `focus` — `FocusInspector`, focused node and tab order.
- `capabilities` — `CapabilityInspector`, terminal feature snapshot.
- `timeline` — `Timeline`, chronological event log.
- `snapshots` — `SnapshotManager`, capture and compare tree states.
- `console` — `DebugConsole`, log-backed console surface.

It also exposes panel-control and query methods (§6.3): `show`/`hide`/`toggle`
/`isVisible`, `getStats`, `startProfiling`/`stopProfiling`, `inspect`,
`highlight`/`clearHighlight`, `traceEvents`/`getEventLog`, `inspectLayout`,
`showDirtyRegions`, `getMemoryStats`, and `takeHeapSnapshot`.

Helpers `createExport`, `exportToJson`, and `createSummary` build and serialise a `DiagnosticExport`.

## The renderer overlay

`CliRenderer` creates DevTools and a debug overlay when constructed with
`debug: true` (or a `DevToolsOptions` object). The overlay composites
absolute-positioned ANSI panels over each frame.

```ts
import { createCliRenderer, DebugPanel } from "@bettertui/core";

const renderer = await createCliRenderer({ debug: true });
renderer.toggleDebugOverlay(DebugPanel.Events); // bind to a key, e.g. backtick
```

The environment variables `BTUI_DEBUG` and `BTUI_SHOW_STATS` (mirroring
OpenTUI's `OTUI_SHOW_STATS`) force the overlay on and start with the performance
panel visible. Access the facade via `renderer.devtools`.

Available panels (`DebugPanel`): `Performance` (frame timing + render stats),
`Tree`, `Layout`, `Events`, `DirtyRegions`.

## Status

Implemented and in-core. Panel 5 per-cell dirty highlighting and Panel 2
click-to-inspect are deferred to a later Rust phase (they need napi cell-buffer
access). See [`API: DevTools`](api/packages/devtools.md) for the full export list.
