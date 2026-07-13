# List View

> A selectable list with keyboard navigation and disabled items.

- **Category:** Data Display
- **Level:** 2 / 5
- **Demonstrates:** List, ListItem, selection
- **Requires:** _None._

## What it shows

This example focuses on **List**. Read the source in
`src/list-view.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs list-view
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `List`
- `ListItem`
- `selection`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `tree-view` — Tree View
- `data-table-basics` — Data Tables
- `tabs-navigation` — Tabs & Accordion
