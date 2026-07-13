use crate::events::Event;
use crate::events::types::EventResult;
use crate::layout::types::LayoutProps;
use crate::tree::color::{Color, NamedColor};
use crate::tree::style::Style;

use super::{Widget, WidgetContext, WidgetId};

/// Badge widget for status indicators.
///
/// Renders a small badge with optional count or status text.
pub struct BadgeWidget {
    pub content: Box<str>,
    pub variant: BadgeVariant,
    pub style: Style,
    pub layout: LayoutProps,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

impl Default for BadgeWidget {
    fn default() -> Self {
        Self {
            content: Box::from(""),
            variant: BadgeVariant::default(),
            style: Style::default(),
            layout: LayoutProps::default(),
        }
    }
}

impl BadgeWidget {
    pub fn new(content: impl Into<Box<str>>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    pub fn success(content: impl Into<Box<str>>) -> Self {
        Self {
            content: content.into(),
            variant: BadgeVariant::Success,
            style: Style {
                fg: Some(Color::Named(NamedColor::White)),
                bg: Some(Color::Named(NamedColor::Green)),
                bold: Some(true),
                ..Style::default()
            },
            ..Default::default()
        }
    }

    pub fn danger(content: impl Into<Box<str>>) -> Self {
        Self {
            content: content.into(),
            variant: BadgeVariant::Danger,
            style: Style {
                fg: Some(Color::Named(NamedColor::White)),
                bg: Some(Color::Named(NamedColor::Red)),
                bold: Some(true),
                ..Style::default()
            },
            ..Default::default()
        }
    }

    pub fn warning(content: impl Into<Box<str>>) -> Self {
        Self {
            content: content.into(),
            variant: BadgeVariant::Warning,
            style: Style {
                fg: Some(Color::Named(NamedColor::Black)),
                bg: Some(Color::Named(NamedColor::Yellow)),
                bold: Some(true),
                ..Style::default()
            },
            ..Default::default()
        }
    }

    pub fn primary(content: impl Into<Box<str>>) -> Self {
        Self {
            content: content.into(),
            variant: BadgeVariant::Primary,
            style: Style {
                fg: Some(Color::Named(NamedColor::White)),
                bg: Some(Color::Named(NamedColor::Blue)),
                bold: Some(true),
                ..Style::default()
            },
            ..Default::default()
        }
    }

    pub fn info(content: impl Into<Box<str>>) -> Self {
        Self {
            content: content.into(),
            variant: BadgeVariant::Info,
            style: Style {
                fg: Some(Color::Named(NamedColor::White)),
                bg: Some(Color::Named(NamedColor::Cyan)),
                bold: Some(true),
                ..Style::default()
            },
            ..Default::default()
        }
    }

    pub fn with_variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
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
}

impl Widget for BadgeWidget {
    fn kind(&self) -> &'static str {
        "Badge"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let display_text = format!(" {} ", self.content);

        let node = crate::tree::render_node::RenderNode {
            kind: crate::tree::node_kind::NodeKind::Box,
            text: Some(Box::from(display_text)),
            style: self.style,
            layout: self.layout,
            ..crate::tree::render_node::RenderNode::default()
        };
        let id = ctx.insert_node(node);
        WidgetId(id)
    }

    fn handle_event(&self, _id: WidgetId, _ctx: &mut WidgetContext, _event: &Event) -> EventResult {
        EventResult::Ignored
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
    fn badge_widget_kind() {
        let w = BadgeWidget::new("NEW");
        assert_eq!(w.kind(), "Badge");
    }

    #[test]
    fn badge_widget_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = BadgeWidget::new("5");
        let id = w.create(&mut ctx);
        let node = ctx.arena.get(id.node_id()).unwrap();
        assert_eq!(node.kind, NodeKind::Box);
        assert_eq!(node.text.as_deref(), Some(" 5 "));
    }

    #[test]
    fn badge_widget_success() {
        let w = BadgeWidget::success("OK");
        assert_eq!(w.variant, BadgeVariant::Success);
        assert!(w.style.bold.unwrap());
    }

    #[test]
    fn badge_widget_danger() {
        let w = BadgeWidget::danger("ERR");
        assert_eq!(w.variant, BadgeVariant::Danger);
    }

    #[test]
    fn badge_widget_primary() {
        let w = BadgeWidget::primary("NEW");
        assert_eq!(w.variant, BadgeVariant::Primary);
        assert!(w.style.bold.unwrap());
    }

    #[test]
    fn badge_widget_info() {
        let w = BadgeWidget::info("INFO");
        assert_eq!(w.variant, BadgeVariant::Info);
        assert!(w.style.bold.unwrap());
    }
}
