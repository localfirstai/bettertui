# Terminal & PTY

BetterTUI can embed a live terminal process inside the UI via a PTY, and parse/emit ANSI through a VT state machine.

## Engine components

- `pty.rs` — `PtyConfig`, `PtyProcess`, `PtyRuntime`, `PtyReader`/`PtyWriter`, `PtyError`
- `terminal/process.rs` — `TerminalRuntime`, `ProcessConfig`, `TerminalState`, `TerminalViewport`
- `vt.rs` — `VtMachine` consumes parser events, maintains `ScreenBuffer`/`Cursor`
- `screen.rs` — `ScreenState` with alternate screen, scrollback, selection

## Known gaps

- The `AnsiParser` + `VtMachine` are wired into tests but not yet into the production PTY read path.
