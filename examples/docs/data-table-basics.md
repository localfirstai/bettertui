# Data Tables

> Static tables and sortable, keyboard-navigable data tables.

- **Category:** Data Display
- **Level:** 2 / 5
- **Demonstrates:** Table, DataTable, DataTableProps, sorting, selection
- **Requires:** _None._

## What it shows

This example focuses on **Table**. Read the source in
`src/data-table-basics.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs data-table-basics
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `Table`
- `DataTable`
- `DataTableProps`
- `sorting`
- `selection`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `tree-view` — Tree View
- `list-view` — List View
- `advanced-data-table` — Advanced Data Table
