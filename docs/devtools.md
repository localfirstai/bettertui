# DevTools

`@bettertui/devtools` provides a working developer-tools package for BetterTUI. It is not a stub.

## Surface

`createDevTools(options?)` returns a `DevTools` instance:

- With `enabled: true`, every record/export method is active and the inspectors collect data.
- With `enabled` omitted or `false`, it returns a no-op instance: the inspectors exist but their `record*` methods do nothing, so there is near-zero runtime cost.

```ts
import { createDevTools } from "@bettertui/devtools";

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

Helpers `createExport`, `exportToJson`, and `createSummary` build and serialise a `DiagnosticExport`.

## Status

Implemented. See [`API: DevTools`](api/packages/devtools.md) for the full export list. The `Scheduler` produces `SchedulerStats` (`frame_count`, `dropped_frames`, `avg_frame_time`, `frame_budget`) that the `PerformanceTracker` consumes.
