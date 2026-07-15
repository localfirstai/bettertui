use crate::Widget;
use crate::WidgetId;
use crate::context::WidgetContext;
use bettertui_engine::input::{Event, EventResult, Key};
use bettertui_engine::taffy::LayoutProps;
use bettertui_engine::tree::Color;
use bettertui_engine::tree::Style;

pub struct PromptComposer {
    pub style: Style,
    pub cursor_style: Style,
    pub placeholder: Option<Box<str>>,
    pub max_lines: usize,
    pub history: Vec<Box<str>>,
    pub history_index: Option<usize>,
}

impl Default for PromptComposer {
    fn default() -> Self {
        Self {
            style: Style::default(),
            cursor_style: Style {
                fg: Some(Color::rgb(255, 255, 255)),
                bg: Some(Color::rgb(100, 100, 100)),
                ..Style::default()
            },
            placeholder: None,
            max_lines: 3,
            history: Vec::new(),
            history_index: None,
        }
    }
}

impl PromptComposer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<Box<str>>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_max_lines(mut self, max_lines: usize) -> Self {
        self.max_lines = max_lines;
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_cursor_style(mut self, style: Style) -> Self {
        self.cursor_style = style;
        self
    }

    pub fn render_content(&self, state: &ComposerState) -> String {
        let text = state.text.as_ref();
        let lines: Vec<&str> = text.lines().collect();
        let mut result = String::new();

        let count = lines.len().min(self.max_lines);
        for line in &lines[..count] {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(line);
        }

        if result.is_empty() {
            return self.placeholder.as_deref().unwrap_or("").to_string();
        }

        result
    }

    pub fn cursor_position(&self, state: &ComposerState) -> (u16, u16) {
        let text = state.text.as_ref();
        let cursor = state.cursor.min(text.len());
        let before_cursor = &text[..cursor];
        let line = before_cursor.lines().count().saturating_sub(1) as u16;
        let col = before_cursor.lines().last().map_or(0, |l| l.len()) as u16;
        (col, line)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ComposerState {
    pub text: Box<str>,
    pub cursor: usize,
    pub selection_start: Option<usize>,
    pub selection_end: Option<usize>,
    pub undo_stack: Vec<Box<str>>,
    pub redo_stack: Vec<Box<str>>,
    pub clipboard: Option<Box<str>>,
}

impl ComposerState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_char(&mut self, ch: char) {
        self.save_undo();
        let mut chars: Vec<char> = self.text.chars().collect();
        let pos = self.cursor.min(chars.len());
        chars.insert(pos, ch);
        self.text = chars.into_iter().collect();
        self.cursor += ch.len_utf8();
        self.clear_selection();
    }

    pub fn delete_char(&mut self) {
        if self.cursor > 0 {
            self.save_undo();
            let mut chars: Vec<char> = self.text.chars().collect();
            let pos = self.cursor.saturating_sub(1);
            if pos < chars.len() {
                chars.remove(pos);
                self.text = chars.into_iter().collect();
                self.cursor = pos;
            }
            self.clear_selection();
        }
    }

    pub fn delete_forward(&mut self) {
        let mut chars: Vec<char> = self.text.chars().collect();
        let pos = self.cursor;
        if pos < chars.len() {
            self.save_undo();
            chars.remove(pos);
            self.text = chars.into_iter().collect();
            self.clear_selection();
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
        self.clear_selection();
    }

    pub fn move_cursor_right(&mut self) {
        let max = self.text.len();
        if self.cursor < max {
            self.cursor += 1;
        }
        self.clear_selection();
    }

    pub fn move_cursor_up(&mut self) {
        let lines: Vec<&str> = self.text.lines().collect();
        if lines.is_empty() {
            return;
        }
        let before = &self.text[..self.cursor];
        let current_line_idx = before.lines().count().saturating_sub(1);
        if current_line_idx > 0 {
            let prev_line = lines[current_line_idx - 1];
            let current_col = before.lines().last().map_or(0, |l| l.len());
            let new_col = current_col.min(prev_line.len());
            let offset: usize = lines[..current_line_idx - 1].iter().map(|l| l.len() + 1).sum();
            self.cursor = offset + new_col;
        }
        self.clear_selection();
    }

    pub fn move_cursor_down(&mut self) {
        let lines: Vec<&str> = self.text.lines().collect();
        if lines.is_empty() {
            return;
        }
        let before = &self.text[..self.cursor];
        let current_line_idx = before.lines().count().saturating_sub(1);
        if current_line_idx < lines.len() - 1 {
            let next_line = lines[current_line_idx + 1];
            let current_col = before.lines().last().map_or(0, |l| l.len());
            let new_col = current_col.min(next_line.len());
            let offset: usize = lines[..=current_line_idx].iter().map(|l| l.len() + 1).sum();
            self.cursor = (offset + new_col).min(self.text.len());
        }
        self.clear_selection();
    }

    pub fn select_all(&mut self) {
        if !self.text.is_empty() {
            self.selection_start = Some(0);
            self.selection_end = Some(self.text.len());
            self.cursor = self.text.len();
        }
    }

    pub fn copy(&mut self) {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let s = start.min(end);
            let e = start.max(end);
            self.clipboard = Some(Box::from(&self.text.as_ref()[s..e]));
        }
    }

    pub fn cut(&mut self) {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let s = start.min(end);
            let e = start.max(end);
            self.clipboard = Some(Box::from(&self.text.as_ref()[s..e]));
            self.save_undo();
            let mut chars: Vec<char> = self.text.chars().collect();
            let char_start = self.text[..s].chars().count();
            let char_end = self.text[..e].chars().count();
            for _ in char_start..char_end {
                if char_start < chars.len() {
                    chars.remove(char_start);
                }
            }
            self.text = chars.into_iter().collect();
            self.cursor = s;
            self.clear_selection();
        }
    }

    pub fn paste(&mut self) {
        if let Some(clip) = self.clipboard.take() {
            self.save_undo();
            let clip_str = clip.as_ref();
            let mut chars: Vec<char> = self.text.chars().collect();
            let start_pos = self.cursor.min(chars.len());
            for (pos, ch) in (start_pos..).zip(clip_str.chars()) {
                chars.insert(pos, ch);
            }
            self.text = chars.into_iter().collect();
            self.cursor += clip_str.len();
            self.clear_selection();
            self.clipboard = Some(clip);
        }
    }

    pub fn undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(self.text.clone());
            self.text = prev;
            self.cursor = self.cursor.min(self.text.len());
            self.clear_selection();
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.text.clone());
            self.text = next;
            self.cursor = self.cursor.min(self.text.len());
            self.clear_selection();
        }
    }

    fn save_undo(&mut self) {
        self.undo_stack.push(self.text.clone());
        self.redo_stack.clear();
        if self.undo_stack.len() > 100 {
            self.undo_stack.remove(0);
        }
    }

    fn clear_selection(&mut self) {
        self.selection_start = None;
        self.selection_end = None;
    }
}

impl Widget for PromptComposer {
    fn kind(&self) -> &'static str {
        "PromptComposer"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let layout = LayoutProps::default();
        let id = ctx.make_box(layout, self.style);
        WidgetId(id)
    }

    fn handle_event(&self, _id: WidgetId, _ctx: &mut WidgetContext, event: &Event) -> EventResult {
        match event {
            Event::Key(key_event) => match key_event.key {
                Key::Character(_ch) => EventResult::Consumed,
                Key::Backspace => EventResult::Consumed,
                Key::Delete => EventResult::Consumed,
                Key::ArrowLeft => EventResult::Consumed,
                Key::ArrowRight => EventResult::Consumed,
                Key::ArrowUp => EventResult::Consumed,
                Key::ArrowDown => EventResult::Consumed,
                Key::Home => EventResult::Consumed,
                Key::End => EventResult::Consumed,
                Key::Enter => EventResult::Consumed,
                _ => EventResult::Ignored,
            },
            _ => EventResult::Ignored,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;
    use bettertui_engine::input::FocusManager;
    use bettertui_engine::scheduler::Scheduler;
    use bettertui_engine::tree::NodeArena;

    fn make_ctx() -> (NodeArena, FocusManager, Scheduler, Theme) {
        (NodeArena::new(), FocusManager::new(), Scheduler::new(), Theme::default())
    }

    #[test]
    fn composer_new() {
        let composer = PromptComposer::new();
        assert_eq!(composer.kind(), "PromptComposer");
        assert_eq!(composer.max_lines, 3);
    }

    #[test]
    fn composer_with_placeholder() {
        let composer = PromptComposer::new().with_placeholder("Type here...");
        assert_eq!(composer.placeholder.as_deref(), Some("Type here..."));
    }

    #[test]
    fn composer_with_max_lines() {
        let composer = PromptComposer::new().with_max_lines(5);
        assert_eq!(composer.max_lines, 5);
    }

    #[test]
    fn composer_render_empty() {
        let composer = PromptComposer::new().with_placeholder("Empty");
        let state = ComposerState::new();
        assert_eq!(composer.render_content(&state), "Empty");
    }

    #[test]
    fn composer_render_text() {
        let composer = PromptComposer::new();
        let mut state = ComposerState::new();
        state.text = Box::from("Hello");
        assert_eq!(composer.render_content(&state), "Hello");
    }

    #[test]
    fn composer_render_multiline() {
        let composer = PromptComposer::new().with_max_lines(2);
        let mut state = ComposerState::new();
        state.text = Box::from("Line1\nLine2\nLine3");
        assert_eq!(composer.render_content(&state), "Line1\nLine2");
    }

    #[test]
    fn composer_cursor_position() {
        let composer = PromptComposer::new();
        let mut state = ComposerState::new();
        state.text = Box::from("Hello\nWorld");
        state.cursor = 8;
        let (col, line) = composer.cursor_position(&state);
        assert_eq!(line, 1);
        assert_eq!(col, 2);
    }

    #[test]
    fn composer_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };
        let composer = PromptComposer::new();
        let id = composer.create(&mut ctx);
        assert!(ctx.arena.contains(id.node_id()));
    }

    #[test]
    fn state_insert_char() {
        let mut state = ComposerState::new();
        state.insert_char('H');
        state.insert_char('i');
        assert_eq!(state.text.as_ref(), "Hi");
        assert_eq!(state.cursor, 2);
    }

    #[test]
    fn state_delete_char() {
        let mut state = ComposerState::new();
        state.text = Box::from("Hi");
        state.cursor = 2;
        state.delete_char();
        assert_eq!(state.text.as_ref(), "H");
        assert_eq!(state.cursor, 1);
    }

    #[test]
    fn state_delete_forward() {
        let mut state = ComposerState::new();
        state.text = Box::from("Hi");
        state.cursor = 0;
        state.delete_forward();
        assert_eq!(state.text.as_ref(), "i");
        assert_eq!(state.cursor, 0);
    }

    #[test]
    fn state_move_cursor() {
        let mut state = ComposerState::new();
        state.text = Box::from("Hello");
        state.move_cursor_right();
        assert_eq!(state.cursor, 1);
        state.move_cursor_left();
        assert_eq!(state.cursor, 0);
    }

    #[test]
    fn state_select_all() {
        let mut state = ComposerState::new();
        state.text = Box::from("Hello");
        state.select_all();
        assert_eq!(state.selection_start, Some(0));
        assert_eq!(state.selection_end, Some(5));
    }

    #[test]
    fn state_copy_paste() {
        let mut state = ComposerState::new();
        state.text = Box::from("Hello");
        state.select_all();
        state.copy();
        state.cursor = 0;
        state.text = Box::from("");
        state.paste();
        assert_eq!(state.text.as_ref(), "Hello");
    }

    #[test]
    fn state_undo_redo() {
        let mut state = ComposerState::new();
        state.insert_char('A');
        state.insert_char('B');
        assert_eq!(state.text.as_ref(), "AB");
        state.undo();
        assert_eq!(state.text.as_ref(), "A");
        state.redo();
        assert_eq!(state.text.as_ref(), "AB");
    }

    #[test]
    fn state_move_up_down() {
        let mut state = ComposerState::new();
        state.text = Box::from("Line1\nLine2");
        state.cursor = 8;
        state.move_cursor_up();
        assert_eq!(state.cursor, 2);
        state.move_cursor_down();
        assert_eq!(state.cursor, 8);
    }

    #[test]
    fn state_cut() {
        let mut state = ComposerState::new();
        state.text = Box::from("Hello");
        state.select_all();
        state.cut();
        assert_eq!(state.text.as_ref(), "");
        assert_eq!(state.clipboard.as_deref(), Some("Hello"));
    }
}
