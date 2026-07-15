use unicode_segmentation::UnicodeSegmentation;

use bettertui_engine::input::{Event, EventResult, Key};
use bettertui_engine::taffy::{LayoutProps, Sizing};
use bettertui_engine::text::display_width;
use bettertui_engine::tree::NodeId;
use bettertui_engine::tree::Style;
use bettertui_engine::tree::{Color, NamedColor};

use crate::callback_types::ChangeCallback;
use crate::{Widget, WidgetContext, WidgetId};

/// Multi-line text input widget.
///
/// Actual value stored in arena node `attributes["_value"]`.
/// Cursor byte offset stored in `state.content_width`.
/// Visual cursor position set in `cursor`.
pub struct TextareaWidget {
    pub placeholder: Box<str>,
    pub value: Box<str>,
    pub rows: u16,
    pub disabled: bool,
    pub style: Style,
    pub layout: LayoutProps,
    pub on_change: Option<ChangeCallback>,
}

impl Default for TextareaWidget {
    fn default() -> Self {
        Self {
            placeholder: Box::from(""),
            value: Box::from(""),
            rows: 3,
            disabled: false,
            style: Style::default(),
            layout: LayoutProps::default(),
            on_change: None,
        }
    }
}

impl TextareaWidget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<Box<str>>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_value(mut self, value: impl Into<Box<str>>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_rows(mut self, rows: u16) -> Self {
        self.rows = rows;
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_layout(mut self, layout: LayoutProps) -> Self {
        self.layout = layout;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    fn display_text(value: &str, placeholder: &str) -> Box<str> {
        if value.is_empty() && !placeholder.is_empty() { placeholder.into() } else { value.into() }
    }

    fn display_style(value: &str, base: Style) -> Style {
        if value.is_empty() { Style { fg: Some(Color::Named(NamedColor::BrightBlack)), ..base } } else { base }
    }

    fn read_value(ctx: &WidgetContext, id: NodeId) -> String {
        ctx.arena.get(id).and_then(|n| n.attributes.get("_value").cloned()).unwrap_or_default()
    }

    fn write_value(ctx: &mut WidgetContext, id: NodeId, value: &str, display: Box<str>) {
        if let Some(node) = ctx.arena.get_mut(id) {
            node.attributes.insert("_value".into(), value.to_string());
        }
        ctx.set_text(id, display);
    }

    fn read_cursor(ctx: &WidgetContext, id: NodeId) -> usize {
        ctx.arena.get(id).map(|n| n.state.content_width as usize).unwrap_or(0)
    }

    fn set_cursor_position(ctx: &mut WidgetContext, id: NodeId, value: &str, byte_offset: usize) {
        use bettertui_engine::tree::{CursorProps, CursorStyle};
        let pos = cursor_to_point(value, byte_offset);
        if let Some(node) = ctx.arena.get_mut(id) {
            node.state.content_width = byte_offset as u32;
            node.cursor = Some(CursorProps { style: CursorStyle::Block, blink: true, position: Some(pos) });
            node.state.mark_render_dirty();
            ctx.arena.mark_changed();
        }
    }

    fn clamp_cursor(value: &str, cursor: usize) -> usize {
        cursor.min(value.len())
    }

    fn cursor_left(value: &str, cursor: usize) -> usize {
        let indices: Vec<usize> = value.grapheme_indices(true).map(|(i, _)| i).collect();
        let mut prev = 0;
        for &i in &indices {
            if i >= cursor {
                return prev;
            }
            prev = i;
        }
        prev
    }

    fn cursor_right(value: &str, cursor: usize) -> usize {
        for (i, g) in value.grapheme_indices(true) {
            if i >= cursor {
                return i + g.len();
            }
        }
        value.len()
    }

    fn line_starts(value: &str) -> Vec<usize> {
        let mut starts = vec![0usize];
        for (i, ch) in value.char_indices() {
            if ch == '\n' {
                starts.push(i + 1);
            }
        }
        starts
    }

    fn cursor_up(value: &str, cursor: usize) -> usize {
        let lines = Self::line_starts(value);
        let current_line = lines.iter().rposition(|&s| s <= cursor).unwrap_or(0);
        if current_line == 0 {
            return 0;
        }
        let line_start = lines[current_line];
        let prev_line_start = lines[current_line - 1];
        let col_visual = display_width(&value[line_start..cursor]);
        let prev_line = &value[prev_line_start..line_start - 1];
        let byte_in_prev = Self::visual_to_byte(prev_line, col_visual);
        prev_line_start + byte_in_prev
    }

    fn cursor_down(value: &str, cursor: usize) -> usize {
        let lines = Self::line_starts(value);
        let current_line = lines.iter().rposition(|&s| s <= cursor).unwrap_or(0);
        if current_line + 1 >= lines.len() {
            return value.len();
        }
        let col_visual = display_width(&value[lines[current_line]..cursor]);
        let next_line_start = lines[current_line + 1];
        let next_line_end = if current_line + 2 < lines.len() { lines[current_line + 2] - 1 } else { value.len() };
        let next_line = &value[next_line_start..next_line_end];
        let byte_in_next = Self::visual_to_byte(next_line, col_visual);
        next_line_start + byte_in_next
    }

    fn home(value: &str, cursor: usize) -> usize {
        let lines = Self::line_starts(value);
        for &s in lines.iter().rev() {
            if s <= cursor {
                return s;
            }
        }
        0
    }

    fn end(value: &str, cursor: usize) -> usize {
        let lines = Self::line_starts(value);
        let current_line = lines.iter().rposition(|&s| s <= cursor).unwrap_or(0);
        if current_line + 1 < lines.len() { lines[current_line + 1] - 1 } else { value.len() }
    }

    /// Convert visual column to byte offset within a line (no newlines).
    fn visual_to_byte(line: &str, target_visual: usize) -> usize {
        let mut visual = 0usize;
        for (i, g) in line.grapheme_indices(true) {
            let w = display_width(g);
            if visual + w > target_visual {
                return i;
            }
            visual += w;
        }
        line.len()
    }
}

impl Widget for TextareaWidget {
    fn kind(&self) -> &'static str {
        "Textarea"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        use std::collections::HashMap;

        let display = Self::display_text(&self.value, &self.placeholder);
        let style = Self::display_style(&self.value, self.style);

        let mut layout = self.layout;
        if layout.height.is_none() {
            layout.height = Some(Sizing::Points(self.rows as f32));
        }

        let mut attrs = HashMap::new();
        attrs.insert("_value".into(), self.value.to_string());

        let node = bettertui_engine::tree::RenderNode {
            kind: bettertui_engine::tree::NodeKind::Input,
            text: Some(display),
            style,
            layout,
            attributes: attrs,
            ..bettertui_engine::tree::RenderNode::default()
        };
        let id = ctx.insert_node(node);
        Self::set_cursor_position(ctx, id, &self.value, self.value.len());
        ctx.set_focusable(id, true);
        WidgetId(id)
    }

    fn handle_event(&self, id: WidgetId, ctx: &mut WidgetContext, event: &Event) -> EventResult {
        if self.disabled {
            return EventResult::Ignored;
        }

        match event {
            Event::Key(key_event) => {
                let nid = id.node_id();
                let mut value = Self::read_value(ctx, nid);
                let mut cursor = Self::clamp_cursor(&value, Self::read_cursor(ctx, nid));

                match key_event.key {
                    Key::Character(c) => {
                        value.insert(cursor, c);
                        cursor += c.len_utf8();
                        if let Some(ref handler) = self.on_change {
                            handler(&value);
                        }
                        let display = Self::display_text(&value, &self.placeholder);
                        Self::write_value(ctx, nid, &value, display);
                        Self::set_cursor_position(ctx, nid, &value, cursor);
                        ctx.request_frame();
                        EventResult::Consumed
                    }
                    Key::Backspace => {
                        if cursor > 0 {
                            let prev = Self::cursor_left(&value, cursor);
                            value.replace_range(prev..cursor, "");
                            cursor = prev;
                            if let Some(ref handler) = self.on_change {
                                handler(&value);
                            }
                            let display = Self::display_text(&value, &self.placeholder);
                            Self::write_value(ctx, nid, &value, display);
                            Self::set_cursor_position(ctx, nid, &value, cursor);
                            ctx.request_frame();
                        }
                        EventResult::Consumed
                    }
                    Key::Delete => {
                        if cursor < value.len() {
                            let next = Self::cursor_right(&value, cursor);
                            value.replace_range(cursor..next, "");
                            if let Some(ref handler) = self.on_change {
                                handler(&value);
                            }
                            let display = Self::display_text(&value, &self.placeholder);
                            Self::write_value(ctx, nid, &value, display);
                            Self::set_cursor_position(ctx, nid, &value, cursor);
                            ctx.request_frame();
                        }
                        EventResult::Consumed
                    }
                    Key::ArrowLeft => {
                        cursor = Self::cursor_left(&value, cursor);
                        Self::set_cursor_position(ctx, nid, &value, cursor);
                        ctx.request_frame();
                        EventResult::Consumed
                    }
                    Key::ArrowRight => {
                        cursor = Self::cursor_right(&value, cursor);
                        Self::set_cursor_position(ctx, nid, &value, cursor);
                        ctx.request_frame();
                        EventResult::Consumed
                    }
                    Key::ArrowUp => {
                        cursor = Self::cursor_up(&value, cursor);
                        Self::set_cursor_position(ctx, nid, &value, cursor);
                        ctx.request_frame();
                        EventResult::Consumed
                    }
                    Key::ArrowDown => {
                        cursor = Self::cursor_down(&value, cursor);
                        Self::set_cursor_position(ctx, nid, &value, cursor);
                        ctx.request_frame();
                        EventResult::Consumed
                    }
                    Key::Home => {
                        cursor = Self::home(&value, cursor);
                        Self::set_cursor_position(ctx, nid, &value, cursor);
                        ctx.request_frame();
                        EventResult::Consumed
                    }
                    Key::End => {
                        cursor = Self::end(&value, cursor);
                        Self::set_cursor_position(ctx, nid, &value, cursor);
                        ctx.request_frame();
                        EventResult::Consumed
                    }
                    Key::Enter => {
                        value.insert(cursor, '\n');
                        cursor += 1;
                        if let Some(ref handler) = self.on_change {
                            handler(&value);
                        }
                        let display = Self::display_text(&value, &self.placeholder);
                        Self::write_value(ctx, nid, &value, display);
                        Self::set_cursor_position(ctx, nid, &value, cursor);
                        ctx.request_frame();
                        EventResult::Consumed
                    }
                    Key::Escape => EventResult::Consumed,
                    _ => EventResult::Ignored,
                }
            }
            _ => EventResult::Ignored,
        }
    }
}

fn cursor_to_point(value: &str, byte_offset: usize) -> bettertui_engine::tree::Point {
    use bettertui_engine::tree::Point;
    let mut visual_x = 0u16;
    let mut y = 0u16;
    let mut byte_pos = 0;
    for line in value.split('\n') {
        let line_end = byte_pos + line.len();
        if byte_offset <= line_end {
            let slice = &value[byte_pos..byte_offset];
            visual_x = display_width(slice) as u16;
            break;
        }
        byte_pos = line_end + 1; // +1 for '\n'
        y = y.saturating_add(1);
    }
    Point { x: visual_x, y }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;
    use bettertui_engine::input::FocusManager;
    use bettertui_engine::input::KeyEvent;
    use bettertui_engine::scheduler::Scheduler;
    use bettertui_engine::tree::NodeArena;
    use bettertui_engine::tree::NodeKind;

    fn make_ctx() -> (NodeArena, FocusManager, Scheduler, Theme) {
        (NodeArena::new(), FocusManager::new(), Scheduler::new(), Theme::default())
    }

    fn create_input<'a>(
        w: &TextareaWidget,
        arena: &'a mut NodeArena,
        focus: &'a mut FocusManager,
        sched: &'a mut Scheduler,
        theme: &'a Theme,
    ) -> (WidgetId, WidgetContext<'a>) {
        let mut ctx = WidgetContext { arena, focus_manager: focus, scheduler: sched, terminal_size: (80, 24), theme };
        let id = w.create(&mut ctx);
        (id, ctx)
    }

    fn key_event(key: Key) -> Event {
        Event::Key(KeyEvent::new(key, NodeId::default()))
    }

    #[test]
    fn textarea_widget_kind() {
        let w = TextareaWidget::new();
        assert_eq!(w.kind(), "Textarea");
    }

    #[test]
    fn textarea_widget_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = TextareaWidget::new().with_placeholder("Enter text...");
        let id = w.create(&mut ctx);
        let node = ctx.arena.get(id.node_id()).expect("Node missing from arena");
        assert_eq!(node.kind, NodeKind::Input);
        assert_eq!(node.text.as_deref(), Some("Enter text..."));
    }

    #[test]
    fn textarea_widget_create_with_value() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let w = TextareaWidget::new().with_value("hello");
        let (id, ctx) = create_input(&w, &mut arena, &mut focus, &mut sched, &theme);
        assert_eq!(TextareaWidget::read_value(&ctx, id.node_id()), "hello");
    }

    #[test]
    fn textarea_widget_with_rows() {
        let w = TextareaWidget::new().with_rows(5);
        assert_eq!(w.rows, 5);
    }

    #[test]
    fn textarea_widget_with_value() {
        let w = TextareaWidget::new().with_value("hello");
        assert_eq!(w.value.as_ref(), "hello");
    }

    #[test]
    fn textarea_widget_disabled() {
        let w = TextareaWidget::new().with_disabled(true);
        assert!(w.disabled);
    }

    #[test]
    fn character_insert_at_cursor() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let w = TextareaWidget::new().with_value("ab");
        let (id, mut ctx) = create_input(&w, &mut arena, &mut focus, &mut sched, &theme);

        TextareaWidget::set_cursor_position(&mut ctx, id.node_id(), "ab", 1);
        let result = w.handle_event(id, &mut ctx, &key_event(Key::Character('X')));
        assert_eq!(result, EventResult::Consumed);
        assert_eq!(TextareaWidget::read_value(&ctx, id.node_id()), "aXb");
        assert_eq!(TextareaWidget::read_cursor(&ctx, id.node_id()), 2);
    }

    #[test]
    fn backspace_at_cursor() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let w = TextareaWidget::new().with_value("abc");
        let (id, mut ctx) = create_input(&w, &mut arena, &mut focus, &mut sched, &theme);

        TextareaWidget::set_cursor_position(&mut ctx, id.node_id(), "abc", 2);
        w.handle_event(id, &mut ctx, &key_event(Key::Backspace));
        assert_eq!(TextareaWidget::read_value(&ctx, id.node_id()), "ac");
        assert_eq!(TextareaWidget::read_cursor(&ctx, id.node_id()), 1);
    }

    #[test]
    fn delete_at_cursor() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let w = TextareaWidget::new().with_value("abc");
        let (id, mut ctx) = create_input(&w, &mut arena, &mut focus, &mut sched, &theme);

        TextareaWidget::set_cursor_position(&mut ctx, id.node_id(), "abc", 1);
        w.handle_event(id, &mut ctx, &key_event(Key::Delete));
        assert_eq!(TextareaWidget::read_value(&ctx, id.node_id()), "ac");
        assert_eq!(TextareaWidget::read_cursor(&ctx, id.node_id()), 1);
    }

    #[test]
    fn enter_inserts_newline() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let w = TextareaWidget::new().with_value("ab");
        let (id, mut ctx) = create_input(&w, &mut arena, &mut focus, &mut sched, &theme);

        TextareaWidget::set_cursor_position(&mut ctx, id.node_id(), "ab", 1);
        w.handle_event(id, &mut ctx, &key_event(Key::Enter));
        assert_eq!(TextareaWidget::read_value(&ctx, id.node_id()), "a\nb");
        assert_eq!(TextareaWidget::read_cursor(&ctx, id.node_id()), 2);
    }

    #[test]
    fn arrow_up_down() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let w = TextareaWidget::new().with_value("line1\nline2\nline3");
        let (id, mut ctx) = create_input(&w, &mut arena, &mut focus, &mut sched, &theme);

        // Start at end (cursor at 'line3' end, pos 17)
        let initial = TextareaWidget::read_cursor(&ctx, id.node_id());
        assert_eq!(initial, 17);

        // ArrowUp to line2
        w.handle_event(id, &mut ctx, &key_event(Key::ArrowUp));
        let cursor2 = TextareaWidget::read_cursor(&ctx, id.node_id());
        assert_eq!(cursor2, 11); // end of "line2"

        // ArrowUp to line1
        w.handle_event(id, &mut ctx, &key_event(Key::ArrowUp));
        let cursor1 = TextareaWidget::read_cursor(&ctx, id.node_id());
        assert_eq!(cursor1, 5); // end of "line1"

        // ArrowDown back to line2
        w.handle_event(id, &mut ctx, &key_event(Key::ArrowDown));
        assert_eq!(TextareaWidget::read_cursor(&ctx, id.node_id()), 11);

        // ArrowDown to line3
        w.handle_event(id, &mut ctx, &key_event(Key::ArrowDown));
        assert_eq!(TextareaWidget::read_cursor(&ctx, id.node_id()), 17);
    }

    #[test]
    fn home_end_multi_line() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let w = TextareaWidget::new().with_value("hello\nworld");
        let (id, mut ctx) = create_input(&w, &mut arena, &mut focus, &mut sched, &theme);

        // Start at end (cursor at 11)
        assert_eq!(TextareaWidget::read_cursor(&ctx, id.node_id()), 11);

        // Home should go to start of last line
        w.handle_event(id, &mut ctx, &key_event(Key::Home));
        assert_eq!(TextareaWidget::read_cursor(&ctx, id.node_id()), 6);

        // Up then Home goes to start of first line
        w.handle_event(id, &mut ctx, &key_event(Key::ArrowUp));
        w.handle_event(id, &mut ctx, &key_event(Key::Home));
        assert_eq!(TextareaWidget::read_cursor(&ctx, id.node_id()), 0);

        // End goes to end of first line
        w.handle_event(id, &mut ctx, &key_event(Key::End));
        assert_eq!(TextareaWidget::read_cursor(&ctx, id.node_id()), 5);
    }

    #[test]
    fn disabled_ignores_events() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let w = TextareaWidget::new().with_disabled(true).with_value("abc");
        let (id, mut ctx) = create_input(&w, &mut arena, &mut focus, &mut sched, &theme);

        let result = w.handle_event(id, &mut ctx, &key_event(Key::Character('X')));
        assert_eq!(result, EventResult::Ignored);
        assert_eq!(TextareaWidget::read_value(&ctx, id.node_id()), "abc");
    }

    #[test]
    fn cursor_to_point_conversion() {
        let p = cursor_to_point("hello", 0);
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);

        let p = cursor_to_point("hello", 5);
        assert_eq!(p.x, 5);
        assert_eq!(p.y, 0);

        let p = cursor_to_point("ab\ncd", 0);
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);

        let p = cursor_to_point("ab\ncd", 2);
        assert_eq!(p.x, 2);
        assert_eq!(p.y, 0);

        let p = cursor_to_point("ab\ncd", 3);
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 1);

        let p = cursor_to_point("ab\ncd", 5);
        assert_eq!(p.x, 2);
        assert_eq!(p.y, 1);
    }

    #[test]
    fn visual_to_byte_conversion() {
        assert_eq!(TextareaWidget::visual_to_byte("hello", 0), 0);
        assert_eq!(TextareaWidget::visual_to_byte("hello", 3), 3);
        assert_eq!(TextareaWidget::visual_to_byte("hello", 5), 5);
    }

    #[test]
    fn line_starts() {
        assert_eq!(TextareaWidget::line_starts("abc"), vec![0]);
        assert_eq!(TextareaWidget::line_starts("ab\ncd"), vec![0, 3]);
        assert_eq!(TextareaWidget::line_starts("\n"), vec![0, 1]);
        assert_eq!(TextareaWidget::line_starts(""), vec![0]);
    }
}
