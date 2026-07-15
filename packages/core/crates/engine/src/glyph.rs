//! Glyph system: character classification, glyph caching, metrics, and lookup tables.
//!
//! Provides the building blocks for efficient text rendering:
//! - [`Glyph`] / [`GlyphCategory`] — Character classification and metadata.
//! - [`GlyphCache`] — In-memory glyph cache with LRU eviction and preloading.
//! - [`GlyphMetrics`] / [`MetricsCache`] — Per-glyph dimensions and bitmap tracking.
//! - [`GlyphTables`] — Pre-built lookup tables for ASCII, box-drawing, and Braille.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::framebuffer::CellAttributes;

// ===========================================================================
// Character classification
// ===========================================================================

/// Opaque identifier for a glyph, derived from the Unicode code point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlyphId(pub u32);

/// Category of a character for rendering purposes.
///
/// Determines width hints, font fallback, and special handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlyphCategory {
    Ascii,
    AsciiExtended,
    Unicode,
    Emoji,
    NerdFont,
    BoxDrawing,
    Braille,
    Cjk,
    Diacritical,
    Symbol,
}

impl GlyphCategory {
    /// Classifies a character into a category.
    pub fn from_char(ch: char) -> Self {
        let cp = ch as u32;

        if cp <= 0x7F {
            return Self::Ascii;
        }
        if cp <= 0xFF {
            return Self::AsciiExtended;
        }
        if is_emoji(cp) {
            return Self::Emoji;
        }
        if is_box_drawing(cp) {
            return Self::BoxDrawing;
        }
        if is_braille(cp) {
            return Self::Braille;
        }
        if is_cjk(cp) {
            return Self::Cjk;
        }
        if is_diacritical(cp) {
            return Self::Diacritical;
        }
        if is_nerd_font(cp) {
            return Self::NerdFont;
        }
        if is_symbol(cp) {
            return Self::Symbol;
        }
        Self::Unicode
    }

    /// Returns the display width hint for this category (1 or 2).
    pub fn width_hint(&self) -> u8 {
        match self {
            Self::Cjk => 2,
            Self::Emoji => 2,
            _ => 1,
        }
    }
}

impl From<GlyphCategory> for &'static str {
    fn from(cat: GlyphCategory) -> Self {
        match cat {
            GlyphCategory::Ascii => "ascii",
            GlyphCategory::AsciiExtended => "ascii_extended",
            GlyphCategory::Unicode => "unicode",
            GlyphCategory::Emoji => "emoji",
            GlyphCategory::NerdFont => "nerd_font",
            GlyphCategory::BoxDrawing => "box_drawing",
            GlyphCategory::Braille => "braille",
            GlyphCategory::Cjk => "cjk",
            GlyphCategory::Diacritical => "diacritical",
            GlyphCategory::Symbol => "symbol",
        }
    }
}

/// A single glyph with metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Glyph {
    pub id: GlyphId,
    pub ch: char,
    pub width: u8,
    pub height: u8,
    pub advance_x: u8,
    pub advance_y: u8,
    pub offset_x: i8,
    pub offset_y: i8,
    pub attributes: CellAttributes,
    pub category: GlyphCategory,
}

impl Glyph {
    pub fn new(ch: char) -> Self {
        Self {
            id: GlyphId(ch as u32),
            ch,
            width: 1,
            height: 1,
            advance_x: 1,
            advance_y: 1,
            offset_x: 0,
            offset_y: 0,
            attributes: CellAttributes::empty(),
            category: GlyphCategory::from_char(ch),
        }
    }

    pub fn with_width(mut self, width: u8) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: u8) -> Self {
        self.height = height;
        self
    }

    pub fn with_advance_x(mut self, advance: u8) -> Self {
        self.advance_x = advance;
        self
    }

    pub fn with_offset(mut self, x: i8, y: i8) -> Self {
        self.offset_x = x;
        self.offset_y = y;
        self
    }

    pub fn with_category(mut self, category: GlyphCategory) -> Self {
        self.category = category;
        self
    }

    pub fn is_wide(&self) -> bool {
        self.width > 1
    }

    pub fn is_emoji(&self) -> bool {
        self.category == GlyphCategory::Emoji
    }

    pub fn is_nerd_font(&self) -> bool {
        self.category == GlyphCategory::NerdFont
    }

    pub fn is_box_drawing(&self) -> bool {
        self.category == GlyphCategory::BoxDrawing
    }

    pub fn is_braille(&self) -> bool {
        self.category == GlyphCategory::Braille
    }

    pub fn is_cjk(&self) -> bool {
        self.category == GlyphCategory::Cjk
    }
}

impl From<char> for Glyph {
    fn from(ch: char) -> Self {
        Self::new(ch)
    }
}

// --- Code point classification helpers ---

fn is_emoji(cp: u32) -> bool {
    matches!(cp,
        0x1F600..=0x1F64F | 0x1F300..=0x1F5FF | 0x1F680..=0x1F6FF |
        0x1F900..=0x1F9FF | 0x2700..=0x27BF | 0xFE00..=0xFE0F |
        0x1F1E0..=0x1F1FF | 0x1FA00..=0x1FA6F | 0x1FA70..=0x1FAFF
    )
}

fn is_box_drawing(cp: u32) -> bool {
    matches!(cp, 0x2500..=0x25FF)
}

fn is_braille(cp: u32) -> bool {
    matches!(cp, 0x2800..=0x28FF)
}

fn is_cjk(cp: u32) -> bool {
    matches!(cp,
        0x4E00..=0x9FFF | 0x3400..=0x4DBF | 0x20000..=0x2A6DF |
        0x2A700..=0x2B73F | 0x2B740..=0x2B81F | 0x2B820..=0x2CEAF |
        0xF900..=0xFAFF | 0x2F800..=0x2FA1F
    )
}

fn is_diacritical(cp: u32) -> bool {
    matches!(cp, 0x0300..=0x036F | 0x1AB0..=0x1AFF | 0x1DC0..=0x1DFF | 0x20D0..=0x20FF | 0xFE20..=0xFE2F)
}

fn is_nerd_font(cp: u32) -> bool {
    matches!(cp,
        0xE000..=0xE00A | 0xE0A0..=0xE0A3 | 0xE0B0..=0xE0C8 |
        0xE0D0..=0xE0D4 | 0xE200..=0xE2A9 | 0xE300..=0xE3E3 |
        0xE500..=0xE5E5 | 0xE700..=0xEC45 | 0xED00..=0xEF99 |
        0xF000..=0xF00F | 0xF100..=0xF299 | 0xF400..=0xF8FF
    )
}

fn is_symbol(cp: u32) -> bool {
    matches!(cp, 0x2000..=0x2BFF)
}

// ===========================================================================
// Metrics
// ===========================================================================

/// Dimensions and bitmap metadata for a glyph.
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

/// Cache statistics for [`MetricsCache`].
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

/// LRU cache for [`GlyphMetrics`].
#[derive(Debug, Clone, Default)]
pub struct MetricsCache {
    metrics: HashMap<GlyphId, GlyphMetrics>,
    total_bytes: usize,
    max_bytes: usize,
}

impl MetricsCache {
    pub fn new(max_bytes: usize) -> Self {
        Self { metrics: HashMap::new(), total_bytes: 0, max_bytes }
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
        let mut candidates: Vec<(GlyphId, Instant)> =
            self.metrics.iter().map(|(id, m)| (*id, m.last_accessed)).collect();

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

// ===========================================================================
// Lookup tables
// ===========================================================================

/// Pre-built lookup tables for common glyph ranges.
///
/// Provides O(1) access to glyphs in the ASCII, box-drawing, and Braille ranges.
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

        Self { ascii, box_drawing, braille, common_emoji }
    }

    pub fn ascii_glyph(&self, ch: char) -> Option<&Glyph> {
        let idx = ch as usize;
        if idx < self.ascii.len() { Some(&self.ascii[idx]) } else { None }
    }

    pub fn box_drawing_glyph(&self, ch: char) -> Option<&Glyph> {
        let cp = ch as u32;
        if (0x2500..=0x257F).contains(&cp) { Some(&self.box_drawing[(cp - 0x2500) as usize]) } else { None }
    }

    pub fn braille_glyph(&self, ch: char) -> Option<&Glyph> {
        let cp = ch as u32;
        if (0x2800..=0x28FF).contains(&cp) { Some(&self.braille[(cp - 0x2800) as usize]) } else { None }
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

// ===========================================================================
// Cache
// ===========================================================================

const DEFAULT_MAX_GLYPHS: usize = 10_000;
const DEFAULT_MAX_BYTES: usize = 16 * 1024 * 1024;
const DEFAULT_MAX_AGE: Duration = Duration::from_secs(60);

/// Cache statistics for [`GlyphCache`].
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
        if self.lookups == 0 { 0.0 } else { self.hits as f64 / self.lookups as f64 }
    }
}

/// In-memory glyph cache with LRU eviction.
///
/// Stores glyphs and their metrics, with automatic eviction based on
/// count, byte budget, and age. Supports preloading common ranges.
pub struct GlyphCache {
    glyphs: HashMap<GlyphId, Glyph>,
    metrics: MetricsCache,
    tables: GlyphTables,
    max_glyphs: usize,
    max_bytes: usize,
    max_age: Duration,
    stats: CacheStats,
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

        let expired: Vec<GlyphId> =
            self.glyphs.iter().filter(|(id, _)| !self.metrics.contains(id)).map(|(id, _)| *id).collect();

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
