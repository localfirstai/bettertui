# Capability Detector

> Inspect terminal capabilities (color, size, platform) from the environment.

- **Category:** Terminal Integration
- **Level:** 2 / 5
- **Demonstrates:** capabilities, process.env, process.stdout
- **Requires:** _None._

## What it shows

This example focuses on **capabilities**. Read the source in
`src/capability-detector.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs capability-detector
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `capabilities`
- `process.env`
- `process.stdout`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `key-inspector` — Key Inspector
- `mouse-input` — Mouse Input
- `clipboard-basics` — Clipboard
