# Markdown Rendering

> Render Markdown documents and syntax-highlighted code blocks.

- **Category:** Markdown & Code
- **Level:** 3 / 5
- **Demonstrates:** Markdown, CodeBlock, highlightCode
- **Requires:** _None._

## What it shows

This example focuses on **Markdown**. Read the source in
`src/markdown-rendering.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs markdown-rendering
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `Markdown`
- `CodeBlock`
- `highlightCode`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `code-block-highlight` — Code & Syntax Highlighting
- `diff-view` — Diff View
- `chat-interface` — Chat Interface
