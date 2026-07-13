# Overlays & Menus

> Tooltip, modal, popover, dropdown, and context menu in one reference.

- **Category:** Overlays & Menus
- **Level:** 2 / 5
- **Demonstrates:** Tooltip, Modal, Popover, Dropdown, ContextMenu
- **Requires:** _None._

## What it shows

This example focuses on **Tooltip**. Read the source in
`src/overlay-showcase.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs overlay-showcase
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `Tooltip`
- `Modal`
- `Popover`
- `Dropdown`
- `ContextMenu`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `tabs-navigation` — Tabs & Accordion
- `status-bar-basics` — Status Bar & Toast
- `status-bar-basics` — Status Bar & Toast
