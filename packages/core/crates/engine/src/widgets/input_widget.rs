use crate::events::types::{Event, EventResult, Key};
use crate::tree::color::{Color, NamedColor};
use crate::tree::layout::LayoutProps;
use crate::tree::style::Style;

use super::callback_types::{ChangeCallback, SubmitCallback};
use super::{Widget, WidgetContext, WidgetId};

/// Single-line text input widget.
///
/// Handles keyboard input, cursor movement, and text editing.
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
}

impl Widget for InputWidget {
    fn kind(&self) -> &'static str {
        "Input"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let display_text = if self.value.is_empty() {
            self.placeholder.clone()
        } else if self.password {
            Box::from("*".repeat(self.value.len()))
        } else {
            self.value.clone()
        };

        let mut style = self.style;
        if self.value.is_empty() {
            style.fg = Some(Color::Named(NamedColor::BrightBlack));
        }

        let node = crate::tree::render_node::RenderNode {
            kind: crate::tree::node_kind::NodeKind::Input,
            text: Some(display_text),
            style,
            layout: self.layout,
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
                        let display_text = if self.password {
                            Box::from("*".repeat(value.len()))
                        } else {
                            Box::from(value)
                        };
                        ctx.set_text(id.node_id(), display_text);
                        ctx.request_frame();
                        EventResult::Consumed
                    }
                    Key::Backspace => {
                        value.pop();
                        if let Some(ref handler) = self.on_change {
                            handler(&value);
                        }
                        let display_text = if self.password {
                            Box::from("*".repeat(value.len()))
                        } else {
                            Box::from(value)
                        };
                        ctx.set_text(id.node_id(), display_text);
                        ctx.request_frame();
                        EventResult::Consumed
                    }
                    Key::Enter => {
                        if let Some(ref handler) = self.on_submit {
                            handler(&self.value);
                        }
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
}
