use crate::tree::WidgetTree;
use bettertui_engine::tree::NodeArena;
use bettertui_engine::tree::NodeId;
use bettertui_engine::tree::RenderNode;

pub struct Pipeline {
    pub dirty: bool,
    pub generation: u64,
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            dirty: true,
            generation: 0,
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    pub fn needs_render(&self) -> bool {
        self.dirty
    }

    pub fn advance_generation(&mut self) -> u64 {
        self.generation += 1;
        self.generation
    }

    pub fn build_render_tree(&self, tree: &WidgetTree, _arena: &NodeArena) -> Vec<NodeId> {
        let mut root_nodes = Vec::new();
        for (widget_id, entry) in tree.iter() {
            if entry.parent.is_none() {
                root_nodes.push(widget_id.node_id());
            }
        }
        root_nodes
    }

    pub fn sync_arena(&self, tree: &WidgetTree, arena: &mut NodeArena) {
        for (widget_id, _entry) in tree.iter() {
            if !arena.contains(widget_id.node_id()) {
                let node = RenderNode::default();
                arena.insert(node);
            }
        }
    }

    pub fn collect_dirty_nodes(&self, tree: &WidgetTree, _arena: &NodeArena) -> Vec<NodeId> {
        let dirty = Vec::new();
        for (_widget_id, _entry) in tree.iter() {}
        dirty
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WidgetId;
    use bettertui_engine::tree::NodeKind;

    fn make_tree() -> (WidgetTree, NodeArena) {
        let mut tree = WidgetTree::new();
        let mut arena = NodeArena::new();

        let root_nid = arena.insert(RenderNode::new(NodeKind::Box));
        let child_nid = arena.insert(RenderNode::new(NodeKind::Text));

        let root_wid = WidgetId(root_nid);
        let child_wid = WidgetId(child_nid);

        tree.insert(root_wid, root_nid, "Box");
        tree.insert(child_wid, child_nid, "Text");
        tree.set_parent(child_wid, root_wid);

        (tree, arena)
    }

    #[test]
    fn pipeline_new() {
        let pipeline = Pipeline::new();
        assert!(pipeline.needs_render());
        assert_eq!(pipeline.generation, 0);
    }

    #[test]
    fn pipeline_mark_dirty() {
        let mut pipeline = Pipeline::new();
        pipeline.clear_dirty();
        assert!(!pipeline.needs_render());
        pipeline.mark_dirty();
        assert!(pipeline.needs_render());
    }

    #[test]
    fn pipeline_advance_generation() {
        let mut pipeline = Pipeline::new();
        assert_eq!(pipeline.advance_generation(), 1);
        assert_eq!(pipeline.advance_generation(), 2);
    }

    #[test]
    fn pipeline_build_render_tree() {
        let (tree, arena) = make_tree();
        let pipeline = Pipeline::new();
        let roots = pipeline.build_render_tree(&tree, &arena);
        assert_eq!(roots.len(), 1);
    }

    #[test]
    fn pipeline_sync_arena() {
        let (tree, mut arena) = make_tree();
        let pipeline = Pipeline::new();
        pipeline.sync_arena(&tree, &mut arena);
        assert!(arena.len() >= 2);
    }

    #[test]
    fn pipeline_collect_dirty() {
        let (tree, arena) = make_tree();
        let pipeline = Pipeline::new();
        let dirty = pipeline.collect_dirty_nodes(&tree, &arena);
        assert!(dirty.is_empty());
    }
}
