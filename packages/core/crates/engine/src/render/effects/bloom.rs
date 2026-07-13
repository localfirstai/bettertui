use crate::framebuffer::FrameBuffer;

use super::{color_to_float, float_to_color};
#[cfg(test)]
use crate::framebuffer::Cell;
use crate::render::{PassPriority, PassResult, RenderPass, RenderPassContext};
use crate::tree::color::Color;

/// Bloom effect: spreads bright areas into neighboring cells.
///
/// Simulates the glow of bright elements bleeding into adjacent dark space.
/// In a terminal, this is approximated by brightening cells adjacent to
/// bright cells, rather than true Gaussian blur.
pub struct BloomPass {
    enabled: bool,
    threshold: f32,
    strength: f32,
    radius: u16,
}

impl BloomPass {
    pub fn new() -> Self {
        Self {
            enabled: true,
            threshold: 0.7,
            strength: 0.3,
            radius: 1,
        }
    }

    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold.clamp(0.0, 1.0);
        self
    }

    pub fn with_strength(mut self, strength: f32) -> Self {
        self.strength = strength.clamp(0.0, 1.0);
        self
    }

    pub fn with_radius(mut self, radius: u16) -> Self {
        self.radius = radius.max(1);
        self
    }
}

impl Default for BloomPass {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderPass for BloomPass {
    fn name(&self) -> &str {
        "bloom"
    }

    fn execute(&mut self, buffer: &mut FrameBuffer, _ctx: &RenderPassContext) -> PassResult {
        let w = buffer.width();
        let h = buffer.height();

        // First pass: find bright cells and compute their bloom contribution
        let mut bloom_buffer = FrameBuffer::new(w, h);

        for y in 0..h {
            for x in 0..w {
                let cell = buffer.get(x, y);
                if cell.is_empty() {
                    continue;
                }

                let brightness = luminance(&cell.fg);
                if brightness < self.threshold {
                    continue;
                }

                let intensity =
                    (brightness - self.threshold) / (1.0 - self.threshold) * self.strength;

                if intensity < 0.01 {
                    continue;
                }

                // Spread bloom to neighbors
                let r = self.radius;
                for dy in 0..=r {
                    for dx in 0..=r {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        // Apply to 8 directions: (±dx, ±dy)
                        for &sx in &[-(dx as i16), dx as i16] {
                            for &sy in &[-(dy as i16), dy as i16] {
                                let nx = x as i16 + sx;
                                let ny = y as i16 + sy;
                                if nx < 0 || ny < 0 {
                                    continue;
                                }
                                let nx = nx as u16;
                                let ny = ny as u16;
                                if nx >= w || ny >= h {
                                    continue;
                                }

                                // Distance-based falloff
                                let falloff = 1.0 / (1.0 + (dx + dy) as f32 * 0.5);
                                let bloom_amount = intensity * falloff;

                                let target = buffer.get(nx, ny);
                                let mut bloom_cell = bloom_buffer.get(nx, ny);

                                if target.fg != Color::Default {
                                    let (tr, tg, tb, ta) = color_to_float(&target.fg);
                                    let (br, bg, bb, _) = color_to_float(&bloom_cell.fg);
                                    let bloom_r = tr * bloom_amount;
                                    let bloom_g = tg * bloom_amount;
                                    let bloom_b = tb * bloom_amount;
                                    bloom_cell.fg = float_to_color(
                                        br + bloom_r,
                                        bg + bloom_g,
                                        bb + bloom_b,
                                        ta,
                                    );
                                }
                                if target.bg != Color::Default {
                                    let (tr, tg, tb, ta) = color_to_float(&target.bg);
                                    let (br, bg, bb, _) = color_to_float(&bloom_cell.bg);
                                    let bloom_r = tr * bloom_amount;
                                    let bloom_g = tg * bloom_amount;
                                    let bloom_b = tb * bloom_amount;
                                    bloom_cell.bg = float_to_color(
                                        br + bloom_r,
                                        bg + bloom_g,
                                        bb + bloom_b,
                                        ta,
                                    );
                                }
                                bloom_buffer.set(nx, ny, bloom_cell);
                            }
                        }
                    }
                }
            }
        }

        // Second pass: add bloom to original buffer
        let mut modified = false;
        for y in 0..h {
            for x in 0..w {
                let mut cell = buffer.get(x, y);
                let bloom = bloom_buffer.get(x, y);
                if bloom.is_empty() {
                    continue;
                }

                if cell.fg != Color::Default && bloom.fg != Color::Default {
                    let (r, g, b, a) = color_to_float(&cell.fg);
                    let (br, bg, bb, _) = color_to_float(&bloom.fg);
                    cell.fg = float_to_color(
                        (r + br).clamp(0.0, 1.0),
                        (g + bg).clamp(0.0, 1.0),
                        (b + bb).clamp(0.0, 1.0),
                        a,
                    );
                }
                if cell.bg != Color::Default && bloom.bg != Color::Default {
                    let (r, g, b, a) = color_to_float(&cell.bg);
                    let (br, bg, bb, _) = color_to_float(&bloom.bg);
                    cell.bg = float_to_color(
                        (r + br).clamp(0.0, 1.0),
                        (g + bg).clamp(0.0, 1.0),
                        (b + bb).clamp(0.0, 1.0),
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

fn luminance(color: &Color) -> f32 {
    if *color == Color::Default {
        return 0.0;
    }
    let (r, g, b, _) = color_to_float(color);
    r * 0.2126 + g * 0.7152 + b * 0.0722
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::FrameBuffer;

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
        // Fill neighbors as dim
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
}
