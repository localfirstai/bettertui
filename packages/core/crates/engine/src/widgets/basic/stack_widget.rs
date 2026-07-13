use crate::input::Event;
use crate::input::EventResult;
use crate::layout::types::{LayoutProps, Position};
use crate::tree::Style;

use crate::widgets::{Widget, WidgetContext, WidgetId};

/// Stack layout widget for z-indexed layering.
///
/// Children are stacked on top of each other with z-index ordering.
/// Useful for overlays, modals, and layered UI compositions.
#[derive(Default)]
pub struct StackWidget {
    pub layout: LayoutProps,
    pub style: Style,
}

impl StackWidget {
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

impl Widget for StackWidget {
    fn kind(&self) -> &'static str {
        "Stack"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let mut layout = self.layout;
        layout.position = Position::Relative;

        let node = crate::tree::RenderNode {
            kind: crate::tree::NodeKind::Box,
            style: self.style,
            layout,
            ..crate::tree::RenderNode::default()
        };
        let id = ctx.insert_node(node);
        WidgetId(id)
    }

    fn handle_event(&self, _id: WidgetId, _ctx: &mut WidgetContext, _event: &Event) -> EventResult {
        EventResult::Ignored
    }
}

/// A positioned child within a StackWidget.
#[derive(Default)]
pub struct StackChild {
    pub z_index: u16,
    pub offset_x: i32,
    pub offset_y: i32,
}

impl StackChild {
    pub fn new(z_index: u16) -> Self {
        Self {
            z_index,
            ..Default::default()
        }
    }

    pub fn with_offset(mut self, x: i32, y: i32) -> Self {
        self.offset_x = x;
        self.offset_y = y;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::FocusManager;
    use crate::scheduler::Scheduler;
    use crate::tree::NodeArena;
    use crate::tree::NodeKind;
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
    fn stack_widget_kind() {
        let w = StackWidget::new();
        assert_eq!(w.kind(), "Stack");
    }

    #[test]
    fn stack_widget_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = StackWidget::new();
        let id = w.create(&mut ctx);
        let node = ctx.arena.get(id.node_id()).unwrap();
        assert_eq!(node.kind, NodeKind::Box);
        assert_eq!(node.layout.position, Position::Relative);
    }

    #[test]
    fn stack_widget_with_layout() {
        let layout = LayoutProps {
            flex_grow: 1.0,
            ..Default::default()
        };
        let w = StackWidget::new().with_layout(layout);
        assert_eq!(w.layout.flex_grow, 1.0);
    }

    #[test]
    fn stack_widget_with_style() {
        let style = Style {
            bold: Some(true),
            ..Style::default()
        };
        let w = StackWidget::new().with_style(style);
        assert!(w.style.bold.unwrap());
    }

    #[test]
    fn stack_child_default() {
        let child = StackChild::default();
        assert_eq!(child.z_index, 0);
        assert_eq!(child.offset_x, 0);
        assert_eq!(child.offset_y, 0);
    }

    #[test]
    fn stack_child_with_offset() {
        let child = StackChild::new(1).with_offset(10, 20);
        assert_eq!(child.z_index, 1);
        assert_eq!(child.offset_x, 10);
        assert_eq!(child.offset_y, 20);
    }
}
