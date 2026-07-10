use super::font::{NerdFont, NerdFontGlyph};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GlyphMetrics {
    pub width: u16,
    pub height: u16,
    pub bearing_x: i16,
    pub bearing_y: i16,
    pub advance_x: u16,
    pub advance_y: u16,
    pub is_monospace: bool,
}

impl Default for GlyphMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl GlyphMetrics {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            bearing_x: 0,
            bearing_y: 0,
            advance_x: 0,
            advance_y: 0,
            is_monospace: true,
        }
    }

    pub fn with_dimensions(mut self, width: u16, height: u16) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_bearing(mut self, x: i16, y: i16) -> Self {
        self.bearing_x = x;
        self.bearing_y = y;
        self
    }

    pub fn with_advance(mut self, x: u16, y: u16) -> Self {
        self.advance_x = x;
        self.advance_y = y;
        self
    }

    pub fn with_monospace(mut self, is_monospace: bool) -> Self {
        self.is_monospace = is_monospace;
        self
    }

    pub fn cell_width(&self, cell_width: u16) -> u16 {
        if self.is_monospace {
            cell_width
        } else {
            self.advance_x
        }
    }

    pub fn is_wide(&self) -> bool {
        self.advance_x > 1
    }
}

#[derive(Debug, Clone, Default)]
pub struct MetricsCache {
    metrics: HashMap<u32, GlyphMetrics>,
    cell_width: u16,
    cell_height: u16,
}

impl MetricsCache {
    pub fn new(cell_width: u16, cell_height: u16) -> Self {
        Self {
            metrics: HashMap::new(),
            cell_width,
            cell_height,
        }
    }

    pub fn get(&self, codepoint: u32) -> Option<&GlyphMetrics> {
        self.metrics.get(&codepoint)
    }

    pub fn get_or_create(&mut self, codepoint: u32, glyph: &NerdFontGlyph) -> &GlyphMetrics {
        if !self.metrics.contains_key(&codepoint) {
            let metrics = self.measure_glyph(glyph);
            self.metrics.insert(codepoint, metrics);
        }
        self.metrics.get(&codepoint).unwrap()
    }

    pub fn insert(&mut self, codepoint: u32, metrics: GlyphMetrics) {
        self.metrics.insert(codepoint, metrics);
    }

    pub fn contains(&self, codepoint: u32) -> bool {
        self.metrics.contains_key(&codepoint)
    }

    pub fn len(&self) -> usize {
        self.metrics.len()
    }

    pub fn is_empty(&self) -> bool {
        self.metrics.is_empty()
    }

    pub fn clear(&mut self) {
        self.metrics.clear();
    }

    pub fn cell_width(&self) -> u16 {
        self.cell_width
    }

    pub fn cell_height(&self) -> u16 {
        self.cell_height
    }

    pub fn set_cell_size(&mut self, width: u16, height: u16) {
        self.cell_width = width;
        self.cell_height = height;
    }

    fn measure_glyph(&self, glyph: &NerdFontGlyph) -> GlyphMetrics {
        let width = glyph.width as u16 * self.cell_width;
        let height = self.cell_height;

        GlyphMetrics::new()
            .with_dimensions(width, height)
            .with_advance(glyph.width as u16, 1)
            .with_monospace(true)
    }

    pub fn measure_all(&mut self, font: &NerdFont) {
        for glyph in &font.glyphs {
            let metrics = self.measure_glyph(glyph);
            self.metrics.insert(glyph.codepoint, metrics);
        }
    }

    pub fn total_memory(&self) -> usize {
        self.metrics.len() * std::mem::size_of::<GlyphMetrics>()
    }
}

#[cfg(test)]
mod tests {
    use super::super::font::{GlyphCategory, NerdFontGlyph};
    use super::*;

    #[test]
    fn metrics_new() {
        let metrics = GlyphMetrics::new();
        assert_eq!(metrics.width, 0);
        assert_eq!(metrics.height, 0);
    }

    #[test]
    fn metrics_builder() {
        let metrics = GlyphMetrics::new()
            .with_dimensions(10, 20)
            .with_bearing(1, 2)
            .with_advance(10, 0)
            .with_monospace(true);

        assert_eq!(metrics.width, 10);
        assert_eq!(metrics.height, 20);
        assert_eq!(metrics.bearing_x, 1);
        assert_eq!(metrics.advance_x, 10);
        assert!(metrics.is_monospace);
    }

    #[test]
    fn metrics_cell_width() {
        let metrics = GlyphMetrics::new().with_advance(1, 0).with_monospace(true);

        assert_eq!(metrics.cell_width(8), 8);
    }

    #[test]
    fn metrics_is_wide() {
        let metrics = GlyphMetrics::new().with_advance(2, 0);
        assert!(metrics.is_wide());
    }

    #[test]
    fn cache_new() {
        let cache = MetricsCache::new(8, 16);
        assert_eq!(cache.cell_width(), 8);
        assert_eq!(cache.cell_height(), 16);
        assert!(cache.is_empty());
    }

    #[test]
    fn cache_get_or_create() {
        let mut cache = MetricsCache::new(8, 16);
        let glyph = NerdFontGlyph::new(0xE0A0, "branch", GlyphCategory::Powerline);

        let metrics = cache.get_or_create(0xE0A0, &glyph);
        assert_eq!(metrics.width, 8);
        assert!(cache.contains(0xE0A0));
    }

    #[test]
    fn cache_measure_all() {
        let mut cache = MetricsCache::new(8, 16);
        let glyphs = vec![
            NerdFontGlyph::new(0xE0A0, "branch", GlyphCategory::Powerline),
            NerdFontGlyph::new(0xE0B0, "right-triangle", GlyphCategory::Powerline),
        ];
        let font = NerdFont::new("TestFont").with_glyphs(glyphs);

        cache.measure_all(&font);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn cache_total_memory() {
        let mut cache = MetricsCache::new(8, 16);
        let glyph = NerdFontGlyph::new(0xE0A0, "branch", GlyphCategory::Powerline);
        cache.get_or_create(0xE0A0, &glyph);

        assert!(cache.total_memory() > 0);
    }
}
