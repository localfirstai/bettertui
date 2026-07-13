# Terminal

"Terminal" covers two concerns: driving the host terminal (raw mode, alternate screen) and embedding a child process via PTY with VT emulation.

## Host terminal (`terminal/`)

`Terminal` manages raw mode, alternate screen, clear, cursor visibility/position, and event polling (`poll_event`). `Drop` restores state.

## Embedded terminal (`terminal/vt/` + `pty/` + `terminal_process/`)

A `TerminalRuntime` spawns a `PtyProcess` (via `portable-pty`). Child output is fed through the `AnsiParser` → `VtMachine` → `ScreenBuffer`, producing a `FrameBuffer` that can be composited into the UI.

See [Architecture: Terminal](architecture/terminal.md), [PTY](architecture/pty.md), and [Guides: Terminal & PTY](guides/terminal.md).

## React components

`@bettertui/react` exports `Terminal`, `TerminalViewport`, and `TerminalProcess` as thin wrappers that emit element descriptors. They are not yet wired to the live native PTY read path, but the engine-level capability (PTY + `AnsiParser`/`VtMachine`) is implemented.

## Status

Engine-level terminal emulation and PTY are implemented and tested. The `VtMachine`/`AnsiParser` are not yet wired into the production PTY read path (known gap).
