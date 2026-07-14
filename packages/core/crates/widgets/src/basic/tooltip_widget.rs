use bettertui_engine::input::Event;
use bettertui_engine::input::EventResult;
use bettertui_engine::layout::LayoutProps;
use bettertui_engine::tree::Style;

use crate::{Widget, WidgetContext, WidgetId};

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
        let node = bettertui_engine::tree::RenderNode {
            kind: bettertui_engine::tree::NodeKind::Box,
            text: Some(self.content.clone()),
            style: self.style,
            layout: self.layout,
            ..bettertui_engine::tree::RenderNode::default()
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
    use crate::theme::Theme;
    use bettertui_engine::input::{Event, EventResult, FocusManager, Key, KeyEvent};
    use bettertui_engine::layout::LayoutProps;
    use bettertui_engine::scheduler::Scheduler;
    use bettertui_engine::tree::{NodeArena, NodeId, NodeKind, Style};

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
        let node = ctx
            .arena
            .get(id.node_id())
            .expect("Node missing from arena");
        assert_eq!(node.kind, NodeKind::Box);
        assert_eq!(node.text.as_deref(), Some("Tooltip content"));
    }

    #[test]
    fn tooltip_widget_with_delay() {
        let w = TooltipWidget::new("Tip").with_delay(1000);
        assert_eq!(w.delay, 1000);
    }

    #[test]
    fn tooltip_widget_with_layout() {
        let layout = LayoutProps::default();
        let w = TooltipWidget::new("x").with_layout(layout);
        assert_eq!(w.layout, layout);
    }

    #[test]
    fn tooltip_widget_with_style() {
        let style = Style {
            bold: Some(true),
            ..Style::default()
        };
        let w = TooltipWidget::new("x").with_style(style);
        assert!(w.style.bold.expect("Node missing from arena"));
    }

    #[test]
    fn tooltip_widget_handle_event_ignored() {
        let w = TooltipWidget::new("x");
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };
        let event = Event::Key(KeyEvent::new(Key::Character('x'), NodeId::default()));
        let result = w.handle_event(WidgetId::default(), &mut ctx, &event);
        assert_eq!(result, EventResult::Ignored);
    }

    #[test]
    fn tooltip_widget_default_delay() {
        let w = TooltipWidget::new("x");
        assert_eq!(w.delay, 500);
    }
}
