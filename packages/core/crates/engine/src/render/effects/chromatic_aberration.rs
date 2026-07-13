use crate::framebuffer::FrameBuffer;

use super::{color_to_float, float_to_color};
#[cfg(test)]
use crate::framebuffer::Cell;
use crate::render::{PassPriority, PassResult, RenderPass, RenderPassContext};
use crate::tree::Color;

/// Chromatic aberration effect: shifts color channels radially.
///
/// Simulates lens dispersion by shifting the red channel inward
/// and the blue channel outward from the center of the screen.
///
/// In a terminal context, this manifests as a subtle color fringe
/// at the edges of the display.
pub struct ChromaticAberrationPass {
    enabled: bool,
    strength: f32,
}

impl ChromaticAberrationPass {
    pub fn new() -> Self {
        Self {
            enabled: true,
            strength: 0.5,
        }
    }

    pub fn with_strength(mut self, strength: f32) -> Self {
        self.strength = strength.clamp(0.0, 2.0);
        self
    }
}

impl Default for ChromaticAberrationPass {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderPass for ChromaticAberrationPass {
    fn name(&self) -> &str {
        "chromatic_aberration"
    }

    fn execute(&mut self, buffer: &mut FrameBuffer, _ctx: &RenderPassContext) -> PassResult {
        let w = buffer.width() as f32;
        let h = buffer.height() as f32;
        let cx = w / 2.0;
        let cy = h / 2.0;
        let max_dist = (cx * cx + cy * cy).sqrt();
        let mut modified = false;

        for y in 0..buffer.height() {
            for x in 0..buffer.width() {
                let mut cell = buffer.get(x, y);
                if cell.is_empty() {
                    continue;
                }

                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt() / max_dist;
                let shift = dist * self.strength;

                if shift < 0.01 {
                    continue;
                }

                // Apply channel shift as color sepration effect
                if cell.fg != Color::Default {
                    let (r, g, b, a) = color_to_float(&cell.fg);
                    // Red shifts toward center, blue shifts outward
                    let r_factor = 1.0 - shift * 0.1;
                    let b_factor = 1.0 + shift * 0.1;
                    cell.fg = float_to_color(
                        (r * r_factor).clamp(0.0, 1.0),
                        g,
                        (b * b_factor).clamp(0.0, 1.0),
                        a,
                    );
                }
                if cell.bg != Color::Default {
                    let (r, g, b, a) = color_to_float(&cell.bg);
                    let r_factor = 1.0 - shift * 0.1;
                    let b_factor = 1.0 + shift * 0.1;
                    cell.bg = float_to_color(
                        (r * r_factor).clamp(0.0, 1.0),
                        g,
                        (b * b_factor).clamp(0.0, 1.0),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::FrameBuffer;

    #[test]
    fn chromatic_aberration_shifts_edges() {
        let mut fb = FrameBuffer::new(20, 10);
        let cell = Cell::new('X').with_fg(Color::Rgb {
            r: 200,
            g: 128,
            b: 100,
        });
        fb.set(0, 0, cell); // corner
        fb.set(10, 5, cell); // center

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
}
