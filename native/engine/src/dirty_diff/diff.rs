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

        let dirty_cells = Self::find_dirty_cells(current, previous);
        self.regions = Self::merge_cells_to_regions(&dirty_cells, current.width());
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

    fn find_dirty_cells(current: &FrameBuffer, previous: &FrameBuffer) -> Vec<(u16, u16)> {
        let mut dirty = Vec::new();
        let w = current.width().min(previous.width());
        let h = current.height().min(previous.height());
        for y in 0..h {
            for x in 0..w {
                if current.get(x, y) != previous.get(x, y) {
                    dirty.push((x, y));
                }
            }
        }
        dirty
    }

    fn merge_cells_to_regions(cells: &[(u16, u16)], width: u16) -> Vec<DirtyRegion> {
        if cells.is_empty() {
            return Vec::new();
        }

        let mut grid = vec![false; (width as usize) * 256];
        for &(x, y) in cells {
            if (y as usize) < 256 {
                grid[(y as usize) * (width as usize) + (x as usize)] = true;
            }
        }

        let mut regions = Vec::new();
        let mut visited = vec![false; grid.len()];

        for &(x, y) in cells {
            let idx = (y as usize) * (width as usize) + (x as usize);
            if visited[idx] {
                continue;
            }

            let mut max_x = x;

            while max_x + 1 < width
                && grid[(y as usize) * (width as usize) + ((max_x + 1) as usize)]
            {
                max_x += 1;
            }

            let mut row = y;
            while row + 1 < 256 {
                let mut can_extend = true;
                for cx in x..=max_x {
                    if !grid[((row + 1) as usize) * (width as usize) + (cx as usize)] {
                        can_extend = false;
                        break;
                    }
                }
                if !can_extend {
                    break;
                }
                row += 1;
            }
            let max_y = row;

            for cy in y..=max_y {
                for cx in x..=max_x {
                    visited[(cy as usize) * (width as usize) + (cx as usize)] = true;
                }
            }

            regions.push(DirtyRegion::new(x, y, max_x - x + 1, max_y - y + 1));
        }

        regions
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
}
