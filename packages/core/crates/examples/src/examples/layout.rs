use std::io::{self, Write};

use bettertui_engine::engine::Engine;
use bettertui_engine::layout::{FlexDirection, LayoutProps};
use bettertui_engine::render::AnsiBackend;
use bettertui_engine::render::Renderer;
use bettertui_engine::tree::{Color, NamedColor, NodeKind, Style};
use bettertui_terminal::Terminal;

pub fn run(terminal: &mut Terminal) -> io::Result<()> {
    let mut out = io::stdout();
    terminal.clear()?;
    terminal.move_cursor(0, 0)?;

    writeln!(out, "\x1b[1;97m━━━ Layout: Flexbox with Nested Containers ━━━\x1b[0m\n")?;

    // Column layout
    writeln!(out, "\x1b[33m[1]\x1b[0m Column layout (default flex direction)...")?;
    let mut engine = Engine::new();
    let root = engine.arena().root();
    for (text, color) in [
        ("  ┌─ Row 1 ─────────────────────────────┐", Color::rgb(255, 200, 100)),
        ("  ├─ Row 2 ─────────────────────────────┤", Color::rgb(100, 200, 255)),
        ("  └─ Row 3 ─────────────────────────────┘", Color::rgb(100, 255, 150)),
    ] {
        let n = engine.create_node(NodeKind::Text);
        engine.set_text(n, text);
        engine.set_style(n, Style::new().fg(color));
        engine.append_child(root, n).unwrap();
    }
    engine.begin_frame();
    engine.commit_frame();
    let mut r = Renderer::new(80, 6);
    r.set_backend(Box::new(AnsiBackend::new()));
    writeln!(out, "{}", String::from_utf8_lossy(&r.render_full(engine.arena_mut()).output_data).trim_end())?;

    // Row layout
    writeln!(out, "\x1b[33m[2]\x1b[0m Row layout (flex direction: row)...")?;
    let mut engine2 = Engine::new();
    let root2 = engine2.arena().root();
    engine2.set_layout(root2, LayoutProps { direction: FlexDirection::Row, ..LayoutProps::default() });
    for (text, bg, fg) in [
        ("  [A]  ", Color::rgb(80, 40, 0), Color::rgb(255, 200, 100)),
        ("  [B]  ", Color::rgb(0, 40, 80), Color::rgb(100, 200, 255)),
        ("  [C]  ", Color::rgb(0, 60, 20), Color::rgb(100, 255, 150)),
    ] {
        let n = engine2.create_node(NodeKind::Text);
        engine2.set_text(n, text);
        engine2.set_style(n, Style::new().bg(bg).fg(fg));
        engine2.append_child(root2, n).unwrap();
    }
    engine2.begin_frame();
    engine2.commit_frame();
    let mut r2 = Renderer::new(80, 4);
    r2.set_backend(Box::new(AnsiBackend::new()));
    writeln!(out, "{}", String::from_utf8_lossy(&r2.render_full(engine2.arena_mut()).output_data).trim_end())?;

    // Nested containers
    writeln!(out, "\n\x1b[33m[3]\x1b[0m Nested containers with flex_grow...")?;
    let mut engine3 = Engine::new();
    let root3 = engine3.arena().root();
    engine3.set_layout(root3, LayoutProps { direction: FlexDirection::Row, ..LayoutProps::default() });
    let left = engine3.create_node(NodeKind::Box);
    let right = engine3.create_node(NodeKind::Box);
    engine3.set_layout(left, LayoutProps { flex_grow: 1.0, ..LayoutProps::default() });
    engine3.set_layout(right, LayoutProps { flex_grow: 2.0, ..LayoutProps::default() });
    engine3.set_style(left, Style::new().bg(Color::rgb(30, 30, 50)));
    engine3.set_style(right, Style::new().bg(Color::rgb(50, 30, 30)));
    let lt = engine3.create_node(NodeKind::Text);
    let rt1 = engine3.create_node(NodeKind::Text);
    let rt2 = engine3.create_node(NodeKind::Text);
    engine3.set_text(lt, "  Left Panel (1x)");
    engine3.set_text(rt1, "  Right Panel (2x)");
    engine3.set_text(rt2, "  Second line");
    engine3.set_style(lt, Style::new().fg(Color::Named(NamedColor::BrightWhite)));
    engine3.set_style(rt1, Style::new().fg(Color::Named(NamedColor::BrightWhite)));
    engine3.set_style(rt2, Style::new().fg(Color::Named(NamedColor::BrightBlack)));
    engine3.append_child(root3, left).unwrap();
    engine3.append_child(root3, right).unwrap();
    engine3.append_child(left, lt).unwrap();
    engine3.append_child(right, rt1).unwrap();
    engine3.append_child(right, rt2).unwrap();
    engine3.begin_frame();
    engine3.commit_frame();
    let mut r3 = Renderer::new(80, 5);
    r3.set_backend(Box::new(AnsiBackend::new()));
    writeln!(out, "{}", String::from_utf8_lossy(&r3.render_full(engine3.arena_mut()).output_data).trim_end())?;

    writeln!(out, "\n\x1b[2;90mPress any key to return to menu...\x1b[0m")?;
    out.flush()?;
    wait_for_any_key(terminal)
}

fn wait_for_any_key(terminal: &mut Terminal) -> io::Result<()> {
    loop {
        if let Some(bettertui_terminal::TerminalEvent::Key(_)) = terminal.poll_event(std::time::Duration::from_millis(100))? { return Ok(()) }
    }
}
