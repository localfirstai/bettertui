# Buttons

> Button variants and press handling — the building block of interactive UIs.

- **Category:** Widgets
- **Level:** 1 / 5
- **Demonstrates:** Button, onPress
- **Requires:** _None._

## What it shows

This example focuses on **Button**. Read the source in
`src/button-basics.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs button-basics
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `Button`
- `onPress`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `form-controls` — Form Controls
- `badge-basics` — Badges, Progress & Spinners
- `overlay-showcase` — Overlays & Menus
