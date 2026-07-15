use bettertui_engine::input::FocusManager;
use bettertui_engine::scheduler::Scheduler;
use bettertui_engine::taffy::LayoutProps;
use bettertui_engine::tree::NodeArena;
use bettertui_engine::tree::NodeId;
use bettertui_engine::tree::NodeKind;
use bettertui_engine::tree::RenderNode;
use bettertui_engine::tree::Style;
use bettertui_engine::tree::Visibility;

use super::theme::Theme;

pub struct WidgetContext<'a> {
    pub arena: &'a mut NodeArena,
    pub focus_manager: &'a mut FocusManager,
    pub scheduler: &'a mut Scheduler,
    pub terminal_size: (u16, u16),
    pub theme: &'a Theme,
}

impl<'a> WidgetContext<'a> {
    pub fn insert_node(&mut self, node: RenderNode) -> NodeId {
        self.arena.insert(node)
    }

    pub fn append_child(&mut self, parent: NodeId, child: NodeId) {
        let _ = self.arena.append_child(parent, child);
    }

    pub fn set_text(&mut self, id: NodeId, text: impl Into<Box<str>>) {
        if let Some(node) = self.arena.get_mut(id) {
            node.set_text(text);
            self.arena.mark_changed();
        }
    }

    pub fn set_style(&mut self, id: NodeId, style: Style) {
        if let Some(node) = self.arena.get_mut(id) {
            node.set_style(style);
            self.arena.mark_changed();
        }
    }

    pub fn set_layout(&mut self, id: NodeId, layout: LayoutProps) {
        if let Some(node) = self.arena.get_mut(id) {
            node.set_layout(layout);
            self.arena.mark_changed();
        }
    }

    pub fn set_visibility(&mut self, id: NodeId, visibility: Visibility) {
        if let Some(node) = self.arena.get_mut(id) {
            node.visibility = visibility;
            node.state.mark_render_dirty();
            self.arena.mark_changed();
        }
    }

    pub fn set_focusable(&mut self, id: NodeId, focusable: bool) {
        if let Some(node) = self.arena.get_mut(id) {
            node.set_focusable(focusable);
            self.arena.mark_changed();
        }
    }

    pub fn remove_subtree(&mut self, id: NodeId) {
        self.arena.remove_subtree(id);
    }

    pub fn request_frame(&mut self) {
        self.scheduler.request_frame();
    }

    pub fn request_high_priority_frame(&mut self) {
        self.scheduler.request_high_priority_frame();
    }

    pub fn make_box(&mut self, layout: LayoutProps, style: Style) -> NodeId {
        let node = RenderNode {
            kind: NodeKind::Box,
            style,
            layout,
            ..RenderNode::default()
        };
        self.arena.insert(node)
    }

    pub fn make_text(&mut self, content: impl Into<Box<str>>, style: Style) -> NodeId {
        let node = RenderNode {
            kind: NodeKind::Text,
            text: Some(content.into()),
            style,
            ..RenderNode::default()
        };
        self.arena.insert(node)
    }

    pub fn make_flex(&mut self, layout: LayoutProps, style: Style) -> NodeId {
        let node = RenderNode {
            kind: NodeKind::Flex,
            style,
            layout,
            ..RenderNode::default()
        };
        self.arena.insert(node)
    }

    pub fn make_spacer(&mut self, layout: LayoutProps) -> NodeId {
        let node = RenderNode {
            kind: NodeKind::Spacer,
            layout,
            ..RenderNode::default()
        };
        self.arena.insert(node)
    }

    pub fn make_separator(&mut self, layout: LayoutProps, style: Style) -> NodeId {
        let node = RenderNode {
            kind: NodeKind::Separator,
            style,
            layout,
            ..RenderNode::default()
        };
        self.arena.insert(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bettertui_engine::taffy::Sizing;
    use bettertui_engine::tree::{Color, NamedColor};

    fn make_context() -> (NodeArena, FocusManager, Scheduler, Theme) {
        (
            NodeArena::new(),
            FocusManager::new(),
            Scheduler::new(),
            Theme::default(),
        )
    }

    #[test]
    fn context_insert_node() {
        let (mut arena, mut focus, mut sched, theme) = make_context();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let id = ctx.insert_node(RenderNode::new(NodeKind::Text));
        assert!(ctx.arena.contains(id));
    }

    #[test]
    fn context_append_child() {
        let (mut arena, mut focus, mut sched, theme) = make_context();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let child = ctx.insert_node(RenderNode::new(NodeKind::Text));
        ctx.append_child(ctx.arena.root(), child);
        assert_eq!(ctx.arena.children(ctx.arena.root()).len(), 1);
    }

    #[test]
    fn context_set_text() {
        let (mut arena, mut focus, mut sched, theme) = make_context();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let id = ctx.insert_node(RenderNode::new(NodeKind::Text));
        ctx.set_text(id, "hello");
        assert_eq!(
            ctx.arena
                .get(id)
                .expect("Node missing from arena")
                .text
                .as_deref(),
            Some("hello")
        );
    }

    #[test]
    fn context_make_box() {
        let (mut arena, mut focus, mut sched, theme) = make_context();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let layout = LayoutProps {
            width: Some(Sizing::Points(10.0)),
            ..Default::default()
        };
        let style = Style {
            bg: Some(Color::Named(NamedColor::Blue)),
            ..Style::default()
        };
        let id = ctx.make_box(layout, style);
        let node = ctx.arena.get(id).expect("Node missing from arena");
        assert_eq!(node.kind, NodeKind::Box);
        assert_eq!(node.layout.width, Some(Sizing::Points(10.0)));
        assert_eq!(node.style.bg, Some(Color::Named(NamedColor::Blue)));
    }

    #[test]
    fn context_make_text() {
        let (mut arena, mut focus, mut sched, theme) = make_context();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let style = Style {
            bold: Some(true),
            ..Style::default()
        };
        let id = ctx.make_text("Hello World", style);
        let node = ctx.arena.get(id).expect("Node missing from arena");
        assert_eq!(node.kind, NodeKind::Text);
        assert_eq!(node.text.as_deref(), Some("Hello World"));
        assert!(node.style.bold.expect("Node missing from arena"));
    }

    #[test]
    fn context_request_frame() {
        let (mut arena, mut focus, mut sched, theme) = make_context();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        ctx.request_frame();
        assert!(ctx.scheduler.has_pending_frames());
    }

    #[test]
    fn context_remove_subtree() {
        let (mut arena, mut focus, mut sched, theme) = make_context();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let child = ctx.insert_node(RenderNode::new(NodeKind::Text));
        ctx.append_child(ctx.arena.root(), child);
        assert!(ctx.arena.contains(child));

        ctx.remove_subtree(child);
        assert!(!ctx.arena.contains(child));
    }

    #[test]
    fn context_make_flex() {
        let (mut arena, mut focus, mut sched, theme) = make_context();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let id = ctx.make_flex(LayoutProps::default(), Style::default());
        assert_eq!(
            ctx.arena.get(id).expect("Node missing from arena").kind,
            NodeKind::Flex
        );
    }

    #[test]
    fn context_make_spacer() {
        let (mut arena, mut focus, mut sched, theme) = make_context();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let id = ctx.make_spacer(LayoutProps::default());
        assert_eq!(
            ctx.arena.get(id).expect("Node missing from arena").kind,
            NodeKind::Spacer
        );
    }
}
