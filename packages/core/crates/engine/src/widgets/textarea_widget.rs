use crate::events::types::{Event, EventResult, Key};
use crate::tree::color::{Color, NamedColor};
use crate::tree::layout::LayoutProps;
use crate::tree::style::Style;

use super::callback_types::ChangeCallback;
use super::{Widget, WidgetContext, WidgetId};

/// Multi-line text input widget.
///
/// Handles keyboard input, cursor movement, and multi-line text editing.
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
}

impl Widget for TextareaWidget {
    fn kind(&self) -> &'static str {
        "Textarea"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let display_text = if self.value.is_empty() {
            self.placeholder.clone()
        } else {
            self.value.clone()
        };

        let mut style = self.style;
        if self.value.is_empty() {
            style.fg = Some(Color::Named(NamedColor::BrightBlack));
        }

        let mut layout = self.layout;
        if layout.height.is_none() {
            layout.height = Some(crate::tree::layout::Sizing::Points(self.rows as f32));
        }

        let node = crate::tree::render_node::RenderNode {
            kind: crate::tree::node_kind::NodeKind::Input,
            text: Some(display_text),
            style,
            layout,
            ..crate::tree::render_node::RenderNode::default()
        };
        let id = ctx.insert_node(node);
        ctx.set_focusable(id, true);
        WidgetId(id)
    }

    fn handle_event(&self, id: WidgetId, ctx: &mut WidgetContext, event: &Event) -> EventResult {
        if self.disabled {
            return EventResult::Ignored;
        }

        match event {
            Event::Key(key_event) => {
                let mut value = self.value.to_string();

                match key_event.key {
                    Key::Character(c) => {
                        value.push(c);
                        if let Some(ref handler) = self.on_change {
                            handler(&value);
                        }
                        ctx.set_text(id.node_id(), value);
                        ctx.request_frame();
                        EventResult::Consumed
                    }
                    Key::Backspace => {
                        value.pop();
                        if let Some(ref handler) = self.on_change {
                            handler(&value);
                        }
                        ctx.set_text(id.node_id(), value);
                        ctx.request_frame();
                        EventResult::Consumed
                    }
                    Key::Enter => {
                        value.push('\n');
                        if let Some(ref handler) = self.on_change {
                            handler(&value);
                        }
                        ctx.set_text(id.node_id(), value);
                        ctx.request_frame();
                        EventResult::Consumed
                    }
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
    use crate::focus::FocusManager;
    use crate::scheduler::Scheduler;
    use crate::tree::arena::NodeArena;
    use crate::tree::node_kind::NodeKind;
    use crate::widgets::theme::Theme;

    fn make_ctx() -> (NodeArena, FocusManager, Scheduler, Theme) {
        (
            NodeArena::new(),
            FocusManager::new(),
            Scheduler::new(),
            Theme::default(),
        )
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
        let node = ctx.arena.get(id.node_id()).unwrap();
        assert_eq!(node.kind, NodeKind::Input);
        assert_eq!(node.text.as_deref(), Some("Enter text..."));
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
}
