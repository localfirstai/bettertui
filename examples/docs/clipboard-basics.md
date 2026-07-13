# Clipboard

> Copy and paste text with the useClipboard hook.

- **Category:** Terminal Integration
- **Level:** 3 / 5
- **Demonstrates:** useClipboard, clipboard
- **Requires:** `clipboard`

## What it shows

This example focuses on **useClipboard**. Read the source in
`src/clipboard-basics.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs clipboard-basics
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `useClipboard`
- `clipboard`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `key-inspector` — Key Inspector
- `capability-detector` — Capability Detector
- `terminal-process-basics` — Terminal Process
