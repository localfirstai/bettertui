use crate::tree::Rect;
use bitflags::bitflags;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PaintBounds {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub padding_left: u16,
    pub padding_right: u16,
    pub padding_top: u16,
    pub padding_bottom: u16,
}

impl PaintBounds {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
            ..Default::default()
        }
    }

    pub fn with_padding(mut self, left: u16, right: u16, top: u16, bottom: u16) -> Self {
        self.padding_left = left;
        self.padding_right = right;
        self.padding_top = top;
        self.padding_bottom = bottom;
        self
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    pub fn content_rect(&self) -> Rect {
        Rect::new(
            self.x + self.padding_left,
            self.y + self.padding_top,
            self.width
                .saturating_sub(self.padding_left + self.padding_right),
            self.height
                .saturating_sub(self.padding_top + self.padding_bottom),
        )
    }

    pub fn right(&self) -> u16 {
        self.x.saturating_add(self.width)
    }

    pub fn bottom(&self) -> u16 {
        self.y.saturating_add(self.height)
    }

    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }

    pub fn intersects(&self, other: &PaintBounds) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }

    pub fn intersect(&self, other: &PaintBounds) -> Option<PaintBounds> {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());
        if right > x && bottom > y {
            Some(PaintBounds::new(x, y, right - x, bottom - y))
        } else {
            None
        }
    }
}

/// Viewport defines the visible region of the terminal.
/// Used for culling: nodes outside the viewport are skipped during render tree building.
/// Unlike ClipBounds (which clips rendering), Viewport is a culling-only concept.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Viewport {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Viewport {
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

    pub fn contains_rect(&self, px: u16, py: u16, pw: u16, ph: u16) -> bool {
        let r = px.saturating_add(pw);
        let b = py.saturating_add(ph);
        r > self.x && px < self.right() && b > self.y && py < self.bottom()
    }

    pub fn intersect(&self, other: &Viewport) -> Option<Viewport> {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let r = self.right().min(other.right());
        let b = self.bottom().min(other.bottom());
        if r > x && b > y {
            Some(Viewport::new(x, y, r - x, b - y))
        } else {
            None
        }
    }

    pub fn offset(&self, dx: i32, dy: i32) -> Viewport {
        Viewport::new(
            (self.x as i32 + dx).max(0) as u16,
            (self.y as i32 + dy).max(0) as u16,
            self.width,
            self.height,
        )
    }

    /// Expand viewport by `padding` cells on all sides.
    /// Prevents objects from popping in/out at viewport edges during scroll.
    pub fn with_padding(&self, padding: u16) -> Viewport {
        Viewport::new(
            self.x.saturating_sub(padding),
            self.y.saturating_sub(padding),
            self.width.saturating_add(padding * 2),
            self.height.saturating_add(padding * 2),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClipBounds {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl ClipBounds {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn from_rect(rect: &Rect) -> Self {
        Self {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }

    pub fn right(&self) -> u16 {
        self.x.saturating_add(self.width)
    }

    pub fn bottom(&self) -> u16 {
        self.y.saturating_add(self.height)
    }

    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }

    pub fn intersect(&self, other: &ClipBounds) -> Option<ClipBounds> {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());
        if right > x && bottom > y {
            Some(ClipBounds::new(x, y, right - x, bottom - y))
        } else {
            None
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PaintFlags: u8 {
        const EMPTY       = 0b0000_0000;
        const BACKGROUND   = 0b0000_0001;
        const BORDER       = 0b0000_0010;
        const TEXT         = 0b0000_0100;
        const SCROLLBAR    = 0b0000_1000;
        const CURSOR       = 0b0001_0000;
        const OVERLAY      = 0b0010_0000;
        const HIDDEN       = 0b0100_0000;
        const NEEDS_CLIP   = 0b1000_0000;
    }
}

#[derive(Clone)]
pub struct PaintContext {
    pub terminal_width: u16,
    pub terminal_height: u16,
    pub clip_stack: Vec<ClipBounds>,
}

impl PaintContext {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            terminal_width: width,
            terminal_height: height,
            clip_stack: Vec::new(),
        }
    }

    pub fn push_clip(&mut self, clip: ClipBounds) {
        let effective = if let Some(parent) = self.clip_stack.last() {
            parent
                .intersect(&clip)
                .unwrap_or(ClipBounds::new(0, 0, 0, 0))
        } else {
            clip
        };
        self.clip_stack.push(effective);
    }

    pub fn pop_clip(&mut self) {
        self.clip_stack.pop();
    }

    pub fn current_clip(&self) -> Option<&ClipBounds> {
        self.clip_stack.last()
    }

    pub fn is_visible(&self, bounds: &PaintBounds) -> bool {
        if let Some(clip) = self.clip_stack.last() {
            clip.intersect(&ClipBounds::new(
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
            ))
            .is_some()
        } else {
            true
        }
    }

    pub fn clipped_bounds(&self, bounds: &PaintBounds) -> Option<PaintBounds> {
        if let Some(clip) = self.clip_stack.last() {
            let cb = ClipBounds::new(bounds.x, bounds.y, bounds.width, bounds.height);
            cb.intersect(clip)
                .map(|c| PaintBounds::new(c.x, c.y, c.width, c.height))
        } else {
            Some(*bounds)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_new() {
        let v = Viewport::new(0, 0, 80, 24);
        assert_eq!(v.width, 80);
        assert_eq!(v.height, 24);
    }

    #[test]
    fn viewport_contains_rect_inside() {
        let v = Viewport::new(0, 0, 80, 24);
        assert!(v.contains_rect(5, 5, 10, 10));
    }

    #[test]
    fn viewport_contains_rect_outside() {
        let v = Viewport::new(0, 0, 80, 24);
        assert!(!v.contains_rect(100, 100, 10, 10));
    }

    #[test]
    fn viewport_contains_rect_partial() {
        let v = Viewport::new(10, 10, 20, 20);
        assert!(v.contains_rect(5, 5, 20, 20));
        assert!(!v.contains_rect(5, 5, 5, 5));
    }

    #[test]
    fn viewport_intersect_overlap() {
        let a = Viewport::new(0, 0, 10, 10);
        let b = Viewport::new(5, 5, 10, 10);
        let c = a.intersect(&b).unwrap();
        assert_eq!(c.x, 5);
        assert_eq!(c.y, 5);
        assert_eq!(c.width, 5);
        assert_eq!(c.height, 5);
    }

    #[test]
    fn viewport_intersect_no_overlap() {
        let a = Viewport::new(0, 0, 5, 5);
        let b = Viewport::new(10, 10, 5, 5);
        assert!(a.intersect(&b).is_none());
    }

    #[test]
    fn viewport_intersect_contained() {
        let outer = Viewport::new(0, 0, 80, 24);
        let inner = Viewport::new(10, 10, 20, 10);
        let c = outer.intersect(&inner).unwrap();
        assert_eq!(c, inner);
    }

    #[test]
    fn viewport_offset_positive() {
        let v = Viewport::new(0, 0, 80, 24);
        let o = v.offset(10, 5);
        assert_eq!(o.x, 10);
        assert_eq!(o.y, 5);
        assert_eq!(o.width, 80);
        assert_eq!(o.height, 24);
    }

    #[test]
    fn viewport_with_padding() {
        let v = Viewport::new(10, 20, 80, 24);
        let p = v.with_padding(5);
        assert_eq!(p.x, 5);
        assert_eq!(p.y, 15);
        assert_eq!(p.width, 90);
        assert_eq!(p.height, 34);
    }

    #[test]
    fn viewport_with_padding_zero() {
        let v = Viewport::new(0, 0, 80, 24);
        let p = v.with_padding(0);
        assert_eq!(p, v);
    }

    #[test]
    fn viewport_with_padding_saturate() {
        let v = Viewport::new(2, 3, 5, 5);
        let p = v.with_padding(10);
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);
        assert_eq!(p.width, 25);
        assert_eq!(p.height, 25);
    }

    #[test]
    fn viewport_offset_clamp_zero() {
        let v = Viewport::new(5, 5, 80, 24);
        let o = v.offset(-10, -10);
        assert_eq!(o.x, 0);
        assert_eq!(o.y, 0);
    }

    #[test]
    fn paint_bounds_default() {
        let b = PaintBounds::default();
        assert_eq!(b.x, 0);
        assert_eq!(b.width, 0);
    }

    #[test]
    fn paint_bounds_new() {
        let b = PaintBounds::new(5, 10, 20, 15);
        assert_eq!(b.x, 5);
        assert_eq!(b.y, 10);
        assert_eq!(b.width, 20);
        assert_eq!(b.height, 15);
    }

    #[test]
    fn paint_bounds_with_padding() {
        let b = PaintBounds::new(0, 0, 20, 10).with_padding(2, 2, 1, 1);
        assert_eq!(b.padding_left, 2);
        assert_eq!(b.content_rect().width, 16);
        assert_eq!(b.content_rect().height, 8);
    }

    #[test]
    fn paint_bounds_contains() {
        let b = PaintBounds::new(5, 5, 10, 10);
        assert!(b.contains(5, 5));
        assert!(b.contains(14, 14));
        assert!(!b.contains(4, 5));
        assert!(!b.contains(15, 15));
    }

    #[test]
    fn paint_bounds_intersect() {
        let a = PaintBounds::new(0, 0, 10, 10);
        let b = PaintBounds::new(5, 5, 10, 10);
        let c = a.intersect(&b).unwrap();
        assert_eq!(c.x, 5);
        assert_eq!(c.y, 5);
        assert_eq!(c.width, 5);
        assert_eq!(c.height, 5);
    }

    #[test]
    fn paint_bounds_no_intersect() {
        let a = PaintBounds::new(0, 0, 5, 5);
        let b = PaintBounds::new(10, 10, 5, 5);
        assert!(a.intersect(&b).is_none());
    }

    #[test]
    fn clip_bounds_new() {
        let c = ClipBounds::new(0, 0, 80, 24);
        assert_eq!(c.width, 80);
        assert_eq!(c.height, 24);
    }

    #[test]
    fn clip_bounds_intersect() {
        let a = ClipBounds::new(0, 0, 10, 10);
        let b = ClipBounds::new(5, 5, 10, 10);
        let c = a.intersect(&b).unwrap();
        assert_eq!(c.x, 5);
        assert_eq!(c.y, 5);
        assert_eq!(c.width, 5);
        assert_eq!(c.height, 5);
    }

    #[test]
    fn paint_flags_bitflags() {
        let flags = PaintFlags::BACKGROUND | PaintFlags::TEXT;
        assert!(flags.contains(PaintFlags::BACKGROUND));
        assert!(flags.contains(PaintFlags::TEXT));
        assert!(!flags.contains(PaintFlags::BORDER));
    }

    #[test]
    fn paint_context_clip_stack() {
        let mut ctx = PaintContext::new(80, 24);
        assert!(ctx.current_clip().is_none());
        ctx.push_clip(ClipBounds::new(0, 0, 80, 24));
        assert!(ctx.current_clip().is_some());
        ctx.pop_clip();
        assert!(ctx.current_clip().is_none());
    }

    #[test]
    fn paint_context_visibility() {
        let mut ctx = PaintContext::new(80, 24);
        let bounds = PaintBounds::new(5, 5, 10, 10);
        assert!(ctx.is_visible(&bounds));
        ctx.push_clip(ClipBounds::new(0, 0, 8, 8));
        assert!(ctx.is_visible(&bounds));
        ctx.pop_clip();
        let outside = PaintBounds::new(50, 50, 10, 10);
        ctx.push_clip(ClipBounds::new(0, 0, 8, 8));
        assert!(!ctx.is_visible(&outside));
        ctx.pop_clip();
    }
}
