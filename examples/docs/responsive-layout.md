# Responsive Layout

> Adapt layout to terminal size with useResize.

- **Category:** Layout
- **Level:** 3 / 5
- **Demonstrates:** useResize, responsive, Grid
- **Requires:** _None._

## What it shows

This example focuses on **useResize**. Read the source in
`src/responsive-layout.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs responsive-layout
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `useResize`
- `responsive`
- `Grid`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `layout-basics` — Layout Basics
- `theming` — Theming
- `widget-gallery` — Widget Gallery
