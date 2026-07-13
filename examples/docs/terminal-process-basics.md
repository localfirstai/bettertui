# Terminal Process

> Spawn and display a child process with TerminalProcess (PTY).

- **Category:** Process & PTY
- **Level:** 4 / 5
- **Demonstrates:** TerminalProcess, pty, process
- **Requires:** `pty`

## What it shows

This example focuses on **TerminalProcess**. Read the source in
`src/terminal-process-basics.tsx` — each example is small, self-contained, and commented.

## Run it

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs terminal-process-basics
```

Or from the example browser:

```bash
pnpm --filter @bettertui/examples dev
```

## Key APIs

- `TerminalProcess`
- `pty`
- `process`

## Common mistakes

- Forgetting to call `runtime?.runtime.dispose()` before `process.exit(0)` on quit.
- Mutating state without re-rendering — call `render(<App />)` after changes.
- Assuming a mouse/PTY capability is present; check `requires` above first.

## Next examples

- `key-inspector` — Key Inspector
- `clipboard-basics` — Clipboard
- `chat-interface` — Chat Interface
