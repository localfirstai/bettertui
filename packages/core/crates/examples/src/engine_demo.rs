use bettertui_engine::engine::Engine;
use bettertui_engine::protocol::Command;
use bettertui_engine::render::AnsiBackend;
use bettertui_engine::render::Renderer;
use bettertui_engine::tree::{NodeKind, Style};

use crate::util;

pub fn run() {
    util::heading("Engine Demo: Command Protocol End-to-End");

    // ── 1. Create engine + build tree via high-level API ──
    println!("[1] Creating engine and building tree...");
    let mut engine = Engine::new();
    let root = engine.arena().root();

    let child = engine.create_node(NodeKind::Text);
    engine.set_text(child, "Hello from BetterTUI!");
    engine.set_style(
        child,
        Style::new()
            .fg(bettertui_engine::tree::Color::Named(
                bettertui_engine::tree::NamedColor::BrightGreen,
            ))
            .bold(true),
    );
    engine.append_child(root, child).unwrap();
    engine.begin_frame();
    engine.commit_frame();

    println!("  Tree:\n{}", engine.print_tree());
    println!("  {}", engine.tree_summary());

    // ── 2. Render to ANSI ──
    println!("\n[2] Rendering to ANSI output...");
    let mut renderer = Renderer::new(80, 24);
    renderer.set_backend(Box::new(AnsiBackend::new()));
    let frame = renderer.render_full(engine.arena_mut());
    let ansi_output = String::from_utf8_lossy(&frame.output_data);
    println!(
        "  Output ({} bytes, {} dirty regions):",
        frame.output_data.len(),
        frame.dirty_regions.len()
    );
    println!("  ─────────────────────");
    println!("{}", ansi_output);
    println!("  ─────────────────────");

    // ── 3. Build tree via raw Command protocol ──
    println!("\n[3] Building tree via raw Command protocol...");
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
            text: "Built via Command enum".into(),
        },
        Command::SetForeground {
            id: child2,
            color: bettertui_engine::tree::Color::Named(
                bettertui_engine::tree::NamedColor::BrightCyan,
            ),
        },
        Command::SetBold {
            id: child2,
            value: true,
        },
        Command::BeginFrame { frame_id: 1 },
        Command::CommitFrame { frame_id: 1 },
    ];
    let result = engine2.process_commands(commands);
    println!("  Processed {}, failed {}", result.processed, result.failed);
    println!("  Tree:\n{}", engine2.print_tree());

    // ── 4. Multi-node tree ──
    println!("\n[4] Building a multi-node tree...");
    let mut engine3 = Engine::new();
    let root3 = engine3.arena().root();
    let box1 = engine3.create_node(NodeKind::Box);
    let text1 = engine3.create_node(NodeKind::Text);
    let text2 = engine3.create_node(NodeKind::Text);
    engine3.set_text(text1, "Node A");
    engine3.set_text(text2, "Node B");
    engine3.set_style(
        text1,
        Style::new().fg(bettertui_engine::tree::Color::rgb(255, 100, 100)),
    );
    engine3.set_style(
        text2,
        Style::new().fg(bettertui_engine::tree::Color::rgb(100, 200, 255)),
    );
    engine3.append_child(root3, box1).unwrap();
    engine3.append_child(box1, text1).unwrap();
    engine3.append_child(box1, text2).unwrap();
    engine3.begin_frame();
    engine3.commit_frame();
    println!("  Tree:\n{}", engine3.print_tree());

    // ── 5. Render all trees ──
    println!("\n[5] Rendering multi-node tree...");
    let mut renderer2 = Renderer::new(80, 24);
    renderer2.set_backend(Box::new(AnsiBackend::new()));
    let frame2 = renderer2.render_full(engine3.arena_mut());
    let ansi_out2 = String::from_utf8_lossy(&frame2.output_data);
    println!("{}", ansi_out2);

    // ── 6. Validate tree ──
    println!("\n[6] Validating trees...");
    println!("  Engine 1 valid: {}", engine.validate().is_ok());
    println!("  Engine 2 valid: {}", engine2.validate().is_ok());
    println!("  Engine 3 valid: {}", engine3.validate().is_ok());
    println!("  Engine 3 node count: {}", engine3.node_count());
}
