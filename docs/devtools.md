# DevTools

BetterTUI's developer tooling ships **inside `@bettertui/core`** at `src/devtools/`. The standalone `@bettertui/devtools` package has been retired.

## Exports

`createDevTools(options?)` returns a `DevTools` instance (full functional when `enabled: true`, no-op otherwise). Inspectors: logger, command, event, performance, tree, scheduler, focus, capability, timeline, snapshot manager. Export helpers: `createExport`, `exportToJson`, `createSummary`. `DebugPanel` enum: `Performance`, `Tree`, `Layout`, `Events`, `DirtyRegions`.

`CliRenderer` creates DevTools and a debug overlay when `debug: true`.

See [Architecture: DevTools](architecture/devtools.md) and [API: DevTools](api/packages/devtools.md).

## Status

Implemented and in-core. Per-cell dirty highlighting and click-to-inspect deferred (need napi cell-buffer access).
