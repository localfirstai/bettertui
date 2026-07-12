use crate::tree::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub underline_color: Color,
    pub attributes: CellAttributes,
}

impl Cell {
    pub fn new(ch: char) -> Self {
        Self {
            ch,
            fg: Color::Default,
            bg: Color::Default,
            underline_color: Color::Default,
            attributes: CellAttributes::empty(),
        }
    }

    pub fn with_fg(mut self, fg: Color) -> Self {
        self.fg = fg;
        self
    }

    pub fn with_bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }

    pub fn with_attrs(mut self, attrs: CellAttributes) -> Self {
        self.attributes = attrs;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.ch == ' '
            && self.fg == Color::Default
            && self.bg == Color::Default
            && self.attributes.is_empty()
    }

    pub fn clear(&mut self) {
        self.ch = ' ';
        self.fg = Color::Default;
        self.bg = Color::Default;
        self.underline_color = Color::Default;
        self.attributes = CellAttributes::empty();
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new(' ')
    }
}

impl From<char> for Cell {
    fn from(ch: char) -> Self {
        Self::new(ch)
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CellAttributes: u8 {
        const BOLD          = 0b0000_0001;
        const ITALIC        = 0b0000_0010;
        const UNDERLINE     = 0b0000_0100;
        const DIM           = 0b0000_1000;
        const STRIKETHROUGH = 0b0001_0000;
        const INVERSE       = 0b0010_0000;
        const HIDDEN        = 0b0100_0000;
    }
}

impl Default for CellAttributes {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_new() {
        let c = Cell::new('A');
        assert_eq!(c.ch, 'A');
        assert_eq!(c.fg, Color::Default);
        assert_eq!(c.bg, Color::Default);
        assert!(c.attributes.is_empty());
    }

    #[test]
    fn cell_default() {
        let c = Cell::default();
        assert_eq!(c.ch, ' ');
        assert!(c.is_empty());
    }

    #[test]
    fn cell_with_fg_bg() {
        use crate::tree::color::NamedColor;
        let c = Cell::new('X')
            .with_fg(Color::Named(NamedColor::Red))
            .with_bg(Color::Named(NamedColor::Blue));
        assert_eq!(c.fg, Color::Named(NamedColor::Red));
        assert_eq!(c.bg, Color::Named(NamedColor::Blue));
    }

    #[test]
    fn cell_is_empty() {
        let mut c = Cell::new(' ');
        assert!(c.is_empty());
        c.ch = 'A';
        assert!(!c.is_empty());
    }

    #[test]
    fn cell_clear() {
        use crate::tree::color::NamedColor;
        let mut c = Cell::new('X')
            .with_fg(Color::Named(NamedColor::Red))
            .with_bg(Color::Named(NamedColor::Blue));
        c.attributes |= CellAttributes::BOLD;
        c.clear();
        assert!(c.is_empty());
        assert_eq!(c.ch, ' ');
    }

    #[test]
    fn cell_attributes_bitflags() {
        let a = CellAttributes::BOLD | CellAttributes::ITALIC;
        assert!(a.contains(CellAttributes::BOLD));
        assert!(a.contains(CellAttributes::ITALIC));
        assert!(!a.contains(CellAttributes::UNDERLINE));
    }

    #[test]
    fn cell_from_char() {
        let c: Cell = 'Z'.into();
        assert_eq!(c.ch, 'Z');
    }

    #[test]
    fn cell_equality() {
        let a = Cell::new('A');
        let b = Cell::new('A');
        let c = Cell::new('B');
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
