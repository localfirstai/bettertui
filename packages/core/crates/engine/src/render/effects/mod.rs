//! Built-in post-processing effects.
//!
//! Each effect implements the `RenderPass` trait and can be added
//! to a `RenderPipeline`.

mod bloom;
mod chromatic_aberration;
mod color_matrix;
mod crt;
mod noise;
mod scanlines;
mod vignette;

pub use bloom::BloomPass;
pub use chromatic_aberration::ChromaticAberrationPass;
pub use color_matrix::{
    BrightnessPass, ColorMatrixPass, ContrastPass, GRAYSCALE_MATRIX, GrayscalePass,
    IDENTITY_MATRIX, INVERT_MATRIX, InvertPass, RainbowPass, SEPIA_MATRIX, SaturationPass,
    SepiaPass,
};
pub use crt::CrtPass;
pub use noise::NoisePass;
pub use scanlines::{ScanlineMode, ScanlinesPass};
pub use vignette::VignettePass;

use crate::framebuffer::Cell;
use crate::tree::Color;

/// Convert a Color to (r, g, b, a) normalized to [0.0, 1.0].
fn color_to_float(color: &Color) -> (f32, f32, f32, f32) {
    let rgba = color.to_rgba(255);
    (
        rgba.r as f32 / 255.0,
        rgba.g as f32 / 255.0,
        rgba.b as f32 / 255.0,
        rgba.a as f32 / 255.0,
    )
}

/// Convert (r, g, b, a) normalized values back to Color.
fn float_to_color(r: f32, g: f32, b: f32, _a: f32) -> Color {
    let clamp = |v: f32| (v * 255.0).clamp(0.0, 255.0) as u8;
    Color::Rgb {
        r: clamp(r),
        g: clamp(g),
        b: clamp(b),
    }
}

/// Apply a 4x4 color matrix to an RGB value.
/// Matrix is row-major: [m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23, m30, m31, m32, m33]
fn apply_color_matrix(r: f32, g: f32, b: f32, a: f32, m: &[f32; 16]) -> (f32, f32, f32, f32) {
    let r2 = r * m[0] + g * m[1] + b * m[2] + a * m[3];
    let g2 = r * m[4] + g * m[5] + b * m[6] + a * m[7];
    let b2 = r * m[8] + g * m[9] + b * m[10] + a * m[11];
    let a2 = r * m[12] + g * m[13] + b * m[14] + a * m[15];
    (r2, g2, b2, a2)
}

/// Apply a color transformation to a single Cell's fg and/or bg color.
pub fn transform_cell_color(cell: &mut Cell, matrix: &[f32; 16], target_fg: bool, target_bg: bool) {
    if target_fg && cell.fg != Color::Default {
        let (r, g, b, a) = color_to_float(&cell.fg);
        let (r2, g2, b2, a2) = apply_color_matrix(r, g, b, a, matrix);
        cell.fg = float_to_color(r2, g2, b2, a2);
    }
    if target_bg && cell.bg != Color::Default {
        let (r, g, b, a) = color_to_float(&cell.bg);
        let (r2, g2, b2, a2) = apply_color_matrix(r, g, b, a, matrix);
        cell.bg = float_to_color(r2, g2, b2, a2);
    }
}
