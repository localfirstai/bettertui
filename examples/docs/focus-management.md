# Focus Management

> Move focus across elements with FocusProvider and useFocus.

- **Category:** Interaction
- **Level:** 3 / 5
- **Demonstrates:** FocusProvider, useFocus, focus
- **Requires:** _None._

## What it shows

This example focuses on **FocusProvider**. Read the source in
`src/focus-management.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs focus-management
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `FocusProvider`
- `useFocus`
- `focus`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `mouse-input` — Mouse Input
- `key-inspector` — Key Inspector
- `form-controls` — Form Controls
