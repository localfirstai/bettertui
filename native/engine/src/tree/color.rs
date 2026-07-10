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
}
