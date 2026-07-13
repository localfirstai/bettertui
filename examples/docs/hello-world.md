# Hello World

> The smallest possible BetterTUI app: a provider, a box, and some text.

- **Category:** Getting Started
- **Level:** 1 / 5
- **Demonstrates:** Provider, Flex, Box, Text
- **Requires:** _None._

## What it shows

This example focuses on **Provider**. Read the source in
`src/hello-world.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs hello-world
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `Provider`
- `Flex`
- `Box`
- `Text`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `layout-basics` — Layout Basics
- `text-styles` — Text & Styles
- `button-basics` — Buttons
