use crate::events::Event;
use crate::events::types::EventResult;
use crate::layout::types::LayoutProps;
use crate::tree::color::{Color, NamedColor};
use crate::tree::style::Style;

use super::{Widget, WidgetContext, WidgetId};

/// Progress indicator widget.
///
/// Renders a progress bar showing completion status.
pub struct ProgressWidget {
    pub value: f32,
    pub max: f32,
    pub style: Style,
    pub layout: LayoutProps,
}

impl Default for ProgressWidget {
    fn default() -> Self {
        Self {
            value: 0.0,
            max: 100.0,
            style: Style {
                fg: Some(Color::Named(NamedColor::Green)),
                ..Style::default()
            },
            layout: LayoutProps::default(),
        }
    }
}

impl ProgressWidget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn with_max(mut self, max: f32) -> Self {
        self.max = max;
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

    pub fn percentage(&self) -> f32 {
        if self.max == 0.0 {
            0.0
        } else {
            (self.value / self.max * 100.0).min(100.0)
        }
    }
}

impl Widget for ProgressWidget {
    fn kind(&self) -> &'static str {
        "Progress"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let percentage = self.percentage();
        let display_text = format!("{:.0}%", percentage);

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
    fn progress_widget_kind() {
        let w = ProgressWidget::new();
        assert_eq!(w.kind(), "Progress");
    }

    #[test]
    fn progress_widget_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = ProgressWidget::new().with_value(50.0);
        let id = w.create(&mut ctx);
        let node = ctx.arena.get(id.node_id()).unwrap();
        assert_eq!(node.kind, NodeKind::Box);
    }

    #[test]
    fn progress_widget_percentage() {
        let w = ProgressWidget::new().with_value(50.0).with_max(100.0);
        assert_eq!(w.percentage(), 50.0);
    }

    #[test]
    fn progress_widget_percentage_capped() {
        let w = ProgressWidget::new().with_value(150.0).with_max(100.0);
        assert_eq!(w.percentage(), 100.0);
    }

    #[test]
    fn progress_widget_zero_max() {
        let w = ProgressWidget::new().with_value(0.0).with_max(0.0);
        assert_eq!(w.percentage(), 0.0);
    }
}
