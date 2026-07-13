// Public constants and types intended for external use
#![allow(dead_code)]

use super::{color_to_float, float_to_color, transform_cell_color};
use crate::framebuffer::FrameBuffer;
use crate::post_process::{PassPriority, PassResult, RenderPass, RenderPassContext};
use crate::tree::color::Color;

#[cfg(test)]
use crate::framebuffer::Cell;

/// A render pass that applies a 4x4 RGBA color matrix to the framebuffer.
pub struct ColorMatrixPass {
    name: &'static str,
    enabled: bool,
    priority: PassPriority,
    matrix: [f32; 16],
    target_fg: bool,
    target_bg: bool,
    strength: f32,
}

impl ColorMatrixPass {
    pub fn new(matrix: [f32; 16]) -> Self {
        Self {
            name: "color_matrix",
            enabled: true,
            priority: PassPriority::Normal,
            matrix,
            target_fg: true,
            target_bg: true,
            strength: 1.0,
        }
    }

    pub fn with_fg_only(mut self) -> Self {
        self.target_fg = true;
        self.target_bg = false;
        self
    }

    pub fn with_bg_only(mut self) -> Self {
        self.target_fg = false;
        self.target_bg = true;
        self
    }

    pub fn with_strength(mut self, strength: f32) -> Self {
        self.strength = strength.clamp(0.0, 1.0);
        self
    }

    pub fn set_matrix(&mut self, matrix: [f32; 16]) {
        self.matrix = matrix;
    }
}

impl RenderPass for ColorMatrixPass {
    fn name(&self) -> &str {
        self.name
    }

    fn execute(&mut self, buffer: &mut FrameBuffer, _ctx: &RenderPassContext) -> PassResult {
        let mut modified = false;
        let w = buffer.width();
        let h = buffer.height();

        if self.strength < 1.0 {
            // Blend between original and matrix-transformed
            let s = self.strength;
            let inv_s = 1.0 - s;
            for y in 0..h {
                for x in 0..w {
                    let mut cell = buffer.get(x, y);
                    if cell.is_empty() {
                        continue;
                    }
                    let mut new_cell = cell;
                    transform_cell_color(
                        &mut new_cell,
                        &self.matrix,
                        self.target_fg,
                        self.target_bg,
                    );
                    // Blend fg
                    if self.target_fg && cell.fg != Color::Default {
                        let (r1, g1, b1, a1) = color_to_float(&cell.fg);
                        let (r2, g2, b2, a2) = color_to_float(&new_cell.fg);
                        cell.fg = float_to_color(
                            r1 * inv_s + r2 * s,
                            g1 * inv_s + g2 * s,
                            b1 * inv_s + b2 * s,
                            a1 * inv_s + a2 * s,
                        );
                    }
                    // Blend bg
                    if self.target_bg && cell.bg != Color::Default {
                        let (r1, g1, b1, a1) = color_to_float(&cell.bg);
                        let (r2, g2, b2, a2) = color_to_float(&new_cell.bg);
                        cell.bg = float_to_color(
                            r1 * inv_s + r2 * s,
                            g1 * inv_s + g2 * s,
                            b1 * inv_s + b2 * s,
                            a1 * inv_s + a2 * s,
                        );
                    }
                    buffer.set(x, y, cell);
                    modified = true;
                }
            }
        } else {
            for y in 0..h {
                for x in 0..w {
                    let mut cell = buffer.get(x, y);
                    if cell.is_empty() {
                        continue;
                    }
                    transform_cell_color(&mut cell, &self.matrix, self.target_fg, self.target_bg);
                    // Skip Default colors (they match terminal background)
                    if cell != buffer.get(x, y) {
                        buffer.set(x, y, cell);
                        modified = true;
                    }
                }
            }
        }

        if modified {
            PassResult::Modified
        } else {
            PassResult::Unchanged
        }
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn priority(&self) -> PassPriority {
        self.priority
    }
}

// ─── Preset Matrices ────────────────────────────────────────────────────

/// Identity matrix: no change.
pub const IDENTITY_MATRIX: [f32; 16] = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];

pub const INVERT_MATRIX: [f32; 16] = [
    -1.0, 0.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0,
];

pub const GRAYSCALE_MATRIX: [f32; 16] = [
    0.2126, 0.7152, 0.0722, 0.0, 0.2126, 0.7152, 0.0722, 0.0, 0.2126, 0.7152, 0.0722, 0.0, 0.0,
    0.0, 0.0, 1.0,
];

pub const SEPIA_MATRIX: [f32; 16] = [
    0.393, 0.769, 0.189, 0.0, 0.349, 0.686, 0.168, 0.0, 0.272, 0.534, 0.131, 0.0, 0.0, 0.0, 0.0,
    1.0,
];

// ─── Convenience Pass Builders ───────────────────────────────────────────

/// Identity pass (useful as placeholder or for disabling).
pub struct IdentityPass {
    enabled: bool,
}

impl Default for IdentityPass {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl RenderPass for IdentityPass {
    fn name(&self) -> &str {
        "identity"
    }

    fn execute(&mut self, _buffer: &mut FrameBuffer, _ctx: &RenderPassContext) -> PassResult {
        PassResult::Unchanged
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn priority(&self) -> PassPriority {
        PassPriority::Normal
    }
}

/// Invert colors pass.
pub type InvertPass = ColorMatrixPass;

/// Brightness adjustment pass.
pub struct BrightnessPass {
    inner: ColorMatrixPass,
}

impl BrightnessPass {
    pub fn new(amount: f32) -> Self {
        let clamped = amount.clamp(-1.0, 1.0);
        let m = if clamped >= 0.0 {
            [
                1.0, 0.0, 0.0, clamped, 0.0, 1.0, 0.0, clamped, 0.0, 0.0, 1.0, clamped, 0.0, 0.0,
                0.0, 1.0,
            ]
        } else {
            let f = 1.0 + clamped;
            [
                f, 0.0, 0.0, 0.0, 0.0, f, 0.0, 0.0, 0.0, 0.0, f, 0.0, 0.0, 0.0, 0.0, 1.0,
            ]
        };
        Self {
            inner: ColorMatrixPass::new(m),
        }
    }
}

impl RenderPass for BrightnessPass {
    fn name(&self) -> &str {
        "brightness"
    }
    fn execute(&mut self, buffer: &mut FrameBuffer, ctx: &RenderPassContext) -> PassResult {
        self.inner.execute(buffer, ctx)
    }
    fn enabled(&self) -> bool {
        self.inner.enabled()
    }
    fn set_enabled(&mut self, enabled: bool) {
        self.inner.set_enabled(enabled);
    }
    fn priority(&self) -> PassPriority {
        self.inner.priority()
    }
}

/// Contrast adjustment pass.
pub struct ContrastPass {
    inner: ColorMatrixPass,
}

impl ContrastPass {
    pub fn new(amount: f32) -> Self {
        let f = amount.clamp(0.0, 2.0);
        let t = (1.0 - f) / 2.0;
        Self {
            inner: ColorMatrixPass::new([
                f, 0.0, 0.0, t, 0.0, f, 0.0, t, 0.0, 0.0, f, t, 0.0, 0.0, 0.0, 1.0,
            ]),
        }
    }
}

impl RenderPass for ContrastPass {
    fn name(&self) -> &str {
        "contrast"
    }
    fn execute(&mut self, buffer: &mut FrameBuffer, ctx: &RenderPassContext) -> PassResult {
        self.inner.execute(buffer, ctx)
    }
    fn enabled(&self) -> bool {
        self.inner.enabled()
    }
    fn set_enabled(&mut self, enabled: bool) {
        self.inner.set_enabled(enabled);
    }
    fn priority(&self) -> PassPriority {
        self.inner.priority()
    }
}

/// Saturation adjustment pass (1.0 = full color, 0.0 = grayscale).
pub struct SaturationPass {
    inner: ColorMatrixPass,
}

impl SaturationPass {
    pub fn new(amount: f32) -> Self {
        let s = amount.clamp(0.0, 2.0);
        let r = 0.2126;
        let g = 0.7152;
        let b = 0.0722;
        let sr = (1.0 - s) * r;
        let sg = (1.0 - s) * g;
        let sb = (1.0 - s) * b;
        Self {
            inner: ColorMatrixPass::new([
                sr + s,
                sg,
                sb,
                0.0,
                sr,
                sg + s,
                sb,
                0.0,
                sr,
                sg,
                sb + s,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ]),
        }
    }
}

impl RenderPass for SaturationPass {
    fn name(&self) -> &str {
        "saturation"
    }
    fn execute(&mut self, buffer: &mut FrameBuffer, ctx: &RenderPassContext) -> PassResult {
        self.inner.execute(buffer, ctx)
    }
    fn enabled(&self) -> bool {
        self.inner.enabled()
    }
    fn set_enabled(&mut self, enabled: bool) {
        self.inner.set_enabled(enabled);
    }
    fn priority(&self) -> PassPriority {
        self.inner.priority()
    }
}

/// Grayscale pass using luminance weighting.
pub type GrayscalePass = ColorMatrixPass;

/// Sepia tone pass.
pub type SepiaPass = ColorMatrixPass;

/// Rainbow pass cycles hue based on x position.
pub struct RainbowPass {
    enabled: bool,
    speed: f32,
    saturation: f32,
}

impl RainbowPass {
    pub fn new() -> Self {
        Self {
            enabled: true,
            speed: 1.0,
            saturation: 1.0,
        }
    }

    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    pub fn with_saturation(mut self, sat: f32) -> Self {
        self.saturation = sat.clamp(0.0, 1.0);
        self
    }
}

impl Default for RainbowPass {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderPass for RainbowPass {
    fn name(&self) -> &str {
        "rainbow"
    }

    fn execute(&mut self, buffer: &mut FrameBuffer, ctx: &RenderPassContext) -> PassResult {
        let mut modified = false;
        let w = buffer.width();
        let h = buffer.height();
        let t = ctx.frame_count as f32 * 0.05 * self.speed;

        for y in 0..h {
            for x in 0..w {
                let mut cell = buffer.get(x, y);
                if cell.is_empty() || cell.ch == ' ' {
                    continue;
                }
                // Only apply to cells with non-default fg
                if cell.fg == Color::Default {
                    continue;
                }
                let (r, g, b, a) = color_to_float(&cell.fg);
                // Convert to HSV-like rotation: hue from x + time
                let hue = (x as f32 * 0.05 + t) % 1.0;
                let gray = r * 0.2126 + g * 0.7152 + b * 0.0722;
                let (hr, hg, hb) = hsv_to_rgb(hue, self.saturation, 1.0);
                let sr = self.saturation;
                let cell_r = gray * (1.0 - sr) + hr * sr;
                let cell_g = gray * (1.0 - sr) + hg * sr;
                let cell_b = gray * (1.0 - sr) + hb * sr;
                cell.fg = float_to_color(cell_r, cell_g, cell_b, a);
                buffer.set(x, y, cell);
                modified = true;
            }
        }

        if modified {
            PassResult::Modified
        } else {
            PassResult::Unchanged
        }
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn priority(&self) -> PassPriority {
        PassPriority::Late
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let i = (h * 6.0).floor();
    let f = h * 6.0 - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);
    match i as u32 % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::FrameBuffer;

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
        // Invert (float): 128 → 126, 64 → 191, 32 → 223
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
        let mut pass = BrightnessPass::new(0.5);
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
        // fg should be unchanged (red)
        assert_eq!(result.fg, Color::Rgb { r: 255, g: 0, b: 0 });
        // bg should be inverted (blue → yellow)
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
        // fg should be inverted (red → cyan)
        assert_eq!(
            result.fg,
            Color::Rgb {
                r: 0,
                g: 255,
                b: 255
            }
        );
        // bg should be unchanged (blue)
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
        // Strength 0.0 = no change
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
}
