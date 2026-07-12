# bettertui-engine

## Purpose

High-performance terminal UI rendering engine written in Rust. Handles all performance-critical operations: rendering, layout, input, animation, text editing, and terminal emulation.

## Responsibilities

- **Rendering:** Double-buffered, cell-based frame buffer with dirty-region diffing.
- **Layout:** Taffy-based flexbox layout adapted to terminal cells.
- **Input:** Keyboard, mouse, paste parsing (ANSI, Kitty, CSI-u).
- **Events:** Event dispatch and bubbling.
- **Scheduler:** Frame timing with priority queues and frame budgeting.
- **Animation:** Tween engine with keyframes.
- **Text editing:** Rope-based buffer with cursor, selection, and undo/redo.
- **PTY:** Embedded terminal process spawning.
- **Clipboard:** System clipboard read/write.
- **Compositing:** Multi-layer screen composition.
- **Capabilities:** Terminal feature detection (true color, mouse, etc.).
- **Widgets:** Trait-based widget system with 25+ widget types.
- **Nerd Font:** Glyph rendering with bundled DroidSansMNerdFont.

## Public API (39 modules)

| Module | Description |
|--------|-------------|
| `tree` | Arena-allocated node tree with slotmap |
| `engine` | Core engine and tree inspector |
| `renderer` | Frame buffer composition and ANSI output |
| `layout` | Flexbox layout via Taffy |
| `events` | Event dispatch and routing |
| `input` | Keyboard, mouse, paste parsing |
| `keyboard` | Key parsing and modifier tracking |
| `mouse` | Mouse button, position, drag tracking |
| `scheduler` | Frame timing and priority scheduling |
| `animation` | Tween/spring/keyframe engine |
| `text` | Rope-based text buffer |
| `widgets` | Widget trait and implementations |
| `compositor` | Multi-layer compositing |
| `painter` | ANSI cell rendering |
| `framebuffer` | Cell grid and dirty-region diffing |
| `protocol` | Command enum and processor |
| `terminal` | Raw mode, VT emulation, screen buffers |
| `capabilities` | Terminal feature detection |
| `clipboard` | System clipboard integration |
| `pty` | Process spawning |
| `ansi` | ANSI escape sequence handling |
| `selection` | Text selection with range management |
| `scrollback` | Scrollback buffer management |
| `dirty_diff` | Frame diffing for incremental rendering |
| `screen` | Screen state management |
| `editor` | Text editing commands |
| `glyph` | Glyph categorization and rendering |
| `nerdfont` | Nerd Font glyph support |
| `graphics` | Color, style, border rendering |
| `render_object` | Render object abstractions |
| `ffi` | Foreign function interface bridge |
| `benchmark` | Performance measurement |
| `mouse` | Mouse event handling |
| `pane` | Pane management |
| `neovim` | Neovim integration |
| `keybinding` | Keybinding system |
| `widget` | Widget framework |

## Dependencies

- `taffy` — Flexbox layout engine
- `crossterm` — Terminal I/O
- `ropey` — Rope data structure for text
- `unicode-width` — Unicode character width
- `unicode-segmentation` — Grapheme cluster handling
- `tracing` / `tracing-subscriber` — Structured logging
- `parking_lot` — Fast synchronization primitives
- `slotmap` — Arena allocation for nodes
- `smallvec` — Stack-allocated vectors
- `bitflags` — Bitflag types
- `portable-pty` — PTY management

## Consumers

- `bettertui-bindings` — napi-rs bindings (Rust crate)

## Internal Structure

```
src/
  lib.rs              # Module declarations, VERSION constant
  tree/               # Arena, NodeId, Style, RenderNode, Color, etc.
  engine/             # Core engine (core.rs, inspector.rs)
  protocol/           # Command enum, processor, buffer, result
  renderer/           # Render pipeline (backend/)
  layout/             # Taffy integration
  events/             # Event dispatch
  input/              # Input parsing
  keyboard/           # Key parsing
  mouse/              # Mouse handling
  scheduler/          # Frame timing
  animation/          # Tween engine
  text/               # Rope-based editor
  widgets/            # Widget trait + 25+ implementations
  compositor/         # Layer compositing
  painter/            # ANSI cell rendering
  framebuffer/        # Cell grid
  capabilities/       # Terminal detection
  clipboard/          # System clipboard
  pty/                # Process spawning
  ansi/               # ANSI parsing
  selection/          # Text selection
  scrollback/         # Scrollback buffer
  dirty_diff/         # Frame diffing
  screen/             # Screen state
  graphics/           # Color/style rendering
  nerdfont/           # Nerd Font glyphs
  glyph/              # Glyph categorization
  render_object/      # Render abstractions
  benchmark/          # Performance harness
  ffi/                # FFI bridge
  neovim/             # Neovim integration
  keybinding/         # Keybindings
  pane/               # Pane management
fonts/
  DroidSansMNerdFont-Regular.otf  # Bundled font
tests/
  # ~1,071 passing tests
```

## Example Usage (from Rust)

```rust
use bettertui_engine::engine::Engine;
use bettertui_engine::protocol::Command;

let mut engine = Engine::new(80, 24);
engine.process_command(Command::CreateNode { id: 1, kind: "box".into() });
engine.process_command(Command::AppendChild { parent: 0, child: 1 });
engine.render(); // Returns ANSI output
```

## Notes

- This is the most mature part of the project with 1,204 passing lib tests.
- Edition 2024. All workspace dependencies defined in root `Cargo.toml`.
- Clippy: `-D warnings` enforced in CI.
