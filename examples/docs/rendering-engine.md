# Rendering & Engine

> The CommandBuffer + reconciler layer the React API builds on.

- **Category:** Rendering & Engine
- **Level:** 4 / 5
- **Demonstrates:** CommandBuffer, createReconciler, Runtime, engine
- **Requires:** _None._

## What it shows

This example focuses on **CommandBuffer**. Read the source in
`src/rendering-engine.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs rendering-engine
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `CommandBuffer`
- `createReconciler`
- `Runtime`
- `engine`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `tree-view` — Tree View
- `tabs-navigation` — Tabs & Accordion
- `theming` — Theming
