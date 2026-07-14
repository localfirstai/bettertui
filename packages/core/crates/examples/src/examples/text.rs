//! Text example: TextEngine buffer and cursor operations.
//!
//! Demonstrates:
//! - `TextEngine` for text buffer management
//! - Cursor movement and text insertion/deletion
//! - Multi-line editing and search
//! - Unicode support with grapheme clusters

use std::io::{self, Write};

use bettertui_engine::engine::Engine;
use bettertui_engine::render::{AnsiBackend, Renderer};
use bettertui_engine::text::{SearchOptions, TextEngine, display_width, grapheme_count};
use bettertui_engine::tree::{Color, NamedColor, NodeKind, Style};
use bettertui_terminal::Terminal;

pub fn run(terminal: &mut Terminal) -> io::Result<()> {
    let mut out = io::stdout();
    terminal.clear()?;
    terminal.move_cursor(0, 0)?;

    let mut engine = Engine::new();
    let root = engine.arena().root();

    let title = engine.create_node(NodeKind::Text);
    engine.set_text(title, "Text: TextEngine Buffer, Cursor, and Search");
    engine.set_style(
        title,
        Style::new()
            .fg(Color::Named(NamedColor::BrightWhite))
            .bold(true),
    );
    engine.append_child(root, title).unwrap();

    let spacer = engine.create_node(NodeKind::Text);
    engine.set_text(spacer, "");
    engine.append_child(root, spacer).unwrap();

    let section1 = engine.create_node(NodeKind::Text);
    engine.set_text(section1, "[1] Basic text buffer...");
    engine.set_style(section1, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section1).unwrap();

    let mut te = TextEngine::with_text("Hello, BetterTUI!");
    let info1 = engine.create_node(NodeKind::Text);
    engine.set_text(info1, format!("    Text: \"{}\"", te.text()));
    engine.append_child(root, info1).unwrap();

    let info1b = engine.create_node(NodeKind::Text);
    engine.set_text(
        info1b,
        format!(
            "    Char count: {}, Line count: {}",
            te.char_count(),
            te.line_count()
        ),
    );
    engine.append_child(root, info1b).unwrap();

    let spacer2 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer2, "");
    engine.append_child(root, spacer2).unwrap();

    let section2 = engine.create_node(NodeKind::Text);
    engine.set_text(section2, "[2] Cursor operations...");
    engine.set_style(section2, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section2).unwrap();

    te.cursor_mut().set_position(7);
    te.insert_str(" Native");
    let info2 = engine.create_node(NodeKind::Text);
    engine.set_text(
        info2,
        format!("    After insert at pos 7: \"{}\"", te.text()),
    );
    engine.append_child(root, info2).unwrap();

    te.cursor_mut().set_position(0);
    te.insert_str(">> ");
    let info2b = engine.create_node(NodeKind::Text);
    engine.set_text(info2b, format!("    After prepend: \"{}\"", te.text()));
    engine.append_child(root, info2b).unwrap();

    let spacer3 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer3, "");
    engine.append_child(root, spacer3).unwrap();

    let section3 = engine.create_node(NodeKind::Text);
    engine.set_text(section3, "[3] Delete operations...");
    engine.set_style(section3, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section3).unwrap();

    let mut te2 = TextEngine::with_text("This is some text to edit");
    te2.cursor_mut().set_position(15);
    for _ in 0..5 {
        te2.delete_char();
    }
    let info3 = engine.create_node(NodeKind::Text);
    engine.set_text(
        info3,
        format!("    After deleting 'text ': \"{}\"", te2.text()),
    );
    engine.append_child(root, info3).unwrap();

    let spacer4 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer4, "");
    engine.append_child(root, spacer4).unwrap();

    let section4 = engine.create_node(NodeKind::Text);
    engine.set_text(section4, "[4] Multi-line editing...");
    engine.set_style(section4, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section4).unwrap();

    let te3 = TextEngine::with_text("Line 1\nLine 2\nLine 3");
    for i in 0..te3.line_count() {
        if let Some(line) = te3.line(i) {
            let info4 = engine.create_node(NodeKind::Text);
            engine.set_text(info4, format!("    [{}] \"{}\"", i, line));
            engine.set_style(info4, Style::new().fg(Color::Named(NamedColor::BrightCyan)));
            engine.append_child(root, info4).unwrap();
        }
    }

    let spacer5 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer5, "");
    engine.append_child(root, spacer5).unwrap();

    let section5 = engine.create_node(NodeKind::Text);
    engine.set_text(section5, "[5] Search...");
    engine.set_style(section5, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section5).unwrap();

    let mut te4 =
        TextEngine::with_text("The quick brown fox jumps over the lazy dog. The fox is quick.");
    let results = te4.search("fox", SearchOptions::default());
    let info5 = engine.create_node(NodeKind::Text);
    engine.set_text(info5, format!("    Found 'fox' {} times", results.len()));
    engine.append_child(root, info5).unwrap();

    for r in &results {
        let info5b = engine.create_node(NodeKind::Text);
        engine.set_text(
            info5b,
            format!(
                "      at chars {}-{} (line {}, col {})",
                r.range.start, r.range.end, r.line, r.column
            ),
        );
        engine.set_style(
            info5b,
            Style::new().fg(Color::Named(NamedColor::BrightBlack)),
        );
        engine.append_child(root, info5b).unwrap();
    }

    let spacer6 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer6, "");
    engine.append_child(root, spacer6).unwrap();

    let section6 = engine.create_node(NodeKind::Text);
    engine.set_text(section6, "[6] Unicode support...");
    engine.set_style(section6, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section6).unwrap();

    let te5 = TextEngine::with_text("日本語 & 中文 & English 🔥");
    let info6 = engine.create_node(NodeKind::Text);
    engine.set_text(info6, format!("    Text: \"{}\"", te5.text()));
    engine.append_child(root, info6).unwrap();

    let info6b = engine.create_node(NodeKind::Text);
    engine.set_text(info6b, format!("    Char count: {}", te5.char_count()));
    engine.append_child(root, info6b).unwrap();

    let info6c = engine.create_node(NodeKind::Text);
    engine.set_text(
        info6c,
        format!("    Display width: {}", display_width(&te5.text())),
    );
    engine.append_child(root, info6c).unwrap();

    let info6d = engine.create_node(NodeKind::Text);
    engine.set_text(
        info6d,
        format!("    Grapheme count: {}", grapheme_count(&te5.text())),
    );
    engine.append_child(root, info6d).unwrap();

    let spacer7 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer7, "");
    engine.append_child(root, spacer7).unwrap();

    let hint = engine.create_node(NodeKind::Text);
    engine.set_text(hint, "Press any key to return to menu...");
    engine.set_style(
        hint,
        Style {
            fg: Some(Color::Named(NamedColor::BrightBlack)),
            dim: Some(true),
            ..Style::new()
        },
    );
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
