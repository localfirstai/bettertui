use bettertui_engine::input::{Event, EventResult, Key};
use bettertui_engine::taffy::LayoutProps;
use bettertui_engine::tree::NodeKind;
use bettertui_engine::tree::Overflow;
use bettertui_engine::tree::RenderNode;
use bettertui_engine::tree::Style;

use crate::{Widget, WidgetContext, WidgetId};

pub struct ScrollAreaWidget {
    pub layout: LayoutProps,
    pub style: Style,
    pub show_scrollbar: bool,
}

impl Default for ScrollAreaWidget {
    fn default() -> Self {
        Self {
            layout: LayoutProps::default(),
            style: Style::default(),
            show_scrollbar: true,
        }
    }
}

impl ScrollAreaWidget {
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

    pub fn with_scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = show;
        self
    }
}

impl Widget for ScrollAreaWidget {
    fn kind(&self) -> &'static str {
        "ScrollArea"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let node = RenderNode {
            kind: NodeKind::Scroll,
            layout: self.layout,
            style: self.style,
            overflow: Overflow::Scroll,
            ..RenderNode::default()
        };
        let id = ctx.insert_node(node);
        WidgetId(id)
    }

    fn handle_event(&self, id: WidgetId, ctx: &mut WidgetContext, event: &Event) -> EventResult {
        match event {
            Event::Key(key_event) => {
                let mut changed = false;
                let result = if let Some(node) = ctx.arena.get_mut(id.node_id()) {
                    match key_event.key {
                        Key::ArrowUp => {
                            node.state.scroll_y = (node.state.scroll_y - 1).max(0);
                            node.state.mark_render_dirty();
                            ctx.request_frame();
                            changed = true;
                            EventResult::Consumed
                        }
                        Key::ArrowDown => {
                            node.state.scroll_y = node.state.scroll_y.saturating_add(1);
                            node.state.mark_render_dirty();
                            ctx.request_frame();
                            changed = true;
                            EventResult::Consumed
                        }
                        Key::PageUp => {
                            node.state.scroll_y = node.state.scroll_y.saturating_sub(10);
                            node.state.mark_render_dirty();
                            ctx.request_frame();
                            changed = true;
                            EventResult::Consumed
                        }
                        Key::PageDown => {
                            node.state.scroll_y = node.state.scroll_y.saturating_add(10);
                            node.state.mark_render_dirty();
                            ctx.request_frame();
                            changed = true;
                            EventResult::Consumed
                        }
                        Key::Home => {
                            node.state.scroll_y = 0;
                            node.state.mark_render_dirty();
                            ctx.request_frame();
                            changed = true;
                            EventResult::Consumed
                        }
                        Key::End => {
                            node.state.scroll_y = i32::MAX;
                            node.state.mark_render_dirty();
                            ctx.request_frame();
                            changed = true;
                            EventResult::Consumed
                        }
                        _ => EventResult::Ignored,
                    }
                } else {
                    EventResult::Ignored
                };
                if changed {
                    ctx.arena.mark_changed();
                }
                result
            }
            _ => EventResult::Ignored,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;
    use bettertui_engine::input::FocusManager;
    use bettertui_engine::scheduler::Scheduler;
    use bettertui_engine::tree::NodeArena;

    fn make_ctx() -> (NodeArena, FocusManager, Scheduler, Theme) {
        (
            NodeArena::new(),
            FocusManager::new(),
            Scheduler::new(),
            Theme::default(),
        )
    }

    #[test]
    fn scroll_area_kind() {
        let w = ScrollAreaWidget::new();
        assert_eq!(w.kind(), "ScrollArea");
    }

    #[test]
    fn scroll_area_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = ScrollAreaWidget::new();
        let id = w.create(&mut ctx);
        let node = ctx
            .arena
            .get(id.node_id())
            .expect("Node missing from arena");
        assert_eq!(node.kind, NodeKind::Scroll);
        assert_eq!(node.overflow, Overflow::Scroll);
    }

    #[test]
    fn scroll_area_scroll_down() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = ScrollAreaWidget::new();
        let id = w.create(&mut ctx);

        let event = Event::Key(bettertui_engine::input::KeyEvent::new(
            Key::ArrowDown,
            id.node_id(),
        ));
        let result = w.handle_event(id, &mut ctx, &event);
        assert_eq!(result, EventResult::Consumed);

        let node = ctx
            .arena
            .get(id.node_id())
            .expect("Node missing from arena");
        assert_eq!(node.state.scroll_y, 1);
    }

    #[test]
    fn scroll_area_scroll_up() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = ScrollAreaWidget::new();
        let id = w.create(&mut ctx);

        let event = Event::Key(bettertui_engine::input::KeyEvent::new(
            Key::ArrowUp,
            id.node_id(),
        ));
        let result = w.handle_event(id, &mut ctx, &event);
        assert_eq!(result, EventResult::Consumed);

        let node = ctx
            .arena
            .get(id.node_id())
            .expect("Node missing from arena");
        assert_eq!(node.state.scroll_y, 0);
    }

    #[test]
    fn scroll_area_page_down() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = ScrollAreaWidget::new();
        let id = w.create(&mut ctx);

        let event = Event::Key(bettertui_engine::input::KeyEvent::new(
            Key::PageDown,
            id.node_id(),
        ));
        w.handle_event(id, &mut ctx, &event);

        let node = ctx
            .arena
            .get(id.node_id())
            .expect("Node missing from arena");
        assert_eq!(node.state.scroll_y, 10);
    }

    #[test]
    fn scroll_area_home() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = ScrollAreaWidget::new();
        let id = w.create(&mut ctx);

        let event = Event::Key(bettertui_engine::input::KeyEvent::new(
            Key::ArrowDown,
            id.node_id(),
        ));
        w.handle_event(id, &mut ctx, &event);
        w.handle_event(id, &mut ctx, &event);

        let event = Event::Key(bettertui_engine::input::KeyEvent::new(
            Key::Home,
            id.node_id(),
        ));
        w.handle_event(id, &mut ctx, &event);

        let node = ctx
            .arena
            .get(id.node_id())
            .expect("Node missing from arena");
        assert_eq!(node.state.scroll_y, 0);
    }

    #[test]
    fn scroll_area_no_scrollbar() {
        let w = ScrollAreaWidget::new().with_scrollbar(false);
        assert!(!w.show_scrollbar);
    }
}
