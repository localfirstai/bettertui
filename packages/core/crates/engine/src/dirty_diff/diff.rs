use crate::framebuffer::FrameBuffer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirtyRegion {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl DirtyRegion {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn right(&self) -> u16 {
        self.x.saturating_add(self.width)
    }

    pub fn bottom(&self) -> u16 {
        self.y.saturating_add(self.height)
    }

    pub fn area(&self) -> u32 {
        (self.width as u32) * (self.height as u32)
    }

    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }

    pub fn intersects(&self, other: &DirtyRegion) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }

    pub fn merge(&self, other: &DirtyRegion) -> DirtyRegion {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let right = self.right().max(other.right());
        let bottom = self.bottom().max(other.bottom());
        DirtyRegion::new(x, y, right - x, bottom - y)
    }

    pub fn can_merge_horizontal(&self, other: &DirtyRegion) -> bool {
        self.y == other.y
            && self.height == other.height
            && (self.right() == other.x || other.right() == self.x)
    }

    pub fn can_merge_vertical(&self, other: &DirtyRegion) -> bool {
        self.x == other.x
            && self.width == other.width
            && (self.bottom() == other.y || other.bottom() == self.y)
    }
}

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
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
            last_generation: 0,
        }
    }

    pub fn compute(
        &mut self,
        current: &FrameBuffer,
        previous: &FrameBuffer,
        generation: u64,
    ) -> &[DirtyRegion] {
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

    pub fn compute_full_repaint(&mut self, width: u16, height: u16) -> &[DirtyRegion] {
        self.regions.clear();
        if width > 0 && height > 0 {
            self.regions.push(DirtyRegion::new(0, 0, width, height));
        }
        &self.regions
    }

    pub fn regions(&self) -> &[DirtyRegion] {
        &self.regions
    }

    pub fn is_empty(&self) -> bool {
        self.regions.is_empty()
    }

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
                        if r.y + r.height == y
                            && start_x >= r.x
                            && start_x + span_width <= r.x + r.width
                        {
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

    fn merge_adjacent_regions(regions: &mut Vec<DirtyRegion>) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::{Cell, FrameBuffer};

    #[test]
    fn dirty_region_new() {
        let r = DirtyRegion::new(5, 10, 20, 15);
        assert_eq!(r.x, 5);
        assert_eq!(r.y, 10);
        assert_eq!(r.width, 20);
        assert_eq!(r.height, 15);
    }

    #[test]
    fn dirty_region_edges() {
        let r = DirtyRegion::new(5, 10, 20, 15);
        assert_eq!(r.right(), 25);
        assert_eq!(r.bottom(), 25);
        assert_eq!(r.area(), 300);
    }

    #[test]
    fn dirty_region_contains() {
        let r = DirtyRegion::new(5, 5, 10, 10);
        assert!(r.contains(5, 5));
        assert!(r.contains(14, 14));
        assert!(!r.contains(4, 5));
        assert!(!r.contains(15, 15));
    }

    #[test]
    fn dirty_region_intersects() {
        let a = DirtyRegion::new(0, 0, 10, 10);
        let b = DirtyRegion::new(5, 5, 10, 10);
        let c = DirtyRegion::new(20, 20, 5, 5);
        assert!(a.intersects(&b));
        assert!(!a.intersects(&c));
    }

    #[test]
    fn dirty_region_merge() {
        let a = DirtyRegion::new(0, 0, 5, 5);
        let b = DirtyRegion::new(5, 0, 5, 5);
        let merged = a.merge(&b);
        assert_eq!(merged, DirtyRegion::new(0, 0, 10, 5));
    }

    #[test]
    fn dirty_region_can_merge_horizontal() {
        let a = DirtyRegion::new(0, 0, 5, 5);
        let b = DirtyRegion::new(5, 0, 5, 5);
        let c = DirtyRegion::new(0, 5, 5, 5);
        assert!(a.can_merge_horizontal(&b));
        assert!(!a.can_merge_horizontal(&c));
    }

    #[test]
    fn dirty_region_can_merge_vertical() {
        let a = DirtyRegion::new(0, 0, 5, 5);
        let b = DirtyRegion::new(0, 5, 5, 5);
        let c = DirtyRegion::new(5, 0, 5, 5);
        assert!(a.can_merge_vertical(&b));
        assert!(!a.can_merge_vertical(&c));
    }

    #[test]
    fn dirty_diff_no_changes() {
        let mut a = FrameBuffer::new(5, 5);
        let mut b = FrameBuffer::new(5, 5);
        a.swap();
        b.swap();
        let mut diff = DirtyDiff::new();
        let regions = diff.compute(&a, &b, 1);
        assert!(regions.is_empty());
    }

    #[test]
    fn dirty_diff_with_changes() {
        let mut a = FrameBuffer::new(5, 5);
        let mut b = FrameBuffer::new(5, 5);
        a.swap();
        b.swap();
        a.set(2, 2, Cell::new('X'));
        let mut diff = DirtyDiff::new();
        let regions = diff.compute(&a, &b, 1);
        assert!(!regions.is_empty());
    }

    #[test]
    fn dirty_diff_full_repaint() {
        let mut diff = DirtyDiff::new();
        let regions = diff.compute_full_repaint(80, 24);
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0], DirtyRegion::new(0, 0, 80, 24));
    }

    #[test]
    fn dirty_diff_generation_caching() {
        let mut a = FrameBuffer::new(5, 5);
        let mut b = FrameBuffer::new(5, 5);
        a.swap();
        b.swap();
        a.set(0, 0, Cell::new('A'));
        let mut diff = DirtyDiff::new();
        let r1 = diff.compute(&a, &b, 1);
        let len1 = r1.len();
        let r2 = diff.compute(&a, &b, 1);
        assert_eq!(r2.len(), len1);
    }

    #[test]
    fn dirty_diff_merge_regions() {
        let mut a = FrameBuffer::new(10, 10);
        let mut b = FrameBuffer::new(10, 10);
        a.swap();
        b.swap();
        a.set(2, 2, Cell::new('X'));
        a.set(3, 2, Cell::new('Y'));
        a.set(2, 3, Cell::new('Z'));
        a.set(3, 3, Cell::new('W'));
        let mut diff = DirtyDiff::new();
        let regions = diff.compute(&a, &b, 1);
        assert!(regions.len() <= 2);
    }

    #[test]
    fn dirty_diff_total_area() {
        let mut diff = DirtyDiff::new();
        diff.compute_full_repaint(80, 24);
        assert_eq!(diff.total_area(), 80 * 24);
    }

    fn make_diff_with_cells(cells: &[(u16, u16)]) -> Vec<DirtyRegion> {
        let mut fb = FrameBuffer::new(20, 20);
        let empty = FrameBuffer::new(20, 20);
        for &(x, y) in cells {
            fb.set(x, y, Cell::new('X'));
        }
        let mut diff = DirtyDiff::new();
        diff.compute(&fb, &empty, 1).to_vec()
    }

    #[test]
    fn merge_cells_horizontal_span() {
        let regions = make_diff_with_cells(&[(0, 0), (1, 0), (2, 0)]);
        assert_eq!(
            regions.len(),
            1,
            "horizontal span should merge into one region"
        );
        assert_eq!(regions[0].x, 0);
        assert_eq!(regions[0].width, 3);
        assert_eq!(regions[0].height, 1);
    }

    #[test]
    fn merge_cells_vertical_stack() {
        let regions = make_diff_with_cells(&[(0, 0), (0, 1), (0, 2)]);
        assert_eq!(regions.len(), 1, "vertical stack should merge into one");
        assert_eq!(regions[0].y, 0);
        assert_eq!(regions[0].height, 3);
    }

    #[test]
    fn merge_cells_separate_regions() {
        let regions = make_diff_with_cells(&[(0, 0), (5, 0), (0, 5), (5, 5)]);
        assert_eq!(regions.len(), 4, "isolated cells should produce 4 regions");
    }

    #[test]
    fn merge_cells_rectangle() {
        let mut cells = Vec::new();
        for y in 0..3 {
            for x in 0..5 {
                cells.push((x, y));
            }
        }
        let regions = make_diff_with_cells(&cells);
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].width, 5);
        assert_eq!(regions[0].height, 3);
    }

    #[test]
    fn merge_cells_non_contiguous_vertical() {
        let regions = make_diff_with_cells(&[(0, 0), (0, 1), (0, 3), (0, 4)]);
        assert!(
            regions.len() >= 2,
            "gap in vertical should create >= 2 regions"
        );
    }

    #[test]
    fn merge_cells_no_changes() {
        let fb = FrameBuffer::new(10, 10);
        let empty = FrameBuffer::new(10, 10);
        let mut diff = DirtyDiff::new();
        let regions = diff.compute(&fb, &empty, 1);
        assert!(regions.is_empty());
    }

    #[test]
    fn merge_adjacent_horizontal_regions() {
        let mut regions = vec![DirtyRegion::new(0, 0, 5, 1), DirtyRegion::new(5, 0, 5, 1)];
        DirtyDiff::merge_adjacent_regions(&mut regions);
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].width, 10);
    }

    #[test]
    fn merge_adjacent_vertical_regions() {
        let mut regions = vec![DirtyRegion::new(0, 0, 5, 3), DirtyRegion::new(0, 3, 5, 3)];
        DirtyDiff::merge_adjacent_regions(&mut regions);
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].height, 6);
    }

    #[test]
    fn merge_non_adjacent_regions_unchanged() {
        let mut regions = vec![DirtyRegion::new(0, 0, 3, 3), DirtyRegion::new(10, 10, 3, 3)];
        DirtyDiff::merge_adjacent_regions(&mut regions);
        assert_eq!(
            regions.len(),
            2,
            "non-adjacent regions should stay separate"
        );
    }

    #[test]
    fn compute_returns_merged_regions() {
        let mut current = FrameBuffer::new(20, 10);
        let mut previous = FrameBuffer::new(20, 10);
        current.swap();
        previous.swap();
        current.set(0, 0, Cell::new('X'));
        current.set(1, 0, Cell::new('Y'));
        current.set(0, 1, Cell::new('Z'));
        current.set(1, 1, Cell::new('W'));
        let mut diff = DirtyDiff::new();
        let regions = diff.compute(&current, &previous, 1);
        assert_eq!(regions.len(), 1, "2x2 block should be one region");
        assert_eq!(regions[0].width, 2);
        assert_eq!(regions[0].height, 2);
    }
}
