# Performance Stress Test

> Measure FPS and render time under large-table / large-tree workloads.

- **Category:** Performance
- **Level:** 5 / 5
- **Demonstrates:** performance, setInterval, DataTable, Tree, metrics
- **Requires:** _None._

## What it shows

This example focuses on **performance**. Read the source in
`src/performance-stress-test.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs performance-stress-test
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `performance`
- `setInterval`
- `DataTable`
- `Tree`
- `metrics`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `live-metrics` — Live Metrics
- `dashboard-app` — Dashboard App
- `advanced-data-table` — Advanced Data Table
