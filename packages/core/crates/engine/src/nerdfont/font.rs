#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NerdFont {
    pub name: String,
    pub family: String,
    pub variant: NerdFontVariant,
    pub glyphs: Vec<NerdFontGlyph>,
    pub is_monospace: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NerdFontVariant {
    #[default]
    Complete,
    Mono,
    Propo,
    SeparatedMono,
    SeparatedPropo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NerdFontGlyph {
    pub codepoint: u32,
    pub name: &'static str,
    pub category: GlyphCategory,
    pub width: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlyphCategory {
    Powerline,
    Devicons,
    FontLogos,
    Octicons,
    Material,
    Weather,
    Pomicons,
    Clock,
    Hashes,
    FileType,
    Indicators,
    PowerSymbols,
    Custom,
}

impl NerdFont {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            family: name.to_string(),
            variant: NerdFontVariant::Complete,
            glyphs: Vec::new(),
            is_monospace: true,
        }
    }

    pub fn with_family(mut self, family: &str) -> Self {
        self.family = family.to_string();
        self
    }

    pub fn with_variant(mut self, variant: NerdFontVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_glyphs(mut self, glyphs: Vec<NerdFontGlyph>) -> Self {
        self.glyphs = glyphs;
        self
    }

    pub fn with_monospace(mut self, is_monospace: bool) -> Self {
        self.is_monospace = is_monospace;
        self
    }

    pub fn glyph_count(&self) -> usize {
        self.glyphs.len()
    }

    pub fn has_glyph(&self, codepoint: u32) -> bool {
        self.glyphs.iter().any(|g| g.codepoint == codepoint)
    }

    pub fn get_glyph(&self, codepoint: u32) -> Option<&NerdFontGlyph> {
        self.glyphs.iter().find(|g| g.codepoint == codepoint)
    }

    pub fn glyphs_by_category(&self, category: GlyphCategory) -> Vec<&NerdFontGlyph> {
        self.glyphs
            .iter()
            .filter(|g| g.category == category)
            .collect()
    }

    pub fn categories(&self) -> Vec<GlyphCategory> {
        let mut categories: Vec<GlyphCategory> = self.glyphs.iter().map(|g| g.category).collect();
        categories.sort_by_key(|c| format!("{:?}", c));
        categories.dedup();
        categories
    }
}

impl NerdFontGlyph {
    pub fn new(codepoint: u32, name: &'static str, category: GlyphCategory) -> Self {
        Self {
            codepoint,
            name,
            category,
            width: 1,
        }
    }

    pub fn with_width(mut self, width: u8) -> Self {
        self.width = width;
        self
    }

    pub fn is_wide(&self) -> bool {
        self.width > 1
    }

    pub fn is_powerline(&self) -> bool {
        self.category == GlyphCategory::Powerline
    }

    pub fn is_devicon(&self) -> bool {
        self.category == GlyphCategory::Devicons
    }
}

impl std::fmt::Display for NerdFontVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NerdFontVariant::Complete => write!(f, "Complete"),
            NerdFontVariant::Mono => write!(f, "Mono"),
            NerdFontVariant::Propo => write!(f, "Propo"),
            NerdFontVariant::SeparatedMono => write!(f, "SeparatedMono"),
            NerdFontVariant::SeparatedPropo => write!(f, "SeparatedPropo"),
        }
    }
}

impl std::fmt::Display for GlyphCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GlyphCategory::Powerline => write!(f, "Powerline"),
            GlyphCategory::Devicons => write!(f, "Devicons"),
            GlyphCategory::FontLogos => write!(f, "FontLogos"),
            GlyphCategory::Octicons => write!(f, "Octicons"),
            GlyphCategory::Material => write!(f, "Material"),
            GlyphCategory::Weather => write!(f, "Weather"),
            GlyphCategory::Pomicons => write!(f, "Pomicons"),
            GlyphCategory::Clock => write!(f, "Clock"),
            GlyphCategory::Hashes => write!(f, "Hashes"),
            GlyphCategory::FileType => write!(f, "FileType"),
            GlyphCategory::Indicators => write!(f, "Indicators"),
            GlyphCategory::PowerSymbols => write!(f, "PowerSymbols"),
            GlyphCategory::Custom => write!(f, "Custom"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nerd_font_new() {
        let font = NerdFont::new("TestFont");
        assert_eq!(font.name, "TestFont");
        assert!(font.is_monospace);
    }

    #[test]
    fn nerd_font_builder() {
        let font = NerdFont::new("TestFont")
            .with_family("TestFamily")
            .with_variant(NerdFontVariant::Mono)
            .with_monospace(false);

        assert_eq!(font.family, "TestFamily");
        assert_eq!(font.variant, NerdFontVariant::Mono);
        assert!(!font.is_monospace);
    }

    #[test]
    fn nerd_font_glyph_count() {
        let glyph = NerdFontGlyph::new(0xE000, "test", GlyphCategory::Powerline);
        let font = NerdFont::new("TestFont").with_glyphs(vec![glyph]);
        assert_eq!(font.glyph_count(), 1);
    }

    #[test]
    fn nerd_font_has_glyph() {
        let glyph = NerdFontGlyph::new(0xE000, "test", GlyphCategory::Powerline);
        let font = NerdFont::new("TestFont").with_glyphs(vec![glyph]);

        assert!(font.has_glyph(0xE000));
        assert!(!font.has_glyph(0xE001));
    }

    #[test]
    fn nerd_font_get_glyph() {
        let glyph = NerdFontGlyph::new(0xE000, "test", GlyphCategory::Powerline);
        let font = NerdFont::new("TestFont").with_glyphs(vec![glyph]);

        assert!(font.get_glyph(0xE000).is_some());
        assert!(font.get_glyph(0xE001).is_none());
    }

    #[test]
    fn nerd_font_glyphs_by_category() {
        let glyphs = vec![
            NerdFontGlyph::new(0xE000, "test1", GlyphCategory::Powerline),
            NerdFontGlyph::new(0xE001, "test2", GlyphCategory::Devicons),
            NerdFontGlyph::new(0xE002, "test3", GlyphCategory::Powerline),
        ];
        let font = NerdFont::new("TestFont").with_glyphs(glyphs);

        let powerline = font.glyphs_by_category(GlyphCategory::Powerline);
        assert_eq!(powerline.len(), 2);
    }

    #[test]
    fn nerd_font_glyph_new() {
        let glyph = NerdFontGlyph::new(0xE000, "test", GlyphCategory::Powerline);
        assert_eq!(glyph.codepoint, 0xE000);
        assert_eq!(glyph.width, 1);
    }

    #[test]
    fn nerd_font_glyph_builder() {
        let glyph = NerdFontGlyph::new(0xE000, "test", GlyphCategory::Powerline).with_width(2);
        assert_eq!(glyph.width, 2);
        assert!(glyph.is_wide());
    }
}
