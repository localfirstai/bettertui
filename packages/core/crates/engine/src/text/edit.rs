//! EditBuffer component providing a complete text editing experience.
//!
//! Wraps TextEngine with editor-specific features: cursor management,
//! selection, undo/redo, line numbers, and input handling.

use super::{SelectionRange, TextEngine};

/// EditBuffer configuration.
#[derive(Debug, Clone)]
pub struct EditBufferConfig {
    /// Whether line numbers are displayed.
    pub line_numbers: bool,
    /// Whether word wrap is enabled.
    pub word_wrap: bool,
    /// Whether the editor is read-only.
    pub read_only: bool,
    /// Tab size in spaces.
    pub tab_size: usize,
    /// Whether to insert spaces instead of tabs.
    pub spaces_for_tabs: bool,
    /// Maximum undo levels.
    pub max_undo: usize,
}

impl Default for EditBufferConfig {
    fn default() -> Self {
        Self {
            line_numbers: true,
            word_wrap: false,
            read_only: false,
            tab_size: 4,
            spaces_for_tabs: true,
            max_undo: 1000,
        }
    }
}

/// The state of the cursor in the editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorStyle {
    /// Block cursor.
    #[default]
    Block,
    /// Underline cursor.
    Underline,
    /// Bar cursor.
    Bar,
}

/// A complete text editor component.
pub struct EditBuffer {
    /// The underlying text engine.
    engine: TextEngine,
    /// EditBuffer configuration.
    config: EditBufferConfig,
    /// Cursor style.
    cursor_style: CursorStyle,
    /// Whether the editor has been modified since last save.
    dirty: bool,
    /// Scroll offset (line).
    scroll_top: usize,
    /// Visible height in lines.
    visible_height: usize,
}

impl Default for EditBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl EditBuffer {
    /// Creates a new empty editor.
    pub fn new() -> Self {
        Self {
            engine: TextEngine::new(),
            config: EditBufferConfig::default(),
            cursor_style: CursorStyle::default(),
            dirty: false,
            scroll_top: 0,
            visible_height: 24,
        }
    }

    /// Creates an editor with custom configuration.
    pub fn with_config(config: EditBufferConfig) -> Self {
        Self {
            engine: TextEngine::new(),
            config,
            cursor_style: CursorStyle::default(),
            dirty: false,
            scroll_top: 0,
            visible_height: 24,
        }
    }

    /// Sets the visible height in lines.
    pub fn set_visible_height(&mut self, height: usize) {
        self.visible_height = height;
    }

    /// Returns the editor configuration.
    pub fn config(&self) -> &EditBufferConfig {
        &self.config
    }

    /// Returns a mutable reference to the editor configuration.
    pub fn config_mut(&mut self) -> &mut EditBufferConfig {
        &mut self.config
    }

    /// Sets the cursor style.
    pub fn set_cursor_style(&mut self, style: CursorStyle) {
        self.cursor_style = style;
    }

    /// Returns the current cursor style.
    pub fn cursor_style(&self) -> CursorStyle {
        self.cursor_style
    }

    /// Returns whether the editor has unsaved changes.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Marks the editor as saved.
    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }

    /// Inserts a character at the cursor position.
    pub fn insert_char(&mut self, ch: char) {
        if self.config.read_only {
            return;
        }
        self.engine.insert_char(ch);
        self.dirty = true;
    }

    /// Inserts a string at the cursor position.
    pub fn insert_str(&mut self, s: &str) {
        if self.config.read_only {
            return;
        }
        self.engine.insert_str(s);
        self.dirty = true;
    }

    /// Inserts a tab (spaces or tab character based on config).
    pub fn insert_tab(&mut self) {
        if self.config.read_only {
            return;
        }
        if self.config.spaces_for_tabs {
            let spaces = " ".repeat(self.config.tab_size);
            self.insert_str(&spaces);
        } else {
            self.insert_char('\t');
        }
    }

    /// Deletes the character before the cursor.
    pub fn delete_backward(&mut self) {
        if self.config.read_only {
            return;
        }
        self.engine.delete_char();
        self.dirty = true;
    }

    /// Deletes the character after the cursor.
    pub fn delete_forward(&mut self) {
        if self.config.read_only {
            return;
        }
        // Move right then delete backward
        self.engine.cursor_mut().move_right();
        self.engine.delete_char();
        self.dirty = true;
    }

    /// Deletes the word before the cursor.
    ///
    /// Deletes from the start of the previous word (per
    /// [`TextBuffer::word_boundary_left`]) through the cursor.
    pub fn delete_word_backward(&mut self) {
        if self.config.read_only {
            return;
        }
        let pos = self.engine.cursor().position();
        let start = self.engine.buffer().word_boundary_left(pos);
        if start < pos {
            self.engine.delete_range(SelectionRange::new(start, pos));
            self.dirty = true;
        }
    }

    /// Deletes the word after the cursor.
    ///
    /// Deletes from the cursor through the end of the next word (per
    /// [`TextBuffer::word_boundary_right`]).
    pub fn delete_word_forward(&mut self) {
        if self.config.read_only {
            return;
        }
        let pos = self.engine.cursor().position();
        let end = self.engine.buffer().word_boundary_right(pos);
        if end > pos {
            self.engine.delete_range(SelectionRange::new(pos, end));
            self.dirty = true;
        }
    }

    /// Moves the cursor to the start of the previous word.
    pub fn move_word_backward(&mut self) {
        let pos = self.engine.cursor().position();
        let target = self.engine.buffer().word_boundary_left(pos);
        self.engine.cursor_mut().set_position(target);
    }

    /// Moves the cursor past the end of the next word.
    pub fn move_word_forward(&mut self) {
        let pos = self.engine.cursor().position();
        let target = self.engine.buffer().word_boundary_right(pos);
        self.engine.cursor_mut().set_position(target);
    }

    /// Inserts a newline.
    pub fn newline(&mut self) {
        if self.config.read_only {
            return;
        }
        self.engine.insert_char('\n');
        self.dirty = true;
    }

    /// Undoes the last operation.
    pub fn undo(&mut self) -> bool {
        let result = self.engine.undo();
        if result {
            self.dirty = true;
        }
        result
    }

    /// Redoes the last undone operation.
    pub fn redo(&mut self) -> bool {
        let result = self.engine.redo();
        if result {
            self.dirty = true;
        }
        result
    }

    /// Returns the underlying text content.
    pub fn content(&self) -> String {
        self.engine.buffer().to_string()
    }

    /// Sets the editor content.
    pub fn set_content(&mut self, content: &str) {
        self.engine.clear();
        self.engine.insert_str(content);
        self.dirty = false;
    }

    /// Returns the number of lines.
    pub fn line_count(&self) -> usize {
        self.engine.line_count()
    }

    /// Returns the current cursor line.
    pub fn cursor_line(&self) -> usize {
        self.engine.cursor().position()
    }

    /// Scrolls to ensure the cursor is visible.
    pub fn ensure_cursor_visible(&mut self) {
        let cursor_line = self.cursor_line();
        if cursor_line < self.scroll_top {
            self.scroll_top = cursor_line;
        } else if cursor_line >= self.scroll_top + self.visible_height {
            self.scroll_top = cursor_line.saturating_sub(self.visible_height - 1);
        }
    }

    /// Returns the scroll offset.
    pub fn scroll_top(&self) -> usize {
        self.scroll_top
    }

    /// Returns the visible height.
    pub fn visible_height(&self) -> usize {
        self.visible_height
    }

    /// Returns a reference to the underlying text engine.
    pub fn engine(&self) -> &TextEngine {
        &self.engine
    }

    /// Returns a mutable reference to the underlying text engine.
    pub fn engine_mut(&mut self) -> &mut TextEngine {
        &mut self.engine
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get() {
        let mut editor = EditBuffer::new();
        editor.insert_str("hello");
        assert_eq!(editor.content(), "hello");
    }

    #[test]
    fn read_only() {
        let mut editor = EditBuffer::with_config(EditBufferConfig { read_only: true, ..Default::default() });
        editor.insert_str("hello");
        assert_eq!(editor.content(), "");
    }

    #[test]
    fn dirty_tracking() {
        let mut editor = EditBuffer::new();
        assert!(!editor.is_dirty());
        editor.insert_char('a');
        assert!(editor.is_dirty());
        editor.mark_saved();
        assert!(!editor.is_dirty());
    }

    #[test]
    fn undo_redo() {
        let mut editor = EditBuffer::new();
        editor.insert_str("hello");
        assert!(editor.undo());
        assert_eq!(editor.content(), "");
        assert!(editor.redo());
        assert_eq!(editor.content(), "hello");
    }

    #[test]
    fn tab_insertion() {
        let mut editor = EditBuffer::new();
        editor.insert_tab();
        assert_eq!(editor.content(), "    ");
    }

    #[test]
    fn tab_as_character() {
        let mut editor = EditBuffer::with_config(EditBufferConfig { spaces_for_tabs: false, ..Default::default() });
        editor.insert_tab();
        assert_eq!(editor.content(), "\t");
    }

    #[test]
    fn delete_backward() {
        let mut editor = EditBuffer::new();
        editor.insert_str("abc");
        editor.delete_backward();
        assert_eq!(editor.content(), "ab");
    }

    #[test]
    fn delete_forward() {
        let mut editor = EditBuffer::new();
        editor.insert_str("abc");
        editor.engine_mut().cursor_mut().move_to_start();
        editor.delete_forward();
        assert_eq!(editor.content(), "bc");
    }

    #[test]
    fn line_count() {
        let mut editor = EditBuffer::new();
        assert_eq!(editor.line_count(), 1);
        editor.insert_str("line1\nline2\nline3");
        assert_eq!(editor.line_count(), 3);
    }

    #[test]
    fn cursor_visibility() {
        let mut editor = EditBuffer::new();
        editor.set_visible_height(5);
        assert_eq!(editor.visible_height(), 5);
        editor.ensure_cursor_visible();
        assert_eq!(editor.scroll_top(), 0);
    }

    #[test]
    fn set_content() {
        let mut editor = EditBuffer::new();
        editor.insert_str("old");
        editor.set_content("new");
        assert_eq!(editor.content(), "new");
        assert!(!editor.is_dirty());
    }

    #[test]
    fn delete_word_backward_removes_whole_previous_word() {
        let mut editor = EditBuffer::new();
        editor.insert_str("one two");
        editor.delete_word_backward();
        assert_eq!(editor.content(), "one ");
        editor.move_word_backward();
        editor.delete_word_backward();
        assert_eq!(editor.content(), "");
    }

    #[test]
    fn delete_word_forward_removes_whole_next_word() {
        let mut editor = EditBuffer::new();
        editor.insert_str("say hello now");
        editor.engine_mut().cursor_mut().set_position("say ".len());
        editor.delete_word_forward();
        assert_eq!(editor.content(), "say  now");
    }

    #[test]
    fn move_word_navigates_between_words() {
        let mut editor = EditBuffer::new();
        editor.insert_str("alpha beta");
        editor.engine_mut().cursor_mut().move_to_start();
        editor.move_word_forward();
        assert_eq!(editor.engine().cursor().position(), "alpha".len());
        editor.move_word_backward();
        assert_eq!(editor.engine().cursor().position(), 0);
    }
}
