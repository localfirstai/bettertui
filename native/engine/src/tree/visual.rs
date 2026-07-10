/// Controls whether a node is rendered and how it affects layout.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Visibility {
    /// Display mode. `None` = node removed from layout entirely.
    pub display: Display,
    /// Opacity multiplied with parent opacity during rendering.
    pub opacity: f32,
    /// Whether to clip children that overflow the node's bounds.
    pub clip: bool,
}

impl Default for Visibility {
    fn default() -> Self {
        Self {
            display: Display::Flex,
            opacity: 1.0,
            clip: false,
        }
    }
}

/// Display mode for visibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Display {
    #[default]
    Flex,
    None,
}

/// Visual offset and layer ordering without affecting layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Transform {
    /// Horizontal offset in terminal cells.
    pub translate_x: i32,
    /// Vertical offset in terminal cells.
    pub translate_y: i32,
    /// Layer ordering. Higher z-index renders on top.
    /// Equal z-index renders in tree order (depth-first).
    pub z_index: i32,
}

/// How content overflows the node's bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Overflow {
    /// Children render outside bounds (may overlap siblings).
    #[default]
    Visible,
    /// Children are clipped at bounds.
    Hidden,
    /// Children are clipped, scrollbar rendered, scroll offsets tracked.
    Scroll,
}

/// Cursor appearance and position for input nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CursorProps {
    pub style: CursorStyle,
    pub blink: bool,
    pub position: Option<Point>,
}

/// Terminal cursor style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorStyle {
    #[default]
    Block,
    Underline,
    Bar,
    None,
}

/// A 2D point in terminal cell coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

/// A 2D size in terminal cell units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

/// A rectangle in terminal cell coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns the right edge (x + width).
    pub fn right(&self) -> u16 {
        self.x + self.width
    }

    /// Returns the bottom edge (y + height).
    pub fn bottom(&self) -> u16 {
        self.y + self.height
    }

    /// Checks if a point is contained within this rectangle.
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x && point.x < self.right() && point.y >= self.y && point.y < self.bottom()
    }

    /// Checks if this rectangle intersects another.
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_visibility() {
        let vis = Visibility::default();
        assert_eq!(vis.display, Display::Flex);
        assert_eq!(vis.opacity, 1.0);
        assert!(!vis.clip);
    }

    #[test]
    fn default_transform() {
        let t = Transform::default();
        assert_eq!(t.translate_x, 0);
        assert_eq!(t.translate_y, 0);
        assert_eq!(t.z_index, 0);
    }

    #[test]
    fn point_creation() {
        let p = Point::new(10, 20);
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 20);
    }

    #[test]
    fn rect_contains() {
        let rect = Rect::new(5, 5, 10, 10);
        assert!(rect.contains(Point::new(7, 7)));
        assert!(rect.contains(Point::new(5, 5)));
        assert!(!rect.contains(Point::new(4, 5)));
        assert!(!rect.contains(Point::new(5, 4)));
        assert!(!rect.contains(Point::new(15, 15)));
    }

    #[test]
    fn rect_intersects() {
        let a = Rect::new(0, 0, 10, 10);
        let b = Rect::new(5, 5, 10, 10);
        let c = Rect::new(20, 20, 10, 10);
        assert!(a.intersects(&b));
        assert!(b.intersects(&a));
        assert!(!a.intersects(&c));
        assert!(!c.intersects(&a));
    }
}
