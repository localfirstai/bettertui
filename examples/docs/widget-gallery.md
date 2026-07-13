# Widget Gallery

> Every BetterTUI component in one switchable reference surface.

- **Category:** Widgets
- **Level:** 2 / 5
- **Demonstrates:** all components, reference
- **Requires:** _None._

## What it shows

This example focuses on **all components**. Read the source in
`src/widget-gallery.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs widget-gallery
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `all components`
- `reference`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `button-basics` — Buttons
- `form-controls` — Form Controls
- `overlay-showcase` — Overlays & Menus
