use crate::tree::Rect;

/// Resolved layout for a single node after layout computation.
///
/// Stores the final position and size in terminal cell coordinates.
/// All values are integers — fractional Taffy output is rounded at the last step.
///
/// **Memory:** 20 bytes per node. Stack-allocated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutResult {
    /// Absolute X position in terminal cells (from left edge).
    pub x: u16,
    /// Absolute Y position in terminal cells (from top edge).
    pub y: u16,
    /// Outer width including padding and border (in cells).
    pub width: u16,
    /// Outer height including padding and border (in cells).
    pub height: u16,
    /// Inner width excluding padding and border (in cells).
    pub content_width: u16,
    /// Inner height excluding padding and border (in cells).
    pub content_height: u16,
}

impl Default for LayoutResult {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            content_width: 0,
            content_height: 0,
        }
    }
}

impl LayoutResult {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
            content_width: width,
            content_height: height,
        }
    }

    /// Convert Taffy's f32 pixel output to terminal cell count.
    /// In BetterTUI, 1 "pixel" = 1 terminal cell.
    pub fn pixels_to_cells(pixels: f32) -> u16 {
        (pixels.round() as i32).max(0) as u16
    }

    /// Returns the bounding rectangle for this layout.
    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    /// Returns the content rectangle (excluding padding/border).
    pub fn content_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.content_width, self.content_height)
    }

    /// Returns the right edge (x + width).
    pub fn right(&self) -> u16 {
        self.x.saturating_add(self.width)
    }

    /// Returns the bottom edge (y + height).
    pub fn bottom(&self) -> u16 {
        self.y.saturating_add(self.height)
    }

    /// Check if a point is within this layout's bounds.
    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_layout_result() {
        let lr = LayoutResult::default();
        assert_eq!(lr.x, 0);
        assert_eq!(lr.y, 0);
        assert_eq!(lr.width, 0);
        assert_eq!(lr.height, 0);
    }

    #[test]
    fn layout_result_new() {
        let lr = LayoutResult::new(5, 10, 20, 15);
        assert_eq!(lr.x, 5);
        assert_eq!(lr.y, 10);
        assert_eq!(lr.width, 20);
        assert_eq!(lr.height, 15);
        assert_eq!(lr.content_width, 20);
        assert_eq!(lr.content_height, 15);
    }

    #[test]
    fn pixels_to_cells_rounding() {
        assert_eq!(LayoutResult::pixels_to_cells(0.0), 0);
        assert_eq!(LayoutResult::pixels_to_cells(1.0), 1);
        assert_eq!(LayoutResult::pixels_to_cells(1.4), 1);
        assert_eq!(LayoutResult::pixels_to_cells(1.5), 2);
        assert_eq!(LayoutResult::pixels_to_cells(1.6), 2);
        assert_eq!(LayoutResult::pixels_to_cells(-1.0), 0);
    }

    #[test]
    fn layout_result_rect() {
        let lr = LayoutResult::new(5, 10, 20, 15);
        let rect = lr.rect();
        assert_eq!(rect.x, 5);
        assert_eq!(rect.y, 10);
        assert_eq!(rect.width, 20);
        assert_eq!(rect.height, 15);
    }

    #[test]
    fn layout_result_edges() {
        let lr = LayoutResult::new(5, 10, 20, 15);
        assert_eq!(lr.right(), 25);
        assert_eq!(lr.bottom(), 25);
    }

    #[test]
    fn layout_result_contains() {
        let lr = LayoutResult::new(5, 5, 10, 10);
        assert!(lr.contains(5, 5));
        assert!(lr.contains(14, 14));
        assert!(!lr.contains(4, 5));
        assert!(!lr.contains(5, 4));
        assert!(!lr.contains(15, 15));
    }

    #[test]
    fn layout_result_content_rect() {
        let mut lr = LayoutResult::new(0, 0, 20, 10);
        lr.content_width = 18;
        lr.content_height = 8;
        let cr = lr.content_rect();
        assert_eq!(cr.width, 18);
        assert_eq!(cr.height, 8);
    }
}
