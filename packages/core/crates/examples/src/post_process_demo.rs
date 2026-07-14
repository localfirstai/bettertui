use bettertui_engine::engine::Engine;
use bettertui_engine::render::effects::*;
use bettertui_engine::render::{AnsiBackend, Renderer};
use bettertui_engine::tree::{Color, NamedColor, NodeKind, Style};

use crate::util;

pub fn run() {
    util::heading("Post-Processing Demo: Render Effects Pipeline");

    // Build a test tree
    let mut engine = Engine::new();
    let root = engine.arena().root();

    let title = engine.create_node(NodeKind::Text);
    engine.set_text(title, "  Post-Processing Effects Demo  ");
    engine.set_style(title, Style::new().fg(Color::Named(NamedColor::BrightWhite)).bold(true));
    engine.append_child(root, title).unwrap();

    let colors = [
        (255, 100, 100, "Red"),
        (100, 200, 100, "Green"),
        (100, 100, 255, "Blue"),
        (255, 255, 100, "Yellow"),
        (255, 100, 255, "Magenta"),
        (100, 255, 255, "Cyan"),
    ];
    for (r, g, b, name) in &colors {
        let bar = engine.create_node(NodeKind::Text);
        engine.set_text(bar, format!("  ████████  {name}  "));
        engine.set_style(bar, Style::new().fg(Color::rgb(*r, *g, *b)).bg(Color::Named(NamedColor::Black)));
        engine.append_child(root, bar).unwrap();
    }

    engine.begin_frame();
    engine.commit_frame();

    // ── Baseline (no effects) ──
    println!("[1] Baseline rendering (no effects)...");
    let mut renderer = Renderer::new(80, 10);
    renderer.set_backend(Box::new(AnsiBackend::new()));
    let frame = renderer.render_full(engine.arena_mut());
    let ansi = String::from_utf8_lossy(&frame.output_data);
    println!("  Output: {} bytes, {} dirty regions",
        frame.output_data.len(), frame.dirty_regions.len());
    println!("{ansi}");

    // ── CRT effect via ScanlinesPass ──
    println!("\n[2] Adding CRT scanline effect...");
    let mut renderer2 = Renderer::new(80, 10);
    renderer2.set_backend(Box::new(AnsiBackend::new()));
    renderer2.pipeline_mut().add_pass(Box::new(
        ScanlinesPass::new().with_intensity(0.3).with_mode(ScanlineMode::OddRows),
    ));
    let frame2 = renderer2.render_full(engine.arena_mut());
    println!("  With scanlines: {} bytes", frame2.output_data.len());

    // ── Color matrix (desaturate) ──
    println!("\n[3] Adding color matrix effect (desaturate)...");
    let mut renderer3 = Renderer::new(80, 10);
    renderer3.set_backend(Box::new(AnsiBackend::new()));
    renderer3.pipeline_mut().add_pass(Box::new(
        ColorMatrixPass::new([
            0.33, 0.33, 0.33, 0.0,
            0.33, 0.33, 0.33, 0.0,
            0.33, 0.33, 0.33, 0.0,
            0.00, 0.00, 0.00, 1.0,
        ]),
    ));
    let frame3 = renderer3.render_full(engine.arena_mut());
    println!("  With desaturate matrix: {} bytes", frame3.output_data.len());

    // ── Vignette ──
    println!("\n[4] Adding vignette effect...");
    let mut renderer4 = Renderer::new(80, 10);
    renderer4.set_backend(Box::new(AnsiBackend::new()));
    renderer4.pipeline_mut().add_pass(Box::new(
        VignettePass::new().with_strength(0.6).with_radius(1.2),
    ));
    let frame4 = renderer4.render_full(engine.arena_mut());
    println!("  With vignette: {} bytes", frame4.output_data.len());

    // ── Multiple effects ──
    println!("\n[5] Multiple effects stacked...");
    let mut renderer5 = Renderer::new(80, 10);
    renderer5.set_backend(Box::new(AnsiBackend::new()));
    renderer5.pipeline_mut().add_pass(Box::new(
        ScanlinesPass::new().with_intensity(0.2).with_mode(ScanlineMode::EvenRows),
    ));
    renderer5.pipeline_mut().add_pass(Box::new(
        VignettePass::new().with_strength(0.3).with_radius(1.5),
    ));
    let frame5 = renderer5.render_full(engine.arena_mut());
    println!("  Combined (scanlines + vignette): {} bytes, {} passes active",
        frame5.output_data.len(),
        renderer5.pipeline().len(),
    );

    println!();
}
