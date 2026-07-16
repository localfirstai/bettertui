# bettertui-engine

## Purpose

High-performance terminal UI rendering engine written in Rust. Handles all performance-critical
operations: rendering, layout, input, animation, text editing, terminal emulation, and PTY.

This crate is both a Rust library (`bettertui_engine`) and a `cdylib` (`bettertui_engine.node`).
When built with the `napi` feature, it exposes the engine directly to Node.js through napi-rs —
there is no separate bindings crate.

## Workspace

The Rust workspace root is `packages/core/Cargo.toml`. Its members are:

| Crate | Path | Type | Purpose |
|-------|------|------|---------|
| `bettertui-engine` | `crates/engine` | `lib` + `cdylib` | Rendering engine (this crate) |
| `bettertui-logger` | `crates/logger` | `lib` | File-based tracing logger for native code |
| `bettertui-benchmark` | `crates/benchmark` | `lib` | Criterion benchmarks (`publish = false`) |

Terminal I/O, VT emulation, PTY, capability detection, and the widget host are **modules inside
`bettertui-engine`** (see `terminal/`, `pty.rs`, `widget` surfaces in `binding.ts`), not separate
crates.

## Building the native addon

The Node.js addon is built from this crate, not from a separate bindings crate:

```bash
# from the repository root
pnpm --filter @bettertui/core build:native
# equivalent to:
napi build --manifest-path crates/engine/Cargo.toml --features napi --release \
  --output-dir ./dist --dts ../crates/engine/index.d.ts
```

This produces `dist/bettertui_engine.node`. `@bettertui/core` loads it at runtime via
`require("bettertui_engine")` and throws a clear error if the addon was not built first.

## Public API (modules)

`src/lib.rs` declares these top-level modules. Most are single flat files; `render/`, `text/`,
`font/`, and `terminal/` are subdirectories:

| Module | Description |
|--------|-------------|
| `tree` | Arena-allocated node tree with slotmap (`NodeId`, `RenderNode`, `Style`, `Color`) |
| `input` | Keyboard, mouse, paste parsing, event bus, focus manager, keymap |
| `animation` | Tween/spring/keyframe engine |
| `ansi` | ANSI escape-sequence encoding/decoding and command palette |
| `dirty_diff` | Frame diffing for incremental rendering |
| `engine` | Core `Engine` and tree `Inspector` |
| `ffi` | C-ABI FFI bridge (`FfiEngine`, filesystem helpers) |
| `font` | Font loading, metrics, Nerd Font registry, ASCII rendering |
| `framebuffer` | Cell grid and `CellAttributes` |
| `glyph` | Glyph categorisation and caching |
| `graphics` | Colour, style, border drawing (`GraphicsContext`) |
| `taffy` | Flexbox layout via Taffy (incl. viewport culling) |
| `plugin` | Plugin host and capability flags |
| `protocol` | `Command` enum and `CommandProcessor` |
| `pty` | Pseudo-terminal abstraction (`PtyProcess`, `PtyRuntime`) |
| `render` | Render pipeline (`Renderer`, `AnsiBackend`, `Painter`, `RenderTree`) + `effects` post-processing |
| `scheduler` | Frame timing and priority scheduling |
| `syntax` | tree-sitter syntax highlighter |
| `terminal` | Raw mode, alternate screen, VT emulation, capability detection, PTY process, scrollback, neovim (`terminal/mod.rs`, `vt.rs`, `capabilities.rs`, `process.rs`, `screen.rs`, `scrollback.rs`, `query.rs`, `neovim.rs`) |
| `text` | Rope-based text engine (`TextEngine`, buffer, cursor, undo, selection, wrap, viewport) |
| `theme` | Theme structs and presets |
| `napi` | napi-rs bindings (compiled only with the `napi` feature) |

## Internal structure

```
src/
  lib.rs              # Module declarations, VERSION constant
  tree.rs             # Arena, NodeId, Style, RenderNode, Color, visibility, transform, overflow, focus, events
  input.rs            # Keyboard/mouse/paste, EventBus, FocusManager, Keymap
  animation.rs        # Tween/spring/keyframe engine
  ansi.rs             # ANSI encoder/parser, command palette
  dirty_diff.rs       # Frame diffing
  engine.rs           # Core Engine + Inspector
  ffi.rs              # C-ABI bridge
  framebuffer.rs      # Cell grid + attributes
  glyph.rs            # Glyph caching
  graphics.rs         # GraphicsContext drawing
  taffy.rs            # Taffy integration + viewport culling
  plugin.rs           # Plugin host
  protocol.rs         # Command enum + processor
  pty.rs              # Pseudo-terminal
  scheduler.rs        # Frame timing
  syntax.rs           # Syntax highlighter
  text.rs             # TextEngine facade
  theme.rs            # Theme structs and presets
  render/             # mod.rs, render.rs (Renderer/AnsiBackend/Painter), effects.rs (post-processing)
  text/               # buffer, cursor, edit, search, selection, styled, undo, unicode, viewport, wrap
  font/               # loader, metrics, provider, registry, ascii
  terminal/           # mod.rs, vt.rs, capabilities.rs, process.rs, screen.rs, scrollback.rs, query.rs, neovim.rs
  bin/                # layout_e2e integration binary
fonts/
  # Bundled Nerd Font
```

## Dependencies

- `taffy` — Flexbox layout engine
- `crossterm` — Terminal I/O
- `ropey` — Rope data structure for text
- `unicode-width` — Unicode character width
- `unicode-segmentation` — Grapheme cluster handling
- `parking_lot` — Fast synchronisation primitives
- `slotmap` — Arena allocation for nodes
- `smallvec` — Stack-allocated vectors
- `bitflags` — Bitflag types
- `portable-pty` — PTY management
- `tree-sitter` + language crates — Syntax highlighting
- `napi` / `napi-derive` — napi-rs bindings (enabled with the `napi` feature)

## Example usage (from Rust)

```rust
use bettertui_engine::engine::Engine;
use bettertui_engine::protocol::Command;

let mut engine = Engine::new(80, 24);
engine.process_command(Command::CreateNode { id: 1, kind: "box".into() });
engine.process_command(Command::AppendChild { parent: 0, child: 1 });
let frame = engine.render(); // Returns a RenderFrame with ANSI output
```

## Notes

- Edition 2024. All workspace dependencies are defined in `packages/core/Cargo.toml`.
- Clippy: `-D warnings` enforced in CI.
- The crate carries a large unit-test suite co-located in `#[cfg(test)] mod tests` blocks and in
  `tests/`. Run `cargo test --manifest-path packages/core/Cargo.toml --lib` to execute the library
  tests.
