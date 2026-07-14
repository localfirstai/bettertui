# AGENTS.md — `bettertui-terminal`

## Overview

Terminal interaction layer extracted from `bettertui-engine`. Handles all host-terminal I/O, VT100/VTxxx emulation, PTY process management, and capability detection.

## Crate Dependencies

- `bettertui-engine` — uses `framebuffer::Cell`, `ansi::AnsiParser`, `input::KeyboardInput`, `tree::{Color, NamedColor}`, `pty::{PtyConfig, PtyError, PtyProcess, PtySize}`.
- `crossterm` — raw terminal I/O, events, screen management.
- `portable-pty` — child PTY process spawning.
- `bitflags` — `TerminalMode` bitmask.

## Key Modules

| Module | Purpose |
|--------|---------|
| `lib.rs` | `Terminal` struct (raw mode, alt screen, cursor, event polling), `TerminalEvent`, `KeyInput`, `Key` |
| `vt.rs` | `VtMachine` — VT100/VTxxx state machine consuming `ParserEvent`s |
| `screen.rs` | `ScreenBuffer` — scrollable screen buffer with cells |
| `scrollback.rs` | `ScrollbackBuffer` — ring buffer for scrollback history |
| `process.rs` | `TerminalRuntime`, `ProcessConfig`, `ProcessSpawner`, `TerminalState` |
| `neovim.rs` | `NeovimProcess`, `NeovimState`, `NeovimConfig` |
| `query.rs` | Terminal capability querying (DA1/DA2/DA3/DSR/DECID/XTVersion/Kitty) |
| `capabilities.rs` | `CapabilityDetector`, feature matrix, brand detection |

## Testing

- Lib tests: `cargo test -p bettertui-terminal --lib --manifest-path packages/core/Cargo.toml` (39 tests)
- Integration tests: `cargo test -p bettertui-terminal --tests ...` (52 tests, moved from engine)
- Test dependencies: `vt100` for ANSI parsing, `insta` for snapshots (via engine dev-deps)

## Error Chain

`PtyError` (engine) → `TerminalError` (terminal) → `NeovimError` (terminal). Each impls `From<T>`.
