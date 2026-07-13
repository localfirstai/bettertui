# Mouse Input

> Capture mouse press and move events and render the pointer position.

- **Category:** Interaction
- **Level:** 3 / 5
- **Demonstrates:** useMouse, MouseState
- **Requires:** `mouse`

## What it shows

This example focuses on **useMouse**. Read the source in
`src/mouse-input.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs mouse-input
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `useMouse`
- `MouseState`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `key-inspector` — Key Inspector
- `capability-detector` — Capability Detector
- `focus-management` — Focus Management
