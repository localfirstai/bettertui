use bettertui_engine::engine::Engine;
use bettertui_engine::render::AnsiBackend;
use bettertui_engine::render::Renderer;
use bettertui_engine::tree::{Color, NamedColor, NodeKind, Style};

use crate::util;

pub fn run() {
    util::heading("Styling Demo: Colors, Borders, Text Effects");

    // ── ANSI named colors ──
    println!("[1] Named colors...");
    let mut engine = Engine::new();
    let root = engine.arena().root();
    let colors = [
        (NamedColor::Red, "Red"),
        (NamedColor::Green, "Green"),
        (NamedColor::Yellow, "Yellow"),
        (NamedColor::Blue, "Blue"),
        (NamedColor::Magenta, "Magenta"),
        (NamedColor::Cyan, "Cyan"),
    ];
    for (i, (named, name)) in colors.iter().enumerate() {
        let node = engine.create_node(NodeKind::Text);
        engine.set_text(node, format!("  {name:<12}  "));
        let bg = if i % 2 == 0 { NamedColor::BrightBlack } else { NamedColor::Black };
        engine.set_style(node, Style::new().fg(Color::Named(*named)).bg(Color::Named(bg)));
        engine.append_child(root, node).unwrap();
    }
    engine.begin_frame();
    engine.commit_frame();
    let mut renderer = Renderer::new(80, 8);
    renderer.set_backend(Box::new(AnsiBackend::new()));
    let frame = renderer.render_full(engine.arena_mut());
    print!("{}", String::from_utf8_lossy(&frame.output_data));

    // ── RGB true color ──
    println!("\n[2] RGB true colors...");
    let mut engine2 = Engine::new();
    let root2 = engine2.arena().root();
    for i in 0..6 {
        let node = engine2.create_node(NodeKind::Text);
        let r = (i * 40).min(255) as u8;
        let g = (255 - i * 30).max(0) as u8;
        let b = (128 + i * 20).min(255) as u8;
        engine2.set_text(node, format!("  RGB({r:>3},{g:>3},{b:>3})  "));
        engine2.set_style(node, Style::new().fg(Color::rgb(r, g, b)).bg(Color::Named(NamedColor::Black)));
        engine2.append_child(root2, node).unwrap();
    }
    engine2.begin_frame();
    engine2.commit_frame();
    let mut renderer2 = Renderer::new(80, 4);
    renderer2.set_backend(Box::new(AnsiBackend::new()));
    let frame2 = renderer2.render_full(engine2.arena_mut());
    print!("{}", String::from_utf8_lossy(&frame2.output_data));

    // ── Text styles ──
    println!("\n[3] Text style attributes...");
    let mut engine3 = Engine::new();
    let root3 = engine3.arena().root();
    let entries: [(&str, Style); 4] = [
        ("Bold", Style::new().bold(true)),
        ("Italic", Style::new().italic(true)),
        ("Underline", Style::new().underline(true)),
        ("Bold+Italic", Style::new().bold(true).italic(true)),
    ];
    for (label, style) in &entries {
        let node = engine3.create_node(NodeKind::Text);
        engine3.set_text(node, format!("  {label:<20}  "));
        let combined = Style {
            fg: Some(Color::Named(NamedColor::BrightWhite)),
            bg: Some(Color::Named(NamedColor::Black)),
            bold: style.bold,
            italic: style.italic,
            underline: style.underline,
            ..Style::default()
        };
        engine3.set_style(node, combined);
        engine3.append_child(root3, node).unwrap();
    }
    engine3.begin_frame();
    engine3.commit_frame();
    let mut renderer3 = Renderer::new(80, 8);
    renderer3.set_backend(Box::new(AnsiBackend::new()));
    let frame3 = renderer3.render_full(engine3.arena_mut());
    print!("{}", String::from_utf8_lossy(&frame3.output_data));

    println!();
}
