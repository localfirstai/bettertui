//! Native hit grid for O(1) mouse event targeting.
//!
//! Double-buffered, screen-sized grid where each cell stores the
//! renderable ID at that position. Mirrors OpenTUI's hit grid pattern
//! with scissor clipping for overflow:hidden support.

/// Axis-aligned clipping rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClipRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// Default ID for empty cells (no renderable at position).
pub const HIT_EMPTY: u64 = 0;

/// Screen-sized dense grid for O(1) hit testing.
///
/// Double-buffered: `current` grid is queried; `next` grid is
/// built during render. After render completes, buffers swap.
///
/// Scissor stack mirrors overflow:hidden containers for clipping.
#[derive(Debug, Clone)]
pub struct HitGrid {
    current: Vec<u64>,
    next: Vec<u64>,
    width: u32,
    height: u32,
    scissor_stack: Vec<ClipRect>,
    dirty: bool,
    resize_invalidated: bool,
}

impl HitGrid {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            current: vec![HIT_EMPTY; size],
            next: vec![HIT_EMPTY; size],
            width,
            height,
            scissor_stack: Vec::with_capacity(8),
            dirty: false,
            resize_invalidated: false,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let new_size = (width * height) as usize;
        self.current.resize(new_size, HIT_EMPTY);
        self.next.resize(new_size, HIT_EMPTY);
        self.current.fill(HIT_EMPTY);
        self.next.fill(HIT_EMPTY);
        self.width = width;
        self.height = height;
        self.resize_invalidated = true;
        self.dirty = true;
    }

    pub fn clear_next(&mut self) {
        self.next.fill(HIT_EMPTY);
    }

    pub fn clear_current(&mut self) {
        self.current.fill(HIT_EMPTY);
    }

    /// Register a renderable's bounding rect in the next grid.
    /// Clipped to current scissor stack. Later renderables overwrite earlier ones.
    pub fn add(&mut self, x: i32, y: i32, width: u32, height: u32, id: u64) {
        let clipped = self.clip_to_scissor(x, y, width, height);
        let clipped = match clipped {
            Some(c) => c,
            None => return,
        };

        let start_x = clipped.x.max(0) as u32;
        let start_y = clipped.y.max(0) as u32;
        let end_x = (clipped.x + clipped.width as i32).min(self.width as i32) as u32;
        let end_y = (clipped.y + clipped.height as i32).min(self.height as i32) as u32;

        if start_x >= end_x || start_y >= end_y {
            return;
        }

        for row in start_y..end_y {
            let row_start = (row * self.width) as usize;
            let s = row_start + start_x as usize;
            let e = row_start + end_x as usize;
            self.next[s..e].fill(id);
        }
    }

    /// Register a renderable in the current grid immediately (for scroll/translate sync).
    pub fn add_current(&mut self, x: i32, y: i32, width: u32, height: u32, id: u64) {
        let clipped = self.clip_to_scissor(x, y, width, height);
        let clipped = match clipped {
            Some(c) => c,
            None => return,
        };

        let start_x = clipped.x.max(0) as u32;
        let start_y = clipped.y.max(0) as u32;
        let end_x = (clipped.x + clipped.width as i32).min(self.width as i32) as u32;
        let end_y = (clipped.y + clipped.height as i32).min(self.height as i32) as u32;

        if start_x >= end_x || start_y >= end_y {
            return;
        }

        for row in start_y..end_y {
            let row_start = (row * self.width) as usize;
            let s = row_start + start_x as usize;
            let e = row_start + end_x as usize;
            self.current[s..e].fill(id);
        }
    }

    /// O(1) hit test: returns renderable ID at (x, y), or HIT_EMPTY.
    pub fn check(&self, x: u32, y: u32) -> u64 {
        if x >= self.width || y >= self.height {
            return HIT_EMPTY;
        }
        let idx = (y * self.width + x) as usize;
        self.current[idx]
    }

    /// Swap buffers after render completes.
    /// Returns true if the hit grid changed (dirty).
    pub fn swap(&mut self) -> bool {
        self.dirty = self.resize_invalidated || self.current != self.next;
        std::mem::swap(&mut self.current, &mut self.next);
        self.next.fill(HIT_EMPTY);
        self.resize_invalidated = false;
        self.dirty
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    // ─── Scissor Stack ────────────────────────────────────────────

    pub fn push_scissor(&mut self, x: i32, y: i32, width: u32, height: u32) {
        let rect = ClipRect { x, y, width, height };
        let effective = match self.scissor_stack.last() {
            Some(parent) => intersect_clip_rect(parent, &rect).unwrap_or(ClipRect { x: 0, y: 0, width: 0, height: 0 }),
            None => rect,
        };
        self.scissor_stack.push(effective);
    }

    pub fn pop_scissor(&mut self) {
        self.scissor_stack.pop();
    }

    pub fn clear_scissors(&mut self) {
        self.scissor_stack.clear();
    }

    fn clip_to_scissor(&self, x: i32, y: i32, width: u32, height: u32) -> Option<ClipRect> {
        let rect = ClipRect { x, y, width, height };
        match self.scissor_stack.last() {
            Some(scissor) => intersect_clip_rect(scissor, &rect),
            None => Some(rect),
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

fn intersect_clip_rect(a: &ClipRect, b: &ClipRect) -> Option<ClipRect> {
    let x = a.x.max(b.x);
    let y = a.y.max(b.y);
    let end_x = (a.x + a.width as i32).min(b.x + b.width as i32);
    let end_y = (a.y + a.height as i32).min(b.y + b.height as i32);

    if x >= end_x || y >= end_y {
        return None;
    }

    Some(ClipRect { x, y, width: (end_x - x) as u32, height: (end_y - y) as u32 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_grid_new() {
        let grid = HitGrid::new(80, 24);
        assert_eq!(grid.dimensions(), (80, 24));
        assert_eq!(grid.check(0, 0), HIT_EMPTY);
    }

    #[test]
    fn hit_grid_add_and_check() {
        let mut grid = HitGrid::new(80, 24);
        grid.add(5, 5, 10, 10, 42);
        grid.swap();
        assert_eq!(grid.check(5, 5), 42);
        assert_eq!(grid.check(14, 14), 42);
        assert_eq!(grid.check(4, 5), HIT_EMPTY);
        assert_eq!(grid.check(5, 4), HIT_EMPTY);
        assert_eq!(grid.check(15, 14), HIT_EMPTY);
    }

    #[test]
    fn hit_grid_overwrite() {
        let mut grid = HitGrid::new(80, 24);
        grid.add(0, 0, 20, 20, 1);
        grid.add(5, 5, 10, 10, 2);
        grid.swap();
        assert_eq!(grid.check(0, 0), 1);
        assert_eq!(grid.check(5, 5), 2);
        assert_eq!(grid.check(7, 7), 2);
    }

    #[test]
    fn hit_grid_out_of_bounds() {
        let mut grid = HitGrid::new(10, 10);
        assert_eq!(grid.check(10, 10), HIT_EMPTY);
        assert_eq!(grid.check(0, 10), HIT_EMPTY);
    }

    #[test]
    fn hit_grid_swap_detects_changes() {
        let mut grid = HitGrid::new(10, 10);
        assert!(!grid.swap()); // empty -> empty = no change
        grid.add(0, 0, 1, 1, 1);
        assert!(grid.swap()); // has content -> different
        assert!(grid.swap()); // current (data) vs next (empty) -> different
        assert!(!grid.swap()); // both empty -> no change
    }

    #[test]
    fn hit_grid_scissor_clips() {
        let mut grid = HitGrid::new(80, 40);
        grid.push_scissor(10, 10, 20, 20);
        grid.add(5, 5, 30, 30, 1);
        grid.swap();
        assert_eq!(grid.check(9, 9), HIT_EMPTY); // outside scissor
        assert_eq!(grid.check(10, 10), 1); // inside scissor
        assert_eq!(grid.check(29, 29), 1);
        assert_eq!(grid.check(30, 30), HIT_EMPTY); // outside scissor
    }

    #[test]
    fn hit_grid_scissor_nesting() {
        let mut grid = HitGrid::new(80, 40);
        grid.push_scissor(0, 0, 50, 50);
        grid.push_scissor(10, 10, 20, 20);
        grid.add(5, 5, 50, 50, 42);
        grid.swap();
        assert_eq!(grid.check(9, 9), HIT_EMPTY); // outside inner scissor
        assert_eq!(grid.check(10, 10), 42); // inside both
        assert_eq!(grid.check(29, 29), 42);
        assert_eq!(grid.check(30, 30), HIT_EMPTY); // outside inner scissor
    }

    #[test]
    fn hit_grid_resize() {
        let mut grid = HitGrid::new(80, 24);
        grid.add(0, 0, 80, 24, 1);
        grid.swap();
        grid.resize(40, 12);
        assert_eq!(grid.check(0, 0), HIT_EMPTY); // cleared after resize
    }

    #[test]
    fn hit_grid_add_current() {
        let mut grid = HitGrid::new(80, 24);
        grid.add_current(0, 0, 10, 10, 42);
        assert_eq!(grid.check(5, 5), 42);
    }

    #[test]
    fn hit_grid_clear_current() {
        let mut grid = HitGrid::new(10, 10);
        grid.add_current(0, 0, 10, 10, 1);
        assert_eq!(grid.check(5, 5), 1);
        grid.clear_current();
        assert_eq!(grid.check(5, 5), HIT_EMPTY);
    }
}
