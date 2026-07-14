use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FontMetrics {
    pub width: u16,
    pub height: u16,
    pub bearing_x: i16,
    pub bearing_y: i16,
    pub advance_x: u16,
    pub advance_y: u16,
    pub is_monospace: bool,
}

impl Default for FontMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl FontMetrics {
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

#[derive(Debug, Clone)]
pub struct FontMetricsCache {
    metrics: HashMap<u32, FontMetrics>,
    cell_width: u16,
    cell_height: u16,
}

impl Default for FontMetricsCache {
    fn default() -> Self {
        Self::new(8, 16)
    }
}

impl FontMetricsCache {
    pub fn new(cell_width: u16, cell_height: u16) -> Self {
        Self {
            metrics: HashMap::new(),
            cell_width,
            cell_height,
        }
    }

    pub fn get(&self, codepoint: u32) -> Option<&FontMetrics> {
        self.metrics.get(&codepoint)
    }

    pub fn insert(&mut self, codepoint: u32, metrics: FontMetrics) {
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

    pub fn total_memory(&self) -> usize {
        self.metrics.len() * std::mem::size_of::<FontMetrics>()
    }

    pub fn preload_standard(&mut self) {
        let default = FontMetrics::new()
            .with_dimensions(self.cell_width, self.cell_height)
            .with_advance(1, 1)
            .with_monospace(true);

        let wide = FontMetrics::new()
            .with_dimensions(self.cell_width * 2, self.cell_height)
            .with_advance(2, 1)
            .with_monospace(true);

        self.metrics.insert(0, default.clone());

        for cp in 0xE000..=0xE00Au16 {
            self.metrics.insert(cp as u32, default.clone());
        }
        for cp in 0xE0A0..=0xE0C8u16 {
            self.metrics.insert(cp as u32, default.clone());
        }
        for cp in 0xE200..=0xE2A9u16 {
            self.metrics.insert(cp as u32, default.clone());
        }
        for cp in 0xE700..=0xEC45u16 {
            self.metrics.insert(cp as u32, default.clone());
        }
        for cp in 0xF000..=0xF299u16 {
            self.metrics.insert(cp as u32, default.clone());
        }
        for cp in 0xF400..=0xF8FFu16 {
            self.metrics.insert(cp as u32, default.clone());
        }

        self.metrics.insert(0xE0B0, wide.clone());
    }

    pub fn stats(&self) -> MetricsStats {
        MetricsStats {
            total_glyphs: self.metrics.len(),
            total_bytes: self.total_memory(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MetricsStats {
    pub total_glyphs: usize,
    pub total_bytes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let retrieved = cache.get(0xE0A0);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().width, 8);
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
    fn cache_set_cell_size() {
        let mut cache = FontMetricsCache::new(8, 16);
        cache.set_cell_size(10, 20);
        assert_eq!(cache.cell_width(), 10);
        assert_eq!(cache.cell_height(), 20);
    }

    #[test]
    fn cache_stats() {
        let mut cache = FontMetricsCache::new(8, 16);
        cache.insert(0xE0A0, FontMetrics::new());
        let stats = cache.stats();
        assert_eq!(stats.total_glyphs, 1);
        assert!(stats.total_bytes > 0);
    }
}
