use bettertui_engine::input::Event;
use bettertui_engine::input::EventResult;
use bettertui_engine::layout::{LayoutProps, RectValues};
use bettertui_engine::tree::Style;

use crate::{Widget, WidgetContext, WidgetId};

#[derive(Default)]
pub struct ContainerWidget {
    pub layout: LayoutProps,
    pub style: Style,
    pub title: Option<Box<str>>,
}

impl ContainerWidget {
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

    pub fn with_title(mut self, title: impl Into<Box<str>>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.layout.padding = Some(RectValues::uniform(padding));
        self
    }
}

impl Widget for ContainerWidget {
    fn kind(&self) -> &'static str {
        "Container"
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
    use bettertui_engine::layout::Sizing;
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
    fn container_kind() {
        let w = ContainerWidget::new();
        assert_eq!(w.kind(), "Container");
    }

    #[test]
    fn container_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = ContainerWidget::new()
            .with_title("My Container")
            .with_padding(2.0);
        let id = w.create(&mut ctx);
        let node = ctx.arena.get(id.node_id()).unwrap();
        assert_eq!(node.kind, NodeKind::Box);
        assert_eq!(w.title.as_deref(), Some("My Container"));
    }

    #[test]
    fn container_with_layout() {
        let layout = LayoutProps {
            width: Some(Sizing::Points(50.0)),
            ..Default::default()
        };
        let w = ContainerWidget::new().with_layout(layout);
        assert_eq!(w.layout.width, Some(Sizing::Points(50.0)));
    }

    #[test]
    fn container_with_style() {
        let style = Style {
            bold: Some(true),
            ..Style::default()
        };
        let w = ContainerWidget::new().with_style(style);
        assert!(w.style.bold.unwrap());
    }

    #[test]
    fn container_with_padding() {
        let w = ContainerWidget::new().with_padding(3.0);
        assert_eq!(w.layout.padding, Some(RectValues::uniform(3.0)));
    }
}
