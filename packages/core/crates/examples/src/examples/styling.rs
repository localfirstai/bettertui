use std::io::{self, Write};

use bettertui_engine::engine::Engine;
use bettertui_engine::render::AnsiBackend;
use bettertui_engine::render::Renderer;
use bettertui_engine::tree::{Color, NamedColor, NodeKind, Style};
use bettertui_terminal::Terminal;

pub fn run(terminal: &mut Terminal) -> io::Result<()> {
    let mut out = io::stdout();
    terminal.clear()?;
    terminal.move_cursor(0, 0)?;

    writeln!(out, "\x1b[1;97m━━━ Styling: Colors, Borders, Text Effects ━━━\x1b[0m\n")?;

    // Named colors
    writeln!(out, "\x1b[33m[1]\x1b[0m Named ANSI colors...")?;
    let mut engine = Engine::new();
    let root = engine.arena().root();
    for (i, (named, name)) in [
        (NamedColor::Red, "Red"),
        (NamedColor::Green, "Green"),
        (NamedColor::Yellow, "Yellow"),
        (NamedColor::Blue, "Blue"),
        (NamedColor::Magenta, "Magenta"),
        (NamedColor::Cyan, "Cyan"),
    ].iter().enumerate() {
        let n = engine.create_node(NodeKind::Text);
        engine.set_text(n, format!("  {name:<12}  "));
        let bg = if i % 2 == 0 { NamedColor::BrightBlack } else { NamedColor::Black };
        engine.set_style(n, Style::new().fg(Color::Named(*named)).bg(Color::Named(bg)));
        engine.append_child(root, n).unwrap();
    }
    engine.begin_frame();
    engine.commit_frame();
    let mut r = Renderer::new(80, 8);
    r.set_backend(Box::new(AnsiBackend::new()));
    write!(out, "{}", String::from_utf8_lossy(&r.render_full(engine.arena_mut()).output_data))?;

    // RGB true color
    writeln!(out, "\n\x1b[33m[2]\x1b[0m RGB true colors...")?;
    let mut engine2 = Engine::new();
    let root2 = engine2.arena().root();
    for i in 0..6 {
        let n = engine2.create_node(NodeKind::Text);
        let (r_v, g_v, b_v) = ((i * 40).min(255) as u8, (255 - i * 30).max(0) as u8, (128 + i * 20).min(255) as u8);
        engine2.set_text(n, format!("  RGB({r_v:>3},{g_v:>3},{b_v:>3})  "));
        engine2.set_style(n, Style::new().fg(Color::rgb(r_v, g_v, b_v)).bg(Color::Named(NamedColor::Black)));
        engine2.append_child(root2, n).unwrap();
    }
    engine2.begin_frame();
    engine2.commit_frame();
    let mut r2 = Renderer::new(80, 4);
    r2.set_backend(Box::new(AnsiBackend::new()));
    write!(out, "{}", String::from_utf8_lossy(&r2.render_full(engine2.arena_mut()).output_data))?;

    // Text styles
    writeln!(out, "\n\x1b[33m[3]\x1b[0m Text style attributes...")?;
    let mut engine3 = Engine::new();
    let root3 = engine3.arena().root();
    for (label, bold, italic, uline) in [
        ("Bold", true, false, false),
        ("Italic", false, true, false),
        ("Underline", false, false, true),
        ("Bold+Italic", true, true, false),
    ] {
        let n = engine3.create_node(NodeKind::Text);
        engine3.set_text(n, format!("  {label:<20}  "));
        let s = Style {
            fg: Some(Color::Named(NamedColor::BrightWhite)),
            bg: Some(Color::Named(NamedColor::Black)),
            bold: Some(bold),
            italic: Some(italic),
            underline: Some(uline),
            ..Style::default()
        };
        engine3.set_style(n, s);
        engine3.append_child(root3, n).unwrap();
    }
    engine3.begin_frame();
    engine3.commit_frame();
    let mut r3 = Renderer::new(80, 8);
    r3.set_backend(Box::new(AnsiBackend::new()));
    write!(out, "{}", String::from_utf8_lossy(&r3.render_full(engine3.arena_mut()).output_data))?;

    writeln!(out, "\n\x1b[2;90mPress any key to return to menu...\x1b[0m")?;
    out.flush()?;
    wait_for_any_key(terminal)
}

fn wait_for_any_key(terminal: &mut Terminal) -> io::Result<()> {
    loop {
        if let Some(bettertui_terminal::TerminalEvent::Key(_)) = terminal.poll_event(std::time::Duration::from_millis(100))? { return Ok(()) }
    }
}
