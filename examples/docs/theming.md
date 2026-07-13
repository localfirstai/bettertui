# Theming

> Apply and switch themes through the Provider's theme prop.

- **Category:** Theming
- **Level:** 2 / 5
- **Demonstrates:** Provider, Theme, useTheme
- **Requires:** _None._

## What it shows

This example focuses on **Provider**. Read the source in
`src/theming.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs theming
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `Provider`
- `Theme`
- `useTheme`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `animation-basics` — Animation & Motion
- `tree-view` — Tree View
- `tabs-navigation` — Tabs & Accordion
