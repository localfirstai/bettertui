use crate::framebuffer::FrameBuffer;

use super::{color_to_float, float_to_color};
#[cfg(test)]
use crate::framebuffer::Cell;
use crate::render::{PassPriority, PassResult, RenderPass, RenderPassContext};
use crate::tree::color::Color;

/// Vignette effect: darkens corners of the screen radially.
///
/// The intensity increases with distance from the center,
/// creating a natural darkening at the edges.
pub struct VignettePass {
    enabled: bool,
    strength: f32,
    radius: f32,
    falloff: f32,
}

impl VignettePass {
    pub fn new() -> Self {
        Self {
            enabled: true,
            strength: 0.4,
            radius: 0.5,
            falloff: 2.0,
        }
    }

    pub fn with_strength(mut self, strength: f32) -> Self {
        self.strength = strength.clamp(0.0, 1.0);
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius.clamp(0.0, 1.0);
        self
    }

    pub fn with_falloff(mut self, falloff: f32) -> Self {
        self.falloff = falloff.max(0.1);
        self
    }
}

impl Default for VignettePass {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderPass for VignettePass {
    fn name(&self) -> &str {
        "vignette"
    }

    fn execute(&mut self, buffer: &mut FrameBuffer, _ctx: &RenderPassContext) -> PassResult {
        let w = buffer.width() as f32;
        let h = buffer.height() as f32;
        let cx = w / 2.0;
        let cy = h / 2.0;
        let max_dist = (cx * cx + cy * cy).sqrt();
        let inner_radius = self.radius * max_dist;
        let mut modified = false;

        for y in 0..buffer.height() {
            for x in 0..buffer.width() {
                let mut cell = buffer.get(x, y);
                if cell.is_empty() {
                    continue;
                }

                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt();
                let attenuation = if dist <= inner_radius {
                    1.0
                } else {
                    let t = ((dist - inner_radius) / (max_dist - inner_radius))
                        .clamp(0.0, 1.0)
                        .powf(self.falloff);
                    1.0 - t * self.strength
                };

                if attenuation >= 1.0 {
                    continue;
                }

                if cell.fg != Color::Default {
                    let (r, g, b, a) = color_to_float(&cell.fg);
                    cell.fg = float_to_color(r * attenuation, g * attenuation, b * attenuation, a);
                }
                if cell.bg != Color::Default {
                    let (r, g, b, a) = color_to_float(&cell.bg);
                    cell.bg = float_to_color(r * attenuation, g * attenuation, b * attenuation, a);
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
        // Center should be brighter than corner
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
}
