# Dashboard App

> A complete monitoring dashboard app with stat cards and an activity feed.

- **Category:** Complete Applications
- **Level:** 5 / 5
- **Demonstrates:** Grid, Progress, Badge, application
- **Requires:** _None._

## What it shows

This example focuses on **Grid**. Read the source in
`src/dashboard-app.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs dashboard-app
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `Grid`
- `Progress`
- `Badge`
- `application`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `live-metrics` — Live Metrics
- `performance-stress-test` — Performance Stress Test
- `widget-gallery` — Widget Gallery
