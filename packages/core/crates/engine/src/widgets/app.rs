use crate::input::Event;
use crate::input::FocusManager;
use crate::scheduler::Scheduler;
use crate::tree::arena::NodeArena;
use crate::widgets::WidgetId;
use crate::widgets::context::WidgetContext;
use crate::widgets::theme::Theme;
use crate::widgets::tree::WidgetTree;

pub struct AppState {
    pub tree: WidgetTree,
    pub arena: NodeArena,
    pub focus_manager: FocusManager,
    pub scheduler: Scheduler,
    pub theme: Theme,
    pub root_id: Option<WidgetId>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            tree: WidgetTree::new(),
            arena: NodeArena::new(),
            focus_manager: FocusManager::new(),
            scheduler: Scheduler::new(),
            theme: Theme::default(),
            root_id: None,
        }
    }

    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn context(&mut self) -> WidgetContext<'_> {
        WidgetContext {
            arena: &mut self.arena,
            focus_manager: &mut self.focus_manager,
            scheduler: &mut self.scheduler,
            terminal_size: (80, 24),
            theme: &self.theme,
        }
    }

    pub fn set_root(&mut self, root_id: WidgetId) {
        self.root_id = Some(root_id);
        self.tree.insert(root_id, root_id.node_id(), "Root");
    }

    pub fn mount(&mut self, parent_id: WidgetId, child_id: WidgetId, kind: &'static str) {
        self.tree.insert(child_id, child_id.node_id(), kind);
        self.tree.set_parent(child_id, parent_id);
        let _ = self
            .arena
            .append_child(parent_id.node_id(), child_id.node_id());
    }

    pub fn unmount(&mut self, widget_id: WidgetId) {
        self.tree.remove(widget_id);
        if let Some(node) = self.arena.remove(widget_id.node_id()) {
            let _ = node;
        }
    }

    pub fn handle_event(&mut self, _event: &Event) {}

    pub fn update(&mut self) {}

    pub fn render(&self) -> &NodeArena {
        &self.arena
    }

    pub fn node_count(&self) -> usize {
        self.arena.len()
    }

    pub fn widget_count(&self) -> usize {
        self.tree.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::node_kind::NodeKind;
    use crate::tree::render_node::RenderNode;

    #[test]
    fn app_state_new() {
        let state = AppState::new();
        assert_eq!(state.node_count(), 1);
        assert_eq!(state.widget_count(), 0);
        assert!(state.root_id.is_none());
    }

    #[test]
    fn app_state_with_theme() {
        let theme = Theme::dark();
        let state = AppState::new().with_theme(theme);
        assert!(!state.theme.colors.is_empty());
    }

    #[test]
    fn app_state_set_root() {
        let mut state = AppState::new();
        let root_nid = state.arena.insert(RenderNode::new(NodeKind::Box));
        let root_id = WidgetId(root_nid);
        state.set_root(root_id);
        assert_eq!(state.root_id, Some(root_id));
    }

    #[test]
    fn app_state_mount() {
        let mut state = AppState::new();
        let root_nid = state.arena.insert(RenderNode::new(NodeKind::Box));
        let root_id = WidgetId(root_nid);
        state.set_root(root_id);

        let child_nid = state.arena.insert(RenderNode::new(NodeKind::Text));
        let child_id = WidgetId(child_nid);
        state.mount(root_id, child_id, "Text");

        assert_eq!(state.tree.children(root_id).len(), 1);
    }

    #[test]
    fn app_state_unmount() {
        let mut state = AppState::new();
        let root_nid = state.arena.insert(RenderNode::new(NodeKind::Box));
        let root_id = WidgetId(root_nid);
        state.set_root(root_id);

        let child_nid = state.arena.insert(RenderNode::new(NodeKind::Text));
        let child_id = WidgetId(child_nid);
        state.mount(root_id, child_id, "Text");
        state.unmount(child_id);

        assert_eq!(state.tree.children(root_id).len(), 0);
    }

    #[test]
    fn app_state_context() {
        let mut state = AppState::new();
        let mut ctx = state.context();
        let nid = ctx.make_box(Default::default(), Default::default());
        assert!(ctx.arena.contains(nid));
    }

    #[test]
    fn app_state_node_count() {
        let mut state = AppState::new();
        assert_eq!(state.node_count(), 1);
        state.arena.insert(RenderNode::new(NodeKind::Text));
        assert_eq!(state.node_count(), 2);
    }
}
