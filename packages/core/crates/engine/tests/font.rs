use bettertui_engine::font::{
    BundledFont, FontMetrics, FontMetricsCache, FontProvider, IconCategory, IconGlyph, IconRegistry,
};

#[test]
fn bundled_font_exists() {
    assert!(BundledFont::new().exists());
}

#[test]
fn bundled_font_bytes() {
    let font = BundledFont::new();
    let data = font.bytes();
    assert!(data.len() > 1000);
    assert_eq!(&data[..4], b"OTTO");
}

#[test]
fn bundled_font_metadata() {
    let font = BundledFont::new();
    let meta = font.metadata();
    assert!(meta.name.contains("NerdFont"));
    assert!(meta.is_monospace);
}

#[test]
fn metrics_new() {
    let m = FontMetrics::new();
    assert_eq!(m.width, 0);
    assert_eq!(m.height, 0);
}

#[test]
fn metrics_builder() {
    let m = FontMetrics::new()
        .with_dimensions(10, 20)
        .with_bearing(1, 2)
        .with_advance(10, 0)
        .with_monospace(true);

    assert_eq!(m.width, 10);
    assert_eq!(m.height, 20);
    assert_eq!(m.bearing_x, 1);
    assert_eq!(m.advance_x, 10);
    assert!(m.is_monospace);
}

#[test]
fn metrics_cell_width() {
    let m = FontMetrics::new().with_advance(1, 0).with_monospace(true);
    assert_eq!(m.cell_width(8), 8);
}

#[test]
fn metrics_is_wide() {
    let m = FontMetrics::new().with_advance(2, 0);
    assert!(m.is_wide());
}

#[test]
fn cache_new() {
    let cache = FontMetricsCache::new(8, 16);
    assert_eq!(cache.cell_width(), 8);
    assert_eq!(cache.cell_height(), 16);
    assert!(cache.is_empty());
}

#[test]
fn cache_insert_and_get() {
    let mut cache = FontMetricsCache::new(8, 16);
    let m = FontMetrics::new().with_dimensions(8, 16);
    cache.insert(0xE0A0, m);
    assert!(cache.contains(0xE0A0));
    assert!(cache.get(0xE0A0).is_some());
    assert_eq!(cache.get(0xE0A0).unwrap().width, 8);
}

#[test]
fn cache_clear() {
    let mut cache = FontMetricsCache::new(8, 16);
    cache.insert(0xE0A0, FontMetrics::new());
    assert!(!cache.is_empty());
    cache.clear();
    assert!(cache.is_empty());
}

#[test]
fn cache_preload() {
    let mut cache = FontMetricsCache::new(8, 16);
    cache.preload_standard();
    assert!(cache.contains(0xE700));
    assert!(cache.contains(0xF400));
}

#[test]
fn cache_total_memory() {
    let mut cache = FontMetricsCache::new(8, 16);
    cache.insert(0xE0A0, FontMetrics::new());
    assert!(cache.total_memory() > 0);
}

#[test]
fn registry_new_empty() {
    let reg = IconRegistry::new();
    assert_eq!(reg.total_count(), 0);
}

#[test]
fn registry_with_builtin() {
    let reg = IconRegistry::with_builtin();
    assert!(reg.total_count() > 1000);
}

#[test]
fn registry_lookup_codepoint() {
    let reg = IconRegistry::with_builtin();
    assert!(reg.lookup_codepoint(0xE0A0).is_some());
}

#[test]
fn registry_lookup_name() {
    let reg = IconRegistry::with_builtin();
    let glyph = reg.lookup_name("dev-rust");
    assert!(glyph.is_some());
    assert_eq!(glyph.unwrap().codepoint, 0xE7A8);
}

#[test]
fn registry_lookup_nf_prefixed() {
    let reg = IconRegistry::with_builtin();
    assert!(reg.lookup_name("nf-dev-rust").is_some());
}

#[test]
fn registry_codepoint_for_name() {
    let reg = IconRegistry::with_builtin();
    assert_eq!(reg.codepoint_for_name("dev-rust"), Some(0xE7A8));
}

#[test]
fn registry_name_for_codepoint() {
    let reg = IconRegistry::with_builtin();
    assert_eq!(reg.name_for_codepoint(0xE0A0), Some("pl-branch"));
}

#[test]
fn registry_resolve_char() {
    let reg = IconRegistry::with_builtin();
    let ch = reg.resolve_char("dev-rust");
    assert!(ch.is_some());
    assert_eq!(ch.unwrap() as u32, 0xE7A8);
}

#[test]
fn registry_categories() {
    let reg = IconRegistry::with_builtin();
    let cats = reg.categories();
    assert!(cats.contains(&IconCategory::Dev));
    assert!(cats.contains(&IconCategory::Fa));
}

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
}

#[test]
fn provider_resolve_icon() {
    let provider = FontProvider::new();
    assert!(provider.resolve_icon("dev-rust").is_some());
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

#[test]
fn icon_glyph_new() {
    let glyph = IconGlyph::new(0xE0A0, "pl-branch", IconCategory::Pl);
    assert_eq!(glyph.codepoint, 0xE0A0);
    assert_eq!(glyph.width, 1);
}

#[test]
fn icon_glyph_to_char() {
    let glyph = IconGlyph::new(0xE0A0, "pl-branch", IconCategory::Pl);
    let ch = glyph.to_char();
    assert!(ch.is_some());
    assert_eq!(ch.unwrap() as u32, 0xE0A0);
}

#[test]
fn icon_glyph_is_powerline() {
    let pl = IconGlyph::new(0xE0A0, "pl-branch", IconCategory::Pl);
    let dev = IconGlyph::new(0xE700, "dev-rust", IconCategory::Dev);
    assert!(pl.is_powerline());
    assert!(!dev.is_powerline());
}

#[test]
fn icon_category_from_prefix() {
    assert_eq!(IconCategory::from_prefix("dev"), IconCategory::Dev);
    assert_eq!(IconCategory::from_prefix("fa"), IconCategory::Fa);
    assert_eq!(IconCategory::from_prefix("weather"), IconCategory::Weather);
    assert_eq!(IconCategory::from_prefix("unknown"), IconCategory::Unknown);
}

#[test]
fn icon_category_all() {
    let all = IconCategory::all();
    assert!(all.contains(&IconCategory::Dev));
    assert!(all.contains(&IconCategory::Fa));
    assert!(all.contains(&IconCategory::Md));
    assert_eq!(all.len(), 16);
}
