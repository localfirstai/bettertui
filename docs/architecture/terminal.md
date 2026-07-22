# Terminal

Host-terminal I/O (raw mode, alternate screen, queries) and the embedded VT state machine. Code: `packages/core/crates/engine/src/terminal/` (modules of `bettertui-engine`): `mod.rs`, `capabilities.rs`, `neovim.rs`, `process.rs`, `query.rs`, `screen.rs`, `scrollback.rs`, `vt.rs`.

## Two layers

- **Host I/O** (`terminal/mod.rs`): `Terminal::enter_raw_mode`, `leave_raw_mode`, `enter/leave_alternate_screen`, `clear`, `hide/show_cursor`, `move_cursor`, `write_bytes`, `poll_event(timeout)`. `Drop` restores terminal state.
- **VT emulation** (`vt.rs`): `VtMachine` consumes `ParserEvent`s (from `ansi.rs`) and maintains screen state — used to render an embedded shell.

## Terminal struct

`enter_raw_mode`, `leave_raw_mode`, `enter_alternate_screen`, `leave_alternate_screen`, `clear`, `hide_cursor`, `show_cursor`, `move_cursor(x, y)`, `write_bytes(&[u8])`, `poll_event(timeout) -> Option<TerminalEvent>`.

## Capability querying

`terminal/query.rs` defines `TerminalQuery` (DA1/DA2/DA3/DSR/DECID/XTVersion/Kitty), `QueryResult`, `full_probe_queries()`, `check_responses(&VtMachine)`. Results feed `capabilities::CapabilityDetector`.

## VT machine (`vt.rs`)

`VtMachine` owns: `ScreenBuffer` (x2: main + alt), `Cursor`, `alt_cursor`, `TerminalMode`, `Pen`. `process(&ParserEvent)` applies VT state transitions. Also defines `KittyKeyEvent::to_keyboard_input()`, `TerminalResponse`/`ResponseKind`, `CursorShape`/`CursorStyle`, `TerminalMode`/`PrivateMode`.

The screen state (`screen.rs`) feeds the compositor and produces a `FrameBuffer` for embedding a live shell inside the UI.

> Known issue: The `AnsiParser` + `VtMachine` are wired into tests but not yet into the production PTY read path.
