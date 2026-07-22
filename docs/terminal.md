# Terminal

"Terminal" covers host terminal I/O (raw mode, alternate screen) and embedded child process PTY with VT emulation.

## Host terminal

`Terminal` manages raw mode, alternate screen, clear, cursor visibility/position, and event polling. `Drop` restores state.

## Embedded terminal

`TerminalRuntime` spawns a `PtyProcess`. Child output feeds through `AnsiParser` → `VtMachine` → `ScreenBuffer`, producing a `FrameBuffer` for compositing into the UI.

See [Architecture: Terminal](architecture/terminal.md), [PTY](architecture/pty.md), and [Guides: Terminal & PTY](guides/terminal.md).

## Status

Engine-level terminal emulation and PTY are implemented and tested. The `VtMachine`/`AnsiParser` are not yet wired into the production PTY read path.
