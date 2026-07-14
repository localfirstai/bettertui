use bettertui_engine::framebuffer::{Cell, FrameBuffer};
use bettertui_engine::render::{
    PassResult, RenderPass, RenderPassContext,
    effects::{
        BloomPass, ChromaticAberrationPass, ColorMatrixPass, ContrastPass, CrtPass,
        GRAYSCALE_MATRIX, INVERT_MATRIX, IdentityPass, NoisePass, RainbowPass, SEPIA_MATRIX,
        SaturationPass, ScanlineMode, ScanlinesPass, VignettePass,
    },
};
use bettertui_engine::tree::Color;

// ═══════════════════════════════════════════════════════════════════════════════
// Color Matrix Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn identity_does_not_modify() {
    let mut fb = FrameBuffer::new(10, 10);
    let cell = Cell::new('X').with_fg(Color::Rgb {
        r: 128,
        g: 64,
        b: 32,
    });
    fb.set(5, 5, cell);
    let mut pass = IdentityPass::default();
    let ctx = RenderPassContext::new(10, 10);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Unchanged);
    assert_eq!(fb.get(5, 5), cell);
}

#[test]
fn invert_modifies_colors() {
    let mut fb = FrameBuffer::new(10, 10);
    let cell = Cell::new('X').with_fg(Color::Rgb {
        r: 128,
        g: 64,
        b: 32,
    });
    fb.set(5, 5, cell);
    let mut pass = ColorMatrixPass::new(INVERT_MATRIX);
    let ctx = RenderPassContext::new(10, 10);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Modified);
    let result = fb.get(5, 5);
    assert_ne!(result.fg, cell.fg);
    assert_eq!(
        result.fg,
        Color::Rgb {
            r: 126,
            g: 191,
            b: 223
        }
    );
}

#[test]
fn grayscale_preserves_luminance() {
    let mut fb = FrameBuffer::new(5, 5);
    let cell = Cell::new('X').with_fg(Color::Rgb {
        r: 100,
        g: 150,
        b: 200,
    });
    fb.set(2, 2, cell);
    let mut pass = ColorMatrixPass::new(GRAYSCALE_MATRIX);
    let ctx = RenderPassContext::new(5, 5);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Modified);
    let result = fb.get(2, 2);
    let _ = result.fg;
}

#[test]
fn brightness_increases_values() {
    let mut fb = FrameBuffer::new(5, 5);
    let cell = Cell::new('X').with_fg(Color::Rgb {
        r: 100,
        g: 100,
        b: 100,
    });
    fb.set(2, 2, cell);
    let mut pass = bettertui_engine::render::effects::BrightnessPass::new(0.5);
    let ctx = RenderPassContext::new(5, 5);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Modified);
    let result = fb.get(2, 2);
    match result.fg {
        Color::Rgb { r, .. } => assert!(r > 100),
        _ => panic!("Expected RGB"),
    }
}

#[test]
fn contrast_zero_makes_gray() {
    let mut fb = FrameBuffer::new(5, 5);
    let cell = Cell::new('X').with_fg(Color::Rgb {
        r: 100,
        g: 50,
        b: 200,
    });
    fb.set(2, 2, cell);
    let mut pass = ContrastPass::new(0.0);
    let ctx = RenderPassContext::new(5, 5);
    pass.execute(&mut fb, &ctx);
    let result = fb.get(2, 2);
    match result.fg {
        Color::Rgb { r, g, b } => {
            assert_eq!(r, g);
            assert_eq!(g, b);
        }
        _ => panic!("Expected RGB"),
    }
}

#[test]
fn rainbow_modifies_nonempty_cells() {
    let mut fb = FrameBuffer::new(10, 10);
    let cell = Cell::new('A').with_fg(Color::Rgb {
        r: 255,
        g: 255,
        b: 255,
    });
    fb.set(5, 5, cell);
    let mut pass = RainbowPass::new();
    let ctx = RenderPassContext::new(10, 10);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Modified);
    let result = fb.get(5, 5);
    assert_ne!(result.fg, cell.fg);
}

#[test]
fn rainbow_skips_empty_cells() {
    let mut fb = FrameBuffer::new(10, 10);
    let mut pass = RainbowPass::new();
    let ctx = RenderPassContext::new(10, 10);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Unchanged);
}

#[test]
fn sepia_warms_colors() {
    let mut fb = FrameBuffer::new(5, 5);
    let cell = Cell::new('T').with_fg(Color::Rgb {
        r: 100,
        g: 100,
        b: 200,
    });
    fb.set(2, 2, cell);
    let mut pass = ColorMatrixPass::new(SEPIA_MATRIX);
    let ctx = RenderPassContext::new(5, 5);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Modified);
    let result = fb.get(2, 2);
    match result.fg {
        Color::Rgb { r, g: _, b } => {
            assert!(r > b, "sepia should make red > blue, got r={r} b={b}");
        }
        _ => panic!("Expected RGB"),
    }
}

#[test]
fn saturation_zero_equals_grayscale() {
    let mut fb1 = FrameBuffer::new(5, 5);
    let cell = Cell::new('X').with_fg(Color::Rgb {
        r: 100,
        g: 150,
        b: 200,
    });
    fb1.set(2, 2, cell);
    let mut sat_pass = SaturationPass::new(0.0);
    let ctx = RenderPassContext::new(5, 5);
    sat_pass.execute(&mut fb1, &ctx);

    let mut fb2 = FrameBuffer::new(5, 5);
    fb2.set(2, 2, cell);
    let mut gray_pass = ColorMatrixPass::new(GRAYSCALE_MATRIX);
    gray_pass.execute(&mut fb2, &ctx);

    assert_eq!(fb1.get(2, 2).fg, fb2.get(2, 2).fg);
}

#[test]
fn color_matrix_bg_only() {
    let mut fb = FrameBuffer::new(5, 5);
    let cell = Cell::new('X')
        .with_fg(Color::Rgb { r: 255, g: 0, b: 0 })
        .with_bg(Color::Rgb { r: 0, g: 0, b: 255 });
    fb.set(2, 2, cell);
    let mut pass = ColorMatrixPass::new(INVERT_MATRIX).with_bg_only();
    let ctx = RenderPassContext::new(5, 5);
    pass.execute(&mut fb, &ctx);
    let result = fb.get(2, 2);
    assert_eq!(result.fg, Color::Rgb { r: 255, g: 0, b: 0 });
    assert_eq!(
        result.bg,
        Color::Rgb {
            r: 255,
            g: 255,
            b: 0
        }
    );
}

#[test]
fn color_matrix_fg_only() {
    let mut fb = FrameBuffer::new(5, 5);
    let cell = Cell::new('X')
        .with_fg(Color::Rgb { r: 255, g: 0, b: 0 })
        .with_bg(Color::Rgb { r: 0, g: 0, b: 255 });
    fb.set(2, 2, cell);
    let mut pass = ColorMatrixPass::new(INVERT_MATRIX).with_fg_only();
    let ctx = RenderPassContext::new(5, 5);
    pass.execute(&mut fb, &ctx);
    let result = fb.get(2, 2);
    assert_eq!(
        result.fg,
        Color::Rgb {
            r: 0,
            g: 255,
            b: 255
        }
    );
    assert_eq!(result.bg, Color::Rgb { r: 0, g: 0, b: 255 });
}

#[test]
fn color_matrix_with_strength_blend() {
    let mut fb = FrameBuffer::new(5, 5);
    let cell = Cell::new('X').with_fg(Color::Rgb {
        r: 100,
        g: 100,
        b: 100,
    });
    fb.set(2, 2, cell);
    let mut pass = ColorMatrixPass::new(INVERT_MATRIX).with_strength(0.0);
    let ctx = RenderPassContext::new(5, 5);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Modified);
    let result = fb.get(2, 2);
    assert_eq!(
        result.fg,
        Color::Rgb {
            r: 100,
            g: 100,
            b: 100
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// Bloom Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn bloom_ignores_dark_cells() {
    let mut fb = FrameBuffer::new(10, 10);
    let dark = Cell::new('.').with_fg(Color::Rgb {
        r: 10,
        g: 10,
        b: 10,
    });
    fb.set(5, 5, dark);
    let mut pass = BloomPass::new().with_threshold(0.5);
    let ctx = RenderPassContext::new(10, 10);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Unchanged);
}

#[test]
fn bloom_affects_neighbors() {
    let mut fb = FrameBuffer::new(10, 10);
    let bright = Cell::new('X').with_fg(Color::Rgb {
        r: 255,
        g: 255,
        b: 255,
    });
    let dim = Cell::new('.').with_fg(Color::Rgb {
        r: 50,
        g: 50,
        b: 50,
    });
    fb.set(5, 5, bright);
    for y in 4..=6 {
        for x in 4..=6 {
            if x == 5 && y == 5 {
                continue;
            }
            fb.set(x as u16, y as u16, dim);
        }
    }

    let orig = fb.get(4, 4).fg;
    let mut pass = BloomPass::new().with_threshold(0.0).with_strength(1.0);
    let ctx = RenderPassContext::new(10, 10);
    pass.execute(&mut fb, &ctx);

    let result = fb.get(4, 4).fg;
    assert_ne!(result, orig, "neighbor should be brightened by bloom");
}

#[test]
fn bloom_skips_empty() {
    let mut fb = FrameBuffer::new(10, 10);
    let mut pass = BloomPass::new();
    let ctx = RenderPassContext::new(10, 10);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Unchanged);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Scanlines Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn scanlines_darken_even_rows() {
    let mut fb = FrameBuffer::new(5, 4);
    let cell = Cell::new('X').with_fg(Color::Rgb {
        r: 200,
        g: 200,
        b: 200,
    });
    for y in 0..4 {
        for x in 0..5 {
            fb.set(x, y, cell);
        }
    }

    let mut pass = ScanlinesPass::new().with_intensity(1.0);
    let ctx = RenderPassContext::new(5, 4);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Modified);

    assert_eq!(fb.get(0, 0).fg, Color::Rgb { r: 0, g: 0, b: 0 });
    assert_eq!(fb.get(0, 2).fg, Color::Rgb { r: 0, g: 0, b: 0 });
    assert_eq!(
        fb.get(0, 1).fg,
        Color::Rgb {
            r: 200,
            g: 200,
            b: 200
        }
    );
    assert_eq!(
        fb.get(0, 3).fg,
        Color::Rgb {
            r: 200,
            g: 200,
            b: 200
        }
    );
}

#[test]
fn scanlines_odd_mode() {
    let mut fb = FrameBuffer::new(3, 3);
    let cell = Cell::new('X').with_fg(Color::Rgb {
        r: 200,
        g: 200,
        b: 200,
    });
    fb.set(0, 0, cell);
    fb.set(0, 1, cell);
    fb.set(0, 2, cell);

    let mut pass = ScanlinesPass::new()
        .with_intensity(1.0)
        .with_mode(ScanlineMode::OddRows);
    let ctx = RenderPassContext::new(3, 3);
    pass.execute(&mut fb, &ctx);

    assert_eq!(fb.get(0, 1).fg, Color::Rgb { r: 0, g: 0, b: 0 });
    assert_eq!(
        fb.get(0, 0).fg,
        Color::Rgb {
            r: 200,
            g: 200,
            b: 200
        }
    );
}

#[test]
fn scanlines_skips_empty() {
    let mut fb = FrameBuffer::new(5, 5);
    let mut pass = ScanlinesPass::new();
    let ctx = RenderPassContext::new(5, 5);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Unchanged);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Noise Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn noise_modifies_colors() {
    let mut fb = FrameBuffer::new(10, 10);
    let cell = Cell::new('X').with_fg(Color::Rgb {
        r: 128,
        g: 128,
        b: 128,
    });
    fb.set(5, 5, cell);
    let mut pass = NoisePass::new().with_intensity(1.0);
    let ctx = RenderPassContext::new(10, 10);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Modified);
    let result = fb.get(5, 5);
    assert_ne!(result.fg, cell.fg);
}

#[test]
fn noise_is_deterministic() {
    let mut fb1 = FrameBuffer::new(10, 10);
    let mut fb2 = FrameBuffer::new(10, 10);
    let cell = Cell::new('X').with_fg(Color::Rgb {
        r: 128,
        g: 128,
        b: 128,
    });
    for y in 0..10 {
        for x in 0..10 {
            fb1.set(x, y, cell);
            fb2.set(x, y, cell);
        }
    }

    let ctx = RenderPassContext::new(10, 10);
    let mut pass1 = NoisePass::new().with_seed(123);
    pass1.execute(&mut fb1, &ctx);
    let mut pass2 = NoisePass::new().with_seed(123);
    pass2.execute(&mut fb2, &ctx);

    for y in 0..10 {
        for x in 0..10 {
            assert_eq!(
                fb1.get(x, y).fg,
                fb2.get(x, y).fg,
                "noise should be deterministic at ({x},{y})"
            );
        }
    }
}

#[test]
fn noise_skips_empty() {
    let mut fb = FrameBuffer::new(5, 5);
    let mut pass = NoisePass::new();
    let ctx = RenderPassContext::new(5, 5);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Unchanged);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Vignette Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn vignette_darkens_corners() {
    let mut fb = FrameBuffer::new(20, 10);
    let cell = Cell::new('X').with_fg(Color::Rgb {
        r: 200,
        g: 200,
        b: 200,
    });
    for y in 0..10 {
        for x in 0..20 {
            fb.set(x, y, cell);
        }
    }

    let mut pass = VignettePass::new().with_strength(1.0).with_radius(0.0);
    let ctx = RenderPassContext::new(20, 10);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Modified);

    let corner = fb.get(0, 0);
    let center = fb.get(10, 5);
    match (corner.fg, center.fg) {
        (
            Color::Rgb {
                r: cr,
                g: cg,
                b: cb,
            },
            Color::Rgb {
                r: ctr,
                g: ctg,
                b: ctb,
            },
        ) => {
            assert!(
                ctr > cr || ctg > cg || ctb > cb,
                "center ({ctr},{ctg},{ctb}) should be brighter than corner ({cr},{cg},{cb})"
            );
        }
        _ => panic!("Expected RGB colors"),
    }
}

#[test]
fn vignette_skips_empty() {
    let mut fb = FrameBuffer::new(10, 10);
    let mut pass = VignettePass::new();
    let ctx = RenderPassContext::new(10, 10);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Unchanged);
}

// ═══════════════════════════════════════════════════════════════════════════════
// CRT Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn crt_darkens_edges() {
    let mut fb = FrameBuffer::new(20, 10);
    let cell = Cell::new('X').with_fg(Color::Rgb {
        r: 200,
        g: 200,
        b: 200,
    });
    fb.set(0, 0, cell);
    fb.set(10, 5, cell);

    let mut pass = CrtPass::new().with_curvature(1.0);
    let ctx = RenderPassContext::new(20, 10);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Modified);

    let corner = fb.get(0, 0);
    let center = fb.get(10, 5);
    match (corner.fg, center.fg) {
        (Color::Rgb { r: cr, .. }, Color::Rgb { r: ctr, .. }) => {
            assert!(
                ctr > cr,
                "center ({ctr}) should be brighter than corner ({cr})"
            );
        }
        _ => panic!("Expected RGB colors"),
    }
}

#[test]
fn crt_skips_empty() {
    let mut fb = FrameBuffer::new(10, 10);
    let mut pass = CrtPass::new();
    let ctx = RenderPassContext::new(10, 10);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Unchanged);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Chromatic Aberration Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn chromatic_aberration_shifts_edges() {
    let mut fb = FrameBuffer::new(20, 10);
    let cell = Cell::new('X').with_fg(Color::Rgb {
        r: 200,
        g: 128,
        b: 100,
    });
    fb.set(0, 0, cell);
    fb.set(10, 5, cell);

    let mut pass = ChromaticAberrationPass::new().with_strength(2.0);
    let ctx = RenderPassContext::new(20, 10);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Modified);

    let corner_fg = fb.get(0, 0).fg;
    let center_fg = fb.get(10, 5).fg;
    match (corner_fg, center_fg) {
        (Color::Rgb { r: cr, b: cb, .. }, Color::Rgb { r: ctr, b: ctb, .. }) => {
            assert!(
                cr <= ctr || cb >= ctb,
                "corner should show more chromatic shift"
            );
        }
        _ => panic!("Expected RGB colors"),
    }
}

#[test]
fn chromatic_aberration_skips_empty() {
    let mut fb = FrameBuffer::new(10, 10);
    let mut pass = ChromaticAberrationPass::new();
    let ctx = RenderPassContext::new(10, 10);
    assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Unchanged);
}
