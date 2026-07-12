use crate::framebuffer::CellAttributes;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlyphId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Glyph {
    pub id: GlyphId,
    pub ch: char,
    pub width: u8,
    pub height: u8,
    pub advance_x: u8,
    pub advance_y: u8,
    pub offset_x: i8,
    pub offset_y: i8,
    pub attributes: CellAttributes,
    pub category: GlyphCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlyphCategory {
    Ascii,
    AsciiExtended,
    Unicode,
    Emoji,
    NerdFont,
    BoxDrawing,
    Braille,
    Cjk,
    Diacritical,
    Symbol,
}

impl Glyph {
    pub fn new(ch: char) -> Self {
        Self {
            id: GlyphId(ch as u32),
            ch,
            width: 1,
            height: 1,
            advance_x: 1,
            advance_y: 1,
            offset_x: 0,
            offset_y: 0,
            attributes: CellAttributes::empty(),
            category: GlyphCategory::from_char(ch),
        }
    }

    pub fn with_width(mut self, width: u8) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: u8) -> Self {
        self.height = height;
        self
    }

    pub fn with_advance_x(mut self, advance: u8) -> Self {
        self.advance_x = advance;
        self
    }

    pub fn with_offset(mut self, x: i8, y: i8) -> Self {
        self.offset_x = x;
        self.offset_y = y;
        self
    }

    pub fn with_category(mut self, category: GlyphCategory) -> Self {
        self.category = category;
        self
    }

    pub fn is_wide(&self) -> bool {
        self.width > 1
    }

    pub fn is_emoji(&self) -> bool {
        self.category == GlyphCategory::Emoji
    }

    pub fn is_nerd_font(&self) -> bool {
        self.category == GlyphCategory::NerdFont
    }

    pub fn is_box_drawing(&self) -> bool {
        self.category == GlyphCategory::BoxDrawing
    }

    pub fn is_braille(&self) -> bool {
        self.category == GlyphCategory::Braille
    }

    pub fn is_cjk(&self) -> bool {
        self.category == GlyphCategory::Cjk
    }
}

impl GlyphCategory {
    pub fn from_char(ch: char) -> Self {
        let cp = ch as u32;

        if cp <= 0x7F {
            return Self::Ascii;
        }

        if cp <= 0xFF {
            return Self::AsciiExtended;
        }

        if is_emoji(cp) {
            return Self::Emoji;
        }

        if is_box_drawing(cp) {
            return Self::BoxDrawing;
        }

        if is_braille(cp) {
            return Self::Braille;
        }

        if is_cjk(cp) {
            return Self::Cjk;
        }

        if is_diacritical(cp) {
            return Self::Diacritical;
        }

        if is_nerd_font(cp) {
            return Self::NerdFont;
        }

        if is_symbol(cp) {
            return Self::Symbol;
        }

        Self::Unicode
    }

    pub fn width_hint(&self) -> u8 {
        match self {
            Self::Cjk => 2,
            Self::Emoji => 2,
            _ => 1,
        }
    }
}

fn is_emoji(cp: u32) -> bool {
    matches!(cp,
        0x1F600..=0x1F64F |
        0x1F300..=0x1F5FF |
        0x1F680..=0x1F6FF |
        0x1F900..=0x1F9FF |
        0x2700..=0x27BF |
        0xFE00..=0xFE0F |
        0x1F1E0..=0x1F1FF |
        0x1FA00..=0x1FA6F |
        0x1FA70..=0x1FAFF
    )
}

fn is_box_drawing(cp: u32) -> bool {
    matches!(cp, 0x2500..=0x25FF)
}

fn is_braille(cp: u32) -> bool {
    matches!(cp, 0x2800..=0x28FF)
}

fn is_cjk(cp: u32) -> bool {
    matches!(cp,
        0x4E00..=0x9FFF |
        0x3400..=0x4DBF |
        0x20000..=0x2A6DF |
        0x2A700..=0x2B73F |
        0x2B740..=0x2B81F |
        0x2B820..=0x2CEAF |
        0xF900..=0xFAFF |
        0x2F800..=0x2FA1F
    )
}

fn is_diacritical(cp: u32) -> bool {
    matches!(cp, 0x0300..=0x036F | 0x1AB0..=0x1AFF | 0x1DC0..=0x1DFF | 0x20D0..=0x20FF | 0xFE20..=0xFE2F)
}

fn is_nerd_font(cp: u32) -> bool {
    matches!(cp,
        0xE000..=0xE00A |
        0xE0A0..=0xE0A3 |
        0xE0B0..=0xE0C8 |
        0xE0D0..=0xE0D4 |
        0xE200..=0xE2A9 |
        0xE300..=0xE3E3 |
        0xE500..=0xE5E5 |
        0xE700..=0xEC45 |
        0xED00..=0xEF99 |
        0xF000..=0xF00F |
        0xF100..=0xF299 |
        0xF400..=0xF8FF
    )
}

fn is_symbol(cp: u32) -> bool {
    matches!(cp, 0x2000..=0x2BFF)
}

impl From<char> for Glyph {
    fn from(ch: char) -> Self {
        Self::new(ch)
    }
}

impl From<GlyphCategory> for &'static str {
    fn from(cat: GlyphCategory) -> Self {
        match cat {
            GlyphCategory::Ascii => "ascii",
            GlyphCategory::AsciiExtended => "ascii_extended",
            GlyphCategory::Unicode => "unicode",
            GlyphCategory::Emoji => "emoji",
            GlyphCategory::NerdFont => "nerd_font",
            GlyphCategory::BoxDrawing => "box_drawing",
            GlyphCategory::Braille => "braille",
            GlyphCategory::Cjk => "cjk",
            GlyphCategory::Diacritical => "diacritical",
            GlyphCategory::Symbol => "symbol",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glyph_new() {
        let g = Glyph::new('A');
        assert_eq!(g.ch, 'A');
        assert_eq!(g.width, 1);
        assert_eq!(g.category, GlyphCategory::Ascii);
    }

    #[test]
    fn glyph_wide() {
        let g = Glyph::new('中').with_width(2);
        assert!(g.is_wide());
    }

    #[test]
    fn glyph_category_ascii() {
        assert_eq!(GlyphCategory::from_char('A'), GlyphCategory::Ascii);
        assert_eq!(GlyphCategory::from_char('z'), GlyphCategory::Ascii);
        assert_eq!(GlyphCategory::from_char('0'), GlyphCategory::Ascii);
    }

    #[test]
    fn glyph_category_emoji() {
        assert_eq!(GlyphCategory::from_char('😀'), GlyphCategory::Emoji);
        assert_eq!(GlyphCategory::from_char('🎉'), GlyphCategory::Emoji);
    }

    #[test]
    fn glyph_category_cjk() {
        assert_eq!(GlyphCategory::from_char('中'), GlyphCategory::Cjk);
        assert_eq!(GlyphCategory::from_char('日'), GlyphCategory::Cjk);
    }

    #[test]
    fn glyph_category_box_drawing() {
        assert_eq!(GlyphCategory::from_char('─'), GlyphCategory::BoxDrawing);
        assert_eq!(GlyphCategory::from_char('│'), GlyphCategory::BoxDrawing);
    }

    #[test]
    fn glyph_category_braille() {
        assert_eq!(GlyphCategory::from_char('⠁'), GlyphCategory::Braille);
        assert_eq!(GlyphCategory::from_char('⣿'), GlyphCategory::Braille);
    }

    #[test]
    fn glyph_category_symbol() {
        assert_eq!(GlyphCategory::from_char('★'), GlyphCategory::Symbol);
        assert_eq!(GlyphCategory::from_char('→'), GlyphCategory::Symbol);
    }

    #[test]
    fn glyph_from_char() {
        let g: Glyph = 'X'.into();
        assert_eq!(g.ch, 'X');
    }

    #[test]
    fn glyph_width_hint() {
        assert_eq!(GlyphCategory::Cjk.width_hint(), 2);
        assert_eq!(GlyphCategory::Emoji.width_hint(), 2);
        assert_eq!(GlyphCategory::Ascii.width_hint(), 1);
    }
}
