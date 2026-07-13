# Badges, Progress & Spinners

> Status badges, progress bars, and the four spinner variants.

- **Category:** Feedback & Status
- **Level:** 1 / 5
- **Demonstrates:** Badge, Progress, Spinner
- **Requires:** _None._

## What it shows

This example focuses on **Badge**. Read the source in
`src/badge-basics.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs badge-basics
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `Badge`
- `Progress`
- `Spinner`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `form-controls` — Form Controls
- `status-bar-basics` — Status Bar & Toast
- `status-bar-basics` — Status Bar & Toast
