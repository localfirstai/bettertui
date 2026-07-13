# Chat Interface

> A chat surface with message history, a thinking indicator, and a composer.

- **Category:** Chat & AI
- **Level:** 3 / 5
- **Demonstrates:** ChatView, PromptComposer, ChatMessage, ThinkingIndicator
- **Requires:** _None._

## What it shows

This example focuses on **ChatView**. Read the source in
`src/chat-interface.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs chat-interface
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `ChatView`
- `PromptComposer`
- `ChatMessage`
- `ThinkingIndicator`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `markdown-rendering` — Markdown Rendering
- `terminal-process-basics` — Terminal Process
- `live-metrics` — Live Metrics
