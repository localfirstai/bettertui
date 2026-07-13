use crate::framebuffer::FrameBuffer;

use super::color_to_float;
#[cfg(test)]
use crate::framebuffer::Cell;
use crate::render::{PassPriority, PassResult, RenderPass, RenderPassContext};
use crate::tree::Color;

/// Scanlines effect: darkens every nth row to simulate CRT scanlines.
///
/// Two modes:
/// - `EvenRows` (default): darkens even rows (0, 2, 4, ...)
/// - `OddRows`: darkens odd rows (1, 3, 5, ...)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanlineMode {
    EvenRows,
    OddRows,
}

pub struct ScanlinesPass {
    enabled: bool,
    intensity: f32,
    mode: ScanlineMode,
}

impl ScanlinesPass {
    pub fn new() -> Self {
        Self {
            enabled: true,
            intensity: 0.3,
            mode: ScanlineMode::EvenRows,
        }
    }

    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity.clamp(0.0, 1.0);
        self
    }

    pub fn with_mode(mut self, mode: ScanlineMode) -> Self {
        self.mode = mode;
        self
    }
}

impl Default for ScanlinesPass {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderPass for ScanlinesPass {
    fn name(&self) -> &str {
        "scanlines"
    }

    fn execute(&mut self, buffer: &mut FrameBuffer, _ctx: &RenderPassContext) -> PassResult {
        let mut modified = false;
        let factor = 1.0 - self.intensity;

        for y in 0..buffer.height() {
            let is_scanline = match self.mode {
                ScanlineMode::EvenRows => y % 2 == 0,
                ScanlineMode::OddRows => y % 2 == 1,
            };
            if !is_scanline {
                continue;
            }

            for x in 0..buffer.width() {
                let mut cell = buffer.get(x, y);
                if cell.is_empty() {
                    continue;
                }

                if cell.fg != Color::Default {
                    let (r, g, b, a) = color_to_float(&cell.fg);
                    clamp_u8(&mut cell.fg, r * factor, g * factor, b * factor, a);
                }
                if cell.bg != Color::Default {
                    let (r, g, b, a) = color_to_float(&cell.bg);
                    clamp_u8(&mut cell.bg, r * factor, g * factor, b * factor, a);
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

fn clamp_u8(color: &mut Color, r: f32, g: f32, b: f32, _a: f32) {
    let clamp = |v: f32| (v * 255.0).clamp(0.0, 255.0) as u8;
    *color = Color::Rgb {
        r: clamp(r),
        g: clamp(g),
        b: clamp(b),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::FrameBuffer;

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

        // Even rows (0, 2) should be fully black
        assert_eq!(fb.get(0, 0).fg, Color::Rgb { r: 0, g: 0, b: 0 });
        assert_eq!(fb.get(0, 2).fg, Color::Rgb { r: 0, g: 0, b: 0 });
        // Odd rows (1, 3) should be unchanged
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

        // Odd row (1) should be dark
        assert_eq!(fb.get(0, 1).fg, Color::Rgb { r: 0, g: 0, b: 0 });
        // Even rows unchanged
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
}
