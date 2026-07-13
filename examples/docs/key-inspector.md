# Key Inspector

> Capture and display raw keypresses, escape sequences, and key history.

- **Category:** Terminal Integration
- **Level:** 2 / 5
- **Demonstrates:** useKeyboard, KeyEvent, escape sequences
- **Requires:** _None._

## What it shows

This example focuses on **useKeyboard**. Read the source in
`src/key-inspector.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs key-inspector
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `useKeyboard`
- `KeyEvent`
- `escape sequences`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `text-styles` — Text & Styles
- `capability-detector` — Capability Detector
- `clipboard-basics` — Clipboard
