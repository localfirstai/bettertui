use super::character::{Glyph, GlyphCategory, GlyphId};
use super::metrics::{GlyphMetrics, MetricsCache};
use super::tables::GlyphTables;
use std::collections::HashMap;
use std::time::{Duration, Instant};

const DEFAULT_MAX_GLYPHS: usize = 10_000;
const DEFAULT_MAX_BYTES: usize = 16 * 1024 * 1024;
const DEFAULT_MAX_AGE: Duration = Duration::from_secs(60);

#[derive(Debug)]
pub struct GlyphCache {
    glyphs: HashMap<GlyphId, Glyph>,
    metrics: MetricsCache,
    tables: GlyphTables,
    max_glyphs: usize,
    max_bytes: usize,
    max_age: Duration,
    stats: CacheStats,
}

#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub insertions: u64,
    pub lookups: u64,
}

impl CacheStats {
    pub fn hit_rate(&self) -> f64 {
        if self.lookups == 0 {
            0.0
        } else {
            self.hits as f64 / self.lookups as f64
        }
    }
}

impl GlyphCache {
    pub fn new() -> Self {
        Self::with_config(DEFAULT_MAX_GLYPHS, DEFAULT_MAX_BYTES)
    }

    pub fn with_config(max_glyphs: usize, max_bytes: usize) -> Self {
        Self {
            glyphs: HashMap::with_capacity(max_glyphs),
            metrics: MetricsCache::new(max_bytes),
            tables: GlyphTables::new(),
            max_glyphs,
            max_bytes,
            max_age: DEFAULT_MAX_AGE,
            stats: CacheStats::default(),
        }
    }

    pub fn get(&mut self, ch: char) -> Option<&Glyph> {
        self.stats.lookups += 1;
        let id = GlyphId(ch as u32);

        if let Some(glyph) = self.glyphs.get(&id) {
            self.stats.hits += 1;
            self.metrics.get(&id);
            Some(glyph)
        } else {
            self.stats.misses += 1;
            None
        }
    }

    pub fn get_or_insert(&mut self, ch: char) -> &Glyph {
        self.stats.lookups += 1;
        let id = GlyphId(ch as u32);
        if !self.glyphs.contains_key(&id) {
            self.stats.misses += 1;
            let glyph = self.create_glyph(ch);
            self.insert(glyph);
        } else {
            self.stats.hits += 1;
        }
        self.glyphs.get(&id).unwrap()
    }

    pub fn get_metrics(&mut self, ch: char) -> Option<&GlyphMetrics> {
        let id = GlyphId(ch as u32);
        self.metrics.get(&id)
    }

    pub fn insert(&mut self, glyph: Glyph) {
        if self.glyphs.len() >= self.max_glyphs {
            self.evict(100);
        }

        let id = glyph.id;
        let metrics = GlyphMetrics::new(id)
            .with_dimensions(glyph.width as u16 * 8, glyph.height as u16 * 16)
            .with_advance(glyph.advance_x as u16 * 8, glyph.advance_y as u16 * 16);

        self.metrics.insert(metrics);
        self.glyphs.insert(id, glyph);
        self.stats.insertions += 1;
    }

    pub fn contains(&self, ch: char) -> bool {
        self.glyphs.contains_key(&GlyphId(ch as u32))
    }

    pub fn len(&self) -> usize {
        self.glyphs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.glyphs.is_empty()
    }

    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    pub fn memory_usage(&self) -> usize {
        self.glyphs.len() * std::mem::size_of::<Glyph>() + self.metrics.total_bytes()
    }

    pub fn evict(&mut self, count: usize) {
        let target = self.glyphs.len().saturating_sub(count);
        let target_bytes = self.max_bytes * target / self.max_glyphs;

        let evicted = self.metrics.evict_lru(target_bytes);
        self.stats.evictions += evicted as u64;

        let expired: Vec<GlyphId> = self
            .glyphs
            .iter()
            .filter(|(id, _)| !self.metrics.contains(id))
            .map(|(id, _)| *id)
            .collect();

        for id in expired {
            self.glyphs.remove(&id);
        }
    }

    pub fn evict_expired(&mut self) {
        let now = Instant::now();
        let expired: Vec<GlyphId> = self.glyphs.keys().cloned().collect();

        for id in expired {
            if let Some(metrics) = self.metrics.get(&id)
                && now.duration_since(metrics.last_accessed) > self.max_age
            {
                self.glyphs.remove(&id);
                self.metrics.remove(&id);
            }
        }
    }

    pub fn clear(&mut self) {
        self.glyphs.clear();
        self.metrics.clear();
        self.stats = CacheStats::default();
    }

    pub fn preload_ascii(&mut self) {
        for i in 0..128u8 {
            self.get_or_insert(i as char);
        }
    }

    pub fn preload_box_drawing(&mut self) {
        for cp in 0x2500..=0x257F {
            if let Some(ch) = char::from_u32(cp) {
                self.get_or_insert(ch);
            }
        }
    }

    pub fn preload_braille(&mut self) {
        for cp in 0x2800..=0x28FF {
            if let Some(ch) = char::from_u32(cp) {
                self.get_or_insert(ch);
            }
        }
    }

    pub fn category_stats(&self) -> HashMap<GlyphCategory, usize> {
        let mut counts = HashMap::new();
        for glyph in self.glyphs.values() {
            *counts.entry(glyph.category).or_insert(0) += 1;
        }
        counts
    }

    fn create_glyph(&self, ch: char) -> Glyph {
        if let Some(table_glyph) = self.tables.get_glyph(ch) {
            return table_glyph.clone();
        }

        let category = GlyphCategory::from_char(ch);
        let width = category.width_hint();

        Glyph::new(ch).with_width(width).with_category(category)
    }
}

impl Default for GlyphCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_new() {
        let cache = GlyphCache::new();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn cache_insert_and_get() {
        let mut cache = GlyphCache::new();
        let glyph = Glyph::new('A');
        cache.insert(glyph);

        let g = cache.get('A');
        assert!(g.is_some());
        assert_eq!(g.unwrap().ch, 'A');
    }

    #[test]
    fn cache_get_or_insert() {
        let mut cache = GlyphCache::new();
        let g = cache.get_or_insert('X');
        assert_eq!(g.ch, 'X');
        assert!(cache.contains('X'));
    }

    #[test]
    fn cache_stats() {
        let mut cache = GlyphCache::new();
        cache.get_or_insert('B');
        cache.get_or_insert('B');
        cache.get('A');

        assert_eq!(cache.stats().lookups, 3);
        assert_eq!(cache.stats().misses, 2);
        assert_eq!(cache.stats().hits, 1);
    }

    #[test]
    fn cache_evict() {
        let mut cache = GlyphCache::with_config(100, 1024 * 1024);
        for i in 0..200u8 {
            cache.insert(Glyph::new(i as char));
        }

        cache.evict(100);
        assert!(cache.len() <= 100);
    }

    #[test]
    fn cache_preload_ascii() {
        let mut cache = GlyphCache::new();
        cache.preload_ascii();
        assert_eq!(cache.len(), 128);
    }

    #[test]
    fn cache_preload_box_drawing() {
        let mut cache = GlyphCache::new();
        cache.preload_box_drawing();
        assert_eq!(cache.len(), 128);
    }

    #[test]
    fn cache_preload_braille() {
        let mut cache = GlyphCache::new();
        cache.preload_braille();
        assert_eq!(cache.len(), 256);
    }

    #[test]
    fn cache_memory_usage() {
        let mut cache = GlyphCache::new();
        cache.preload_ascii();

        let usage = cache.memory_usage();
        assert!(usage > 0);
    }

    #[test]
    fn cache_category_stats() {
        let mut cache = GlyphCache::new();
        cache.preload_ascii();
        cache.preload_box_drawing();

        let stats = cache.category_stats();
        assert!(*stats.get(&GlyphCategory::Ascii).unwrap() > 0);
        assert!(*stats.get(&GlyphCategory::BoxDrawing).unwrap() > 0);
    }

    #[test]
    fn cache_clear() {
        let mut cache = GlyphCache::new();
        cache.preload_ascii();
        cache.clear();

        assert_eq!(cache.len(), 0);
        assert_eq!(cache.stats().hits, 0);
    }
}
