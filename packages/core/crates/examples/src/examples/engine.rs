//! Engine example: Command protocol and tree management.
//!
//! Demonstrates:
//! - Building trees with `Engine` high-level API
//! - Using the `Command` protocol directly
//! - Rendering with `Renderer` + `AnsiBackend`
//! - Tree validation

use std::io::{self, Write};

use bettertui_engine::engine::Engine;
use bettertui_engine::protocol::Command;
use bettertui_engine::render::{AnsiBackend, Renderer};
use bettertui_engine::taffy::{FlexDirection, LayoutProps};
use bettertui_engine::tree::{Color, NamedColor, NodeKind, Style};
use bettertui_terminal::Terminal;

pub fn run(terminal: &mut Terminal) -> io::Result<()> {
    let mut out = io::stdout();
    terminal.clear()?;
    terminal.move_cursor(0, 0)?;

    let mut engine = Engine::new();
    let root = engine.arena().root();

    let title = engine.create_node(NodeKind::Text);
    engine.set_text(title, "Engine: Command Protocol End-to-End");
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
    engine.set_text(section1, "[1] Creating engine and building tree...");
    engine.set_style(section1, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section1).unwrap();

    let child = engine.create_node(NodeKind::Text);
    engine.set_text(child, "    Hello from BetterTUI!");
    engine.set_style(
        child,
        Style::new()
            .fg(Color::Named(NamedColor::BrightGreen))
            .bold(true),
    );
    engine.append_child(root, child).unwrap();

    let info = engine.create_node(NodeKind::Text);
    engine.set_text(
        info,
        format!("    {}", engine.tree_summary().replace("\n", "\n    ")),
    );
    engine.set_style(info, Style::new().fg(Color::Named(NamedColor::BrightBlack)));
    engine.append_child(root, info).unwrap();

    let spacer2 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer2, "");
    engine.append_child(root, spacer2).unwrap();

    let section2 = engine.create_node(NodeKind::Text);
    engine.set_text(section2, "[2] Rendering to ANSI output...");
    engine.set_style(section2, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section2).unwrap();

    let bytes_info = engine.create_node(NodeKind::Text);
    engine.set_text(bytes_info, "    See rendered output below:");
    engine.append_child(root, bytes_info).unwrap();

    let spacer3 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer3, "");
    engine.append_child(root, spacer3).unwrap();

    let section3 = engine.create_node(NodeKind::Text);
    engine.set_text(section3, "[3] Building tree via raw Command protocol...");
    engine.set_style(section3, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section3).unwrap();

    let mut engine2 = Engine::new();
    let root2 = engine2.arena().root();
    let child2 = engine2.create_node(NodeKind::Text);
    let commands = vec![
        Command::AppendChild {
            parent: root2,
            child: child2,
        },
        Command::SetText {
            id: child2,
            text: "    Built via Command enum".into(),
        },
        Command::SetForeground {
            id: child2,
            color: Color::Named(NamedColor::BrightCyan),
        },
        Command::SetBold {
            id: child2,
            value: true,
        },
        Command::BeginFrame { frame_id: 1 },
        Command::CommitFrame { frame_id: 1 },
    ];
    let result = engine2.process_commands(commands);

    let cmd_result = engine.create_node(NodeKind::Text);
    engine.set_text(
        cmd_result,
        format!(
            "    Processed {}, failed {}",
            result.processed, result.failed
        ),
    );
    engine.append_child(root, cmd_result).unwrap();

    let spacer4 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer4, "");
    engine.append_child(root, spacer4).unwrap();

    let section4 = engine.create_node(NodeKind::Text);
    engine.set_text(section4, "[4] Multi-node tree...");
    engine.set_style(section4, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section4).unwrap();

    let mut engine3 = Engine::new();
    let root3 = engine3.arena().root();
    engine3.set_layout(
        root3,
        LayoutProps {
            direction: FlexDirection::Row,
            ..LayoutProps::default()
        },
    );

    for (text, color) in [
        ("Node A", Color::rgb(255, 100, 100)),
        ("Node B", Color::rgb(100, 200, 255)),
    ] {
        let n = engine3.create_node(NodeKind::Text);
        engine3.set_text(n, format!("  {}  ", text));
        engine3.set_style(n, Style::new().fg(color));
        engine3.append_child(root3, n).unwrap();
    }
    engine3.begin_frame();
    engine3.commit_frame();

    let node_count = engine.create_node(NodeKind::Text);
    engine.set_text(
        node_count,
        format!("    Engine 3 node count: {}", engine3.node_count()),
    );
    engine.append_child(root, node_count).unwrap();

    let spacer5 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer5, "");
    engine.append_child(root, spacer5).unwrap();

    let section5 = engine.create_node(NodeKind::Text);
    engine.set_text(section5, "[5] Validation...");
    engine.set_style(section5, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section5).unwrap();

    let validation = engine.create_node(NodeKind::Text);
    engine.set_text(
        validation,
        format!("    Engine 1 valid: {}", engine.validate().is_ok()),
    );
    engine.append_child(root, validation).unwrap();

    let validation2 = engine.create_node(NodeKind::Text);
    engine.set_text(
        validation2,
        format!("    Engine 2 valid: {}", engine2.validate().is_ok()),
    );
    engine.append_child(root, validation2).unwrap();

    let validation3 = engine.create_node(NodeKind::Text);
    engine.set_text(
        validation3,
        format!("    Engine 3 valid: {}", engine3.validate().is_ok()),
    );
    engine.append_child(root, validation3).unwrap();

    let spacer6 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer6, "");
    engine.append_child(root, spacer6).unwrap();

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
