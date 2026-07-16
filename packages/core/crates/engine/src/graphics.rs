//! Graphics context for higher-level drawing operations.
//!
//! Provides a drawing API on top of [`FrameBuffer`] with styled primitives,
//! text rendering, and geometric shapes (lines, rectangles, boxes).

use crate::framebuffer::{Cell, CellAttributes, FrameBuffer};
use crate::tree::Color;

/// A 2D point for drawing operations.
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

/// A rectangle for drawing operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height }
    }

    /// Returns `true` if `point` is inside this rectangle.
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x
            && point.x < self.x.saturating_add(self.width)
            && point.y >= self.y
            && point.y < self.y.saturating_add(self.height)
    }

    /// Returns the right edge x coordinate (exclusive).
    pub fn right(&self) -> u16 {
        self.x.saturating_add(self.width)
    }

    /// Returns the bottom edge y coordinate (exclusive).
    pub fn bottom(&self) -> u16 {
        self.y.saturating_add(self.height)
    }
}

/// Drawing style for graphics operations.
#[derive(Debug, Clone, Default)]
pub struct DrawStyle {
    /// Foreground color (if `None`, uses `Color::Default`).
    pub fg: Option<Color>,
    /// Background color (if `None`, uses `Color::Default`).
    pub bg: Option<Color>,
    /// Text attributes (bold, italic, etc.).
    pub attributes: CellAttributes,
}

impl DrawStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    pub fn bold(mut self) -> Self {
        self.attributes |= CellAttributes::BOLD;
        self
    }

    pub fn italic(mut self) -> Self {
        self.attributes |= CellAttributes::ITALIC;
        self
    }

    pub fn underline(mut self) -> Self {
        self.attributes |= CellAttributes::UNDERLINE;
        self
    }
}

/// Graphics context providing high-level drawing operations on a [`FrameBuffer`].
///
/// Wraps a mutable reference to a frame buffer and provides convenience methods
/// for drawing characters, strings, lines, rectangles, and filled regions.
pub struct GraphicsContext<'a> {
    buffer: &'a mut FrameBuffer,
}

impl<'a> GraphicsContext<'a> {
    /// Creates a new graphics context wrapping a frame buffer.
    pub fn new(buffer: &'a mut FrameBuffer) -> Self {
        Self { buffer }
    }

    /// Clears the entire buffer.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Clears a specific rectangular region.
    pub fn clear_rect(&mut self, rect: Rect) {
        for y in rect.y..rect.bottom().min(self.buffer.height()) {
            for x in rect.x..rect.right().min(self.buffer.width()) {
                self.buffer.set(x, y, Cell::default());
            }
        }
    }

    /// Sets a single cell.
    pub fn set_cell(&mut self, x: u16, y: u16, cell: Cell) {
        if x < self.buffer.width() && y < self.buffer.height() {
            self.buffer.set(x, y, cell);
        }
    }

    /// Draws a character at the given position with the given style.
    pub fn draw_char(&mut self, x: u16, y: u16, ch: char, style: &DrawStyle) {
        let mut cell = Cell::new(ch);
        if let Some(fg) = &style.fg {
            cell = cell.with_fg(*fg);
        }
        if let Some(bg) = &style.bg {
            cell = cell.with_bg(*bg);
        }
        cell = cell.with_attrs(style.attributes);
        self.set_cell(x, y, cell);
    }

    /// Draws a string starting at the given position.
    pub fn draw_str(&mut self, x: u16, y: u16, text: &str, style: &DrawStyle) {
        for (i, ch) in text.chars().enumerate() {
            let px = x.saturating_add(i as u16);
            if px >= self.buffer.width() {
                break;
            }
            self.draw_char(px, y, ch, style);
        }
    }

    /// Draws a horizontal line of `width` characters.
    pub fn draw_hline(&mut self, x: u16, y: u16, width: u16, ch: char, style: &DrawStyle) {
        for i in 0..width {
            let px = x.saturating_add(i);
            if px >= self.buffer.width() {
                break;
            }
            self.draw_char(px, y, ch, style);
        }
    }

    /// Draws a vertical line of `height` characters.
    pub fn draw_vline(&mut self, x: u16, y: u16, height: u16, ch: char, style: &DrawStyle) {
        for i in 0..height {
            let py = y.saturating_add(i);
            if py >= self.buffer.height() {
                break;
            }
            self.draw_char(x, py, ch, style);
        }
    }

    /// Draws a rectangle outline using ASCII characters (`-`, `|`, `+`).
    pub fn draw_rect(&mut self, rect: Rect, style: &DrawStyle) {
        if rect.width == 0 || rect.height == 0 {
            return;
        }
        self.draw_hline(rect.x, rect.y, rect.width, '-', style);
        self.draw_hline(rect.x, rect.y.saturating_add(rect.height - 1), rect.width, '-', style);
        self.draw_vline(rect.x, rect.y, rect.height, '|', style);
        self.draw_vline(rect.x.saturating_add(rect.width - 1), rect.y, rect.height, '|', style);
        self.draw_char(rect.x, rect.y, '+', style);
        self.draw_char(rect.x.saturating_add(rect.width - 1), rect.y, '+', style);
        self.draw_char(rect.x, rect.y.saturating_add(rect.height - 1), '+', style);
        self.draw_char(rect.x.saturating_add(rect.width - 1), rect.y.saturating_add(rect.height - 1), '+', style);
    }

    /// Fills a rectangle with the given character.
    pub fn fill_rect(&mut self, rect: Rect, ch: char, style: &DrawStyle) {
        for y in rect.y..rect.bottom().min(self.buffer.height()) {
            for x in rect.x..rect.right().min(self.buffer.width()) {
                self.draw_char(x, y, ch, style);
            }
        }
    }

    /// Draws a box with Unicode box-drawing characters.
    pub fn draw_box(&mut self, rect: Rect, style: &DrawStyle) {
        if rect.width < 2 || rect.height < 2 {
            return;
        }
        self.draw_char(rect.x, rect.y, '\u{250C}', style);
        self.draw_hline(rect.x + 1, rect.y, rect.width - 2, '\u{2500}', style);
        self.draw_char(rect.x + rect.width - 1, rect.y, '\u{2510}', style);
        self.draw_char(rect.x, rect.y + rect.height - 1, '\u{2514}', style);
        self.draw_hline(rect.x + 1, rect.y + rect.height - 1, rect.width - 2, '\u{2500}', style);
        self.draw_char(rect.x + rect.width - 1, rect.y + rect.height - 1, '\u{2518}', style);
        self.draw_vline(rect.x, rect.y + 1, rect.height - 2, '\u{2502}', style);
        self.draw_vline(rect.x + rect.width - 1, rect.y + 1, rect.height - 2, '\u{2502}', style);
    }

    /// Returns a reference to the underlying buffer.
    pub fn buffer(&self) -> &FrameBuffer {
        self.buffer
    }

    /// Returns a mutable reference to the underlying buffer.
    pub fn buffer_mut(&mut self) -> &mut FrameBuffer {
        self.buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::CellAttributes;
    use crate::tree::NamedColor;

    #[test]
    fn point_new() {
        let p = Point::new(3, 5);
        assert_eq!(p.x, 3);
        assert_eq!(p.y, 5);
    }

    #[test]
    fn rect_contains_inside() {
        let r = Rect::new(2, 3, 10, 10);
        assert!(r.contains(Point::new(5, 5)));
    }

    #[test]
    fn rect_contains_edge() {
        let r = Rect::new(0, 0, 5, 5);
        assert!(r.contains(Point::new(0, 0)));
        assert!(!r.contains(Point::new(5, 5)));
    }

    #[test]
    fn rect_contains_outside() {
        let r = Rect::new(2, 3, 10, 10);
        assert!(!r.contains(Point::new(1, 3)));
        assert!(!r.contains(Point::new(2, 2)));
        assert!(!r.contains(Point::new(12, 13)));
    }

    #[test]
    fn rect_right_bottom() {
        let r = Rect::new(3, 5, 10, 20);
        assert_eq!(r.right(), 13);
        assert_eq!(r.bottom(), 25);
    }

    #[test]
    fn rect_with_zero_size() {
        let r = Rect::new(0, 0, 0, 0);
        assert!(!r.contains(Point::new(0, 0)));
    }

    #[test]
    fn draw_style_builder() {
        let style = DrawStyle::new()
            .fg(Color::Named(NamedColor::Red))
            .bg(Color::Named(NamedColor::Blue))
            .bold()
            .italic()
            .underline();
        assert_eq!(style.fg, Some(Color::Named(NamedColor::Red)));
        assert_eq!(style.bg, Some(Color::Named(NamedColor::Blue)));
        assert!(style.attributes.contains(CellAttributes::BOLD));
        assert!(style.attributes.contains(CellAttributes::ITALIC));
        assert!(style.attributes.contains(CellAttributes::UNDERLINE));
    }

    fn make_buffer(w: u16, h: u16) -> FrameBuffer {
        let mut buf = FrameBuffer::new(w, h);
        buf.clear();
        buf
    }

    #[test]
    fn graphics_clear_rect() {
        let mut buf = make_buffer(10, 10);
        buf.set(0, 0, Cell::new('X'));
        buf.set(9, 9, Cell::new('X'));
        let mut ctx = GraphicsContext::new(&mut buf);
        ctx.clear_rect(Rect::new(0, 0, 10, 10));
        assert_eq!(ctx.buffer().get(0, 0).ch, ' ');
    }

    #[test]
    fn graphics_draw_char() {
        let mut buf = make_buffer(10, 10);
        let style = DrawStyle::new().fg(Color::Named(NamedColor::Red));
        let mut ctx = GraphicsContext::new(&mut buf);
        ctx.draw_char(3, 4, '@', &style);
        let cell = ctx.buffer().get(3, 4);
        assert_eq!(cell.ch, '@');
    }

    #[test]
    fn graphics_draw_char_out_of_bounds() {
        let mut buf = make_buffer(5, 5);
        let mut ctx = GraphicsContext::new(&mut buf);
        // Should not panic
        ctx.draw_char(10, 10, 'X', &DrawStyle::new());
        assert_eq!(ctx.buffer().get(0, 0).ch, ' ');
    }

    #[test]
    fn graphics_draw_str() {
        let mut buf = make_buffer(20, 5);
        let style = DrawStyle::new().fg(Color::Named(NamedColor::Green));
        let mut ctx = GraphicsContext::new(&mut buf);
        ctx.draw_str(0, 0, "Hello", &style);
        assert_eq!(ctx.buffer().get(0, 0).ch, 'H');
        assert_eq!(ctx.buffer().get(4, 0).ch, 'o');
    }

    #[test]
    fn graphics_draw_str_truncated() {
        let mut buf = make_buffer(3, 5);
        let style = DrawStyle::new();
        let mut ctx = GraphicsContext::new(&mut buf);
        ctx.draw_str(0, 0, "Hello!", &style);
        assert_eq!(ctx.buffer().get(0, 0).ch, 'H');
        assert_eq!(ctx.buffer().get(2, 0).ch, 'l');
        // 'l' at buf[3] would be out of bounds
    }

    #[test]
    fn graphics_draw_hline() {
        let mut buf = make_buffer(10, 5);
        let mut ctx = GraphicsContext::new(&mut buf);
        ctx.draw_hline(2, 2, 5, '-', &DrawStyle::new());
        assert_eq!(ctx.buffer().get(2, 2).ch, '-');
        assert_eq!(ctx.buffer().get(6, 2).ch, '-');
    }

    #[test]
    fn graphics_draw_vline() {
        let mut buf = make_buffer(10, 10);
        let mut ctx = GraphicsContext::new(&mut buf);
        ctx.draw_vline(5, 1, 4, '|', &DrawStyle::new());
        assert_eq!(ctx.buffer().get(5, 1).ch, '|');
        assert_eq!(ctx.buffer().get(5, 4).ch, '|');
    }

    #[test]
    fn graphics_draw_rect() {
        let mut buf = make_buffer(10, 10);
        let mut ctx = GraphicsContext::new(&mut buf);
        ctx.draw_rect(Rect::new(1, 1, 5, 4), &DrawStyle::new());
        assert_eq!(ctx.buffer().get(1, 1).ch, '+');
        assert_eq!(ctx.buffer().get(5, 1).ch, '+');
        assert_eq!(ctx.buffer().get(1, 4).ch, '+');
        assert_eq!(ctx.buffer().get(5, 4).ch, '+');
        assert_eq!(ctx.buffer().get(3, 1).ch, '-');
        assert_eq!(ctx.buffer().get(1, 2).ch, '|');
    }

    #[test]
    fn graphics_draw_rect_zero_size() {
        let mut buf = make_buffer(10, 10);
        let mut ctx = GraphicsContext::new(&mut buf);
        // Should not panic
        ctx.draw_rect(Rect::new(0, 0, 0, 0), &DrawStyle::new());
        ctx.draw_rect(Rect::new(1, 1, 1, 0), &DrawStyle::new());
    }

    #[test]
    fn graphics_fill_rect() {
        let mut buf = make_buffer(5, 5);
        let mut ctx = GraphicsContext::new(&mut buf);
        ctx.fill_rect(Rect::new(0, 0, 3, 3), '#', &DrawStyle::new());
        for y in 0..3 {
            for x in 0..3 {
                assert_eq!(ctx.buffer().get(x, y).ch, '#');
            }
        }
    }

    #[test]
    fn graphics_draw_box_small() {
        let mut buf = make_buffer(5, 5);
        let mut ctx = GraphicsContext::new(&mut buf);
        ctx.draw_box(Rect::new(0, 0, 5, 3), &DrawStyle::new());
        // Corners with Unicode box-drawing
        assert_eq!(ctx.buffer().get(0, 0).ch, '\u{250C}');
        assert_eq!(ctx.buffer().get(4, 0).ch, '\u{2510}');
        assert_eq!(ctx.buffer().get(0, 2).ch, '\u{2514}');
        assert_eq!(ctx.buffer().get(4, 2).ch, '\u{2518}');
    }

    #[test]
    fn graphics_draw_box_too_small() {
        let mut buf = make_buffer(5, 5);
        let mut ctx = GraphicsContext::new(&mut buf);
        // width < 2 or height < 2: should no-op
        ctx.draw_box(Rect::new(0, 0, 1, 5), &DrawStyle::new());
        ctx.draw_box(Rect::new(0, 0, 5, 1), &DrawStyle::new());
        // No panic = pass
    }

    #[test]
    fn graphics_clear() {
        let mut buf = make_buffer(5, 5);
        let mut ctx = GraphicsContext::new(&mut buf);
        ctx.draw_char(0, 0, 'X', &DrawStyle::new());
        assert_eq!(ctx.buffer().get(0, 0).ch, 'X');
        ctx.clear();
        assert_eq!(ctx.buffer().get(0, 0).ch, ' ');
    }

    #[test]
    fn graphics_set_cell() {
        let mut buf = make_buffer(5, 5);
        let mut ctx = GraphicsContext::new(&mut buf);
        ctx.set_cell(2, 2, Cell::new('Z'));
        assert_eq!(ctx.buffer().get(2, 2).ch, 'Z');
    }

    #[test]
    fn graphics_set_cell_out_of_bounds() {
        let mut buf = make_buffer(3, 3);
        let mut ctx = GraphicsContext::new(&mut buf);
        ctx.set_cell(10, 10, Cell::new('Z'));
        // No panic = pass
    }

    #[test]
    fn graphics_buffer_access() {
        let mut buf = make_buffer(5, 5);
        let mut ctx = GraphicsContext::new(&mut buf);
        assert_eq!(ctx.buffer().width(), 5);
        ctx.buffer_mut().clear();
    }
}
