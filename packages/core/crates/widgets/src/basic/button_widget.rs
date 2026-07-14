use bettertui_engine::input::Event;
use bettertui_engine::input::EventResult;
use bettertui_engine::layout::LayoutProps;
use bettertui_engine::tree::Style;
use bettertui_engine::tree::{Color, NamedColor};

use crate::callback_types::AsyncCallback;
use crate::{Widget, WidgetContext, WidgetId};

/// Button widget for clickable actions.
///
/// Renders a styled button with different variants and states.
pub struct ButtonWidget {
    pub label: Box<str>,
    pub variant: ButtonVariant,
    pub disabled: bool,
    pub style: Style,
    pub layout: LayoutProps,
    pub on_press: Option<AsyncCallback>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Primary,
    Secondary,
    Danger,
    Ghost,
    Link,
}

impl Default for ButtonWidget {
    fn default() -> Self {
        Self {
            label: Box::from(""),
            variant: ButtonVariant::default(),
            disabled: false,
            style: Style::default(),
            layout: LayoutProps::default(),
            on_press: None,
        }
    }
}

impl ButtonWidget {
    pub fn new(label: impl Into<Box<str>>) -> Self {
        Self {
            label: label.into(),
            ..Default::default()
        }
    }

    pub fn primary(label: impl Into<Box<str>>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Primary,
            style: Style {
                fg: Some(Color::Named(NamedColor::White)),
                bg: Some(Color::Named(NamedColor::Blue)),
                bold: Some(true),
                ..Style::default()
            },
            ..Default::default()
        }
    }

    pub fn danger(label: impl Into<Box<str>>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Danger,
            style: Style {
                fg: Some(Color::Named(NamedColor::White)),
                bg: Some(Color::Named(NamedColor::Red)),
                bold: Some(true),
                ..Style::default()
            },
            ..Default::default()
        }
    }

    pub fn ghost(label: impl Into<Box<str>>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Ghost,
            ..Default::default()
        }
    }

    pub fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
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

    pub fn on_press(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_press = Some(Box::new(handler));
        self
    }
}

impl Widget for ButtonWidget {
    fn kind(&self) -> &'static str {
        "Button"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let node = bettertui_engine::tree::RenderNode {
            kind: bettertui_engine::tree::NodeKind::Box,
            text: Some(self.label.clone()),
            style: self.style,
            layout: self.layout,
            ..bettertui_engine::tree::RenderNode::default()
        };
        let id = ctx.insert_node(node);
        WidgetId(id)
    }

    fn handle_event(&self, _id: WidgetId, _ctx: &mut WidgetContext, event: &Event) -> EventResult {
        if self.disabled {
            return EventResult::Ignored;
        }

        match event {
            Event::Key(key_event) => {
                if key_event.key == bettertui_engine::input::Key::Enter {
                    if let Some(ref handler) = self.on_press {
                        handler();
                    }
                    EventResult::Consumed
                } else {
                    EventResult::Ignored
                }
            }
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
    use bettertui_engine::tree::NodeKind;

    fn make_ctx() -> (NodeArena, FocusManager, Scheduler, Theme) {
        (
            NodeArena::new(),
            FocusManager::new(),
            Scheduler::new(),
            Theme::default(),
        )
    }

    #[test]
    fn button_widget_kind() {
        let w = ButtonWidget::new("Click me");
        assert_eq!(w.kind(), "Button");
    }

    #[test]
    fn button_widget_primary() {
        let w = ButtonWidget::primary("Submit");
        assert_eq!(w.variant, ButtonVariant::Primary);
        assert!(w.style.bold.unwrap());
    }

    #[test]
    fn button_widget_danger() {
        let w = ButtonWidget::danger("Delete");
        assert_eq!(w.variant, ButtonVariant::Danger);
    }

    #[test]
    fn button_widget_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = ButtonWidget::new("Click");
        let id = w.create(&mut ctx);
        let node = ctx.arena.get(id.node_id()).unwrap();
        assert_eq!(node.kind, NodeKind::Box);
        assert_eq!(node.text.as_deref(), Some("Click"));
    }

    #[test]
    fn button_widget_disabled() {
        let w = ButtonWidget::new("Click").with_disabled(true);
        assert!(w.disabled);
    }
}
