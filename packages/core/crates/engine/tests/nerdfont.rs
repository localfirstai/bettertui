use bettertui_engine::nerdfont::{
    ErrorCode, GlyphCategory, GlyphMetrics, LocalFont, LocalFontDetector, MetricsCache, NerdFont,
    NerdFontDetector, NerdFontGlyph, NerdFontVariant, WarningCode,
};
use std::path::PathBuf;

#[test]
fn font_new() {
    let font = NerdFont::new("TestFont");
    assert_eq!(font.name, "TestFont");
    assert!(font.is_monospace);
}

#[test]
fn font_builder() {
    let font = NerdFont::new("TestFont")
        .with_family("TestFamily")
        .with_variant(NerdFontVariant::Mono)
        .with_monospace(false);

    assert_eq!(font.family, "TestFamily");
    assert_eq!(font.variant, NerdFontVariant::Mono);
    assert!(!font.is_monospace);
}

#[test]
fn font_glyph_count() {
    let glyph = NerdFontGlyph::new(0xE000, "test", GlyphCategory::Powerline);
    let font = NerdFont::new("TestFont").with_glyphs(vec![glyph]);
    assert_eq!(font.glyph_count(), 1);
}

#[test]
fn font_has_glyph() {
    let glyph = NerdFontGlyph::new(0xE000, "test", GlyphCategory::Powerline);
    let font = NerdFont::new("TestFont").with_glyphs(vec![glyph]);

    assert!(font.has_glyph(0xE000));
    assert!(!font.has_glyph(0xE001));
}

#[test]
fn font_get_glyph() {
    let glyph = NerdFontGlyph::new(0xE000, "test", GlyphCategory::Powerline);
    let font = NerdFont::new("TestFont").with_glyphs(vec![glyph]);

    assert!(font.get_glyph(0xE000).is_some());
    assert!(font.get_glyph(0xE001).is_none());
}

#[test]
fn font_glyphs_by_category() {
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
fn glyph_new() {
    let glyph = NerdFontGlyph::new(0xE000, "test", GlyphCategory::Powerline);
    assert_eq!(glyph.codepoint, 0xE000);
    assert_eq!(glyph.width, 1);
}

#[test]
fn glyph_builder() {
    let glyph = NerdFontGlyph::new(0xE000, "test", GlyphCategory::Powerline).with_width(2);
    assert_eq!(glyph.width, 2);
    assert!(glyph.is_wide());
}

#[test]
fn validate_empty_font() {
    let font = NerdFont::new("TestFont");
    let result = font.validate();
    assert!(result.is_valid());
    assert_eq!(result.glyph_count, 0);
}

#[test]
fn validate_valid_glyphs() {
    let glyphs = vec![
        NerdFontGlyph::new(0xE0A0, "branch", GlyphCategory::Powerline),
        NerdFontGlyph::new(0xE0B0, "right-triangle", GlyphCategory::Powerline),
    ];
    let font = NerdFont::new("TestFont").with_glyphs(glyphs);

    let result = font.validate();
    assert!(result.is_valid());
    assert_eq!(result.valid_glyphs, 2);
}

#[test]
fn validate_duplicate_codepoint() {
    let glyphs = vec![
        NerdFontGlyph::new(0xE0A0, "branch1", GlyphCategory::Powerline),
        NerdFontGlyph::new(0xE0A0, "branch2", GlyphCategory::Powerline),
    ];
    let font = NerdFont::new("TestFont").with_glyphs(glyphs);

    let result = font.validate();
    assert!(!result.is_valid());
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code == ErrorCode::DuplicateCodepoint)
    );
}

#[test]
fn validate_invalid_codepoint() {
    let glyphs = vec![NerdFontGlyph::new(
        0xD800,
        "surrogate",
        GlyphCategory::Powerline,
    )];
    let font = NerdFont::new("TestFont").with_glyphs(glyphs);

    let result = font.validate();
    assert!(!result.is_valid());
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code == ErrorCode::InvalidCodepoint)
    );
}

#[test]
fn validate_zero_width() {
    let glyphs = vec![NerdFontGlyph::new(0xE0A0, "branch", GlyphCategory::Powerline).with_width(0)];
    let font = NerdFont::new("TestFont").with_glyphs(glyphs);

    let result = font.validate();
    assert!(!result.is_valid());
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code == ErrorCode::InvalidWidth)
    );
}

#[test]
fn validate_missing_name_warning() {
    let glyphs = vec![NerdFontGlyph::new(0xE0A0, "", GlyphCategory::Powerline)];
    let font = NerdFont::new("TestFont").with_glyphs(glyphs);

    let result = font.validate();
    assert!(result.is_valid());
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.code == WarningCode::MissingName)
    );
}

#[test]
fn validate_coverage_percentage() {
    let glyphs = vec![
        NerdFontGlyph::new(0xE0A0, "branch", GlyphCategory::Powerline),
        NerdFontGlyph::new(0xE0B0, "right-triangle", GlyphCategory::Powerline),
    ];
    let font = NerdFont::new("TestFont").with_glyphs(glyphs);

    let result = font.validate();
    assert_eq!(result.coverage_percentage(), 100.0);
}

#[test]
fn local_bundled_font_exists() {
    let font = LocalFont::bundled();
    assert!(font.exists());
    assert!(font.is_bundled);
}

#[test]
fn local_bundled_font_load() {
    let font = LocalFont::bundled();
    let data = font.load_bytes().unwrap();
    assert!(data.len() > 1000);
    assert_eq!(&data[0..4], b"OTTO");
}

#[test]
fn local_font_new() {
    let font = LocalFont::new(PathBuf::from("/tmp/TestFont.otf"));
    assert_eq!(font.name, "TestFont");
    assert!(!font.is_bundled);
}

#[test]
fn local_detector_new() {
    let detector = LocalFontDetector::new();
    assert!(detector.has_bundled_font());
    assert!(detector.any_font_available());
}

#[test]
fn local_detector_best_font() {
    let detector = LocalFontDetector::new();
    let best = detector.best_font();
    assert!(best.exists());
}

#[test]
fn local_detector_find_font() {
    let detector = LocalFontDetector::new();
    assert!(detector.find_font("DroidSans").is_some());
    assert!(detector.find_font("NonExistent").is_none());
}

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

#[test]
fn detector_new() {
    let detector = NerdFontDetector::new();
    assert!(detector.available_fonts().is_empty());
    assert!(detector.local_detector().has_bundled_font());
}

#[test]
fn detector_has_local_font() {
    let detector = NerdFontDetector::new();
    assert!(detector.has_local_font("DroidSans"));
    assert!(!detector.has_local_font("NonExistentFont"));
}
