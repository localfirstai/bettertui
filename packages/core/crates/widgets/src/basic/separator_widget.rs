use bettertui_engine::input::Event;
use bettertui_engine::input::EventResult;
use bettertui_engine::layout::LayoutProps;
use bettertui_engine::tree::Style;

use crate::{Widget, WidgetContext, WidgetId};

/// Visual divider line widget.
///
/// Renders a horizontal or vertical line to separate content areas.
#[derive(Default)]
pub struct SeparatorWidget {
    pub orientation: SeparatorOrientation,
    pub layout: LayoutProps,
    pub style: Style,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl SeparatorWidget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn horizontal() -> Self {
        Self {
            orientation: SeparatorOrientation::Horizontal,
            ..Default::default()
        }
    }

    pub fn vertical() -> Self {
        Self {
            orientation: SeparatorOrientation::Vertical,
            ..Default::default()
        }
    }

    pub fn with_orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_layout(mut self, layout: LayoutProps) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for SeparatorWidget {
    fn kind(&self) -> &'static str {
        "Separator"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let node = bettertui_engine::tree::RenderNode {
            kind: bettertui_engine::tree::NodeKind::Separator,
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
    fn separator_widget_kind() {
        let w = SeparatorWidget::new();
        assert_eq!(w.kind(), "Separator");
    }

    #[test]
    fn separator_widget_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = SeparatorWidget::horizontal();
        let id = w.create(&mut ctx);
        let node = ctx
            .arena
            .get(id.node_id())
            .expect("Node missing from arena");
        assert_eq!(node.kind, NodeKind::Separator);
    }

    #[test]
    fn separator_widget_vertical() {
        let w = SeparatorWidget::vertical();
        assert_eq!(w.orientation, SeparatorOrientation::Vertical);
    }

    #[test]
    fn separator_widget_with_style() {
        let style = Style {
            bold: Some(true),
            ..Style::default()
        };
        let w = SeparatorWidget::new().with_style(style);
        assert!(w.style.bold.expect("Node missing from arena"));
    }
}
