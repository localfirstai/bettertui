//! Graphics context for higher-level drawing operations.
//!
//! Provides a drawing API on top of the FrameBuffer with styled primitives,
//! text rendering, and geometric shapes.

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
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns true if the point is inside the rectangle.
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x
            && point.x < self.x.saturating_add(self.width)
            && point.y >= self.y
            && point.y < self.y.saturating_add(self.height)
    }

    /// Returns the right edge x coordinate.
    pub fn right(&self) -> u16 {
        self.x.saturating_add(self.width)
    }

    /// Returns the bottom edge y coordinate.
    pub fn bottom(&self) -> u16 {
        self.y.saturating_add(self.height)
    }
}

/// Drawing style for graphics operations.
#[derive(Debug, Clone, Default)]
pub struct DrawStyle {
    /// Foreground color.
    pub fg: Option<Color>,
    /// Background color.
    pub bg: Option<Color>,
    /// Text attributes.
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

/// Graphics context providing high-level drawing operations.
pub struct GraphicsContext<'a> {
    buffer: &'a mut FrameBuffer,
}

impl<'a> GraphicsContext<'a> {
    /// Creates a new GraphicsContext wrapping a FrameBuffer.
    pub fn new(buffer: &'a mut FrameBuffer) -> Self {
        Self { buffer }
    }

    /// Clears the entire buffer.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Clears a specific region.
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

    /// Draws a character at the given position.
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

    /// Draws a horizontal line.
    pub fn draw_hline(&mut self, x: u16, y: u16, width: u16, ch: char, style: &DrawStyle) {
        for i in 0..width {
            let px = x.saturating_add(i);
            if px >= self.buffer.width() {
                break;
            }
            self.draw_char(px, y, ch, style);
        }
    }

    /// Draws a vertical line.
    pub fn draw_vline(&mut self, x: u16, y: u16, height: u16, ch: char, style: &DrawStyle) {
        for i in 0..height {
            let py = y.saturating_add(i);
            if py >= self.buffer.height() {
                break;
            }
            self.draw_char(x, py, ch, style);
        }
    }

    /// Draws a rectangle outline.
    pub fn draw_rect(&mut self, rect: Rect, style: &DrawStyle) {
        if rect.width == 0 || rect.height == 0 {
            return;
        }
        // Top and bottom
        self.draw_hline(rect.x, rect.y, rect.width, '-', style);
        self.draw_hline(
            rect.x,
            rect.y.saturating_add(rect.height - 1),
            rect.width,
            '-',
            style,
        );
        // Left and right
        self.draw_vline(rect.x, rect.y, rect.height, '|', style);
        self.draw_vline(
            rect.x.saturating_add(rect.width - 1),
            rect.y,
            rect.height,
            '|',
            style,
        );
        // Corners
        self.draw_char(rect.x, rect.y, '+', style);
        self.draw_char(rect.x.saturating_add(rect.width - 1), rect.y, '+', style);
        self.draw_char(rect.x, rect.y.saturating_add(rect.height - 1), '+', style);
        self.draw_char(
            rect.x.saturating_add(rect.width - 1),
            rect.y.saturating_add(rect.height - 1),
            '+',
            style,
        );
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
        // Top
        self.draw_char(rect.x, rect.y, '\u{250C}', style); // ┌
        self.draw_hline(rect.x + 1, rect.y, rect.width - 2, '\u{2500}', style); // ─
        self.draw_char(rect.x + rect.width - 1, rect.y, '\u{2510}', style); // ┐

        // Bottom
        self.draw_char(rect.x, rect.y + rect.height - 1, '\u{2514}', style); // └
        self.draw_hline(
            rect.x + 1,
            rect.y + rect.height - 1,
            rect.width - 2,
            '\u{2500}',
            style,
        ); // ─
        self.draw_char(
            rect.x + rect.width - 1,
            rect.y + rect.height - 1,
            '\u{2518}',
            style,
        ); // ┘

        // Sides
        self.draw_vline(rect.x, rect.y + 1, rect.height - 2, '\u{2502}', style); // │
        self.draw_vline(
            rect.x + rect.width - 1,
            rect.y + 1,
            rect.height - 2,
            '\u{2502}',
            style,
        ); // │
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

    #[test]
    fn rect_contains() {
        let rect = Rect::new(5, 5, 10, 10);
        assert!(rect.contains(Point::new(5, 5)));
        assert!(rect.contains(Point::new(14, 14)));
        assert!(!rect.contains(Point::new(4, 5)));
        assert!(!rect.contains(Point::new(5, 15)));
    }

    #[test]
    fn rect_edges() {
        let rect = Rect::new(2, 3, 10, 5);
        assert_eq!(rect.right(), 12);
        assert_eq!(rect.bottom(), 8);
    }

    #[test]
    fn draw_style_chain() {
        let style = DrawStyle::new()
            .fg(Color::rgb(255, 0, 0))
            .bg(Color::rgb(0, 0, 0))
            .bold()
            .italic();
        assert!(style.fg.is_some());
        assert!(style.bg.is_some());
        assert!(style.attributes.contains(CellAttributes::BOLD));
        assert!(style.attributes.contains(CellAttributes::ITALIC));
    }

    #[test]
    fn graphics_clear() {
        let mut fb = FrameBuffer::new(10, 10);
        fb.set(0, 0, Cell::new('x'));
        let mut gfx = GraphicsContext::new(&mut fb);
        gfx.clear();
        assert!(gfx.buffer().get(0, 0).is_empty());
    }

    #[test]
    fn draw_char() {
        let mut fb = FrameBuffer::new(10, 10);
        let mut gfx = GraphicsContext::new(&mut fb);
        let style = DrawStyle::new().fg(Color::rgb(255, 0, 0));
        gfx.draw_char(0, 0, 'A', &style);
        assert_eq!(gfx.buffer().get(0, 0).ch, 'A');
    }

    #[test]
    fn draw_str() {
        let mut fb = FrameBuffer::new(10, 10);
        let mut gfx = GraphicsContext::new(&mut fb);
        let style = DrawStyle::new();
        gfx.draw_str(0, 0, "hello", &style);
        assert_eq!(gfx.buffer().get(0, 0).ch, 'h');
        assert_eq!(gfx.buffer().get(4, 0).ch, 'o');
    }

    #[test]
    fn draw_hline() {
        let mut fb = FrameBuffer::new(10, 10);
        let mut gfx = GraphicsContext::new(&mut fb);
        let style = DrawStyle::new();
        gfx.draw_hline(2, 0, 5, '-', &style);
        assert_eq!(gfx.buffer().get(2, 0).ch, '-');
        assert_eq!(gfx.buffer().get(6, 0).ch, '-');
        assert!(gfx.buffer().get(7, 0).is_empty());
    }

    #[test]
    fn fill_rect() {
        let mut fb = FrameBuffer::new(10, 10);
        let mut gfx = GraphicsContext::new(&mut fb);
        let style = DrawStyle::new();
        gfx.fill_rect(Rect::new(1, 1, 3, 3), '#', &style);
        assert_eq!(gfx.buffer().get(1, 1).ch, '#');
        assert_eq!(gfx.buffer().get(3, 3).ch, '#');
        assert!(gfx.buffer().get(0, 0).is_empty());
    }

    #[test]
    fn clear_rect() {
        let mut fb = FrameBuffer::new(10, 10);
        let mut gfx = GraphicsContext::new(&mut fb);
        let style = DrawStyle::new();
        gfx.fill_rect(Rect::new(0, 0, 10, 10), '#', &style);
        gfx.clear_rect(Rect::new(2, 2, 3, 3));
        assert_eq!(gfx.buffer().get(0, 0).ch, '#');
        assert!(gfx.buffer().get(2, 2).is_empty());
    }
}
