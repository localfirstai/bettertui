# Layout Basics

> Flex rows/columns, grids, spacers, and nested layout composition.

- **Category:** Layout
- **Level:** 1 / 5
- **Demonstrates:** Flex, Grid, Spacer, Box
- **Requires:** _None._

## What it shows

This example focuses on **Flex**. Read the source in
`src/layout-basics.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs layout-basics
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `Flex`
- `Grid`
- `Spacer`
- `Box`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `text-styles` — Text & Styles
- `box-borders` — Boxes & Borders
- `responsive-layout` — Responsive Layout
