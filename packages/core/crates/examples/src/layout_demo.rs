use bettertui_engine::engine::Engine;
use bettertui_engine::layout::{FlexDirection, LayoutProps};
use bettertui_engine::render::AnsiBackend;
use bettertui_engine::render::Renderer;
use bettertui_engine::tree::{Color, NodeKind, Style};

use crate::util;

pub fn run() {
    util::heading("Layout Demo: Flexbox with Nested Containers");

    // ── Column layout (default) ──
    println!("[1] Column layout (default flex direction)...");
    render_layout(80, 12, |engine, root| {
        let item1 = engine.create_node(NodeKind::Text);
        let item2 = engine.create_node(NodeKind::Text);
        let item3 = engine.create_node(NodeKind::Text);
        engine.set_text(item1, "  ┌─ Row 1 ─────────────────────────────┐");
        engine.set_text(item2, "  ├─ Row 2 ─────────────────────────────┤");
        engine.set_text(item3, "  └─ Row 3 ─────────────────────────────┘");
        engine.set_style(item1, Style::new().fg(Color::rgb(255, 200, 100)));
        engine.set_style(item2, Style::new().fg(Color::rgb(100, 200, 255)));
        engine.set_style(item3, Style::new().fg(Color::rgb(100, 255, 150)));
        engine.append_child(root, item1).unwrap();
        engine.append_child(root, item2).unwrap();
        engine.append_child(root, item3).unwrap();
    });

    // ── Row layout ──
    println!("\n[2] Row layout (flex direction: row)...");
    let mut engine = Engine::new();
    let root = engine.arena().root();
    engine.set_layout(
        root,
        LayoutProps {
            direction: FlexDirection::Row,
            ..LayoutProps::default()
        },
    );
    let item1 = engine.create_node(NodeKind::Text);
    let item2 = engine.create_node(NodeKind::Text);
    let item3 = engine.create_node(NodeKind::Text);
    engine.set_text(item1, "  [A]  ");
    engine.set_text(item2, "  [B]  ");
    engine.set_text(item3, "  [C]  ");
    engine.set_style(
        item1,
        Style::new()
            .bg(Color::rgb(80, 40, 0))
            .fg(Color::rgb(255, 200, 100)),
    );
    engine.set_style(
        item2,
        Style::new()
            .bg(Color::rgb(0, 40, 80))
            .fg(Color::rgb(100, 200, 255)),
    );
    engine.set_style(
        item3,
        Style::new()
            .bg(Color::rgb(0, 60, 20))
            .fg(Color::rgb(100, 255, 150)),
    );
    engine.append_child(root, item1).unwrap();
    engine.append_child(root, item2).unwrap();
    engine.append_child(root, item3).unwrap();
    engine.begin_frame();
    engine.commit_frame();
    let mut renderer = Renderer::new(80, 6);
    renderer.set_backend(Box::new(AnsiBackend::new()));
    let frame = renderer.render_full(engine.arena_mut());
    println!("{}", String::from_utf8_lossy(&frame.output_data));

    // ── Nested containers ──
    println!("\n[3] Nested containers...");
    let mut engine2 = Engine::new();
    let root2 = engine2.arena().root();
    engine2.set_layout(
        root2,
        LayoutProps {
            direction: FlexDirection::Row,
            ..LayoutProps::default()
        },
    );
    let left = engine2.create_node(NodeKind::Box);
    let right = engine2.create_node(NodeKind::Box);
    engine2.set_layout(
        left,
        LayoutProps {
            flex_grow: 1.0,
            ..LayoutProps::default()
        },
    );
    engine2.set_layout(
        right,
        LayoutProps {
            flex_grow: 2.0,
            ..LayoutProps::default()
        },
    );
    engine2.set_style(left, Style::new().bg(Color::rgb(30, 30, 50)));
    engine2.set_style(right, Style::new().bg(Color::rgb(50, 30, 30)));

    let l_text = engine2.create_node(NodeKind::Text);
    let r_text1 = engine2.create_node(NodeKind::Text);
    let r_text2 = engine2.create_node(NodeKind::Text);
    engine2.set_text(l_text, "  Left Panel (1x)");
    engine2.set_text(r_text1, "  Right Panel (2x)");
    engine2.set_text(r_text2, "  Second line");
    engine2.set_style(
        l_text,
        Style::new().fg(Color::Named(
            bettertui_engine::tree::NamedColor::BrightWhite,
        )),
    );
    engine2.set_style(
        r_text1,
        Style::new().fg(Color::Named(
            bettertui_engine::tree::NamedColor::BrightWhite,
        )),
    );
    engine2.set_style(
        r_text2,
        Style::new().fg(Color::Named(
            bettertui_engine::tree::NamedColor::BrightBlack,
        )),
    );

    engine2.append_child(root2, left).unwrap();
    engine2.append_child(root2, right).unwrap();
    engine2.append_child(left, l_text).unwrap();
    engine2.append_child(right, r_text1).unwrap();
    engine2.append_child(right, r_text2).unwrap();

    engine2.begin_frame();
    engine2.commit_frame();
    let mut renderer2 = Renderer::new(80, 6);
    renderer2.set_backend(Box::new(AnsiBackend::new()));
    let frame2 = renderer2.render_full(engine2.arena_mut());
    let ansi = String::from_utf8_lossy(&frame2.output_data);
    println!("{}", ansi);

    // ── Layout validation ──
    println!(
        "Validation: engine.validate() = {}",
        engine.validate().is_ok()
    );
    println!(
        "Validation: engine2.validate() = {}",
        engine2.validate().is_ok()
    );
}

fn render_layout(w: u16, h: u16, build: impl FnOnce(&mut Engine, bettertui_engine::tree::NodeId)) {
    let mut engine = Engine::new();
    let root = engine.arena().root();
    build(&mut engine, root);
    engine.begin_frame();
    engine.commit_frame();
    let mut renderer = Renderer::new(w, h);
    renderer.set_backend(Box::new(AnsiBackend::new()));
    let frame = renderer.render_full(engine.arena_mut());
    println!("{}", String::from_utf8_lossy(&frame.output_data));
}
