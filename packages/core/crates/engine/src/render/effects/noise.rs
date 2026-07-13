use super::{color_to_float, float_to_color};
use crate::framebuffer::FrameBuffer;
use crate::render::{PassPriority, PassResult, RenderPass, RenderPassContext};
use crate::tree::color::Color;

#[cfg(test)]
use crate::framebuffer::Cell;

/// Noise effect: applies random per-pixel brightness variation.
///
/// Uses a simple seeded hash for deterministic noise per frame,
/// so successive renders with the same frame_count produce identical output.
pub struct NoisePass {
    enabled: bool,
    intensity: f32,
    seed: u32,
}

impl NoisePass {
    pub fn new() -> Self {
        Self {
            enabled: true,
            intensity: 0.1,
            seed: 42,
        }
    }

    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity.clamp(0.0, 1.0);
        self
    }

    pub fn with_seed(mut self, seed: u32) -> Self {
        self.seed = seed;
        self
    }
}

impl Default for NoisePass {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderPass for NoisePass {
    fn name(&self) -> &str {
        "noise"
    }

    fn execute(&mut self, buffer: &mut FrameBuffer, ctx: &RenderPassContext) -> PassResult {
        let w = buffer.width();
        let h = buffer.height();
        let s = self.seed.wrapping_add(ctx.frame_count as u32);
        let mut modified = false;

        for y in 0..h {
            for x in 0..w {
                let mut cell = buffer.get(x, y);
                if cell.is_empty() {
                    continue;
                }

                let noise = hash_noise(x as u32, y as u32, s);
                let offset = (noise - 0.5) * self.intensity * 2.0;

                if cell.fg != Color::Default {
                    let (r, g, b, a) = color_to_float(&cell.fg);
                    cell.fg = float_to_color(
                        (r + offset).clamp(0.0, 1.0),
                        (g + offset).clamp(0.0, 1.0),
                        (b + offset).clamp(0.0, 1.0),
                        a,
                    );
                }
                if cell.bg != Color::Default {
                    let (r, g, b, a) = color_to_float(&cell.bg);
                    cell.bg = float_to_color(
                        (r + offset).clamp(0.0, 1.0),
                        (g + offset).clamp(0.0, 1.0),
                        (b + offset).clamp(0.0, 1.0),
                        a,
                    );
                }

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
        PassPriority::Normal
    }
}

/// Simple hash-based 2D noise returning values in [0.0, 1.0].
fn hash_noise(x: u32, y: u32, seed: u32) -> f32 {
    let mut h = seed;
    h = h.wrapping_mul(0x9E3779B9).wrapping_add(x);
    h = h.wrapping_mul(0x9E3779B9).wrapping_add(y);
    h ^= h >> 16;
    h = h.wrapping_mul(0x85EBCA6B);
    h ^= h >> 13;
    h = h.wrapping_mul(0xC2B2AE35);
    h ^= h >> 16;
    (h as f32) / (u32::MAX as f32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::FrameBuffer;

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
}
