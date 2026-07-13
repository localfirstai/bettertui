# Tree View

> Expand/collapse a file-tree with keyboard navigation and selection.

- **Category:** Data Display
- **Level:** 2 / 5
- **Demonstrates:** Tree, TreeNode, navigation
- **Requires:** _None._

## What it shows

This example focuses on **Tree**. Read the source in
`src/tree-view.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs tree-view
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `Tree`
- `TreeNode`
- `navigation`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `live-metrics` — Live Metrics
- `tabs-navigation` — Tabs & Accordion
- `performance-stress-test` — Performance Stress Test
