# Advanced Data Table

> Large, sortable, filterable data table with keyboard navigation.

- **Category:** Data Display
- **Level:** 4 / 5
- **Demonstrates:** DataTable, sorting, filtering, performance
- **Requires:** _None._

## What it shows

This example focuses on **DataTable**. Read the source in
`src/advanced-data-table.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs advanced-data-table
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `DataTable`
- `sorting`
- `filtering`
- `performance`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `data-table-basics` — Data Tables
- `tree-view` — Tree View
- `live-metrics` — Live Metrics
