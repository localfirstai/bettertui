//! Built-in post-processing effects.
//!
//! Each effect implements the `RenderPass` trait and can be added
//! to a `RenderPipeline`.

use crate::framebuffer::Cell;
use crate::framebuffer::FrameBuffer;
use crate::render::{PassPriority, PassResult, RenderPass, RenderPassContext};
use crate::tree::Color;

// ═══════════════════════════════════════════════════════════════════════════════
// Helper Functions
// ═══════════════════════════════════════════════════════════════════════════════

fn color_to_float(color: &Color) -> (f32, f32, f32, f32) {
    let rgba = color.to_rgba(255);
    (
        rgba.r as f32 / 255.0,
        rgba.g as f32 / 255.0,
        rgba.b as f32 / 255.0,
        rgba.a as f32 / 255.0,
    )
}

fn float_to_color(r: f32, g: f32, b: f32, _a: f32) -> Color {
    let clamp = |v: f32| (v * 255.0).clamp(0.0, 255.0) as u8;
    Color::Rgb {
        r: clamp(r),
        g: clamp(g),
        b: clamp(b),
    }
}

fn apply_color_matrix(r: f32, g: f32, b: f32, a: f32, m: &[f32; 16]) -> (f32, f32, f32, f32) {
    let r2 = r * m[0] + g * m[1] + b * m[2] + a * m[3];
    let g2 = r * m[4] + g * m[5] + b * m[6] + a * m[7];
    let b2 = r * m[8] + g * m[9] + b * m[10] + a * m[11];
    let a2 = r * m[12] + g * m[13] + b * m[14] + a * m[15];
    (r2, g2, b2, a2)
}

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

// ═══════════════════════════════════════════════════════════════════════════════
// Color Matrix
// ═══════════════════════════════════════════════════════════════════════════════

#[allow(dead_code)]
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

pub type InvertPass = ColorMatrixPass;

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

pub type GrayscalePass = ColorMatrixPass;
pub type SepiaPass = ColorMatrixPass;

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
                if cell.fg == Color::Default {
                    continue;
                }
                let (r, g, b, a) = color_to_float(&cell.fg);
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

// ═══════════════════════════════════════════════════════════════════════════════
// Bloom
// ═══════════════════════════════════════════════════════════════════════════════

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

                let r = self.radius;
                for dy in 0..=r {
                    for dx in 0..=r {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
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

// ═══════════════════════════════════════════════════════════════════════════════
// Scanlines
// ═══════════════════════════════════════════════════════════════════════════════

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

// ═══════════════════════════════════════════════════════════════════════════════
// Noise
// ═══════════════════════════════════════════════════════════════════════════════

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

// ═══════════════════════════════════════════════════════════════════════════════
// Vignette
// ═══════════════════════════════════════════════════════════════════════════════

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

// ═══════════════════════════════════════════════════════════════════════════════
// CRT
// ═══════════════════════════════════════════════════════════════════════════════

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

                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt() / max_dist;

                let darken = 1.0 - dist * dist * self.curvature;

                let (r, g, b, _a) = color_to_float(&cell.fg);
                let luminance = r * 0.2126 + g * 0.7152 + b * 0.0722;
                let glow = luminance * self.glow_strength;

                let new_r = (r * darken + glow).clamp(0.0, 1.0);
                let new_g = (g * darken + glow).clamp(0.0, 1.0);
                let new_b = (b * darken + glow).clamp(0.0, 1.0);

                let clamp_u8 = |v: f32| (v * 255.0) as u8;
                cell.fg = Color::Rgb {
                    r: clamp_u8(new_r),
                    g: clamp_u8(new_g),
                    b: clamp_u8(new_b),
                };

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

// ═══════════════════════════════════════════════════════════════════════════════
// Chromatic Aberration
// ═══════════════════════════════════════════════════════════════════════════════

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

                if cell.fg != Color::Default {
                    let (r, g, b, a) = color_to_float(&cell.fg);
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
