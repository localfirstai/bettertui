use crate::font::loader::{BundledFont, FontMetadata};
use crate::font::metrics::FontMetricsCache;
use crate::font::registry::{IconCategory, IconGlyph, IconRegistry};

pub struct FontProvider {
    registry: IconRegistry,
    bundled: BundledFont,
    metrics: FontMetricsCache,
}

impl Default for FontProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl FontProvider {
    pub fn new() -> Self {
        Self {
            registry: IconRegistry::with_builtin(),
            bundled: BundledFont::new(),
            metrics: FontMetricsCache::new(8, 16),
        }
    }

    pub fn with_cell_size(mut self, width: u16, height: u16) -> Self {
        self.metrics.set_cell_size(width, height);
        self
    }

    pub fn registry(&self) -> &IconRegistry {
        &self.registry
    }

    pub fn bundled_font(&self) -> &BundledFont {
        &self.bundled
    }

    pub fn metrics(&self) -> &FontMetricsCache {
        &self.metrics
    }

    pub fn metrics_mut(&mut self) -> &mut FontMetricsCache {
        &mut self.metrics
    }

    pub fn metadata(&self) -> &FontMetadata {
        self.bundled.metadata()
    }

    pub fn font_bytes(&self) -> &'static [u8] {
        self.bundled.bytes()
    }

    pub fn lookup_name(&self, name: &str) -> Option<&IconGlyph> {
        self.registry.lookup_name(name)
    }

    pub fn lookup_codepoint(&self, codepoint: u32) -> Option<&IconGlyph> {
        self.registry.lookup_codepoint(codepoint)
    }

    pub fn resolve_icon(&self, name: &str) -> Option<char> {
        self.registry.resolve_char(name)
    }

    pub fn icons_by_category(&self, category: IconCategory) -> Vec<&IconGlyph> {
        self.registry.icons_by_category(category)
    }

    pub fn total_icons(&self) -> usize {
        self.registry.total_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_new() {
        let provider = FontProvider::new();
        assert!(provider.total_icons() > 1000);
        assert!(provider.bundled_font().exists());
    }

    #[test]
    fn provider_lookup_icon() {
        let provider = FontProvider::new();
        let glyph = provider.lookup_name("dev-rust");
        assert!(glyph.is_some());
        assert_eq!(glyph.unwrap().codepoint, 0xE7A8);
    }

    #[test]
    fn provider_lookup_codepoint() {
        let provider = FontProvider::new();
        let glyph = provider.lookup_codepoint(0xE7A8);
        assert!(glyph.is_some());
        assert_eq!(glyph.unwrap().name, "dev-rust");
    }

    #[test]
    fn provider_resolve_icon() {
        let provider = FontProvider::new();
        let ch = provider.resolve_icon("dev-rust");
        assert!(ch.is_some());
        assert_eq!(ch.unwrap() as u32, 0xE7A8);
    }

    #[test]
    fn provider_font_bytes() {
        let provider = FontProvider::new();
        let bytes = provider.font_bytes();
        assert!(bytes.len() > 1000);
        assert_eq!(&bytes[..4], b"OTTO");
    }

    #[test]
    fn provider_metadata() {
        let provider = FontProvider::new();
        let meta = provider.metadata();
        assert!(meta.name.contains("NerdFont"));
        assert!(meta.is_monospace);
    }

    #[test]
    fn provider_with_cell_size() {
        let provider = FontProvider::new().with_cell_size(10, 20);
        assert_eq!(provider.metrics().cell_width(), 10);
        assert_eq!(provider.metrics().cell_height(), 20);
    }

    #[test]
    fn provider_icons_by_category() {
        let provider = FontProvider::new();
        let devicons = provider.icons_by_category(IconCategory::Dev);
        assert!(devicons.len() > 100);
    }

    #[test]
    fn provider_lookup_nf_prefixed() {
        let provider = FontProvider::new();
        let glyph = provider.lookup_name("nf-dev-rust");
        assert!(glyph.is_some());
    }
}
