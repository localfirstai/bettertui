# Terminal

"Terminal" covers two concerns: driving the host terminal (raw mode, alternate screen) and embedding a child process via PTY with VT emulation.

## Host terminal (`terminal/`)

`Terminal` manages raw mode, alternate screen, clear, cursor visibility/position, and event polling (`poll_event`). `Drop` restores state.

## Embedded terminal (`terminal/vt/` + `pty/` + `terminal_process/`)

A `TerminalRuntime` spawns a `PtyProcess` (via `portable-pty`). Child output is fed through the `AnsiParser` → `VtMachine` → `ScreenBuffer`, producing a `FrameBuffer` that can be composited into the UI.

See [Architecture: Terminal](architecture/Terminal.md), [PTY](architecture/PTY.md), and [Guides: Terminal & PTY](guides/terminal.md).

## React component?

There is **no** React `Terminal` component yet. The capability exists at the engine/native layer but is not exposed as a TS widget.

## Status

Engine-level terminal emulation and PTY are implemented and tested. The `VtMachine`/`AnsiParser` are not yet wired into the production PTY read path (known gap).
