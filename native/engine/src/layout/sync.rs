use crate::tree::{NodeArena, NodeId};

use super::compute::LayoutEngine;

/// Synchronizes the layout tree with the node arena.
pub struct LayoutTreeSync;

impl LayoutTreeSync {
    /// Sync the entire node arena to the layout engine.
    pub fn sync_full(arena: &NodeArena, layout: &mut LayoutEngine) {
        let root = arena.root();
        Self::sync_subtree(arena, layout, root);
    }

    fn sync_subtree(arena: &NodeArena, layout: &mut LayoutEngine, id: NodeId) {
        let node = match arena.get(id) {
            Some(n) => n,
            None => return,
        };

        if node.has_children() {
            layout.register_container(id);
        } else {
            layout.register_node(id);
        }

        layout.set_style(id, &node.layout);

        let children = node.children.clone();
        for child_id in children {
            Self::sync_subtree(arena, layout, child_id);
            layout.add_child(id, child_id);
        }
    }

    /// Sync a single node's properties to the layout engine.
    pub fn sync_node(arena: &NodeArena, layout: &mut LayoutEngine, id: NodeId) {
        if let Some(node) = arena.get(id) {
            if !layout.has_node(id) {
                if node.has_children() {
                    layout.register_container(id);
                } else {
                    layout.register_node(id);
                }
            }
            layout.set_style(id, &node.layout);
        }
    }

    /// Remove a node from the layout engine.
    pub fn remove_node(arena: &NodeArena, layout: &mut LayoutEngine, id: NodeId) {
        if let Some(node) = arena.get(id) {
            let children = node.children.clone();
            for child_id in children {
                Self::remove_node(arena, layout, child_id);
            }
        }
        layout.remove_node(id);
    }

    /// Compute layout for the arena's tree.
    pub fn compute(
        arena: &NodeArena,
        layout: &mut LayoutEngine,
        width: u16,
        height: u16,
    ) -> Result<(), super::compute::LayoutError> {
        let root = arena.root();
        layout.compute_layout(root, width, height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::LayoutEngine;
    use crate::tree::RenderNode;

    fn create_test_arena() -> NodeArena {
        NodeArena::new()
    }

    #[test]
    fn sync_empty_arena() {
        let arena = create_test_arena();
        let mut layout = LayoutEngine::new();
        LayoutTreeSync::sync_full(&arena, &mut layout);
        assert_eq!(layout.node_count(), 1);
    }

    #[test]
    fn sync_arena_with_children() {
        let mut arena = create_test_arena();
        let root = arena.root();
        let child1 = arena.insert(RenderNode::text("hello"));
        let child2 = arena.insert(RenderNode::box_node());
        arena.append_child(root, child1).unwrap();
        arena.append_child(root, child2).unwrap();

        let mut layout = LayoutEngine::new();
        LayoutTreeSync::sync_full(&arena, &mut layout);
        assert_eq!(layout.node_count(), 3);
    }

    #[test]
    fn sync_nested_tree() {
        let mut arena = create_test_arena();
        let root = arena.root();
        let a = arena.insert(RenderNode::box_node());
        let b = arena.insert(RenderNode::text("b"));
        let c = arena.insert(RenderNode::text("c"));

        arena.append_child(root, a).unwrap();
        arena.append_child(a, b).unwrap();
        arena.append_child(a, c).unwrap();

        let mut layout = LayoutEngine::new();
        LayoutTreeSync::sync_full(&arena, &mut layout);
        assert_eq!(layout.node_count(), 4);
    }

    #[test]
    fn sync_single_node() {
        let mut arena = create_test_arena();
        let root = arena.root();
        let child = arena.insert(RenderNode::text("child"));
        arena.append_child(root, child).unwrap();

        let mut layout = LayoutEngine::new();
        LayoutTreeSync::sync_node(&arena, &mut layout, root);
        assert_eq!(layout.node_count(), 1);

        LayoutTreeSync::sync_node(&arena, &mut layout, child);
        assert_eq!(layout.node_count(), 2);
    }

    #[test]
    fn compute_layout_after_sync() {
        let mut arena = create_test_arena();
        let root = arena.root();
        let child = arena.insert(RenderNode::text("hello"));
        arena.append_child(root, child).unwrap();

        let mut layout = LayoutEngine::new();
        LayoutTreeSync::sync_full(&arena, &mut layout);

        let result = LayoutTreeSync::compute(&arena, &mut layout, 80, 24);
        assert!(result.is_ok());
    }

    #[test]
    fn remove_node_from_layout() {
        let mut arena = create_test_arena();
        let root = arena.root();
        let child = arena.insert(RenderNode::text("child"));
        arena.append_child(root, child).unwrap();

        let mut layout = LayoutEngine::new();
        LayoutTreeSync::sync_full(&arena, &mut layout);
        assert_eq!(layout.node_count(), 2);

        LayoutTreeSync::remove_node(&arena, &mut layout, child);
        assert_eq!(layout.node_count(), 1);
    }

    #[test]
    fn remove_parent_removes_children() {
        let mut arena = create_test_arena();
        let root = arena.root();
        let parent = arena.insert(RenderNode::box_node());
        let child = arena.insert(RenderNode::text("child"));
        arena.append_child(root, parent).unwrap();
        arena.append_child(parent, child).unwrap();

        let mut layout = LayoutEngine::new();
        LayoutTreeSync::sync_full(&arena, &mut layout);
        assert_eq!(layout.node_count(), 3);

        LayoutTreeSync::remove_node(&arena, &mut layout, parent);
        assert_eq!(layout.node_count(), 1);
    }
}
