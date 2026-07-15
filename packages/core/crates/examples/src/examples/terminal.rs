//! Terminal example: Raw mode and event handling.
//!
//! Demonstrates:
//! - `Terminal::enter_raw_mode()` for raw input
//! - `Terminal::enter_alternate_screen()` for clean buffer
//! - `Terminal::poll_event()` for keyboard/mouse/resize events
//! - Building reactive UI with `Engine` and `Renderer`

use std::io::{self, Write};
use std::time::Duration;

use bettertui_engine::engine::Engine;
use bettertui_engine::render::{AnsiBackend, Renderer};
use bettertui_engine::taffy::{FlexDirection, LayoutProps, Sizing};
use bettertui_engine::tree::{Color, NamedColor, NodeKind, Style};
use bettertui_terminal::{Key, Terminal, TerminalEvent};

pub fn run(terminal: &mut Terminal) -> io::Result<()> {
    terminal.hide_cursor()?;

    let (mut w, mut h) = (terminal.size().0, terminal.size().1);
    let mut frame = 0u64;

    loop {
        let mut engine = Engine::new();
        let root = engine.arena().root();

        engine.set_layout(
            root,
            LayoutProps {
                direction: FlexDirection::Column,
                width: Some(Sizing::Points(w as f32)),
                height: Some(Sizing::Points(h as f32)),
                ..LayoutProps::default()
            },
        );

        let border_box = engine.create_node(NodeKind::Box);
        engine.set_style(border_box, Style::new().fg(Color::rgb(71, 85, 105)));
        engine.append_child(root, border_box).unwrap();

        let title = engine.create_node(NodeKind::Text);
        engine.set_text(
            title,
            format!("BetterTUI Terminal Example - Frame {}", frame),
        );
        engine.set_style(
            title,
            Style::new()
                .fg(Color::Named(NamedColor::BrightWhite))
                .bold(true),
        );
        engine.append_child(border_box, title).unwrap();

        let spacer1 = engine.create_node(NodeKind::Text);
        engine.set_text(spacer1, "");
        engine.append_child(border_box, spacer1).unwrap();

        let size_info = engine.create_node(NodeKind::Text);
        engine.set_text(size_info, format!("Terminal: {}x{}", w, h));
        engine.set_style(
            size_info,
            Style::new().fg(Color::Named(NamedColor::BrightCyan)),
        );
        engine.append_child(border_box, size_info).unwrap();

        let frame_info = engine.create_node(NodeKind::Text);
        engine.set_text(frame_info, format!("Frame: #{}", frame));
        engine.set_style(
            frame_info,
            Style::new().fg(Color::Named(NamedColor::BrightGreen)),
        );
        engine.append_child(border_box, frame_info).unwrap();

        let spacer2 = engine.create_node(NodeKind::Text);
        engine.set_text(spacer2, "");
        engine.append_child(border_box, spacer2).unwrap();

        let ascii_box = engine.create_node(NodeKind::Text);
        engine.set_text(ascii_box, "  +----- BetterTUI -----+");
        engine.set_style(ascii_box, Style::new().fg(Color::Named(NamedColor::Cyan)));
        engine.append_child(border_box, ascii_box).unwrap();

        let line1 = engine.create_node(NodeKind::Text);
        engine.set_text(line1, "  |    Native Engine    |");
        engine.append_child(border_box, line1).unwrap();

        let line2 = engine.create_node(NodeKind::Text);
        engine.set_text(line2, "  |    Command Proto    |");
        engine.append_child(border_box, line2).unwrap();

        let line3 = engine.create_node(NodeKind::Text);
        engine.set_text(line3, "  |    Widgets Layer    |");
        engine.append_child(border_box, line3).unwrap();

        let ascii_bottom = engine.create_node(NodeKind::Text);
        engine.set_text(ascii_bottom, "  +---------------------+");
        engine.set_style(
            ascii_bottom,
            Style::new().fg(Color::Named(NamedColor::Cyan)),
        );
        engine.append_child(border_box, ascii_bottom).unwrap();

        let spacer3 = engine.create_node(NodeKind::Text);
        engine.set_text(spacer3, "");
        engine.append_child(border_box, spacer3).unwrap();

        let help = engine.create_node(NodeKind::Text);
        engine.set_text(help, "Press Esc to return to menu | Resize to test");
        engine.set_style(help, Style::new().fg(Color::rgb(148, 163, 184)));
        engine.append_child(border_box, help).unwrap();

        engine.begin_frame();
        engine.commit_frame();

        terminal.clear()?;
        terminal.move_cursor(0, 0)?;

        let mut renderer = Renderer::new(w, h);
        renderer.set_backend(Box::new(AnsiBackend::new()));
        let output = renderer.render_full(engine.arena_mut());

        let mut out = io::stdout();
        out.write_all(&output.output_data)?;
        out.flush()?;

        match terminal.poll_event(Duration::from_millis(50))? {
            Some(TerminalEvent::Key(k)) => {
                if k.code == Key::Esc {
                    break;
                }
            }
            Some(TerminalEvent::Resize(_nw, _nh)) => {
                let _ = terminal.refresh_size();
                let (w2, h2) = terminal.size();
                w = w2;
                h = h2;
            }
            _ => {}
        }

        frame += 1;
    }

    terminal.clear()?;
    terminal.show_cursor()?;
    Ok(())
}
