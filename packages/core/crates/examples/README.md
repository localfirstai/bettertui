# bettertui-examples

## Purpose

Native Rust examples for BetterTUI, intended as production-ready
demonstrations for crate documentation. They show how to combine the core
crates (`engine`, `terminal`, `logger`) into a working TUI
application.

## Responsibilities

- **Launcher app:** `app::App` is an interactive example browser built with
  the `Engine` + `Renderer` + `Terminal` stack. It lists examples by
  category, supports filtering (`/`), navigation (`↑↓`/`jk`/`Tab`), and
  launches the selected example.
- **Categorized examples:** `examples/` ships focused demos for each
  capability area.
- **Theming:** `theme::Theme` provides `DARK` / `LIGHT` palettes used across
  the launcher and examples.

## Example Categories

| Category | Module | Demos |
|----------|--------|-------|
| Engine | `examples/engine.rs` | Command protocol, tree building, validation, ANSI rendering |
| Layout | `examples/layout.rs` | Flexbox column/row layouts with nested containers |
| Styling | `examples/styling.rs` | Named colors, RGB true color, bold/italic/underline |
| Text | `examples/text.rs` | Rope-based text engine, cursor, selection |
| Effects | `examples/post_process.rs` | Post-processing / render effects |
| Terminal | `examples/terminal.rs` | Raw mode, alternate screen, event polling |
| Syntax | `examples/syntax.rs` | tree-sitter syntax highlighting |

## Entry point

`src/main.rs` initializes `tracing`, creates a `Terminal`, and runs the
`App`. `Ctrl+C` quits.

## Dependencies

- `bettertui-engine`, `bettertui-terminal`, `bettertui-logger`
- `crossterm`, `tracing`, `tracing-subscriber`

## Usage

```bash
cargo run --manifest-path packages/core/Cargo.toml -p bettertui-examples
```

## Recommended patterns

1. **Terminal setup:** `bettertui_terminal::Terminal` for raw mode and alternate screen.
2. **Tree building:** `bettertui_engine::Engine` for imperative tree manipulation.
3. **Rendering:** `bettertui_engine::render::Renderer` + `AnsiBackend`.
4. **Theming:** `bettertui_engine::theme::Theme` for light/dark color schemes.
5. **Logging:** `bettertui_logger::init()` for production logging.

## Notes

- This is a `publish = false` development crate (version `0.0.0`); it is not
  published to crates.io.
