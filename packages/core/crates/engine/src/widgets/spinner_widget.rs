use crate::events::Event;
use crate::events::types::EventResult;
use crate::tree::layout::LayoutProps;
use crate::tree::style::Style;

use super::{Widget, WidgetContext, WidgetId};

/// Loading spinner widget.
///
/// Renders an animated spinner to indicate loading state.
#[derive(Default)]
pub struct SpinnerWidget {
    pub label: Option<Box<str>>,
    pub spinner_type: SpinnerType,
    pub style: Style,
    pub layout: LayoutProps,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpinnerType {
    #[default]
    Dots,
    Line,
    Braille,
    Arc,
}

impl SpinnerWidget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_label(mut self, label: impl Into<Box<str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_type(mut self, spinner_type: SpinnerType) -> Self {
        self.spinner_type = spinner_type;
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

impl Widget for SpinnerWidget {
    fn kind(&self) -> &'static str {
        "Spinner"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let display_text = self
            .label
            .as_ref()
            .map(|l| format!("⠋ {}", l))
            .unwrap_or_else(|| "⠋".to_string());

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
    fn spinner_widget_kind() {
        let w = SpinnerWidget::new();
        assert_eq!(w.kind(), "Spinner");
    }

    #[test]
    fn spinner_widget_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = SpinnerWidget::new().with_label("Loading...");
        let id = w.create(&mut ctx);
        let node = ctx.arena.get(id.node_id()).unwrap();
        assert_eq!(node.kind, NodeKind::Box);
        assert_eq!(node.text.as_deref(), Some("⠋ Loading..."));
    }

    #[test]
    fn spinner_widget_with_type() {
        let w = SpinnerWidget::new().with_type(SpinnerType::Arc);
        assert_eq!(w.spinner_type, SpinnerType::Arc);
    }
}
