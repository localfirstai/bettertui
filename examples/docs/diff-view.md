# Diff View

> Show a diff between two versions of a text with the Diff component.

- **Category:** Markdown & Code
- **Level:** 3 / 5
- **Demonstrates:** Diff, diffing
- **Requires:** _None._

## What it shows

This example focuses on **Diff**. Read the source in
`src/diff-view.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs diff-view
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `Diff`
- `diffing`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `markdown-rendering` — Markdown Rendering
- `code-block-highlight` — Code & Syntax Highlighting
- `chat-interface` — Chat Interface
