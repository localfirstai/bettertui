//! Scrollback buffer: ring buffer of terminal lines with configurable max size.

use bettertui_engine::framebuffer::Cell;
const DEFAULT_SCROLLBACK_LINES: usize = 10_000;

#[derive(Debug, Clone)]
pub struct ScrollbackLine {
    pub cells: Vec<Cell>,
    pub line_width: u16,
    pub is_wrapped: bool,
}

impl ScrollbackLine {
    pub fn new(width: u16) -> Self {
        Self {
            cells: Vec::with_capacity(width as usize),
            line_width: width,
            is_wrapped: false,
        }
    }

    pub fn with_cells(cells: Vec<Cell>, width: u16, wrapped: bool) -> Self {
        Self {
            cells,
            line_width: width,
            is_wrapped: wrapped,
        }
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn cell(&self, col: u16) -> Option<&Cell> {
        if col < self.cells.len() as u16 {
            Some(&self.cells[col as usize])
        } else {
            None
        }
    }

    pub fn text(&self) -> String {
        self.cells.iter().map(|c| c.ch).collect()
    }
}

#[derive(Debug, Clone)]
pub struct ScrollbackBuffer {
    lines: Vec<ScrollbackLine>,
    max_lines: usize,
    current_width: u16,
}

impl Default for ScrollbackBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl ScrollbackBuffer {
    pub fn new() -> Self {
        Self {
            lines: Vec::with_capacity(1024),
            max_lines: DEFAULT_SCROLLBACK_LINES,
            current_width: 80,
        }
    }

    pub fn with_max_lines(max_lines: usize) -> Self {
        Self {
            lines: Vec::with_capacity(max_lines.min(1024)),
            max_lines,
            current_width: 80,
        }
    }

    pub fn with_width(width: u16) -> Self {
        Self {
            lines: Vec::with_capacity(1024),
            max_lines: DEFAULT_SCROLLBACK_LINES,
            current_width: width,
        }
    }

    pub fn push_line(&mut self, cells: Vec<Cell>, width: u16, wrapped: bool) {
        let line = ScrollbackLine::with_cells(cells, width, wrapped);
        self.lines.push(line);

        if self.lines.len() > self.max_lines {
            let excess = self.lines.len() - self.max_lines;
            self.lines.drain(0..excess);
        }
    }

    pub fn push_text_line(&mut self, text: &str, width: u16) {
        let cells: Vec<Cell> = text.chars().map(Cell::new).collect();
        self.push_line(cells, width, false);
    }

    pub fn clear(&mut self) {
        self.lines.clear();
    }

    pub fn resize(&mut self, new_width: u16) {
        self.current_width = new_width;
        for line in &mut self.lines {
            line.line_width = new_width;
        }
    }

    pub fn set_max_lines(&mut self, max: usize) {
        self.max_lines = max;
        if self.lines.len() > max {
            self.lines.truncate(max);
        }
    }

    pub fn line(&self, index: usize) -> Option<&ScrollbackLine> {
        if index < self.lines.len() {
            Some(&self.lines[self.lines.len() - 1 - index])
        } else {
            None
        }
    }

    pub fn line_absolute(&self, index: usize) -> Option<&ScrollbackLine> {
        self.lines.get(index)
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn max_lines(&self) -> usize {
        self.max_lines
    }

    pub fn current_width(&self) -> u16 {
        self.current_width
    }

    pub fn visible_lines(&self, offset: u32, count: u16) -> Vec<&ScrollbackLine> {
        let count = count as usize;
        let start = offset as usize;
        let end = (start + count).min(self.lines.len());
        (start..end)
            .map(|i| &self.lines[self.lines.len() - 1 - i])
            .collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ScrollbackLine> {
        self.lines.iter()
    }

    pub fn estimated_memory(&self) -> usize {
        self.lines.len() * self.current_width as usize * std::mem::size_of::<Cell>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scrollback_new() {
        let sb = ScrollbackBuffer::new();
        assert!(sb.is_empty());
        assert_eq!(sb.len(), 0);
    }

    #[test]
    fn scrollback_default() {
        let sb = ScrollbackBuffer::default();
        assert!(sb.is_empty());
    }

    #[test]
    fn scrollback_with_max_lines() {
        let sb = ScrollbackBuffer::with_max_lines(5000);
        assert_eq!(sb.max_lines(), 5000);
    }

    #[test]
    fn scrollback_push_line() {
        let mut sb = ScrollbackBuffer::new();
        let cells = vec![Cell::new('H'), Cell::new('i')];
        sb.push_line(cells, 80, false);
        assert_eq!(sb.len(), 1);
    }

    #[test]
    fn scrollback_push_text_line() {
        let mut sb = ScrollbackBuffer::new();
        sb.push_text_line("Hello, World!", 80);
        assert_eq!(sb.len(), 1);
        let line = sb.line(0).unwrap();
        assert_eq!(line.text(), "Hello, World!");
    }

    #[test]
    fn scrollback_max_lines_enforced() {
        let mut sb = ScrollbackBuffer::with_max_lines(5);
        for i in 0..10 {
            sb.push_text_line(&format!("Line {}", i), 80);
        }
        assert_eq!(sb.len(), 5);
    }

    #[test]
    fn scrollback_line_from_bottom() {
        let mut sb = ScrollbackBuffer::new();
        sb.push_text_line("first", 80);
        sb.push_text_line("second", 80);
        sb.push_text_line("third", 80);

        assert_eq!(sb.line(0).unwrap().text(), "third");
        assert_eq!(sb.line(1).unwrap().text(), "second");
        assert_eq!(sb.line(2).unwrap().text(), "first");
    }

    #[test]
    fn scrollback_clear() {
        let mut sb = ScrollbackBuffer::new();
        sb.push_text_line("test", 80);
        sb.clear();
        assert!(sb.is_empty());
    }

    #[test]
    fn scrollback_resize() {
        let mut sb = ScrollbackBuffer::with_width(80);
        sb.resize(120);
        assert_eq!(sb.current_width(), 120);
    }

    #[test]
    fn scrollback_visible_lines() {
        let mut sb = ScrollbackBuffer::new();
        for i in 0..10 {
            sb.push_text_line(&format!("Line {}", i), 80);
        }
        let visible = sb.visible_lines(0, 3);
        assert_eq!(visible.len(), 3);
        assert_eq!(visible[0].text(), "Line 9");
        assert_eq!(visible[2].text(), "Line 7");
    }

    #[test]
    fn scrollback_line_cell() {
        let mut sb = ScrollbackBuffer::new();
        sb.push_text_line("Hi", 80);
        let line = sb.line(0).unwrap();
        assert_eq!(line.cell(0).map(|c| c.ch), Some('H'));
        assert_eq!(line.cell(1).map(|c| c.ch), Some('i'));
    }

    #[test]
    fn scrollback_estimated_memory() {
        let mut sb = ScrollbackBuffer::new();
        sb.push_text_line("test", 80);
        assert!(sb.estimated_memory() > 0);
    }

    #[test]
    fn scrollback_line_new() {
        let line = ScrollbackLine::new(80);
        assert!(line.is_empty());
        assert_eq!(line.line_width, 80);
    }
}
