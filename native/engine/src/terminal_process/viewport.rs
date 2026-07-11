use crate::pty::PtySize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollMode {
    Fixed,
    Scrollable,
    Infinite,
}

#[derive(Debug, Clone)]
pub struct TerminalViewport {
    cols: u16,
    rows: u16,
    scroll_offset: u32,
    scrollback_lines: u32,
    scroll_mode: ScrollMode,
    pixel_width: u32,
    pixel_height: u32,
    cell_width: u16,
    cell_height: u16,
}

impl Default for TerminalViewport {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalViewport {
    pub fn new() -> Self {
        Self {
            cols: 80,
            rows: 24,
            scroll_offset: 0,
            scrollback_lines: 10000,
            scroll_mode: ScrollMode::Scrollable,
            pixel_width: 0,
            pixel_height: 0,
            cell_width: 1,
            cell_height: 1,
        }
    }

    pub fn with_size(cols: u16, rows: u16) -> Self {
        Self {
            cols,
            rows,
            scroll_offset: 0,
            scrollback_lines: 10000,
            scroll_mode: ScrollMode::Scrollable,
            pixel_width: 0,
            pixel_height: 0,
            cell_width: 1,
            cell_height: 1,
        }
    }

    pub fn cols(&self) -> u16 {
        self.cols
    }

    pub fn rows(&self) -> u16 {
        self.rows
    }

    pub fn scroll_offset(&self) -> u32 {
        self.scroll_offset
    }

    pub fn scrollback_lines(&self) -> u32 {
        self.scrollback_lines
    }

    pub fn scroll_mode(&self) -> ScrollMode {
        self.scroll_mode
    }

    pub fn pixel_width(&self) -> u32 {
        self.pixel_width
    }

    pub fn pixel_height(&self) -> u32 {
        self.pixel_height
    }

    pub fn cell_width(&self) -> u16 {
        self.cell_width
    }

    pub fn cell_height(&self) -> u16 {
        self.cell_height
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
    }

    pub fn resize_with_pixels(&mut self, cols: u16, rows: u16, pixel_w: u32, pixel_h: u32) {
        self.cols = cols;
        self.rows = rows;
        self.pixel_width = pixel_w;
        self.pixel_height = pixel_h;
    }

    pub fn set_cell_size(&mut self, width: u16, height: u16) {
        self.cell_width = width;
        self.cell_height = height;
    }

    pub fn set_scrollback_lines(&mut self, lines: u32) {
        self.scrollback_lines = lines;
    }

    pub fn set_scroll_mode(&mut self, mode: ScrollMode) {
        self.scroll_mode = mode;
    }

    pub fn scroll_up(&mut self, lines: u32) {
        if self.scroll_mode == ScrollMode::Fixed {
            return;
        }
        self.scroll_offset = self
            .scroll_offset
            .saturating_add(lines)
            .min(self.scrollback_lines);
    }

    pub fn scroll_down(&mut self, lines: u32) {
        if self.scroll_mode == ScrollMode::Fixed {
            return;
        }
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }

    pub fn scroll_reset(&mut self) {
        self.scroll_offset = 0;
    }

    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = self.scrollback_lines;
    }

    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
    }

    pub fn is_scrolled(&self) -> bool {
        self.scroll_offset > 0
    }

    pub fn visible_line_count(&self) -> u32 {
        self.rows as u32
    }

    pub fn total_line_count(&self) -> u32 {
        self.scrollback_lines + self.rows as u32
    }

    pub fn to_pty_size(&self) -> PtySize {
        PtySize {
            cols: self.cols,
            rows: self.rows,
            pixel_width: self.pixel_width,
            pixel_height: self.pixel_height,
        }
    }

    pub fn total_cells(&self) -> u32 {
        self.cols as u32 * self.rows as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_new() {
        let vp = TerminalViewport::new();
        assert_eq!(vp.cols(), 80);
        assert_eq!(vp.rows(), 24);
        assert!(!vp.is_scrolled());
    }

    #[test]
    fn viewport_default() {
        let vp = TerminalViewport::default();
        assert_eq!(vp.cols(), 80);
    }

    #[test]
    fn viewport_with_size() {
        let vp = TerminalViewport::with_size(120, 40);
        assert_eq!(vp.cols(), 120);
        assert_eq!(vp.rows(), 40);
    }

    #[test]
    fn viewport_resize() {
        let mut vp = TerminalViewport::new();
        vp.resize(100, 30);
        assert_eq!(vp.cols(), 100);
        assert_eq!(vp.rows(), 30);
    }

    #[test]
    fn viewport_scroll_up_down() {
        let mut vp = TerminalViewport::new();
        vp.scroll_up(5);
        assert_eq!(vp.scroll_offset(), 5);
        assert!(vp.is_scrolled());

        vp.scroll_down(2);
        assert_eq!(vp.scroll_offset(), 3);
    }

    #[test]
    fn viewport_scroll_reset() {
        let mut vp = TerminalViewport::new();
        vp.scroll_up(10);
        vp.scroll_reset();
        assert_eq!(vp.scroll_offset(), 0);
        assert!(!vp.is_scrolled());
    }

    #[test]
    fn viewport_scroll_to_top_bottom() {
        let mut vp = TerminalViewport::new();
        vp.scroll_to_top();
        assert_eq!(vp.scroll_offset(), vp.scrollback_lines());

        vp.scroll_to_bottom();
        assert_eq!(vp.scroll_offset(), 0);
    }

    #[test]
    fn viewport_fixed_mode() {
        let mut vp = TerminalViewport::new();
        vp.set_scroll_mode(ScrollMode::Fixed);
        vp.scroll_up(5);
        assert_eq!(vp.scroll_offset(), 0);
    }

    #[test]
    fn viewport_to_pty_size() {
        let mut vp = TerminalViewport::new();
        vp.resize_with_pixels(120, 40, 960, 640);
        let size = vp.to_pty_size();
        assert_eq!(size.cols, 120);
        assert_eq!(size.rows, 40);
        assert_eq!(size.pixel_width, 960);
        assert_eq!(size.pixel_height, 640);
    }

    #[test]
    fn viewport_total_cells() {
        let vp = TerminalViewport::with_size(80, 25);
        assert_eq!(vp.total_cells(), 2000);
    }

    #[test]
    fn viewport_scroll_mode() {
        let vp = TerminalViewport::new();
        assert_eq!(vp.scroll_mode(), ScrollMode::Scrollable);
    }

    #[test]
    fn viewport_visible_line_count() {
        let vp = TerminalViewport::with_size(80, 30);
        assert_eq!(vp.visible_line_count(), 30);
    }
}
