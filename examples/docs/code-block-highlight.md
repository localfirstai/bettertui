# Code & Syntax Highlighting

> Tree-sitter syntax highlighting via CodeBlock and inline Code.

- **Category:** Markdown & Code
- **Level:** 3 / 5
- **Demonstrates:** CodeBlock, Code, highlightCode, syntax
- **Requires:** _None._

## What it shows

This example focuses on **CodeBlock**. Read the source in
`src/code-block-highlight.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs code-block-highlight
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `CodeBlock`
- `Code`
- `highlightCode`
- `syntax`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `markdown-rendering` — Markdown Rendering
- `diff-view` — Diff View
- `text-styles` — Text & Styles
