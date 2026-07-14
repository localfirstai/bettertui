use unicode_segmentation::UnicodeSegmentation;

use bettertui_engine::input::{Event, EventResult, Key};
use bettertui_engine::layout::LayoutProps;
use bettertui_engine::text::{display_width, grapheme_count};
use bettertui_engine::tree::NodeId;
use bettertui_engine::tree::Style;
use bettertui_engine::tree::{Color, NamedColor};

use crate::callback_types::{ChangeCallback, SubmitCallback};
use crate::{Widget, WidgetContext, WidgetId};

/// Single-line text input widget.
///
/// The actual value is stored in arena node `attributes["_value"]`.
/// Display text (placeholder/password-masked) is stored in `text`.
/// Cursor byte offset is stored in `state.content_width`.
pub struct InputWidget {
    pub placeholder: Box<str>,
    pub value: Box<str>,
    pub password: bool,
    pub disabled: bool,
    pub style: Style,
    pub layout: LayoutProps,
    pub on_change: Option<ChangeCallback>,
    pub on_submit: Option<SubmitCallback>,
}

impl Default for InputWidget {
    fn default() -> Self {
        Self {
            placeholder: Box::from(""),
            value: Box::from(""),
            password: false,
            disabled: false,
            style: Style::default(),
            layout: LayoutProps::default(),
            on_change: None,
            on_submit: None,
        }
    }
}

impl InputWidget {
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

    pub fn with_password(mut self, password: bool) -> Self {
        self.password = password;
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

    pub fn on_submit(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_submit = Some(Box::new(handler));
        self
    }

    fn mask_password(value: &str) -> String {
        let count = grapheme_count(value);
        "*".repeat(count)
    }

    fn display_text(value: &str, placeholder: &str, password: bool) -> Box<str> {
        if value.is_empty() && !placeholder.is_empty() {
            placeholder.into()
        } else if password {
            Self::mask_password(value).into()
        } else {
            value.into()
        }
    }

    fn display_style(value: &str, base: Style) -> Style {
        if value.is_empty() {
            Style {
                fg: Some(Color::Named(NamedColor::BrightBlack)),
                ..base
            }
        } else {
            base
        }
    }

    fn read_value(ctx: &WidgetContext, id: NodeId) -> String {
        ctx.arena
            .get(id)
            .and_then(|n| n.attributes.get("_value").cloned())
            .unwrap_or_default()
    }

    fn write_value(ctx: &mut WidgetContext, id: NodeId, value: &str, display: Box<str>) {
        if let Some(node) = ctx.arena.get_mut(id) {
            node.attributes.insert("_value".into(), value.to_string());
        }
        ctx.set_text(id, display);
    }

    fn read_cursor(ctx: &WidgetContext, id: NodeId) -> usize {
        ctx.arena
            .get(id)
            .map(|n| n.state.content_width as usize)
            .unwrap_or(0)
    }

    fn set_cursor_position(
        ctx: &mut WidgetContext,
        id: NodeId,
        value: &str,
        byte_offset: usize,
        password: bool,
    ) {
        use bettertui_engine::tree::{CursorProps, CursorStyle, Point};

        let clamped = byte_offset.min(value.len());
        let visual_x = if password {
            grapheme_count(&value[..clamped]) as u16
        } else {
            display_width(&value[..clamped]) as u16
        };

        if let Some(node) = ctx.arena.get_mut(id) {
            node.state.content_width = clamped as u32;
            node.cursor = Some(CursorProps {
                style: CursorStyle::Block,
                blink: true,
                position: Some(Point { x: visual_x, y: 0 }),
            });
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
}

impl Widget for InputWidget {
    fn kind(&self) -> &'static str {
        "Input"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        use std::collections::HashMap;

        let display = Self::display_text(&self.value, &self.placeholder, self.password);
        let style = Self::display_style(&self.value, self.style);

        let mut attrs = HashMap::new();
        attrs.insert("_value".into(), self.value.to_string());

        let node = bettertui_engine::tree::RenderNode {
            kind: bettertui_engine::tree::NodeKind::Input,
            text: Some(display),
            style,
            layout: self.layout,
            attributes: attrs,
            ..bettertui_engine::tree::RenderNode::default()
        };
        let id = ctx.insert_node(node);
        Self::set_cursor_position(ctx, id, &self.value, self.value.len(), self.password);
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
                        let display = Self::display_text(&value, &self.placeholder, self.password);
                        Self::write_value(ctx, nid, &value, display);
                        Self::set_cursor_position(ctx, nid, &value, cursor, self.password);
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
                            let display =
                                Self::display_text(&value, &self.placeholder, self.password);
                            Self::write_value(ctx, nid, &value, display);
                            Self::set_cursor_position(ctx, nid, &value, cursor, self.password);
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
                            let display =
                                Self::display_text(&value, &self.placeholder, self.password);
                            Self::write_value(ctx, nid, &value, display);
                            Self::set_cursor_position(ctx, nid, &value, cursor, self.password);
                            ctx.request_frame();
                        }
                        EventResult::Consumed
                    }
                    Key::ArrowLeft => {
                        cursor = Self::cursor_left(&value, cursor);
                        Self::set_cursor_position(ctx, nid, &value, cursor, self.password);
                        ctx.request_frame();
                        EventResult::Consumed
                    }
                    Key::ArrowRight => {
                        cursor = Self::cursor_right(&value, cursor);
                        Self::set_cursor_position(ctx, nid, &value, cursor, self.password);
                        ctx.request_frame();
                        EventResult::Consumed
                    }
                    Key::Home => {
                        cursor = 0;
                        Self::set_cursor_position(ctx, nid, &value, cursor, self.password);
                        ctx.request_frame();
                        EventResult::Consumed
                    }
                    Key::End => {
                        cursor = value.len();
                        Self::set_cursor_position(ctx, nid, &value, cursor, self.password);
                        ctx.request_frame();
                        EventResult::Consumed
                    }
                    Key::Enter => {
                        if let Some(ref handler) = self.on_submit {
                            handler(&value);
                        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use bettertui_engine::input::FocusManager;
    use bettertui_engine::input::KeyEvent;
    use bettertui_engine::scheduler::Scheduler;
    use bettertui_engine::tree::NodeArena;
    use bettertui_engine::tree::NodeKind;

    fn make_ctx() -> (NodeArena, FocusManager, Scheduler, Theme) {
        (
            NodeArena::new(),
            FocusManager::new(),
            Scheduler::new(),
            Theme::default(),
        )
    }

    fn create_input<'a>(
        w: &InputWidget,
        arena: &'a mut NodeArena,
        focus: &'a mut FocusManager,
        sched: &'a mut Scheduler,
        theme: &'a Theme,
    ) -> (WidgetId, WidgetContext<'a>) {
        let mut ctx = WidgetContext {
            arena,
            focus_manager: focus,
            scheduler: sched,
            terminal_size: (80, 24),
            theme,
        };
        let id = w.create(&mut ctx);
        (id, ctx)
    }

    fn key_event(key: Key) -> Event {
        Event::Key(KeyEvent::new(key, NodeId::default()))
    }

    #[test]
    fn input_widget_kind() {
        let w = InputWidget::new();
        assert_eq!(w.kind(), "Input");
    }

    #[test]
    fn input_widget_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = InputWidget::new().with_placeholder("Enter text...");
        let id = w.create(&mut ctx);
        let node = ctx.arena.get(id.node_id()).unwrap();
        assert_eq!(node.kind, NodeKind::Input);
        assert_eq!(node.text.as_deref(), Some("Enter text..."));
    }

    #[test]
    fn input_widget_create_with_value() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let w = InputWidget::new().with_value("hi");
        let (id, ctx) = create_input(&w, &mut arena, &mut focus, &mut sched, &theme);
        assert_eq!(
            ctx.arena.get(id.node_id()).unwrap().text.as_deref(),
            Some("hi")
        );
        assert_eq!(InputWidget::read_value(&ctx, id.node_id()), "hi");
        assert_eq!(InputWidget::read_cursor(&ctx, id.node_id()), 2);
    }

    #[test]
    fn input_widget_with_value() {
        let w = InputWidget::new().with_value("hello");
        assert_eq!(w.value.as_ref(), "hello");
    }

    #[test]
    fn input_widget_password() {
        let w = InputWidget::new().with_password(true);
        assert!(w.password);
    }

    #[test]
    fn input_widget_disabled() {
        let w = InputWidget::new().with_disabled(true);
        assert!(w.disabled);
    }

    #[test]
    fn password_masking() {
        assert_eq!(InputWidget::mask_password("hello"), "*****");
        assert_eq!(InputWidget::mask_password(""), "");
        assert_eq!(InputWidget::mask_password("héllo"), "*****");
    }

    #[test]
    fn cursor_left_right_basic() {
        assert_eq!(InputWidget::cursor_left("hello", 3), 2);
        assert_eq!(InputWidget::cursor_right("hello", 3), 4);
    }

    #[test]
    fn cursor_left_right_cjk() {
        let text = "a\u{4e2d}c";
        assert_eq!(InputWidget::cursor_left(text, 4), 1);
        assert_eq!(InputWidget::cursor_right(text, 1), 4);
    }

    #[test]
    fn cursor_left_boundary() {
        assert_eq!(InputWidget::cursor_left("hello", 0), 0);
        assert_eq!(InputWidget::cursor_right("hello", 5), 5);
    }

    #[test]
    fn clamp_cursor() {
        assert_eq!(InputWidget::clamp_cursor("hello", 10), 5);
        assert_eq!(InputWidget::clamp_cursor("hello", 3), 3);
    }

    #[test]
    fn character_insert_at_cursor() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let w = InputWidget::new().with_value("ab");
        let (id, mut ctx) = create_input(&w, &mut arena, &mut focus, &mut sched, &theme);

        // Move cursor to position 1
        InputWidget::set_cursor_position(&mut ctx, id.node_id(), "ab", 1, false);

        // Insert 'X' at cursor
        let result = w.handle_event(id, &mut ctx, &key_event(Key::Character('X')));
        assert_eq!(result, EventResult::Consumed);
        assert_eq!(InputWidget::read_value(&ctx, id.node_id()), "aXb");
        assert_eq!(InputWidget::read_cursor(&ctx, id.node_id()), 2);
    }

    #[test]
    fn backspace_at_cursor() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let w = InputWidget::new().with_value("abc");
        let (id, mut ctx) = create_input(&w, &mut arena, &mut focus, &mut sched, &theme);

        // Move cursor to position 2
        InputWidget::set_cursor_position(&mut ctx, id.node_id(), "abc", 2, false);

        // Backspace
        let result = w.handle_event(id, &mut ctx, &key_event(Key::Backspace));
        assert_eq!(result, EventResult::Consumed);
        assert_eq!(InputWidget::read_value(&ctx, id.node_id()), "ac");
        assert_eq!(InputWidget::read_cursor(&ctx, id.node_id()), 1);
    }

    #[test]
    fn delete_at_cursor() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let w = InputWidget::new().with_value("abc");
        let (id, mut ctx) = create_input(&w, &mut arena, &mut focus, &mut sched, &theme);

        // Move cursor to position 1
        InputWidget::set_cursor_position(&mut ctx, id.node_id(), "abc", 1, false);

        // Delete
        let result = w.handle_event(id, &mut ctx, &key_event(Key::Delete));
        assert_eq!(result, EventResult::Consumed);
        assert_eq!(InputWidget::read_value(&ctx, id.node_id()), "ac");
        assert_eq!(InputWidget::read_cursor(&ctx, id.node_id()), 1);
    }

    #[test]
    fn arrow_left_right() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let w = InputWidget::new().with_value("abc");
        let (id, mut ctx) = create_input(&w, &mut arena, &mut focus, &mut sched, &theme);

        // Start at end (cursor=3)
        let result = w.handle_event(id, &mut ctx, &key_event(Key::ArrowLeft));
        assert_eq!(result, EventResult::Consumed);
        assert_eq!(InputWidget::read_cursor(&ctx, id.node_id()), 2);

        w.handle_event(id, &mut ctx, &key_event(Key::ArrowLeft));
        assert_eq!(InputWidget::read_cursor(&ctx, id.node_id()), 1);

        w.handle_event(id, &mut ctx, &key_event(Key::ArrowRight));
        assert_eq!(InputWidget::read_cursor(&ctx, id.node_id()), 2);

        w.handle_event(id, &mut ctx, &key_event(Key::ArrowRight));
        assert_eq!(InputWidget::read_cursor(&ctx, id.node_id()), 3);
    }

    #[test]
    fn home_end() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let w = InputWidget::new().with_value("abc");
        let (id, mut ctx) = create_input(&w, &mut arena, &mut focus, &mut sched, &theme);

        w.handle_event(id, &mut ctx, &key_event(Key::Home));
        assert_eq!(InputWidget::read_cursor(&ctx, id.node_id()), 0);

        w.handle_event(id, &mut ctx, &key_event(Key::End));
        assert_eq!(InputWidget::read_cursor(&ctx, id.node_id()), 3);
    }

    #[test]
    fn disabled_ignores_events() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let w = InputWidget::new().with_disabled(true).with_value("abc");
        let (id, mut ctx) = create_input(&w, &mut arena, &mut focus, &mut sched, &theme);

        let result = w.handle_event(id, &mut ctx, &key_event(Key::Character('X')));
        assert_eq!(result, EventResult::Ignored);
        assert_eq!(InputWidget::read_value(&ctx, id.node_id()), "abc");
    }

    use crate::theme::Theme;
}
