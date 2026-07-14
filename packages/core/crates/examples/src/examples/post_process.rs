use std::io::{self, Write};

use bettertui_engine::engine::Engine;
use bettertui_engine::render::effects::*;
use bettertui_engine::render::{AnsiBackend, Renderer};
use bettertui_engine::tree::{Color, NamedColor, NodeKind, Style};
use bettertui_terminal::Terminal;

pub fn run(terminal: &mut Terminal) -> io::Result<()> {
    let mut out = io::stdout();
    terminal.clear()?;
    terminal.move_cursor(0, 0)?;

    writeln!(out, "\x1b[1;97m━━━ Post-Processing: Render Effects Pipeline ━━━\x1b[0m\n")?;

    // Build a test tree
    let mut engine = Engine::new();
    let root = engine.arena().root();
    let title = engine.create_node(NodeKind::Text);
    engine.set_text(title, "  Post-Processing Effects Demo  ");
    engine.set_style(title, Style::new().fg(Color::Named(NamedColor::BrightWhite)).bold(true));
    engine.append_child(root, title).unwrap();
    for (r, g, b, name) in &[
        (255u8, 100u8, 100u8, "Red"),
        (100u8, 200u8, 100u8, "Green"),
        (100u8, 100u8, 255u8, "Blue"),
        (255u8, 255u8, 100u8, "Yellow"),
        (255u8, 100u8, 255u8, "Magenta"),
        (100u8, 255u8, 255u8, "Cyan"),
    ] {
        let bar = engine.create_node(NodeKind::Text);
        engine.set_text(bar, format!("  ████████  {name}  "));
        engine.set_style(bar, Style::new().fg(Color::rgb(*r, *g, *b)).bg(Color::Named(NamedColor::Black)));
        engine.append_child(root, bar).unwrap();
    }
    engine.begin_frame();
    engine.commit_frame();

    // Baseline
    writeln!(out, "\x1b[33m[1]\x1b[0m Baseline rendering (no effects)...")?;
    let mut r = Renderer::new(80, 10);
    r.set_backend(Box::new(AnsiBackend::new()));
    let frame = r.render_full(engine.arena_mut());
    writeln!(out, "  {} bytes, {} dirty regions", frame.output_data.len(), frame.dirty_regions.len())?;
    writeln!(out, "{}", String::from_utf8_lossy(&frame.output_data).trim_end())?;

    // Scanlines
    writeln!(out, "\n\x1b[33m[2]\x1b[0m CRT scanline effect...")?;
    let mut r2 = Renderer::new(80, 10);
    r2.set_backend(Box::new(AnsiBackend::new()));
    r2.pipeline_mut().add_pass(Box::new(
        ScanlinesPass::new().with_intensity(0.3).with_mode(ScanlineMode::OddRows),
    ));
    let f2 = r2.render_full(engine.arena_mut());
    writeln!(out, "  With scanlines: {} bytes", f2.output_data.len())?;

    // Color matrix (desaturate)
    writeln!(out, "\n\x1b[33m[3]\x1b[0m Color matrix (desaturate)...")?;
    let mut r3 = Renderer::new(80, 10);
    r3.set_backend(Box::new(AnsiBackend::new()));
    r3.pipeline_mut().add_pass(Box::new(ColorMatrixPass::new([
        0.33, 0.33, 0.33, 0.0,
        0.33, 0.33, 0.33, 0.0,
        0.33, 0.33, 0.33, 0.0,
        0.00, 0.00, 0.00, 1.0,
    ])));
    let f3 = r3.render_full(engine.arena_mut());
    writeln!(out, "  With desaturate: {} bytes", f3.output_data.len())?;

    // Vignette
    writeln!(out, "\n\x1b[33m[4]\x1b[0m Vignette effect...")?;
    let mut r4 = Renderer::new(80, 10);
    r4.set_backend(Box::new(AnsiBackend::new()));
    r4.pipeline_mut().add_pass(Box::new(
        VignettePass::new().with_strength(0.6).with_radius(1.2),
    ));
    let f4 = r4.render_full(engine.arena_mut());
    writeln!(out, "  With vignette: {} bytes", f4.output_data.len())?;

    // Combined
    writeln!(out, "\n\x1b[33m[5]\x1b[0m Combined (scanlines + vignette)...")?;
    let mut r5 = Renderer::new(80, 10);
    r5.set_backend(Box::new(AnsiBackend::new()));
    r5.pipeline_mut().add_pass(Box::new(
        ScanlinesPass::new().with_intensity(0.2).with_mode(ScanlineMode::EvenRows),
    ));
    r5.pipeline_mut().add_pass(Box::new(
        VignettePass::new().with_strength(0.3).with_radius(1.5),
    ));
    let f5 = r5.render_full(engine.arena_mut());
    writeln!(out, "  Combined: {} bytes, {} passes", f5.output_data.len(), r5.pipeline().len())?;

    writeln!(out, "\n\x1b[2;90mPress any key to return to menu...\x1b[0m")?;
    out.flush()?;
    wait_for_any_key(terminal)
}

fn wait_for_any_key(terminal: &mut Terminal) -> io::Result<()> {
    loop {
        if let Some(bettertui_terminal::TerminalEvent::Key(_)) = terminal.poll_event(std::time::Duration::from_millis(100))? { return Ok(()) }
    }
}
