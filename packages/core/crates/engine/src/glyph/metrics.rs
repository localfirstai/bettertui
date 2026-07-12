use super::character::GlyphId;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct GlyphMetrics {
    pub glyph_id: GlyphId,
    pub width: u16,
    pub height: u16,
    pub bearing_x: i16,
    pub bearing_y: i16,
    pub advance_x: u16,
    pub advance_y: u16,
    pub bitmap_offset: usize,
    pub bitmap_size: usize,
    pub last_accessed: Instant,
    pub access_count: u64,
}

impl GlyphMetrics {
    pub fn new(glyph_id: GlyphId) -> Self {
        Self {
            glyph_id,
            width: 0,
            height: 0,
            bearing_x: 0,
            bearing_y: 0,
            advance_x: 0,
            advance_y: 0,
            bitmap_offset: 0,
            bitmap_size: 0,
            last_accessed: Instant::now(),
            access_count: 0,
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

    pub fn with_bitmap(mut self, offset: usize, size: usize) -> Self {
        self.bitmap_offset = offset;
        self.bitmap_size = size;
        self
    }

    pub fn touch(&mut self) {
        self.last_accessed = Instant::now();
        self.access_count += 1;
    }

    pub fn bytes_used(&self) -> usize {
        std::mem::size_of::<Self>() + self.bitmap_size
    }
}

#[derive(Debug, Clone, Default)]
pub struct MetricsCache {
    metrics: HashMap<GlyphId, GlyphMetrics>,
    total_bytes: usize,
    max_bytes: usize,
}

impl MetricsCache {
    pub fn new(max_bytes: usize) -> Self {
        Self {
            metrics: HashMap::new(),
            total_bytes: 0,
            max_bytes,
        }
    }

    pub fn get(&mut self, glyph_id: &GlyphId) -> Option<&GlyphMetrics> {
        if let Some(m) = self.metrics.get_mut(glyph_id) {
            m.touch();
            Some(m)
        } else {
            None
        }
    }

    pub fn insert(&mut self, metrics: GlyphMetrics) {
        let bytes = metrics.bytes_used();
        self.total_bytes += bytes;
        self.metrics.insert(metrics.glyph_id, metrics);
    }

    pub fn remove(&mut self, glyph_id: &GlyphId) -> Option<GlyphMetrics> {
        if let Some(m) = self.metrics.remove(glyph_id) {
            self.total_bytes -= m.bytes_used();
            Some(m)
        } else {
            None
        }
    }

    pub fn contains(&self, glyph_id: &GlyphId) -> bool {
        self.metrics.contains_key(glyph_id)
    }

    pub fn len(&self) -> usize {
        self.metrics.len()
    }

    pub fn is_empty(&self) -> bool {
        self.metrics.is_empty()
    }

    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    pub fn max_bytes(&self) -> usize {
        self.max_bytes
    }

    pub fn evict_lru(&mut self, target_bytes: usize) -> usize {
        let mut evicted = 0;
        let mut candidates: Vec<(GlyphId, Instant)> = self
            .metrics
            .iter()
            .map(|(id, m)| (*id, m.last_accessed))
            .collect();

        candidates.sort_by_key(|(_, time)| *time);

        for (id, _) in candidates {
            if self.total_bytes <= target_bytes {
                break;
            }
            if let Some(m) = self.metrics.remove(&id) {
                self.total_bytes -= m.bytes_used();
                evicted += 1;
            }
        }

        evicted
    }

    pub fn clear(&mut self) {
        self.metrics.clear();
        self.total_bytes = 0;
    }

    pub fn stats(&self) -> MetricsStats {
        let mut category_counts = HashMap::new();
        let mut total_accesses = 0u64;
        let mut oldest = Instant::now();
        let mut newest = Instant::now();

        for metrics in self.metrics.values() {
            *category_counts.entry(metrics.glyph_id).or_insert(0) += 1;
            total_accesses += metrics.access_count;
            if metrics.last_accessed < oldest {
                oldest = metrics.last_accessed;
            }
            if metrics.last_accessed > newest {
                newest = metrics.last_accessed;
            }
        }

        MetricsStats {
            total_glyphs: self.metrics.len(),
            total_bytes: self.total_bytes,
            max_bytes: self.max_bytes,
            total_accesses,
            oldest_access: oldest,
            newest_access: newest,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MetricsStats {
    pub total_glyphs: usize,
    pub total_bytes: usize,
    pub max_bytes: usize,
    pub total_accesses: u64,
    pub oldest_access: Instant,
    pub newest_access: Instant,
}

impl std::fmt::Display for MetricsStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Glyphs: {}, Memory: {}/{} bytes, Accesses: {}",
            self.total_glyphs, self.total_bytes, self.max_bytes, self.total_accesses
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn cache_insert_and_get() {
        let mut cache = MetricsCache::new(1024 * 1024);
        let m = GlyphMetrics::new(GlyphId(65)).with_dimensions(8, 16);
        cache.insert(m);

        assert!(cache.contains(&GlyphId(65)));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn cache_remove() {
        let mut cache = MetricsCache::new(1024 * 1024);
        let m = GlyphMetrics::new(GlyphId(65));
        cache.insert(m);

        assert!(cache.remove(&GlyphId(65)).is_some());
        assert!(!cache.contains(&GlyphId(65)));
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn cache_evict_lru() {
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
    fn cache_stats() {
        let mut cache = MetricsCache::new(1024);
        let m = GlyphMetrics::new(GlyphId(65)).with_bitmap(0, 128);
        cache.insert(m);

        let stats = cache.stats();
        assert_eq!(stats.total_glyphs, 1);
        assert!(stats.total_bytes >= 128);
    }

    #[test]
    fn cache_clear() {
        let mut cache = MetricsCache::new(1024);
        for i in 0..10 {
            let m = GlyphMetrics::new(GlyphId(i));
            cache.insert(m);
        }

        cache.clear();
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.total_bytes(), 0);
    }
}
