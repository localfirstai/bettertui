//! Screen state: cursor tracking, alternate screen mode, and scroll regions.

use crate::process::TerminalViewport;
use crate::scrollback::ScrollbackBuffer;
use bettertui_engine::framebuffer::Cell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlternateScreen {
    Main,
    Alternate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorStyle {
    Block,
    Underline,
    Bar,
    Hidden,
}

/// Cursor position, visibility, and rendering style within a screen.
#[derive(Debug, Clone)]
pub struct CursorState {
    x: u16,
    y: u16,
    visible: bool,
    style: CursorStyle,
}

impl Default for CursorState {
    fn default() -> Self {
        Self::new()
    }
}

impl CursorState {
    pub fn new() -> Self {
        Self { x: 0, y: 0, visible: true, style: CursorStyle::Block }
    }

    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn y(&self) -> u16 {
        self.y
    }

    pub fn position(&self) -> (u16, u16) {
        (self.x, self.y)
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn style(&self) -> CursorStyle {
        self.style
    }

    pub fn set_position(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn set_style(&mut self, style: CursorStyle) {
        self.style = style;
    }
}

/// High-level screen state: viewport, alternate screen mode, cursor,
/// scrollback buffer, and selection tracking.
#[derive(Debug, Clone)]
pub struct ScreenState {
    viewport: TerminalViewport,
    alternate_screen: AlternateScreen,
    cursor: CursorState,
    scrollback: ScrollbackBuffer,
    selection_active: bool,
    selection_start: Option<(u16, u16)>,
    selection_end: Option<(u16, u16)>,
    dirty: bool,
}

impl Default for ScreenState {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenState {
    pub fn new() -> Self {
        Self {
            viewport: TerminalViewport::new(),
            alternate_screen: AlternateScreen::Main,
            cursor: CursorState::new(),
            scrollback: ScrollbackBuffer::new(),
            selection_active: false,
            selection_start: None,
            selection_end: None,
            dirty: true,
        }
    }

    pub fn with_size(cols: u16, rows: u16) -> Self {
        Self { viewport: TerminalViewport::with_size(cols, rows), ..Self::new() }
    }

    // ── Public accessors ──

    pub fn viewport(&self) -> &TerminalViewport {
        &self.viewport
    }

    pub fn cursor(&self) -> &CursorState {
        &self.cursor
    }

    pub fn cursor_mut(&mut self) -> &mut CursorState {
        &mut self.cursor
    }

    pub fn scrollback(&self) -> &ScrollbackBuffer {
        &self.scrollback
    }

    pub fn alternate_screen(&self) -> AlternateScreen {
        self.alternate_screen
    }

    pub fn is_alternate_screen(&self) -> bool {
        self.alternate_screen == AlternateScreen::Alternate
    }

    pub fn selection_active(&self) -> bool {
        self.selection_active
    }

    pub fn selection_start(&self) -> Option<(u16, u16)> {
        self.selection_start
    }

    pub fn selection_end(&self) -> Option<(u16, u16)> {
        self.selection_end
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    // ── Public mutators ──

    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.viewport.resize(cols, rows);
        self.scrollback.resize(cols);
        self.dirty = true;
    }

    pub fn enter_alternate_screen(&mut self) {
        self.alternate_screen = AlternateScreen::Alternate;
        self.cursor = CursorState::new();
        self.dirty = true;
    }

    pub fn leave_alternate_screen(&mut self) {
        self.alternate_screen = AlternateScreen::Main;
        self.cursor = CursorState::new();
        self.dirty = true;
    }

    pub fn scroll_up(&mut self, lines: u32) {
        self.viewport.scroll_up(lines);
    }

    pub fn scroll_down(&mut self, lines: u32) {
        self.viewport.scroll_down(lines);
    }

    pub fn scroll_reset(&mut self) {
        self.viewport.scroll_reset();
    }

    pub fn set_selection(&mut self, start: (u16, u16), end: (u16, u16)) {
        self.selection_active = true;
        self.selection_start = Some(start);
        self.selection_end = Some(end);
        self.dirty = true;
    }

    pub fn clear_selection(&mut self) {
        self.selection_active = false;
        self.selection_start = None;
        self.selection_end = None;
        self.dirty = true;
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn push_scrollback_line(&mut self, cells: Vec<Cell>, width: u16, wrapped: bool) {
        self.scrollback.push_line(cells, width, wrapped);
    }

    pub fn buffer_size(&self) -> (u16, u16) {
        (self.viewport.cols(), self.viewport.rows())
    }

    pub fn total_cells(&self) -> u32 {
        self.viewport.total_cells()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_state_new() {
        let state = ScreenState::new();
        assert_eq!(state.viewport().cols(), 80);
        assert!(!state.is_alternate_screen());
        assert!(state.is_dirty());
    }

    #[test]
    fn screen_state_default() {
        let state = ScreenState::default();
        assert_eq!(state.viewport().rows(), 24);
    }

    #[test]
    fn screen_state_with_size() {
        let state = ScreenState::with_size(120, 40);
        assert_eq!(state.viewport().cols(), 120);
        assert_eq!(state.viewport().rows(), 40);
    }

    #[test]
    fn screen_state_resize() {
        let mut state = ScreenState::new();
        state.resize(100, 30);
        assert_eq!(state.viewport().cols(), 100);
        assert_eq!(state.buffer_size(), (100, 30));
    }

    #[test]
    fn screen_state_alternate() {
        let mut state = ScreenState::new();
        state.enter_alternate_screen();
        assert!(state.is_alternate_screen());
        state.leave_alternate_screen();
        assert!(!state.is_alternate_screen());
    }

    #[test]
    fn screen_state_selection() {
        let mut state = ScreenState::new();
        state.set_selection((0, 0), (5, 5));
        assert!(state.selection_active());
        state.clear_selection();
        assert!(!state.selection_active());
    }

    #[test]
    fn screen_state_dirty() {
        let mut state = ScreenState::new();
        state.clear_dirty();
        assert!(!state.is_dirty());
        state.mark_dirty();
        assert!(state.is_dirty());
    }

    #[test]
    fn screen_state_scroll() {
        let mut state = ScreenState::new();
        state.scroll_up(5);
        assert!(state.viewport().is_scrolled());
        state.scroll_reset();
        assert!(!state.viewport().is_scrolled());
    }

    #[test]
    fn screen_state_total_cells() {
        let state = ScreenState::with_size(80, 25);
        assert_eq!(state.total_cells(), 2000);
    }

    #[test]
    fn screen_state_push_scrollback() {
        let mut state = ScreenState::new();
        state.push_scrollback_line(vec![Cell::new('A')], 80, false);
        assert_eq!(state.scrollback().len(), 1);
    }

    #[test]
    fn cursor_state_new() {
        let cursor = CursorState::new();
        assert_eq!(cursor.x(), 0);
        assert_eq!(cursor.y(), 0);
        assert!(cursor.visible());
    }

    #[test]
    fn cursor_state_hide_show() {
        let mut cursor = CursorState::new();
        cursor.hide();
        assert!(!cursor.visible());
        cursor.show();
        assert!(cursor.visible());
    }

    #[test]
    fn cursor_state_style() {
        let mut cursor = CursorState::new();
        cursor.set_style(CursorStyle::Bar);
        assert_eq!(cursor.style(), CursorStyle::Bar);
    }

    #[test]
    fn cursor_state_position() {
        let mut cursor = CursorState::new();
        cursor.set_position(3, 7);
        assert_eq!(cursor.position(), (3, 7));
    }

    #[test]
    fn alternate_screen_equality() {
        assert_eq!(AlternateScreen::Main, AlternateScreen::Main);
        assert_ne!(AlternateScreen::Main, AlternateScreen::Alternate);
    }
}
