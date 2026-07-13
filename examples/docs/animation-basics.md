# Animation & Motion

> Drive values over time with useAnimation, easings, and useTimeline.

- **Category:** Animation & Motion
- **Level:** 3 / 5
- **Demonstrates:** useAnimation, easings, useTimeline, motion
- **Requires:** _None._

## What it shows

This example focuses on **useAnimation**. Read the source in
`src/animation-basics.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs animation-basics
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `useAnimation`
- `easings`
- `useTimeline`
- `motion`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `theming` — Theming
- `text-styles` — Text & Styles
- `live-metrics` — Live Metrics
