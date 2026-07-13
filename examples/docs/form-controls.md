# Form Controls

> All input widgets: text, textarea, toggles, slider, radio, select, combobox.

- **Category:** Forms & Input
- **Level:** 2 / 5
- **Demonstrates:** Input, Textarea, Checkbox, Switch, Slider, Radio, Select, Combobox
- **Requires:** _None._

## What it shows

This example focuses on **Input**. Read the source in
`src/form-controls.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs form-controls
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `Input`
- `Textarea`
- `Checkbox`
- `Switch`
- `Slider`
- `Radio`
- `Select`
- `Combobox`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `counter` — Counter
- `badge-basics` — Badges, Progress & Spinners
- `data-table-basics` — Data Tables
