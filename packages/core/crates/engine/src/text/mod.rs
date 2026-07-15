//! Text engine: buffer, cursor, selection, search, undo, unicode utilities, wrapping, viewport, and styled text.

mod buffer;
mod cursor;
mod edit;
mod search;
mod selection;
mod styled;
mod undo;
mod unicode;
mod viewport;
mod wrap;

pub use buffer::TextBuffer;
pub use cursor::{Cursor, CursorPosition};
pub use edit::{CursorStyle, EditBuffer, EditBufferConfig};
pub use search::{SearchEngine, SearchOptions, SearchResult};
pub use selection::{Selection, SelectionRange};
pub use styled::{StyledSpan, StyledText};
pub use undo::{UndoAction, UndoManager};
pub use unicode::{
    byte_offset_to_display_width, char_width, display_width, display_width_to_byte_offset, grapheme_clusters,
    grapheme_count, grapheme_width, is_box_drawing, is_emoji, is_nerd_font_glyph, is_powerline, is_wide_char,
    is_zero_width, truncate_to_width, truncate_with_ellipsis,
};
pub use viewport::{TextAlign, TextViewport, ViewportConfig, ViewportLine, layout_text};
pub use wrap::{WrapMode, WrappedLine, wrap_text};

pub struct TextEngine {
    buffer: TextBuffer,
    cursor: Cursor,
    selection: Selection,
    undo_manager: UndoManager,
    search_engine: SearchEngine,
}

impl Default for TextEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TextEngine {
    pub fn new() -> Self {
        Self {
            buffer: TextBuffer::new(),
            cursor: Cursor::new(),
            selection: Selection::new(),
            undo_manager: UndoManager::new(),
            search_engine: SearchEngine::new(),
        }
    }

    pub fn with_text(text: &str) -> Self {
        Self {
            buffer: TextBuffer::with_text(text),
            cursor: Cursor::new(),
            selection: Selection::new(),
            undo_manager: UndoManager::new(),
            search_engine: SearchEngine::new(),
        }
    }

    pub fn buffer(&self) -> &TextBuffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut TextBuffer {
        &mut self.buffer
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    pub fn undo_manager(&self) -> &UndoManager {
        &self.undo_manager
    }

    pub fn insert_char(&mut self, ch: char) {
        let pos = self.cursor.position();
        let action = UndoAction::InsertChar { pos, ch };
        self.undo_manager.push(action);
        self.buffer.insert_char(pos, ch);
        self.cursor.move_right();
        self.selection.clear();
    }

    pub fn insert_str(&mut self, s: &str) {
        let pos = self.cursor.position();
        let action = UndoAction::InsertStr { pos, text: s.to_string() };
        self.undo_manager.push(action);
        self.buffer.insert_str(pos, s);
        self.cursor.move_right_by(s.len());
        self.selection.clear();
    }

    pub fn delete_char(&mut self) {
        if self.cursor.position() > 0 {
            let pos = self.cursor.position() - 1;
            let ch = self.buffer.char_at(pos);
            let action = UndoAction::DeleteChar { pos, ch };
            self.undo_manager.push(action);
            self.buffer.delete_char(pos);
            self.cursor.move_left();
            self.selection.clear();
        }
    }

    pub fn delete_range(&mut self, range: SelectionRange) {
        let text = self.buffer.substring(range.start, range.end);
        let action = UndoAction::DeleteRange { range, text: text.clone() };
        self.undo_manager.push(action);
        self.buffer.delete_range(range.start, range.end);
        self.cursor.set_position(range.start);
        self.selection.clear();
    }

    pub fn undo(&mut self) -> bool {
        if let Some(action) = self.undo_manager.undo() {
            match action {
                UndoAction::InsertChar { pos, .. } => {
                    self.buffer.delete_char(pos);
                    self.cursor.set_position(pos);
                }
                UndoAction::InsertStr { pos, text } => {
                    self.buffer.delete_range(pos, pos + text.len());
                    self.cursor.set_position(pos);
                }
                UndoAction::DeleteChar { pos, ch } => {
                    self.buffer.insert_char(pos, ch);
                    self.cursor.set_position(pos + 1);
                }
                UndoAction::DeleteRange { range, text } => {
                    self.buffer.insert_str(range.start, &text);
                    self.cursor.set_position(range.end);
                }
            }
            self.selection.clear();
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some(action) = self.undo_manager.redo() {
            match action {
                UndoAction::InsertChar { pos, ch } => {
                    self.buffer.insert_char(pos, ch);
                    self.cursor.set_position(pos + 1);
                }
                UndoAction::InsertStr { pos, text } => {
                    self.buffer.insert_str(pos, &text);
                    self.cursor.set_position(pos + text.len());
                }
                UndoAction::DeleteChar { pos, .. } => {
                    self.buffer.delete_char(pos);
                    self.cursor.set_position(pos);
                }
                UndoAction::DeleteRange { range, .. } => {
                    self.buffer.delete_range(range.start, range.end);
                    self.cursor.set_position(range.start);
                }
            }
            self.selection.clear();
            true
        } else {
            false
        }
    }

    pub fn search(&mut self, pattern: &str, options: SearchOptions) -> Vec<SearchResult> {
        self.search_engine.search(&self.buffer, pattern, options)
    }

    pub fn replace(&mut self, pattern: &str, replacement: &str, options: SearchOptions) -> usize {
        let results = self.search_engine.search(&self.buffer, pattern, options);
        let mut count = 0;

        for result in results.into_iter().rev() {
            self.buffer.delete_range(result.range.start, result.range.end);
            self.buffer.insert_str(result.range.start, replacement);
            count += 1;
        }

        count
    }

    pub fn line_count(&self) -> usize {
        self.buffer.line_count()
    }

    pub fn line(&self, line: usize) -> Option<String> {
        self.buffer.line(line)
    }

    pub fn line_length(&self, line: usize) -> Option<usize> {
        self.buffer.line_length(line)
    }

    pub fn char_count(&self) -> usize {
        self.buffer.char_count()
    }

    pub fn word_count(&self) -> usize {
        self.buffer.word_count()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.cursor.reset();
        self.selection.clear();
        self.undo_manager.clear();
    }

    pub fn text(&self) -> String {
        self.buffer.to_string()
    }

    pub fn line_to_char(&self, line: usize) -> usize {
        self.buffer.line_to_char(line)
    }

    pub fn char_to_line(&self, char_idx: usize) -> usize {
        self.buffer.char_to_line(char_idx)
    }
}
