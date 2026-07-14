use std::io::{self, Write};

use bettertui_engine::engine::Engine;
use bettertui_engine::protocol::Command;
use bettertui_engine::render::AnsiBackend;
use bettertui_engine::render::Renderer;
use bettertui_engine::tree::{Color, NamedColor, NodeKind, Style};
use bettertui_terminal::Terminal;

pub fn run(terminal: &mut Terminal) -> io::Result<()> {
    let mut out = io::stdout();
    terminal.clear()?;
    terminal.move_cursor(0, 0)?;

    writeln!(out, "\x1b[1;97m━━━ Engine: Command Protocol End-to-End ━━━\x1b[0m\n")?;

    // 1. Create engine + build tree via high-level API
    writeln!(out, "\x1b[33m[1]\x1b[0m Creating engine and building tree...")?;
    let mut engine = Engine::new();
    let root = engine.arena().root();
    let child = engine.create_node(NodeKind::Text);
    engine.set_text(child, "Hello from BetterTUI!");
    engine.set_style(child, Style::new().fg(Color::Named(NamedColor::BrightGreen)).bold(true));
    engine.append_child(root, child).unwrap();
    engine.begin_frame();
    engine.commit_frame();
    writeln!(out, "  Tree:\n{}", engine.print_tree())?;
    writeln!(out, "  {}", engine.tree_summary())?;

    // 2. Render to ANSI
    writeln!(out, "\n\x1b[33m[2]\x1b[0m Rendering to ANSI output...")?;
    let mut renderer = Renderer::new(80, 24);
    renderer.set_backend(Box::new(AnsiBackend::new()));
    let frame = renderer.render_full(engine.arena_mut());
    writeln!(out, "  Output: {} bytes, {} dirty regions", frame.output_data.len(), frame.dirty_regions.len())?;
    writeln!(out, "  {}", String::from_utf8_lossy(&frame.output_data).trim_end())?;

    // 3. Build tree via raw Command protocol
    writeln!(out, "\n\x1b[33m[3]\x1b[0m Building tree via raw Command protocol...")?;
    let mut engine2 = Engine::new();
    let root2 = engine2.arena().root();
    let child2 = engine2.create_node(NodeKind::Text);
    let commands = vec![
        Command::AppendChild { parent: root2, child: child2 },
        Command::SetText { id: child2, text: "Built via Command enum".into() },
        Command::SetForeground { id: child2, color: Color::Named(NamedColor::BrightCyan) },
        Command::SetBold { id: child2, value: true },
        Command::BeginFrame { frame_id: 1 },
        Command::CommitFrame { frame_id: 1 },
    ];
    let result = engine2.process_commands(commands);
    writeln!(out, "  Processed {}, failed {}", result.processed, result.failed)?;

    // 4. Multi-node tree
    writeln!(out, "\n\x1b[33m[4]\x1b[0m Multi-node tree...")?;
    let mut engine3 = Engine::new();
    let root3 = engine3.arena().root();
    let box1 = engine3.create_node(NodeKind::Box);
    let t1 = engine3.create_node(NodeKind::Text);
    let t2 = engine3.create_node(NodeKind::Text);
    engine3.set_text(t1, "Node A");
    engine3.set_text(t2, "Node B");
    engine3.set_style(t1, Style::new().fg(Color::rgb(255, 100, 100)));
    engine3.set_style(t2, Style::new().fg(Color::rgb(100, 200, 255)));
    engine3.append_child(root3, box1).unwrap();
    engine3.append_child(box1, t1).unwrap();
    engine3.append_child(box1, t2).unwrap();
    engine3.begin_frame();
    engine3.commit_frame();

    // 5. Validate
    writeln!(out, "\n\x1b[33m[5]\x1b[0m Validation...")?;
    writeln!(out, "  Engine 1 valid: {}", engine.validate().is_ok())?;
    writeln!(out, "  Engine 2 valid: {}", engine2.validate().is_ok())?;
    writeln!(out, "  Engine 3 valid: {}", engine3.validate().is_ok())?;
    writeln!(out, "  Engine 3 node count: {}", engine3.node_count())?;

    writeln!(out, "\n\x1b[2;90mPress any key to return to menu...\x1b[0m")?;
    out.flush()?;
    wait_for_any_key(terminal)
}

fn wait_for_any_key(terminal: &mut Terminal) -> io::Result<()> {
    loop {
        if let Some(bettertui_terminal::TerminalEvent::Key(_)) = terminal.poll_event(std::time::Duration::from_millis(100))? { return Ok(()) }
    }
}
