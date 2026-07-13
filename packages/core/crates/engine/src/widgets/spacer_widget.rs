use crate::input::Event;
use crate::input::EventResult;
use crate::layout::types::{LayoutProps, Sizing};

use super::{Widget, WidgetContext, WidgetId};

pub struct SpacerWidget {
    pub layout: LayoutProps,
}

impl SpacerWidget {
    pub fn new() -> Self {
        Self {
            layout: LayoutProps::default(),
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.layout.width = Some(Sizing::Points(width));
        self.layout.height = Some(Sizing::Points(height));
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.layout.width = Some(Sizing::Points(width));
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.layout.height = Some(Sizing::Points(height));
        self
    }
}

impl Default for SpacerWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for SpacerWidget {
    fn kind(&self) -> &'static str {
        "Spacer"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let id = ctx.make_spacer(self.layout);
        WidgetId(id)
    }

    fn handle_event(&self, _id: WidgetId, _ctx: &mut WidgetContext, _event: &Event) -> EventResult {
        EventResult::Ignored
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::FocusManager;
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
    fn spacer_widget_kind() {
        let w = SpacerWidget::new();
        assert_eq!(w.kind(), "Spacer");
    }

    #[test]
    fn spacer_widget_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = SpacerWidget::new().with_size(5.0, 3.0);
        let id = w.create(&mut ctx);
        let node = ctx.arena.get(id.node_id()).unwrap();
        assert_eq!(node.kind, NodeKind::Spacer);
        assert_eq!(node.layout.width, Some(Sizing::Points(5.0)));
        assert_eq!(node.layout.height, Some(Sizing::Points(3.0)));
    }

    #[test]
    fn spacer_widget_with_width() {
        let w = SpacerWidget::new().with_width(10.0);
        assert_eq!(w.layout.width, Some(Sizing::Points(10.0)));
        assert!(w.layout.height.is_none());
    }

    #[test]
    fn spacer_widget_with_height() {
        let w = SpacerWidget::new().with_height(7.0);
        assert!(w.layout.width.is_none());
        assert_eq!(w.layout.height, Some(Sizing::Points(7.0)));
    }
}
