//! Styling example: Colors and text effects.
//!
//! Demonstrates:
//! - Named ANSI colors with `Color::Named(NamedColor::*)`
//! - RGB true colors with `Color::rgb(r, g, b)`
//! - Text attributes: bold, italic, underline

use std::io::{self, Write};

use bettertui_engine::engine::Engine;
use bettertui_engine::render::{AnsiBackend, Renderer};
use bettertui_engine::tree::{Color, NamedColor, NodeKind, Style};
use bettertui_terminal::Terminal;

pub fn run(terminal: &mut Terminal) -> io::Result<()> {
    let mut out = io::stdout();
    terminal.clear()?;
    terminal.move_cursor(0, 0)?;

    let mut engine = Engine::new();
    let root = engine.arena().root();

    let title = engine.create_node(NodeKind::Text);
    engine.set_text(title, "Styling: Colors, Borders, Text Effects");
    engine.set_style(title, Style::new().fg(Color::Named(NamedColor::BrightWhite)).bold(true));
    engine.append_child(root, title).unwrap();

    let spacer = engine.create_node(NodeKind::Text);
    engine.set_text(spacer, "");
    engine.append_child(root, spacer).unwrap();

    let section1 = engine.create_node(NodeKind::Text);
    engine.set_text(section1, "[1] Named ANSI colors...");
    engine.set_style(section1, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section1).unwrap();

    for (i, (named, name)) in [
        (NamedColor::Red, "Red"),
        (NamedColor::Green, "Green"),
        (NamedColor::Yellow, "Yellow"),
        (NamedColor::Blue, "Blue"),
        (NamedColor::Magenta, "Magenta"),
        (NamedColor::Cyan, "Cyan"),
    ]
    .iter()
    .enumerate()
    {
        let n = engine.create_node(NodeKind::Text);
        engine.set_text(n, format!("  {:12}  ", name));
        let bg = if i % 2 == 0 { NamedColor::BrightBlack } else { NamedColor::Black };
        engine.set_style(n, Style::new().fg(Color::Named(*named)).bg(Color::Named(bg)));
        engine.append_child(root, n).unwrap();
    }

    let spacer2 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer2, "");
    engine.append_child(root, spacer2).unwrap();

    let section2 = engine.create_node(NodeKind::Text);
    engine.set_text(section2, "[2] RGB true colors...");
    engine.set_style(section2, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section2).unwrap();

    for i in 0..6 {
        let n = engine.create_node(NodeKind::Text);
        let (r, g, b) = ((i * 40).min(255) as u8, (255 - i * 30).max(0) as u8, (128 + i * 20).min(255) as u8);
        engine.set_text(n, format!("  RGB({:>3},{:>3},{:>3})  ", r, g, b));
        engine.set_style(n, Style::new().fg(Color::rgb(r, g, b)).bg(Color::Named(NamedColor::Black)));
        engine.append_child(root, n).unwrap();
    }

    let spacer3 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer3, "");
    engine.append_child(root, spacer3).unwrap();

    let section3 = engine.create_node(NodeKind::Text);
    engine.set_text(section3, "[3] Text style attributes...");
    engine.set_style(section3, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section3).unwrap();

    for (label, bold, italic, underline) in [
        ("Bold", true, false, false),
        ("Italic", false, true, false),
        ("Underline", false, false, true),
        ("Bold+Italic", true, true, false),
    ] {
        let n = engine.create_node(NodeKind::Text);
        engine.set_text(n, format!("  {:20}  ", label));
        engine.set_style(
            n,
            Style {
                fg: Some(Color::Named(NamedColor::BrightWhite)),
                bg: Some(Color::Named(NamedColor::Black)),
                bold: Some(bold),
                italic: Some(italic),
                underline: Some(underline),
                ..Style::default()
            },
        );
        engine.append_child(root, n).unwrap();
    }

    let spacer4 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer4, "");
    engine.append_child(root, spacer4).unwrap();

    let hint = engine.create_node(NodeKind::Text);
    engine.set_text(hint, "Press any key to return to menu...");
    engine.set_style(hint, Style { fg: Some(Color::Named(NamedColor::BrightBlack)), dim: Some(true), ..Style::new() });
    engine.append_child(root, hint).unwrap();

    engine.begin_frame();
    engine.commit_frame();

    let mut renderer = Renderer::new(80, 24);
    renderer.set_backend(Box::new(AnsiBackend::new()));
    let frame = renderer.render_full(engine.arena_mut());
    out.write_all(&frame.output_data)?;
    out.flush()?;

    wait_for_any_key(terminal)
}

fn wait_for_any_key(terminal: &mut Terminal) -> io::Result<()> {
    loop {
        if let Some(bettertui_terminal::TerminalEvent::Key(_)) =
            terminal.poll_event(std::time::Duration::from_millis(100))?
        {
            return Ok(());
        }
    }
}
