# bettertui_engine

High-performance terminal UI engine written in Rust. The native core of the BetterTUI framework: layout, rendering, text editing, input, animation, terminal emulation, PTY management, font/glyph handling, and syntax highlighting - all in a single dependency.

## Install

```toml
[dependencies]
bettertui_engine = "0.1"
```

## Usage

```rust
use bettertui_engine::render::Renderer;
use bettertui_engine::tree::{NodeArena, NodeKind, RenderNode};

let mut arena = NodeArena::new();
let root = arena.insert(RenderNode::new(NodeKind::Box));

let mut renderer = Renderer::new(80, 24);
let frame = renderer.render(&mut arena);
println!("{}", String::from_utf8_lossy(&frame.output_data));
```

## Feature flags

| Feature | Default | Description |
|---|---|---|
| `diagnostics` | yes | Render/frame/cache diagnostic counters. Disable with `default-features = false` for zero-overhead release builds. |
| `napi` | no | Node.js bindings via napi-rs. Only needed when building the `bettertui_engine.node` addon; not used by pure-Rust consumers. |
| `production` | no | Convenience alias enabling `release-logs-warn`. |
| `release-logs-off` / `-error` / `-warn` / `-info` / `-debug` | no | Compile-time log gating; statically removes `tracing` events below the chosen level from release builds. Pick at most one. |

## Modules

Key public modules: `tree` (arena node tree), `taffy` (flexbox layout), `render` (painter, pipeline, backends), `framebuffer`, `dirty_diff`, `ansi`, `text` (buffer, editing, search, wrap), `input` (keys, mouse, focus, keybindings), `terminal` (VT state machine, capabilities, scrollback), `pty`, `animation`, `font`, `syntax`, `scheduler`, `theme`, `protocol`.

## License

MIT. See [LICENSE](./LICENSE).
