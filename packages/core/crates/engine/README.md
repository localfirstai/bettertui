# bettertui-engine

## Purpose

High-performance terminal UI rendering engine written in Rust. Handles all performance-critical operations: rendering, layout, input, animation, text editing, and terminal emulation.

## Responsibilities

- **Rendering:** Double-buffered, cell-based frame buffer with dirty-region diffing.
- **Layout:** Taffy-based flexbox layout adapted to terminal cells.
- **Input:** Keyboard, mouse, paste parsing (ANSI, Kitty, CSI-u).
- **Events:** DOM-style capture → target → bubble event dispatch and routing.
- **Scheduler:** Frame timing with priority queues and frame budgeting.
- **Animation:** Tween/spring/keyframe engine with colour interpolation.
- **Text editing:** Rope-based buffer with cursor, selection, and undo/redo.
- **PTY:** Embedded terminal process spawning.
- **Terminal I/O + VT:** Raw mode, alternate screen, VT emulation, capability detection (in the `bettertui-terminal` crate).
- **Widgets:** Trait-based widget framework (in the `bettertui-widgets` crate).
- **Nerd Font:** Glyph rendering with a bundled font.

## Public API (modules)

The crate's `src/lib.rs` declares these top-level modules (most are single flat files; `render/`, `text/`, and `font/` are subdirectories):

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
| `layout` | Flexbox layout via Taffy (incl. viewport culling) |
| `plugin` | Plugin host and capability flags |
| `protocol` | `Command` enum and `CommandProcessor` |
| `pty` | Pseudo-terminal abstraction (`PtyProcess`, `PtyRuntime`) |
| `render` | Render pipeline (`Renderer`, `AnsiBackend`, `Painter`, `RenderTree`) + `effects` post-processing |
| `scheduler` | Frame timing and priority scheduling |
| `syntax` | tree-sitter syntax highlighter |
| `text` | Rope-based text engine (`TextEngine`, buffer, cursor, undo, selection, wrap, viewport) |

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
- `napi` / `napi-derive` — napi-rs bindings (consumed via the `bettertui-bindings` crate)

## Consumers

- `bettertui-bindings` — napi-rs bindings crate
- `bettertui-widgets` — widget framework crate (depends on engine)
- `bettertui-terminal` — terminal crate (depends on engine)

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
  layout.rs           # Taffy integration + viewport culling
  plugin.rs           # Plugin host
  protocol.rs         # Command enum + processor
  pty.rs              # Pseudo-terminal
  scheduler.rs        # Frame timing
  syntax.rs           # Syntax highlighter
  text.rs             # TextEngine facade
  render/             # mod.rs, render.rs (Renderer/AnsiBackend/Painter), effects.rs (post-processing)
  text/               # buffer, cursor, edit, search, selection, styled, undo, unicode, viewport, wrap
  font/               # loader, metrics, provider, registry, ascii
  bin/                # layout_e2e integration binary
fonts/
  # Bundled Nerd Font (DroidSansMNerdFont-Regular.otf)
```

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

- This is the most mature part of the project: 310 Rust lib tests (verified via `cargo test -p bettertui-engine --lib`). The `bettertui-terminal` and `bettertui-widgets` crates add 153 and 257 lib tests respectively, for 720 in total.
- Edition 2024. All workspace dependencies defined in the root `Cargo.toml` (`packages/core/Cargo.toml`).
- Clippy: `-D warnings` enforced in CI.
