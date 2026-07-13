# Boxes & Borders

> Bordered boxes, padding, and titled panes as the core container primitives.

- **Category:** Containers
- **Level:** 1 / 5
- **Demonstrates:** Box, Pane, border, padding
- **Requires:** _None._

## What it shows

This example focuses on **Box**. Read the source in
`src/box-borders.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs box-borders
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `Box`
- `Pane`
- `border`
- `padding`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `layout-basics` — Layout Basics
- `text-styles` — Text & Styles
- `scroll-area-basics` — Scroll Area
