//! BetterTUI Examples - Production-ready demonstrations for crates.io documentation.
//!
//! This crate contains comprehensive examples showing how to use BetterTUI's
//! core crates in a production environment:
//!
//! - `bettertui-engine`: Core rendering engine, tree management, command protocol
//! - `bettertui-widgets`: Widget framework, context, reconciler
//! - `bettertui-terminal`: Terminal handling, raw mode, event polling
//! - `bettertui-logger`: Structured logging to daily rotating files
//!
//! # Running the Examples
//!
//! ```bash
//! cargo run --manifest-path packages/core/Cargo.toml -p bettertui-examples
//! ```
//!
//! # Architecture
//!
//! The examples demonstrate the recommended patterns:
//!
//! 1. **Terminal Setup**: Use `bettertui_terminal::Terminal` for raw mode and alternate screen
//! 2. **Tree Building**: Use `bettertui_engine::Engine` for imperative tree manipulation
//! 3. **Rendering**: Use `bettertui_engine::render::Renderer` with `AnsiBackend`
//! 4. **Widgets**: Use `bettertui_widgets::WidgetHost` for reactive component lifecycle
//! 5. **Logging**: Use `bettertui_logger::init()` for production logging

pub mod app;
pub mod examples;
pub mod theme;

use std::io;

use bettertui_terminal::Terminal;

fn main() -> io::Result<()> {
    bettertui_logger::init();

    let mut terminal = Terminal::new();
    let mut app = app::App::new();
    app.run(&mut terminal)
}
