use crate::input::Event;
use crate::input::EventResult;
use crate::layout::types::{FlexDirection, Gap, LayoutProps};
use crate::tree::style::Style;

use super::{Widget, WidgetContext, WidgetId};

/// Grid layout widget for terminal grids.
///
/// Implements CSS Grid-like layout using flexbox under the hood.
/// Children are arranged in a grid with configurable rows and columns.
pub struct GridWidget {
    pub columns: u16,
    pub rows: u16,
    pub column_gap: f32,
    pub row_gap: f32,
    pub layout: LayoutProps,
    pub style: Style,
}

impl Default for GridWidget {
    fn default() -> Self {
        Self {
            columns: 1,
            rows: 1,
            column_gap: 0.0,
            row_gap: 0.0,
            layout: LayoutProps::default(),
            style: Style::default(),
        }
    }
}

impl GridWidget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_columns(mut self, columns: u16) -> Self {
        self.columns = columns;
        self
    }

    pub fn with_rows(mut self, rows: u16) -> Self {
        self.rows = rows;
        self
    }

    pub fn with_column_gap(mut self, gap: f32) -> Self {
        self.column_gap = gap;
        self
    }

    pub fn with_row_gap(mut self, gap: f32) -> Self {
        self.row_gap = gap;
        self
    }

    pub fn with_gap(mut self, gap: f32) -> Self {
        self.column_gap = gap;
        self.row_gap = gap;
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

impl Widget for GridWidget {
    fn kind(&self) -> &'static str {
        "Grid"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let mut layout = self.layout;
        layout.direction = FlexDirection::Column;
        layout.gap = Some(Gap::new(self.row_gap, self.column_gap));

        let mut style = self.style;
        style.grid_columns = Some(self.columns);
        style.grid_rows = Some(self.rows);

        let node = crate::tree::render_node::RenderNode {
            kind: crate::tree::node_kind::NodeKind::Flex,
            style,
            layout,
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
    fn grid_widget_kind() {
        let w = GridWidget::new();
        assert_eq!(w.kind(), "Grid");
    }

    #[test]
    fn grid_widget_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = GridWidget::new().with_columns(3).with_rows(2).with_gap(1.0);
        let id = w.create(&mut ctx);
        let node = ctx.arena.get(id.node_id()).unwrap();
        assert_eq!(node.kind, NodeKind::Flex);
        assert_eq!(node.style.grid_columns, Some(3));
        assert_eq!(node.style.grid_rows, Some(2));
        assert_eq!(node.layout.gap, Some(Gap::new(1.0, 1.0)));
    }

    #[test]
    fn grid_widget_with_layout() {
        let layout = LayoutProps {
            flex_grow: 1.0,
            ..Default::default()
        };
        let w = GridWidget::new().with_layout(layout);
        assert_eq!(w.layout.flex_grow, 1.0);
    }

    #[test]
    fn grid_widget_with_style() {
        let style = Style {
            bold: Some(true),
            ..Style::default()
        };
        let w = GridWidget::new().with_style(style);
        assert!(w.style.bold.unwrap());
    }
}
