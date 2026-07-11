# DevTools

DevTools (inspector, profiler, error overlay) are **planned**, not implemented. `@bettertui/devtools` currently exports only:

```ts
export interface DevToolsOptions { enabled: boolean; port: number; }
export function createDevTools(_options?: Partial<DevToolsOptions>): unknown {
  return null;
}
```

## Intended scope (from architecture)

- Inspector: tree visualization, node properties, layout, resolved styles.
- Profiler: frame/layout/render timing, memory.
- Overlay: performance, layout grid, dirty regions, hit-test.

None of this exists yet. The `Scheduler` already produces `SchedulerStats` (`frame_count`, `dropped_frames`, `avg_frame_time`, `frame_budget`) that a future profiler can consume.

## Status

Stub. No runtime, no `@bettertui/devtools` package beyond the placeholder.
