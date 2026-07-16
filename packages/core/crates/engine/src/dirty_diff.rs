//! Dirty region computation: diffs two framebuffers to find changed areas for incremental rendering.
//!
//! This module provides [`DirtyDiff`] and [`DirtyRegion`] for computing minimal
//! repaint areas between frames, enabling efficient terminal output.

use crate::framebuffer::FrameBuffer;

/// A rectangular region of the screen that needs repainting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirtyRegion {
    /// Left edge (column).
    pub x: u16,
    /// Top edge (row).
    pub y: u16,
    /// Width in cells.
    pub width: u16,
    /// Height in cells.
    pub height: u16,
}

impl DirtyRegion {
    /// Creates a new dirty region at `(x, y)` with the given dimensions.
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height }
    }

    /// Returns the right edge (exclusive).
    pub fn right(&self) -> u16 {
        self.x.saturating_add(self.width)
    }

    /// Returns the bottom edge (exclusive).
    pub fn bottom(&self) -> u16 {
        self.y.saturating_add(self.height)
    }

    /// Returns the area in cells.
    pub fn area(&self) -> u32 {
        (self.width as u32) * (self.height as u32)
    }

    /// Returns `true` if `(x, y)` is inside this region.
    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }

    /// Returns `true` if this region overlaps `other`.
    pub fn intersects(&self, other: &DirtyRegion) -> bool {
        self.x < other.right() && self.right() > other.x && self.y < other.bottom() && self.bottom() > other.y
    }

    /// Returns the smallest region that contains both `self` and `other`.
    pub fn merge(&self, other: &DirtyRegion) -> DirtyRegion {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let right = self.right().max(other.right());
        let bottom = self.bottom().max(other.bottom());
        DirtyRegion::new(x, y, right - x, bottom - y)
    }

    /// Returns `true` if `self` and `other` can be merged horizontally
    /// (same row span, adjacent columns).
    pub fn can_merge_horizontal(&self, other: &DirtyRegion) -> bool {
        self.y == other.y && self.height == other.height && (self.right() == other.x || other.right() == self.x)
    }

    /// Returns `true` if `self` and `other` can be merged vertically
    /// (same column span, adjacent rows).
    pub fn can_merge_vertical(&self, other: &DirtyRegion) -> bool {
        self.x == other.x && self.width == other.width && (self.bottom() == other.y || other.bottom() == self.y)
    }
}

/// Computes dirty regions between two framebuffers for incremental rendering.
///
/// Maintains a generation counter for caching: calling [`compute`](Self::compute)
/// with the same generation returns cached results without recomputation.
#[derive(Debug)]
pub struct DirtyDiff {
    regions: Vec<DirtyRegion>,
    last_generation: u64,
}

impl Default for DirtyDiff {
    fn default() -> Self {
        Self::new()
    }
}

impl DirtyDiff {
    /// Creates a new DirtyDiff with no regions.
    pub fn new() -> Self {
        Self { regions: Vec::new(), last_generation: 0 }
    }

    /// Computes dirty regions by diffing `current` against `previous`.
    ///
    /// Returns a slice of [`DirtyRegion`]s representing the changed areas.
    /// Results are cached per generation: repeated calls with the same generation
    /// return the cached result.
    pub fn compute(&mut self, current: &FrameBuffer, previous: &FrameBuffer, generation: u64) -> &[DirtyRegion] {
        if generation == self.last_generation && !self.regions.is_empty() {
            return &self.regions;
        }
        self.last_generation = generation;

        self.regions.clear();
        let w = current.width().min(previous.width());
        let h = current.height().min(previous.height());
        Self::compute_dirty_regions(current, previous, w, h, &mut self.regions);
        Self::merge_adjacent_regions(&mut self.regions);
        &self.regions
    }

    /// Sets a full repaint region covering the entire screen.
    pub fn compute_full_repaint(&mut self, width: u16, height: u16) -> &[DirtyRegion] {
        self.regions.clear();
        if width > 0 && height > 0 {
            self.regions.push(DirtyRegion::new(0, 0, width, height));
        }
        &self.regions
    }

    /// Returns the computed dirty regions.
    pub fn regions(&self) -> &[DirtyRegion] {
        &self.regions
    }

    /// Returns `true` if there are no dirty regions.
    pub fn is_empty(&self) -> bool {
        self.regions.is_empty()
    }

    /// Returns the total area of all dirty regions combined.
    pub fn total_area(&self) -> u32 {
        self.regions.iter().map(|r| r.area()).sum()
    }

    fn compute_dirty_regions(
        current: &FrameBuffer,
        previous: &FrameBuffer,
        w: u16,
        h: u16,
        regions: &mut Vec<DirtyRegion>,
    ) {
        for y in 0..h {
            let mut x = 0;
            while x < w {
                if current.get(x, y) != previous.get(x, y) {
                    let start_x = x;
                    while x + 1 < w && current.get(x + 1, y) != previous.get(x + 1, y) {
                        x += 1;
                    }
                    let span_width = x - start_x + 1;

                    let mut merged = false;
                    for r in regions.iter_mut().rev() {
                        if r.y + r.height == y && start_x >= r.x && start_x + span_width <= r.x + r.width {
                            r.height += 1;
                            merged = true;
                            break;
                        }
                    }
                    if !merged {
                        regions.push(DirtyRegion::new(start_x, y, span_width, 1));
                    }
                }
                x += 1;
            }
        }
    }

    /// Merges adjacent regions horizontally or vertically.
    #[doc(hidden)]
    pub fn merge_adjacent_regions(regions: &mut Vec<DirtyRegion>) {
        if regions.len() < 2 {
            return;
        }

        let mut changed = true;
        while changed {
            changed = false;
            let mut j = 0;
            while j + 1 < regions.len() {
                let (left, right) = (regions[j], regions[j + 1]);
                if left.can_merge_horizontal(&right) || left.can_merge_vertical(&right) {
                    let merged = left.merge(&right);
                    regions[j] = merged;
                    regions.swap_remove(j + 1);
                    changed = true;
                } else {
                    j += 1;
                }
            }
        }
    }
}
