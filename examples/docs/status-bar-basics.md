# Status Bar & Toast

> Persistent status line and transient toast notifications.

- **Category:** Feedback & Status
- **Level:** 2 / 5
- **Demonstrates:** StatusLine, Toast
- **Requires:** _None._

## What it shows

This example focuses on **StatusLine**. Read the source in
`src/status-bar-basics.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs status-bar-basics
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `StatusLine`
- `Toast`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `badge-basics` — Badges, Progress & Spinners
- `overlay-showcase` — Overlays & Menus
- `live-metrics` — Live Metrics
