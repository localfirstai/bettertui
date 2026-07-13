# Live Metrics

> A simulated real-time system dashboard with auto-updating metrics.

- **Category:** Performance
- **Level:** 4 / 5
- **Demonstrates:** setInterval, DataTable, Progress, live data
- **Requires:** _None._

## What it shows

This example focuses on **setInterval**. Read the source in
`src/live-metrics.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs live-metrics
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `setInterval`
- `DataTable`
- `Progress`
- `live data`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `performance-stress-test` — Performance Stress Test
- `tree-view` — Tree View
- `tabs-navigation` — Tabs & Accordion
