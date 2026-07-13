use super::color_to_float;
use crate::framebuffer::FrameBuffer;
use crate::render::{PassPriority, PassResult, RenderPass, RenderPassContext};
use crate::tree::Color;

#[cfg(test)]
use crate::framebuffer::Cell;

/// CRT monitor effect: slight glow, slight darkening, color bleeding.
///
/// Simulates the look of an old CRT display with:
/// - Slightly darkened edges (built-in vignette)
/// - Slight color bleed (subtle saturation increase)
/// - Dim glow effect on bright cells
pub struct CrtPass {
    enabled: bool,
    glow_strength: f32,
    curvature: f32,
}

impl CrtPass {
    pub fn new() -> Self {
        Self {
            enabled: true,
            glow_strength: 0.15,
            curvature: 0.2,
        }
    }

    pub fn with_glow(mut self, strength: f32) -> Self {
        self.glow_strength = strength.clamp(0.0, 1.0);
        self
    }

    pub fn with_curvature(mut self, curvature: f32) -> Self {
        self.curvature = curvature.clamp(0.0, 1.0);
        self
    }
}

impl Default for CrtPass {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderPass for CrtPass {
    fn name(&self) -> &str {
        "crt"
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

                // Distance from center (normalized)
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt() / max_dist;

                // Curvature darkening at edges
                let darken = 1.0 - dist * dist * self.curvature;

                // Glow: bright cells get slight bloom
                let (r, g, b, _a) = color_to_float(&cell.fg);
                let luminance = r * 0.2126 + g * 0.7152 + b * 0.0722;
                let glow = luminance * self.glow_strength;

                // Apply
                let new_r = (r * darken + glow).clamp(0.0, 1.0);
                let new_g = (g * darken + glow).clamp(0.0, 1.0);
                let new_b = (b * darken + glow).clamp(0.0, 1.0);

                let clamp_u8 = |v: f32| (v * 255.0) as u8;
                cell.fg = Color::Rgb {
                    r: clamp_u8(new_r),
                    g: clamp_u8(new_g),
                    b: clamp_u8(new_b),
                };

                // Also dim background slightly at edges
                if cell.bg != Color::Default {
                    let (br, bg, bb, _) = color_to_float(&cell.bg);
                    cell.bg = Color::Rgb {
                        r: clamp_u8((br * darken).clamp(0.0, 1.0)),
                        g: clamp_u8((bg * darken).clamp(0.0, 1.0)),
                        b: clamp_u8((bb * darken).clamp(0.0, 1.0)),
                    };
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
    fn crt_darkens_edges() {
        let mut fb = FrameBuffer::new(20, 10);
        let cell = Cell::new('X').with_fg(Color::Rgb {
            r: 200,
            g: 200,
            b: 200,
        });
        fb.set(0, 0, cell); // corner
        fb.set(10, 5, cell); // center

        let mut pass = CrtPass::new().with_curvature(1.0);
        let ctx = RenderPassContext::new(20, 10);
        assert_eq!(pass.execute(&mut fb, &ctx), PassResult::Modified);

        let corner = fb.get(0, 0);
        let center = fb.get(10, 5);
        // Center should be brighter than corner
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
}
