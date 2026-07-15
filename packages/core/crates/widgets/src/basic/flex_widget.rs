use bettertui_engine::input::Event;
use bettertui_engine::input::EventResult;
use bettertui_engine::taffy::{AlignItems, FlexDirection, Gap, JustifyContent, LayoutProps};
use bettertui_engine::tree::Style;

use crate::{Widget, WidgetContext, WidgetId};

pub struct FlexWidget {
    pub direction: FlexDirection,
    pub justify: JustifyContent,
    pub align: AlignItems,
    pub gap: Option<Gap>,
    pub layout: LayoutProps,
    pub style: Style,
}

impl Default for FlexWidget {
    fn default() -> Self {
        Self {
            direction: FlexDirection::Column,
            justify: JustifyContent::FlexStart,
            align: AlignItems::Stretch,
            gap: None,
            layout: LayoutProps::default(),
            style: Style::default(),
        }
    }
}

impl FlexWidget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn row() -> Self {
        Self {
            direction: FlexDirection::Row,
            ..Self::default()
        }
    }

    pub fn column() -> Self {
        Self::default()
    }

    pub fn with_direction(mut self, direction: FlexDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_justify(mut self, justify: JustifyContent) -> Self {
        self.justify = justify;
        self
    }

    pub fn with_align(mut self, align: AlignItems) -> Self {
        self.align = align;
        self
    }

    pub fn with_gap(mut self, gap: Gap) -> Self {
        self.gap = Some(gap);
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_flex_grow(mut self, grow: f32) -> Self {
        self.layout.flex_grow = grow;
        self
    }
}

impl Widget for FlexWidget {
    fn kind(&self) -> &'static str {
        "Flex"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let mut layout = self.layout;
        layout.direction = self.direction;
        layout.justify = self.justify;
        layout.align = self.align;
        layout.gap = self.gap;

        let id = ctx.make_flex(layout, self.style);
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
    fn flex_widget_kind() {
        let w = FlexWidget::new();
        assert_eq!(w.kind(), "Flex");
    }

    #[test]
    fn flex_widget_row() {
        let w = FlexWidget::row();
        assert_eq!(w.direction, FlexDirection::Row);
    }

    #[test]
    fn flex_widget_column() {
        let w = FlexWidget::column();
        assert_eq!(w.direction, FlexDirection::Column);
    }

    #[test]
    fn flex_widget_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = FlexWidget::row()
            .with_gap(Gap::uniform(2.0))
            .with_justify(JustifyContent::Center);
        let id = w.create(&mut ctx);
        let node = ctx
            .arena
            .get(id.node_id())
            .expect("Node missing from arena");
        assert_eq!(node.kind, NodeKind::Flex);
        assert_eq!(node.layout.direction, FlexDirection::Row);
        assert_eq!(node.layout.gap, Some(Gap::uniform(2.0)));
        assert_eq!(node.layout.justify, JustifyContent::Center);
    }

    #[test]
    fn flex_widget_with_style() {
        let style = Style {
            bold: Some(true),
            ..Style::default()
        };
        let w = FlexWidget::new().with_style(style);
        assert!(w.style.bold.expect("Node missing from arena"));
    }
}
