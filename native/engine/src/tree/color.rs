/// Represents a color with its intent.
///
/// Different terminals support different color modes. A color defined as
/// `Indexed(196)` should remain `Indexed(196)` even if the terminal
/// supports true color — this preserves theme portability. Only when
/// rendering do we resolve to the best available representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Color {
    Named(NamedColor),
    Indexed(u8),
    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },
    #[default]
    Default,
}

/// Color intent preserves the original color space for rendering decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorIntent {
    /// Color is defined as RGB values
    Rgb,
    /// Color is an ANSI index (preserves palette slot)
    Indexed,
    /// Color is the terminal default
    Default,
}

/// RGBA color with alpha channel for compositing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    /// Create new RGBA color
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create RGB color (alpha = 255)
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Parse from hex string (#RGB, #RGBA, #RRGGBB, #RRGGBBAA)
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
                Some(Self::rgb(r, g, b))
            }
            4 => {
                let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
                let a = u8::from_str_radix(&hex[3..4], 16).ok()? * 17;
                Some(Self::new(r, g, b, a))
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Self::rgb(r, g, b))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Self::new(r, g, b, a))
            }
            _ => None,
        }
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        if self.a == 255 {
            format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        } else {
            format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
        }
    }

    /// Linearly interpolate between two colors
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let inv_t = 1.0 - t;
        Self {
            r: (self.r as f32 * inv_t + other.r as f32 * t) as u8,
            g: (self.g as f32 * inv_t + other.g as f32 * t) as u8,
            b: (self.b as f32 * inv_t + other.b as f32 * t) as u8,
            a: (self.a as f32 * inv_t + other.a as f32 * t) as u8,
        }
    }

    /// Alpha blend this color over another
    pub fn blend_over(&self, background: &Self) -> Self {
        let alpha = self.a as f32 / 255.0;
        let inv_alpha = 1.0 - alpha;
        Self {
            r: (self.r as f32 * alpha + background.r as f32 * inv_alpha) as u8,
            g: (self.g as f32 * alpha + background.g as f32 * inv_alpha) as u8,
            b: (self.b as f32 * alpha + background.b as f32 * inv_alpha) as u8,
            a: 255,
        }
    }
}

impl Default for Rgba {
    fn default() -> Self {
        Self::rgb(0, 0, 0)
    }
}

impl From<Rgba> for Color {
    fn from(rgba: Rgba) -> Self {
        Color::Rgb {
            r: rgba.r,
            g: rgba.g,
            b: rgba.b,
        }
    }
}

/// The 16 standard terminal colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum NamedColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    #[default]
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb { r, g, b }
    }

    /// Get the color intent (original color space)
    pub fn intent(&self) -> ColorIntent {
        match self {
            Self::Named(_) => ColorIntent::Rgb,
            Self::Indexed(_) => ColorIntent::Indexed,
            Self::Rgb { .. } => ColorIntent::Rgb,
            Self::Default => ColorIntent::Default,
        }
    }

    /// Parse color from string (hex, named, etc.)
    pub fn parse(s: &str) -> Option<Self> {
        // Try hex first
        if let Some(rgba) = Rgba::from_hex(s) {
            return Some(rgba.into());
        }

        // Try named colors
        match s.to_lowercase().as_str() {
            "black" => Some(Self::Named(NamedColor::Black)),
            "red" => Some(Self::Named(NamedColor::Red)),
            "green" => Some(Self::Named(NamedColor::Green)),
            "yellow" => Some(Self::Named(NamedColor::Yellow)),
            "blue" => Some(Self::Named(NamedColor::Blue)),
            "magenta" | "purple" => Some(Self::Named(NamedColor::Magenta)),
            "cyan" | "teal" => Some(Self::Named(NamedColor::Cyan)),
            "white" | "default" => Some(Self::Named(NamedColor::White)),
            "gray" | "grey" | "dark_gray" | "darkgrey" => {
                Some(Self::Named(NamedColor::BrightBlack))
            }
            "bright_red" | "light_red" => Some(Self::Named(NamedColor::BrightRed)),
            "bright_green" | "light_green" => Some(Self::Named(NamedColor::BrightGreen)),
            "bright_yellow" | "light_yellow" => Some(Self::Named(NamedColor::BrightYellow)),
            "bright_blue" | "light_blue" => Some(Self::Named(NamedColor::BrightBlue)),
            "bright_magenta" | "light_magenta" | "pink" => {
                Some(Self::Named(NamedColor::BrightMagenta))
            }
            "bright_cyan" | "light_cyan" => Some(Self::Named(NamedColor::BrightCyan)),
            "bright_white" | "light_gray" | "lightgrey" | "lightgray" => {
                Some(Self::Named(NamedColor::BrightWhite))
            }
            _ => None,
        }
    }

    /// Convert to RGBA (for compositing)
    pub fn to_rgba(&self, alpha: u8) -> Rgba {
        match self {
            Self::Named(named) => {
                let (r, g, b) = named.to_rgb();
                Rgba::new(r, g, b, alpha)
            }
            Self::Indexed(idx) => {
                let (r, g, b) = indexed_to_rgb(*idx);
                Rgba::new(r, g, b, alpha)
            }
            Self::Rgb { r, g, b } => Rgba::new(*r, *g, *b, alpha),
            Self::Default => Rgba::new(0, 0, 0, alpha),
        }
    }

    /// Linearly interpolate between two colors
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let c1 = self.to_rgba(255);
        let c2 = other.to_rgba(255);
        let blended = c1.lerp(&c2, t);
        Color::Rgb {
            r: blended.r,
            g: blended.g,
            b: blended.b,
        }
    }
}

impl NamedColor {
    /// Returns the ANSI color index (0-15) for this named color.
    pub fn ansi_index(&self) -> u8 {
        match self {
            Self::Black => 0,
            Self::Red => 1,
            Self::Green => 2,
            Self::Yellow => 3,
            Self::Blue => 4,
            Self::Magenta => 5,
            Self::Cyan => 6,
            Self::White => 7,
            Self::BrightBlack => 8,
            Self::BrightRed => 9,
            Self::BrightGreen => 10,
            Self::BrightYellow => 11,
            Self::BrightBlue => 12,
            Self::BrightMagenta => 13,
            Self::BrightCyan => 14,
            Self::BrightWhite => 15,
        }
    }

    /// Convert to RGB values
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Self::Black => (0, 0, 0),
            Self::Red => (170, 0, 0),
            Self::Green => (0, 170, 0),
            Self::Yellow => (170, 85, 0),
            Self::Blue => (0, 0, 170),
            Self::Magenta => (170, 0, 170),
            Self::Cyan => (0, 170, 170),
            Self::White => (170, 170, 170),
            Self::BrightBlack => (85, 85, 85),
            Self::BrightRed => (255, 85, 85),
            Self::BrightGreen => (85, 255, 85),
            Self::BrightYellow => (255, 255, 85),
            Self::BrightBlue => (85, 85, 255),
            Self::BrightMagenta => (255, 85, 255),
            Self::BrightCyan => (85, 255, 255),
            Self::BrightWhite => (255, 255, 255),
        }
    }

    /// Create from ANSI color index (0-15)
    pub fn from_ansi_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(Self::Black),
            1 => Some(Self::Red),
            2 => Some(Self::Green),
            3 => Some(Self::Yellow),
            4 => Some(Self::Blue),
            5 => Some(Self::Magenta),
            6 => Some(Self::Cyan),
            7 => Some(Self::White),
            8 => Some(Self::BrightBlack),
            9 => Some(Self::BrightRed),
            10 => Some(Self::BrightGreen),
            11 => Some(Self::BrightYellow),
            12 => Some(Self::BrightBlue),
            13 => Some(Self::BrightMagenta),
            14 => Some(Self::BrightCyan),
            15 => Some(Self::BrightWhite),
            _ => None,
        }
    }
}

/// Convert ANSI 256 indexed color to RGB
fn indexed_to_rgb(index: u8) -> (u8, u8, u8) {
    match index {
        0..=15 => NamedColor::from_ansi_index(index)
            .map(|c| c.to_rgb())
            .unwrap_or((0, 0, 0)),
        16..=231 => {
            // 6x6x6 color cube
            let idx = index - 16;
            let r = idx / 36;
            let g = (idx % 36) / 6;
            let b = idx % 6;
            let conv = |v: u8| if v == 0 { 0 } else { 55 + v * 40 };
            (conv(r), conv(g), conv(b))
        }
        232..=255 => {
            // Grayscale ramp
            let gray = 8 + (index - 232) * 10;
            (gray, gray, gray)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_color() {
        assert_eq!(Color::default(), Color::Default);
    }

    #[test]
    fn color_equality() {
        assert_eq!(Color::Named(NamedColor::Red), Color::Named(NamedColor::Red));
        assert_ne!(
            Color::Named(NamedColor::Red),
            Color::Named(NamedColor::Blue)
        );
        assert_eq!(
            Color::Rgb { r: 255, g: 0, b: 0 },
            Color::Rgb { r: 255, g: 0, b: 0 }
        );
    }

    #[test]
    fn named_color_indices() {
        assert_eq!(NamedColor::Black.ansi_index(), 0);
        assert_eq!(NamedColor::Red.ansi_index(), 1);
        assert_eq!(NamedColor::BrightWhite.ansi_index(), 15);
    }

    #[test]
    fn default_named_color() {
        assert_eq!(NamedColor::default(), NamedColor::White);
    }

    #[test]
    fn color_intent() {
        assert_eq!(Color::Named(NamedColor::Red).intent(), ColorIntent::Rgb);
        assert_eq!(Color::Indexed(196).intent(), ColorIntent::Indexed);
        assert_eq!(Color::Rgb { r: 0, g: 0, b: 0 }.intent(), ColorIntent::Rgb);
        assert_eq!(Color::Default.intent(), ColorIntent::Default);
    }

    #[test]
    fn color_parse_hex() {
        let c = Color::parse("#FF0000").unwrap();
        assert_eq!(c, Color::Rgb { r: 255, g: 0, b: 0 });

        let c = Color::parse("#00FF00").unwrap();
        assert_eq!(c, Color::Rgb { r: 0, g: 255, b: 0 });
    }

    #[test]
    fn color_parse_named() {
        let c = Color::parse("red").unwrap();
        assert_eq!(c, Color::Named(NamedColor::Red));

        let c = Color::parse("blue").unwrap();
        assert_eq!(c, Color::Named(NamedColor::Blue));

        let c = Color::parse("purple").unwrap();
        assert_eq!(c, Color::Named(NamedColor::Magenta));
    }

    #[test]
    fn color_lerp() {
        let c1 = Color::Rgb { r: 0, g: 0, b: 0 };
        let c2 = Color::Rgb {
            r: 255,
            g: 255,
            b: 255,
        };
        let blended = c1.lerp(&c2, 0.5);
        match blended {
            Color::Rgb { r, g, b } => {
                assert_eq!(r, 127);
                assert_eq!(g, 127);
                assert_eq!(b, 127);
            }
            _ => panic!("Expected RGB color"),
        }
    }

    // ─── Rgba Tests ────────────────────────────────────────────────────────

    #[test]
    fn rgba_new() {
        let c = Rgba::new(255, 128, 0, 200);
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 128);
        assert_eq!(c.b, 0);
        assert_eq!(c.a, 200);
    }

    #[test]
    fn rgba_rgb() {
        let c = Rgba::rgb(255, 128, 0);
        assert_eq!(c.a, 255);
    }

    #[test]
    fn rgba_from_hex() {
        let c = Rgba::from_hex("#FF0000").unwrap();
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 0);
        assert_eq!(c.b, 0);
        assert_eq!(c.a, 255);

        let c = Rgba::from_hex("#FF000080").unwrap();
        assert_eq!(c.a, 128);
    }

    #[test]
    fn rgba_to_hex() {
        let c = Rgba::rgb(255, 128, 0);
        assert_eq!(c.to_hex(), "#FF8000");

        let c = Rgba::new(255, 128, 0, 128);
        assert_eq!(c.to_hex(), "#FF800080");
    }

    #[test]
    fn rgba_lerp() {
        let c1 = Rgba::rgb(0, 0, 0);
        let c2 = Rgba::rgb(255, 255, 255);
        let blended = c1.lerp(&c2, 0.5);
        assert_eq!(blended.r, 127);
        assert_eq!(blended.g, 127);
        assert_eq!(blended.b, 127);
    }

    #[test]
    fn rgba_blend_over() {
        let fg = Rgba::new(255, 0, 0, 128);
        let bg = Rgba::rgb(0, 0, 255);
        let result = fg.blend_over(&bg);
        assert!(result.r > 0);
        assert!(result.b > 0);
        assert_eq!(result.a, 255);
    }

    #[test]
    fn named_color_to_rgb() {
        let (r, g, b) = NamedColor::Red.to_rgb();
        assert_eq!((r, g, b), (170, 0, 0));

        let (r, g, b) = NamedColor::BrightWhite.to_rgb();
        assert_eq!((r, g, b), (255, 255, 255));
    }

    #[test]
    fn named_color_from_index() {
        assert_eq!(NamedColor::from_ansi_index(0), Some(NamedColor::Black));
        assert_eq!(NamedColor::from_ansi_index(1), Some(NamedColor::Red));
        assert_eq!(
            NamedColor::from_ansi_index(15),
            Some(NamedColor::BrightWhite)
        );
        assert_eq!(NamedColor::from_ansi_index(16), None);
    }

    #[test]
    fn indexed_to_rgb_conversion() {
        // Black (index 0)
        let (r, g, b) = indexed_to_rgb(0);
        assert_eq!((r, g, b), (0, 0, 0));

        // White (index 15)
        let (r, g, b) = indexed_to_rgb(15);
        assert_eq!((r, g, b), (255, 255, 255));

        // Color cube (index 16 = black)
        let (r, g, b) = indexed_to_rgb(16);
        assert_eq!((r, g, b), (0, 0, 0));

        // Grayscale (index 232)
        let (r, g, b) = indexed_to_rgb(232);
        assert_eq!((r, g, b), (8, 8, 8));
    }

    #[test]
    fn color_to_rgba() {
        let c = Color::Named(NamedColor::Red);
        let rgba = c.to_rgba(200);
        assert_eq!(rgba.r, 170);
        assert_eq!(rgba.a, 200);
    }
}
