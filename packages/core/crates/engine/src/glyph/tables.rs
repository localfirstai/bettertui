use super::character::{Glyph, GlyphCategory};

#[derive(Debug)]
pub struct GlyphTables {
    ascii: Vec<Glyph>,
    box_drawing: Vec<Glyph>,
    braille: Vec<Glyph>,
    common_emoji: Vec<Glyph>,
}

impl GlyphTables {
    pub fn new() -> Self {
        let mut ascii = Vec::with_capacity(128);
        for i in 0..128u8 {
            ascii.push(Glyph::new(i as char).with_category(GlyphCategory::Ascii));
        }

        let mut box_drawing = Vec::with_capacity(128);
        for cp in 0x2500..=0x257F {
            if let Some(ch) = char::from_u32(cp) {
                box_drawing.push(Glyph::new(ch).with_category(GlyphCategory::BoxDrawing));
            }
        }

        let mut braille = Vec::with_capacity(256);
        for cp in 0x2800..=0x28FF {
            if let Some(ch) = char::from_u32(cp) {
                braille.push(Glyph::new(ch).with_category(GlyphCategory::Braille));
            }
        }

        let common_emoji = vec![
            Glyph::new('😀').with_category(GlyphCategory::Emoji),
            Glyph::new('😂').with_category(GlyphCategory::Emoji),
            Glyph::new('🤔').with_category(GlyphCategory::Emoji),
            Glyph::new('👍').with_category(GlyphCategory::Emoji),
            Glyph::new('👎').with_category(GlyphCategory::Emoji),
            Glyph::new('❤').with_category(GlyphCategory::Emoji),
            Glyph::new('🎉').with_category(GlyphCategory::Emoji),
            Glyph::new('🔥').with_category(GlyphCategory::Emoji),
        ];

        Self {
            ascii,
            box_drawing,
            braille,
            common_emoji,
        }
    }

    pub fn ascii_glyph(&self, ch: char) -> Option<&Glyph> {
        let idx = ch as usize;
        if idx < self.ascii.len() {
            Some(&self.ascii[idx])
        } else {
            None
        }
    }

    pub fn box_drawing_glyph(&self, ch: char) -> Option<&Glyph> {
        let cp = ch as u32;
        if (0x2500..=0x257F).contains(&cp) {
            Some(&self.box_drawing[(cp - 0x2500) as usize])
        } else {
            None
        }
    }

    pub fn braille_glyph(&self, ch: char) -> Option<&Glyph> {
        let cp = ch as u32;
        if (0x2800..=0x28FF).contains(&cp) {
            Some(&self.braille[(cp - 0x2800) as usize])
        } else {
            None
        }
    }

    pub fn get_glyph(&self, ch: char) -> Option<&Glyph> {
        let category = GlyphCategory::from_char(ch);
        match category {
            GlyphCategory::Ascii => self.ascii_glyph(ch),
            GlyphCategory::BoxDrawing => self.box_drawing_glyph(ch),
            GlyphCategory::Braille => self.braille_glyph(ch),
            _ => None,
        }
    }

    pub fn common_emoji(&self) -> &[Glyph] {
        &self.common_emoji
    }
}

impl Default for GlyphTables {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tables_new() {
        let tables = GlyphTables::new();
        assert_eq!(tables.ascii.len(), 128);
        assert_eq!(tables.box_drawing.len(), 128);
        assert_eq!(tables.braille.len(), 256);
    }

    #[test]
    fn ascii_glyph() {
        let tables = GlyphTables::new();
        let g = tables.ascii_glyph('A').unwrap();
        assert_eq!(g.ch, 'A');
        assert_eq!(g.category, GlyphCategory::Ascii);
    }

    #[test]
    fn box_drawing_glyph() {
        let tables = GlyphTables::new();
        let g = tables.box_drawing_glyph('─').unwrap();
        assert_eq!(g.ch, '─');
        assert_eq!(g.category, GlyphCategory::BoxDrawing);
    }

    #[test]
    fn braille_glyph() {
        let tables = GlyphTables::new();
        let g = tables.braille_glyph('⠁').unwrap();
        assert_eq!(g.ch, '⠁');
        assert_eq!(g.category, GlyphCategory::Braille);
    }

    #[test]
    fn get_glyph_dispatches() {
        let tables = GlyphTables::new();
        assert!(tables.get_glyph('A').is_some());
        assert!(tables.get_glyph('─').is_some());
        assert!(tables.get_glyph('⠁').is_some());
    }
}
