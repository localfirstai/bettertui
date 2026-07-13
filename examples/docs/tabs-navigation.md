# Tabs & Accordion

> Switchable tabs and expandable accordion sections for content organization.

- **Category:** Navigation
- **Level:** 2 / 5
- **Demonstrates:** Tabs, TabItem, Accordion
- **Requires:** _None._

## What it shows

This example focuses on **Tabs**. Read the source in
`src/tabs-navigation.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs tabs-navigation
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `Tabs`
- `TabItem`
- `Accordion`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `tree-view` — Tree View
- `theming` — Theming
- `scroll-area-basics` — Scroll Area
