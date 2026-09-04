# bettertui_engine

High-performance terminal UI engine written in Rust. The native core of [BetterTUI](https://bettertui.dev): layout, rendering, text editing, input, animation, terminal emulation, PTY management, font/glyph handling, and syntax highlighting - all in a single dependency.

[![crates.io](https://img.shields.io/crates/v/bettertui_engine.svg)](https://crates.io/crates/bettertui_engine)
[![docs.rs](https://img.shields.io/docsrs/bettertui_engine)](https://docs.rs/bettertui_engine)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## What it does

- **Flexbox layout** - Taffy-powered layout engine (flexbox and grid semantics)
- **Rendering** - ANSI output pipeline with framebuffer, painter, render passes, and dirty-region diffing so only changed cells are redrawn
- **Node tree** - arena-allocated render tree with generational indices (no borrow-check fights, cheap inserts/removes)
- **Text editing** - rope-backed buffer with undo/redo, search, wrapping, selection, cursor and viewport management
- **Input** - keyboard parsing, mouse events, focus management and traversal, declarative keybindings
- **Terminal emulation** - embedded VT state machine, terminal capability queries, scrollback buffers
- **PTY management** - spawn and drive pseudo-terminals via portable-pty
- **Animation** - timelines, tweens, easing functions
- **Fonts and glyphs** - font loading/metrics plus built-in NerdFont icon registry generated at build time
- **Syntax highlighting** - tree-sitter grammars for JavaScript, TypeScript, Rust, Python, JSON, HTML, CSS, Bash
- **Scheduling and events** - frame scheduler, event bus, emitters, and a composable event pipeline
- **Theming** - light/dark themes with spacing and border tokens

## Install

```toml
[dependencies]
bettertui_engine = "0.1"
```

## Quick start

Build a small node tree, lay it out, and render one ANSI frame to stdout:

```rust
use bettertui_engine::render::Renderer;
use bettertui_engine::taffy::Sizing;
use bettertui_engine::tree::{NodeArena, NodeKind, RenderNode};

fn main() {
    let mut arena = NodeArena::new();

    let root = arena.insert(RenderNode::box_node());
    let label = arena.insert(RenderNode::text("Hello from bettertui_engine"));
    arena.append_child(root, label).unwrap();

    {
        let root_node = arena.get_mut(root).unwrap();
        root_node.layout.width = Some(Sizing::Points(80.0));
        root_node.layout.height = Some(Sizing::Points(24.0));
    }

    let mut renderer = Renderer::new(80, 24);
    let frame = renderer.render(&mut arena);
    print!("{}", String::from_utf8_lossy(&frame.output_data));
}
```

## Feature flags

| Feature | Default | Description |
|---|---|---|
| `diagnostics` | yes | Render/frame/cache diagnostic counters. Disable with `default-features = false` for zero-overhead release builds. |
| `napi` | no | Node.js bindings via napi-rs. Only needed when building the `bettertui_engine.node` addon; not used by pure-Rust consumers. |
| `production` | no | Convenience alias enabling `release-logs-warn`. |
| `release-logs-off` / `-error` / `-warn` / `-info` / `-debug` | no | Compile-time log gating; statically removes `tracing` events below the chosen level from release builds. Pick at most one. |

## Modules

| Module | Purpose |
|---|---|
| `tree` | Arena-allocated render tree: nodes, colors, styles, traversal |
| `taffy` | Layout engine integration, layout props/results, render-tree builder |
| `render` | Renderer, painter, render passes/pipeline, ANSI backend |
| `framebuffer` | Cell grid with per-cell attributes |
| `dirty_diff` | Region diffing between frames for partial redraws |
| `ansi` | Escape-sequence encoding, clipboard OSC sequences |
| `text` | Rope buffer, editing, undo, search, wrap, selection, cursor, viewport |
| `input` | Keys, mouse, focus manager, keybindings |
| `terminal` | VT state machine, capabilities, queries, screen, scrollback, process glue |
| `pty` | Pseudo-terminal spawning and IO |
| `animation` | Timelines, tweens, easing |
| `event_bus` / `event_emitter` / `event_pipeline` | Event distribution primitives |
| `font` / `glyph` | Font loading, metrics, glyph rasterization support |
| `graphics` / `graphics_protocol` | Terminal graphics protocol support |
| `hit_grid` | Hit testing from mouse coordinates to nodes |
| `logger` | Filtering, formatting, panic hooks, diagnostics |
| `plugin` | Plugin host, capabilities, slot registry |
| `protocol` | Command protocol types |
| `scheduler` | Frame scheduling and pacing |
| `span_feed` | Styled span streaming |
| `syntax` | Tree-sitter based syntax highlighting |
| `theme` | Theme definitions (light/dark), spacing/border tokens |
| `clock` | Monotonic time source |

## Documentation

- API reference: https://docs.rs/bettertui_engine
- Project home: https://bettertui.dev
- Source: https://github.com/localfirstai/bettertui

## Ecosystem

`bettertui_engine` powers [BetterTUI](https://bettertui.dev). It works standalone in any pure-Rust project, and the same crate compiles into the Node.js addon used by the npm package [`@bettertui/core`](https://www.npmjs.com/package/@bettertui/core) when the optional `napi` feature is enabled.

## Minimum supported Rust version

Rust 1.88 (edition 2024).

## License

MIT. See [LICENSE](./LICENSE).
