# @bettertui/devtools

Developer tooling for BetterTUI.

## What's inside

- `createDevTools(options?)` — the public entry point. Returns a fully functional `DevTools` instance when `options.enabled` is `true`. When `enabled` is omitted or `false`, it returns a no-op implementation with near-zero overhead (all inspectors are still present but their record methods do nothing).
- Inspectors: `CommandInspector`, `EventInspector`, `PerformanceTracker`, `TreeInspector`, `SchedulerInspector`, `FocusInspector`, `CapabilityInspector`, `Timeline`, `SnapshotManager`.
- `Logger` for structured logging.
- `createExport`, `exportToJson`, `createSummary` for diagnostics.

## Example

```ts
import { createDevTools } from "@bettertui/devtools";

const devtools = createDevTools({ enabled: true });
devtools.recordFrame({ duration: 4.2, commandCount: 12, dirtyRegionCount: 3 });
const json = devtools.exportJson();
```

## Status

Implemented. `createDevTools()` is a working factory, not a stub. See [`docs/api/packages/devtools.md`](../../docs/api/packages/devtools.md).
