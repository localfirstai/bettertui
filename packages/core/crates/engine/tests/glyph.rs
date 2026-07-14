//! Tests for the glyph module (character classification, cache, metrics, tables).

use bettertui_engine::glyph::{
    Glyph, GlyphCache, GlyphCategory, GlyphId, GlyphMetrics, GlyphTables, MetricsCache,
};

// ---------------------------------------------------------------------------
// Character / Glyph tests
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Metrics tests
// ---------------------------------------------------------------------------

#[test]
fn metrics_new() {
    let m = GlyphMetrics::new(GlyphId(65));
    assert_eq!(m.glyph_id, GlyphId(65));
    assert_eq!(m.width, 0);
    assert_eq!(m.access_count, 0);
}

#[test]
fn metrics_with_dimensions() {
    let m = GlyphMetrics::new(GlyphId(65))
        .with_dimensions(8, 16)
        .with_bearing(0, 12)
        .with_advance(8, 0)
        .with_bitmap(0, 128);
    assert_eq!(m.width, 8);
    assert_eq!(m.height, 16);
    assert_eq!(m.bitmap_size, 128);
}

#[test]
fn metrics_touch() {
    let mut m = GlyphMetrics::new(GlyphId(65));
    assert_eq!(m.access_count, 0);
    m.touch();
    assert_eq!(m.access_count, 1);
    m.touch();
    assert_eq!(m.access_count, 2);
}

#[test]
fn metrics_bytes_used() {
    let m = GlyphMetrics::new(GlyphId(65)).with_bitmap(0, 128);
    assert!(m.bytes_used() >= 128);
}

#[test]
fn metrics_cache_insert_and_get() {
    let mut cache = MetricsCache::new(1024 * 1024);
    let m = GlyphMetrics::new(GlyphId(65)).with_dimensions(8, 16);
    cache.insert(m);

    assert!(cache.contains(&GlyphId(65)));
    assert_eq!(cache.len(), 1);
}

#[test]
fn metrics_cache_remove() {
    let mut cache = MetricsCache::new(1024 * 1024);
    let m = GlyphMetrics::new(GlyphId(65));
    cache.insert(m);

    assert!(cache.remove(&GlyphId(65)).is_some());
    assert!(!cache.contains(&GlyphId(65)));
    assert_eq!(cache.len(), 0);
}

#[test]
fn metrics_cache_evict_lru() {
    let mut cache = MetricsCache::new(1024);
    for i in 0..100 {
        let m = GlyphMetrics::new(GlyphId(i)).with_bitmap(0, 16);
        cache.insert(m);
    }

    let evicted = cache.evict_lru(512);
    assert!(evicted > 0);
    assert!(cache.total_bytes() <= 512);
}

#[test]
fn metrics_cache_stats() {
    let mut cache = MetricsCache::new(1024);
    let m = GlyphMetrics::new(GlyphId(65)).with_bitmap(0, 128);
    cache.insert(m);

    let stats = cache.stats();
    assert_eq!(stats.total_glyphs, 1);
    assert!(stats.total_bytes >= 128);
}

#[test]
fn metrics_cache_clear() {
    let mut cache = MetricsCache::new(1024);
    for i in 0..10 {
        let m = GlyphMetrics::new(GlyphId(i));
        cache.insert(m);
    }

    cache.clear();
    assert_eq!(cache.len(), 0);
    assert_eq!(cache.total_bytes(), 0);
}

// ---------------------------------------------------------------------------
// Tables tests
// ---------------------------------------------------------------------------

#[test]
fn tables_new() {
    let tables = GlyphTables::new();
    assert_eq!(tables.ascii_glyph('A').map(|g| g.ch), Some('A'));
    assert!(tables.box_drawing_glyph('─').is_some());
    assert!(tables.braille_glyph('⠁').is_some());
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

// ---------------------------------------------------------------------------
// GlyphCache tests
// ---------------------------------------------------------------------------

#[test]
fn glyph_cache_new() {
    let cache = GlyphCache::new();
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
}

#[test]
fn glyph_cache_insert_and_get() {
    let mut cache = GlyphCache::new();
    let glyph = Glyph::new('A');
    cache.insert(glyph);

    let g = cache.get('A');
    assert!(g.is_some());
    assert_eq!(g.unwrap().ch, 'A');
}

#[test]
fn glyph_cache_get_or_insert() {
    let mut cache = GlyphCache::new();
    let g = cache.get_or_insert('X');
    assert_eq!(g.ch, 'X');
    assert!(cache.contains('X'));
}

#[test]
fn glyph_cache_stats() {
    let mut cache = GlyphCache::new();
    cache.get_or_insert('B');
    cache.get_or_insert('B');
    cache.get('A');

    assert_eq!(cache.stats().lookups, 3);
    assert_eq!(cache.stats().misses, 2);
    assert_eq!(cache.stats().hits, 1);
}

#[test]
fn glyph_cache_evict() {
    let mut cache = GlyphCache::with_config(100, 1024 * 1024);
    for i in 0..200u8 {
        cache.insert(Glyph::new(i as char));
    }

    cache.evict(100);
    assert!(cache.len() <= 100);
}

#[test]
fn glyph_cache_preload_ascii() {
    let mut cache = GlyphCache::new();
    cache.preload_ascii();
    assert_eq!(cache.len(), 128);
}

#[test]
fn glyph_cache_preload_box_drawing() {
    let mut cache = GlyphCache::new();
    cache.preload_box_drawing();
    assert_eq!(cache.len(), 128);
}

#[test]
fn glyph_cache_preload_braille() {
    let mut cache = GlyphCache::new();
    cache.preload_braille();
    assert_eq!(cache.len(), 256);
}

#[test]
fn glyph_cache_memory_usage() {
    let mut cache = GlyphCache::new();
    cache.preload_ascii();

    let usage = cache.memory_usage();
    assert!(usage > 0);
}

#[test]
fn glyph_cache_category_stats() {
    let mut cache = GlyphCache::new();
    cache.preload_ascii();
    cache.preload_box_drawing();

    let stats = cache.category_stats();
    assert!(*stats.get(&GlyphCategory::Ascii).unwrap() > 0);
    assert!(*stats.get(&GlyphCategory::BoxDrawing).unwrap() > 0);
}

#[test]
fn glyph_cache_clear() {
    let mut cache = GlyphCache::new();
    cache.preload_ascii();
    cache.clear();

    assert_eq!(cache.len(), 0);
    assert_eq!(cache.stats().hits, 0);
}
