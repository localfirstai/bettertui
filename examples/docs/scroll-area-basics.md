# Scroll Area

> Scroll long content vertically with a visible scrollbar.

- **Category:** Containers
- **Level:** 2 / 5
- **Demonstrates:** ScrollArea, scrolling
- **Requires:** _None._

## What it shows

This example focuses on **ScrollArea**. Read the source in
`src/scroll-area-basics.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs scroll-area-basics
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `ScrollArea`
- `scrolling`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `list-view` — List View
- `tree-view` — Tree View
- `advanced-data-table` — Advanced Data Table
