use std::collections::HashMap;

use super::compute::{LayoutEngine, LayoutError};
use super::result::LayoutResult;
use crate::tree::NodeId;
use crate::tree::arena::NodeArena;

pub struct LayoutTreeSync {
    layout: LayoutEngine,
    results: HashMap<NodeId, LayoutResult>,
}

impl Default for LayoutTreeSync {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutTreeSync {
    pub fn new() -> Self {
        Self {
            layout: LayoutEngine::new(),
            results: HashMap::new(),
        }
    }

    pub fn sync_full(&mut self, arena: &NodeArena) {
        for (id, node) in arena.iter() {
            if !self.layout.has_node(id) {
                if let Some(text) = &node.text {
                    self.layout.register_text_node(id, &node.layout, text);
                } else {
                    self.layout.register_container(id, &node.layout);
                }
            } else if node.state.layout_dirty {
                if let Some(text) = &node.text {
                    self.layout.update_text(id, text);
                }
                self.layout.update_style(id, &node.layout);
            }
        }
    }

    pub fn sync_node(&mut self, arena: &NodeArena, id: NodeId) {
        if let Some(node) = arena.get(id) {
            if !self.layout.has_node(id) {
                if let Some(text) = &node.text {
                    self.layout.register_text_node(id, &node.layout, text);
                } else {
                    self.layout.register_container(id, &node.layout);
                }
            } else if node.state.layout_dirty {
                if let Some(text) = &node.text {
                    self.layout.update_text(id, text);
                }
                self.layout.update_style(id, &node.layout);
            }
        }
    }

    pub fn remove_node(&mut self, id: NodeId) {
        self.layout.remove_node(id);
    }

    pub fn sync_children(&mut self, arena: &NodeArena, parent: NodeId) {
        let children = arena.children(parent);
        for child in &children {
            self.sync_node(arena, *child);
            self.layout.add_child(parent, *child);
        }
    }

    pub fn compute(&mut self, root: NodeId, width: u16, height: u16) -> Result<(), LayoutError> {
        self.layout
            .compute_layout(root, width as f32, height as f32)?;
        self.results = self.layout.collect_results();
        Ok(())
    }

    pub fn results(&self) -> &HashMap<NodeId, LayoutResult> {
        &self.results
    }

    pub fn node_count(&self) -> usize {
        self.layout.node_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::types::Sizing;
    use crate::tree::arena::NodeArena;
    use crate::tree::node_kind::NodeKind;
    use crate::tree::render_node::RenderNode;

    #[test]
    fn sync_full_basic() {
        let arena = NodeArena::new();
        let mut sync = LayoutTreeSync::new();
        sync.sync_full(&arena);
        assert_eq!(sync.node_count(), 1);
    }

    #[test]
    fn sync_node_with_children() {
        let mut arena = NodeArena::new();
        let child = arena.insert(RenderNode::new(NodeKind::Box));
        arena.append_child(arena.root(), child).unwrap();

        let mut sync = LayoutTreeSync::new();
        sync.sync_node(&arena, arena.root());
        sync.sync_node(&arena, child);
        sync.sync_children(&arena, arena.root());
        assert_eq!(sync.node_count(), 2);
    }

    #[test]
    fn compute_layout() {
        let arena = NodeArena::new();
        let mut sync = LayoutTreeSync::new();
        sync.sync_full(&arena);
        sync.compute(arena.root(), 80, 24).unwrap();
        let results = sync.results();
        assert!(results.contains_key(&arena.root()));
    }

    #[test]
    fn remove_node_from_layout() {
        let mut arena = NodeArena::new();
        let child = arena.insert(RenderNode::new(NodeKind::Box));
        arena.append_child(arena.root(), child).unwrap();

        let mut sync = LayoutTreeSync::new();
        sync.sync_full(&arena);
        assert_eq!(sync.node_count(), 2);
        sync.remove_node(child);
        assert_eq!(sync.node_count(), 1);
    }

    #[test]
    fn sync_adds_new_children() {
        let mut arena = NodeArena::new();
        let child = arena.insert(RenderNode::new(NodeKind::Box));
        arena.append_child(arena.root(), child).unwrap();

        let mut sync = LayoutTreeSync::new();
        sync.sync_full(&arena);
        assert_eq!(sync.node_count(), 2);
    }

    #[test]
    fn sync_updates_existing() {
        let arena = NodeArena::new();
        let mut sync = LayoutTreeSync::new();
        sync.sync_full(&arena);
        assert_eq!(sync.node_count(), 1);
        sync.sync_full(&arena);
        assert_eq!(sync.node_count(), 1);
    }

    #[test]
    fn sync_with_styled_node() {
        let mut arena = NodeArena::new();
        let mut node = RenderNode::new(NodeKind::Box);
        node.layout.width = Some(Sizing::Points(100.0));
        node.layout.height = Some(Sizing::Points(50.0));
        let id = arena.insert(node);
        arena.append_child(arena.root(), id).unwrap();

        let mut sync = LayoutTreeSync::new();
        sync.sync_full(&arena);
        assert_eq!(sync.node_count(), 2);
        sync.compute(arena.root(), 80, 24).unwrap();
        let results = sync.results();
        assert!(results.contains_key(&id));
    }

    #[test]
    fn sync_children_multiple() {
        let mut arena = NodeArena::new();
        let c1 = arena.insert(RenderNode::new(NodeKind::Box));
        let c2 = arena.insert(RenderNode::new(NodeKind::Box));
        arena.append_child(arena.root(), c1).unwrap();
        arena.append_child(arena.root(), c2).unwrap();

        let mut sync = LayoutTreeSync::new();
        sync.sync_full(&arena);
        assert_eq!(sync.node_count(), 3);
    }

    #[test]
    fn sync_remove_then_add() {
        let mut arena = NodeArena::new();
        let child = arena.insert(RenderNode::new(NodeKind::Box));
        arena.append_child(arena.root(), child).unwrap();

        let mut sync = LayoutTreeSync::new();
        sync.sync_full(&arena);
        assert_eq!(sync.node_count(), 2);
        sync.remove_node(child);
        assert_eq!(sync.node_count(), 1);
        sync.sync_node(&arena, child);
        assert_eq!(sync.node_count(), 2);
    }
}
