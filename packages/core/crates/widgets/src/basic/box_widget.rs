use bettertui_engine::input::Event;
use bettertui_engine::input::EventResult;
use bettertui_engine::taffy::LayoutProps;
use bettertui_engine::tree::Style;

use crate::{Widget, WidgetContext, WidgetId};

#[derive(Default)]
pub struct BoxWidget {
    pub layout: LayoutProps,
    pub style: Style,
}

impl BoxWidget {
    pub fn new() -> Self {
        Self::default()
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

impl Widget for BoxWidget {
    fn kind(&self) -> &'static str {
        "Box"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let id = ctx.make_box(self.layout, self.style);
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
    use bettertui_engine::taffy::Sizing;
    use bettertui_engine::tree::NodeArena;

    fn make_ctx() -> (NodeArena, FocusManager, Scheduler, Theme) {
        (NodeArena::new(), FocusManager::new(), Scheduler::new(), Theme::default())
    }

    #[test]
    fn box_widget_kind() {
        let w = BoxWidget::new();
        assert_eq!(w.kind(), "Box");
    }

    #[test]
    fn box_widget_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = BoxWidget::new();
        let id = w.create(&mut ctx);
        let node = ctx.arena.get(id.node_id()).expect("Node missing from arena");
        assert_eq!(node.kind, bettertui_engine::tree::NodeKind::Box);
    }

    #[test]
    fn box_widget_with_layout() {
        let layout = LayoutProps { width: Some(Sizing::Points(20.0)), ..Default::default() };
        let w = BoxWidget::new().with_layout(layout);
        assert_eq!(w.layout.width, Some(Sizing::Points(20.0)));
    }

    #[test]
    fn box_widget_with_style() {
        let style = Style { bold: Some(true), ..Style::default() };
        let w = BoxWidget::new().with_style(style);
        assert!(w.style.bold.expect("Node missing from arena"));
    }
}
