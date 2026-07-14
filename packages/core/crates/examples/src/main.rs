mod app;
mod examples;
mod theme;

use std::io;

use bettertui_terminal::Terminal;

fn main() -> io::Result<()> {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        .try_init();

    let mut terminal = Terminal::new();
    let mut app = app::App::new();
    app.run(&mut terminal)
}
