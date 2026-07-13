# Text & Styles

> Typography, ANSI color, emoji and wide-character rendering, and inline code.

- **Category:** Typography
- **Level:** 1 / 5
- **Demonstrates:** Text, Heading, Label, Code, Blockquote, color, emoji
- **Requires:** `unicode`

## What it shows

This example focuses on **Text**. Read the source in
`src/text-styles.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs text-styles
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `Text`
- `Heading`
- `Label`
- `Code`
- `Blockquote`
- `color`
- `emoji`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `layout-basics` — Layout Basics
- `box-borders` — Boxes & Borders
- `markdown-rendering` — Markdown Rendering
