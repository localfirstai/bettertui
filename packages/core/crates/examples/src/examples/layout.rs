//! Layout example: Flexbox with nested containers.
//!
//! Demonstrates:
//! - `FlexDirection::Column` and `FlexDirection::Row`
//! - `LayoutProps` with `flex_grow` for responsive layouts
//! - Nested containers

use std::io::{self, Write};

use bettertui_engine::engine::Engine;
use bettertui_engine::layout::{FlexDirection, LayoutProps, Sizing};
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
    engine.set_text(title, "Layout: Flexbox with Nested Containers");
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
    engine.set_text(section1, "[1] Column layout (default flex direction)...");
    engine.set_style(section1, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section1).unwrap();

    for (text, color) in [
        ("  Row 1", Color::rgb(255, 200, 100)),
        ("  Row 2", Color::rgb(100, 200, 255)),
        ("  Row 3", Color::rgb(100, 255, 150)),
    ] {
        let n = engine.create_node(NodeKind::Text);
        engine.set_text(n, text);
        engine.set_style(n, Style::new().fg(color));
        engine.append_child(root, n).unwrap();
    }

    let spacer2 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer2, "");
    engine.append_child(root, spacer2).unwrap();

    let section2 = engine.create_node(NodeKind::Text);
    engine.set_text(section2, "[2] Row layout (flex direction: row)...");
    engine.set_style(section2, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section2).unwrap();

    let row_container = engine.create_node(NodeKind::Flex);
    engine.set_layout(
        row_container,
        LayoutProps {
            direction: FlexDirection::Row,
            ..LayoutProps::default()
        },
    );
    engine.append_child(root, row_container).unwrap();

    for (text, bg, fg) in [
        ("  [A]  ", Color::rgb(80, 40, 0), Color::rgb(255, 200, 100)),
        ("  [B]  ", Color::rgb(0, 40, 80), Color::rgb(100, 200, 255)),
        ("  [C]  ", Color::rgb(0, 60, 20), Color::rgb(100, 255, 150)),
    ] {
        let n = engine.create_node(NodeKind::Text);
        engine.set_text(n, text);
        engine.set_style(n, Style::new().bg(bg).fg(fg));
        engine.append_child(row_container, n).unwrap();
    }

    let spacer3 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer3, "");
    engine.append_child(root, spacer3).unwrap();

    let section3 = engine.create_node(NodeKind::Text);
    engine.set_text(section3, "[3] Nested containers with flex_grow...");
    engine.set_style(section3, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section3).unwrap();

    let outer = engine.create_node(NodeKind::Flex);
    engine.set_layout(
        outer,
        LayoutProps {
            direction: FlexDirection::Row,
            width: Some(Sizing::Points(60.0)),
            ..LayoutProps::default()
        },
    );
    engine.append_child(root, outer).unwrap();

    let left = engine.create_node(NodeKind::Box);
    engine.set_layout(
        left,
        LayoutProps {
            flex_grow: 1.0,
            ..LayoutProps::default()
        },
    );
    engine.set_style(left, Style::new().bg(Color::rgb(30, 30, 50)));
    engine.append_child(outer, left).unwrap();

    let left_text = engine.create_node(NodeKind::Text);
    engine.set_text(left_text, "  Left (1x)  ");
    engine.set_style(
        left_text,
        Style::new().fg(Color::Named(NamedColor::BrightWhite)),
    );
    engine.append_child(left, left_text).unwrap();

    let right = engine.create_node(NodeKind::Box);
    engine.set_layout(
        right,
        LayoutProps {
            flex_grow: 2.0,
            ..LayoutProps::default()
        },
    );
    engine.set_style(right, Style::new().bg(Color::rgb(50, 30, 30)));
    engine.append_child(outer, right).unwrap();

    let right_text = engine.create_node(NodeKind::Text);
    engine.set_text(right_text, "  Right (2x)  ");
    engine.set_style(
        right_text,
        Style::new().fg(Color::Named(NamedColor::BrightWhite)),
    );
    engine.append_child(right, right_text).unwrap();

    let spacer4 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer4, "");
    engine.append_child(root, spacer4).unwrap();

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
