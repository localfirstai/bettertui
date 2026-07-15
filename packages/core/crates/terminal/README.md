# bettertui-terminal

## Purpose

Terminal interaction layer: crossterm-based terminal event handling,
capability detection, process/PTY management, VT emulation, scrollback,
and screen management. This crate sits on top of `bettertui-engine` and
provides the OS-terminal glue the engine needs.

## Responsibilities

- **Terminal lifecycle:** Raw mode, alternate screen, cursor show/hide,
  clear, and event polling via a thin `Terminal` wrapper.
- **Input mapping:** Maps crossterm key/mouse/resize events into
  BetterTUI's `TerminalEvent` / `Key` / `KeyInput` types.
- **Capability detection:** Detects terminal brand, color support, unicode
  widths, input modes, rendering features, and clipboard support.
- **Process management:** Spawns and supervises child processes via PTY
  (`ProcessSpawner`, `TerminalRuntime`), with config builder, status
  tracking, and viewport control.
- **VT emulation:** A VT state machine (`VtMachine`) with cursor, pen,
  screen buffers, private modes, and response parsing.
- **Scrollback & screen:** Scrollback buffer and screen management helpers.
- **Neovim integration:** Neovim-specific terminal query/control helpers.

## Public API (modules)

| Module | Re-exports |
|--------|-----------|
| `capabilities` | `CapabilityDetector`, `TerminalBrand`, `ColorSupport`, `UnicodeCapabilities`, `UnicodeVersion`, `EmojiWidth`, `CjkWidth`, `InputCapabilities`, `MouseModes`, `RenderCapabilities`, `GraphicsCapabilities`, `ClipboardCapabilities`, `FeatureMatrix`, `WindowMetrics`, `QueryOrigin`, `global_capabilities()` |
| `process` | `ProcessConfig`, `ProcessConfigBuilder`, `ProcessSpawner`, `ProcessStatus`, `SpawnResult`, `TerminalRuntime`, `TerminalState`, `TerminalViewport`, `TerminalError`, `ScrollMode` |
| `screen` | Screen management types (`pub use screen::*`) |
| `scrollback` | Scrollback buffer types (`pub use scrollback::*`) |
| `vt` | `VtMachine`, `Cursor`, `CursorShape`, `CursorStyle`, `Pen`, `PrivateMode`, `ScreenBuffer`, `TerminalMode`, `TerminalResponse`, `ResponseKind`, `KittyKeyEvent` |
| `neovim` | Neovim terminal integration |
| `query` | Terminal query/response helpers |

The crate also exposes a top-level `Terminal` struct with `new`, `size`,
`refresh_size`, `enter_raw_mode`, `leave_raw_mode`, `enter_alternate_screen`,
`leave_alternate_screen`, `clear`, `hide_cursor`, `show_cursor`,
`move_cursor`, `write_bytes`, `flush`, and `poll_event`. The `Terminal`
struct automatically restores terminal state on `Drop`.

## Dependencies

- `bettertui-engine` — core engine (PTY, tree, input types)
- `crossterm` — terminal I/O and events
- `tracing` — diagnostics
- `portable-pty` — PTY management
- `bitflags` — capability bitflags

## Consumers

- `bettertui-bindings` — exposes terminal capabilities and event bus to Node.js
- `bettertui-widgets` — depends transitively via engine

## Build & Test

```bash
cargo test -p bettertui-terminal
```

## Notes

- 153 lib tests (verified via `cargo test -p bettertui-terminal --lib`).
- This crate does **not** contain the rendering/layout engine — that lives
  in `bettertui-engine`. It focuses on the host terminal abstraction.
