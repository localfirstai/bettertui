//! Post-processing example: Render effects pipeline.
//!
//! Demonstrates:
//! - `ScanlinesPass` for CRT-style scanlines
//! - `ColorMatrixPass` for color transformations
//! - `VignettePass` for vignette effect
//! - Combining multiple passes in a pipeline

use std::io::{self, Write};

use bettertui_engine::engine::Engine;
use bettertui_engine::render::effects::{ColorMatrixPass, ScanlineMode, ScanlinesPass, VignettePass};
use bettertui_engine::render::{AnsiBackend, Renderer};
use bettertui_engine::tree::{Color, NamedColor, NodeKind, Style};
use bettertui_terminal::Terminal;

pub fn run(terminal: &mut Terminal) -> io::Result<()> {
    let mut out = io::stdout();
    terminal.clear()?;
    terminal.move_cursor(0, 0)?;

    let mut base_engine = Engine::new();
    let root = base_engine.arena().root();

    let title = base_engine.create_node(NodeKind::Text);
    base_engine.set_text(title, "  Post-Processing Effects Demo  ");
    base_engine.set_style(title, Style::new().fg(Color::Named(NamedColor::BrightWhite)).bold(true));
    base_engine.append_child(root, title).unwrap();

    for (r, g, b, name) in &[
        (255u8, 100u8, 100u8, "Red"),
        (100u8, 200u8, 100u8, "Green"),
        (100u8, 100u8, 255u8, "Blue"),
        (255u8, 255u8, 100u8, "Yellow"),
        (255u8, 100u8, 255u8, "Magenta"),
        (100u8, 255u8, 255u8, "Cyan"),
    ] {
        let bar = base_engine.create_node(NodeKind::Text);
        base_engine
            .set_text(bar, format!("  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  {}  ", name));
        base_engine.set_style(bar, Style::new().fg(Color::rgb(*r, *g, *b)).bg(Color::Named(NamedColor::Black)));
        base_engine.append_child(root, bar).unwrap();
    }

    base_engine.begin_frame();
    base_engine.commit_frame();

    let mut display_engine = Engine::new();
    let display_root = display_engine.arena().root();

    let display_title = display_engine.create_node(NodeKind::Text);
    display_engine.set_text(display_title, "Post-Processing: Render Effects Pipeline");
    display_engine.set_style(display_title, Style::new().fg(Color::Named(NamedColor::BrightWhite)).bold(true));
    display_engine.append_child(display_root, display_title).unwrap();

    let spacer1 = display_engine.create_node(NodeKind::Text);
    display_engine.set_text(spacer1, "");
    display_engine.append_child(display_root, spacer1).unwrap();

    let section1 = display_engine.create_node(NodeKind::Text);
    display_engine.set_text(section1, "[1] Baseline rendering (no effects)...");
    display_engine.set_style(section1, Style::new().fg(Color::Named(NamedColor::Yellow)));
    display_engine.append_child(display_root, section1).unwrap();

    let mut r = Renderer::new(80, 10);
    r.set_backend(Box::new(AnsiBackend::new()));
    let frame = r.render_full(base_engine.arena_mut());

    let bytes1 = display_engine.create_node(NodeKind::Text);
    display_engine.set_text(
        bytes1,
        format!("    {} bytes, {} dirty regions", frame.output_data.len(), frame.dirty_regions.len()),
    );
    display_engine.append_child(display_root, bytes1).unwrap();

    let spacer2 = display_engine.create_node(NodeKind::Text);
    display_engine.set_text(spacer2, "");
    display_engine.append_child(display_root, spacer2).unwrap();

    let section2 = display_engine.create_node(NodeKind::Text);
    display_engine.set_text(section2, "[2] CRT scanline effect...");
    display_engine.set_style(section2, Style::new().fg(Color::Named(NamedColor::Yellow)));
    display_engine.append_child(display_root, section2).unwrap();

    let mut r2 = Renderer::new(80, 10);
    r2.set_backend(Box::new(AnsiBackend::new()));
    r2.pipeline_mut().add_pass(Box::new(ScanlinesPass::new().with_intensity(0.3).with_mode(ScanlineMode::OddRows)));
    let f2 = r2.render_full(base_engine.arena_mut());

    let bytes2 = display_engine.create_node(NodeKind::Text);
    display_engine.set_text(bytes2, format!("    With scanlines: {} bytes", f2.output_data.len()));
    display_engine.append_child(display_root, bytes2).unwrap();

    let spacer3 = display_engine.create_node(NodeKind::Text);
    display_engine.set_text(spacer3, "");
    display_engine.append_child(display_root, spacer3).unwrap();

    let section3 = display_engine.create_node(NodeKind::Text);
    display_engine.set_text(section3, "[3] Color matrix (desaturate)...");
    display_engine.set_style(section3, Style::new().fg(Color::Named(NamedColor::Yellow)));
    display_engine.append_child(display_root, section3).unwrap();

    let mut r3 = Renderer::new(80, 10);
    r3.set_backend(Box::new(AnsiBackend::new()));
    r3.pipeline_mut().add_pass(Box::new(ColorMatrixPass::new([
        0.33, 0.33, 0.33, 0.0, 0.33, 0.33, 0.33, 0.0, 0.33, 0.33, 0.33, 0.0, 0.00, 0.00, 0.00, 1.0,
    ])));
    let f3 = r3.render_full(base_engine.arena_mut());

    let bytes3 = display_engine.create_node(NodeKind::Text);
    display_engine.set_text(bytes3, format!("    With desaturate: {} bytes", f3.output_data.len()));
    display_engine.append_child(display_root, bytes3).unwrap();

    let spacer4 = display_engine.create_node(NodeKind::Text);
    display_engine.set_text(spacer4, "");
    display_engine.append_child(display_root, spacer4).unwrap();

    let section4 = display_engine.create_node(NodeKind::Text);
    display_engine.set_text(section4, "[4] Vignette effect...");
    display_engine.set_style(section4, Style::new().fg(Color::Named(NamedColor::Yellow)));
    display_engine.append_child(display_root, section4).unwrap();

    let mut r4 = Renderer::new(80, 10);
    r4.set_backend(Box::new(AnsiBackend::new()));
    r4.pipeline_mut().add_pass(Box::new(VignettePass::new().with_strength(0.6).with_radius(1.2)));
    let f4 = r4.render_full(base_engine.arena_mut());

    let bytes4 = display_engine.create_node(NodeKind::Text);
    display_engine.set_text(bytes4, format!("    With vignette: {} bytes", f4.output_data.len()));
    display_engine.append_child(display_root, bytes4).unwrap();

    let spacer5 = display_engine.create_node(NodeKind::Text);
    display_engine.set_text(spacer5, "");
    display_engine.append_child(display_root, spacer5).unwrap();

    let section5 = display_engine.create_node(NodeKind::Text);
    display_engine.set_text(section5, "[5] Combined (scanlines + vignette)...");
    display_engine.set_style(section5, Style::new().fg(Color::Named(NamedColor::Yellow)));
    display_engine.append_child(display_root, section5).unwrap();

    let mut r5 = Renderer::new(80, 10);
    r5.set_backend(Box::new(AnsiBackend::new()));
    r5.pipeline_mut().add_pass(Box::new(ScanlinesPass::new().with_intensity(0.2).with_mode(ScanlineMode::EvenRows)));
    r5.pipeline_mut().add_pass(Box::new(VignettePass::new().with_strength(0.3).with_radius(1.5)));
    let f5 = r5.render_full(base_engine.arena_mut());

    let bytes5 = display_engine.create_node(NodeKind::Text);
    display_engine
        .set_text(bytes5, format!("    Combined: {} bytes, {} passes", f5.output_data.len(), r5.pipeline().len()));
    display_engine.append_child(display_root, bytes5).unwrap();

    let spacer6 = display_engine.create_node(NodeKind::Text);
    display_engine.set_text(spacer6, "");
    display_engine.append_child(display_root, spacer6).unwrap();

    let hint = display_engine.create_node(NodeKind::Text);
    display_engine.set_text(hint, "Press any key to return to menu...");
    display_engine
        .set_style(hint, Style { fg: Some(Color::Named(NamedColor::BrightBlack)), dim: Some(true), ..Style::new() });
    display_engine.append_child(display_root, hint).unwrap();

    display_engine.begin_frame();
    display_engine.commit_frame();

    let mut renderer = Renderer::new(80, 24);
    renderer.set_backend(Box::new(AnsiBackend::new()));
    let frame = renderer.render_full(display_engine.arena_mut());
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
