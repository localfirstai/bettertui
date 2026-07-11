use crate::events::Event;
use crate::events::types::EventResult;
use crate::tree::layout::LayoutProps;
use crate::tree::style::Style;

use super::{Widget, WidgetContext, WidgetId};

/// Tooltip widget for hover information.
///
/// Renders a small popup with informational text.
pub struct TooltipWidget {
    pub content: Box<str>,
    pub delay: u32,
    pub style: Style,
    pub layout: LayoutProps,
}

impl Default for TooltipWidget {
    fn default() -> Self {
        Self {
            content: Box::from(""),
            delay: 500,
            style: Style::default(),
            layout: LayoutProps::default(),
        }
    }
}

impl TooltipWidget {
    pub fn new(content: impl Into<Box<str>>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    pub fn with_delay(mut self, delay: u32) -> Self {
        self.delay = delay;
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

impl Widget for TooltipWidget {
    fn kind(&self) -> &'static str {
        "Tooltip"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let node = crate::tree::render_node::RenderNode {
            kind: crate::tree::node_kind::NodeKind::Box,
            text: Some(self.content.clone()),
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
    fn tooltip_widget_kind() {
        let w = TooltipWidget::new("Help text");
        assert_eq!(w.kind(), "Tooltip");
    }

    #[test]
    fn tooltip_widget_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = TooltipWidget::new("Tooltip content");
        let id = w.create(&mut ctx);
        let node = ctx.arena.get(id.node_id()).unwrap();
        assert_eq!(node.kind, NodeKind::Box);
        assert_eq!(node.text.as_deref(), Some("Tooltip content"));
    }

    #[test]
    fn tooltip_widget_with_delay() {
        let w = TooltipWidget::new("Tip").with_delay(1000);
        assert_eq!(w.delay, 1000);
    }
}
