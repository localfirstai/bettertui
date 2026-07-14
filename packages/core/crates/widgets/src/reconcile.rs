use bettertui_engine::tree::NodeId;

use super::{WidgetId, WidgetTree};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReconcileOp {
    Insert {
        parent: WidgetId,
        widget_id: WidgetId,
        node_id: NodeId,
        kind: &'static str,
    },
    Remove {
        widget_id: WidgetId,
    },
    Move {
        widget_id: WidgetId,
        new_parent: WidgetId,
    },
    Update {
        widget_id: WidgetId,
    },
}

pub struct Reconciler {
    pending_ops: Vec<ReconcileOp>,
}

impl Default for Reconciler {
    fn default() -> Self {
        Self::new()
    }
}

impl Reconciler {
    pub fn new() -> Self {
        Self {
            pending_ops: Vec::new(),
        }
    }

    pub fn reconcile(
        &mut self,
        old_tree: &WidgetTree,
        new_tree: &WidgetTree,
        root: WidgetId,
    ) -> Vec<ReconcileOp> {
        self.pending_ops.clear();
        self.diff_subtree(old_tree, new_tree, root);
        std::mem::take(&mut self.pending_ops)
    }

    fn diff_subtree(&mut self, old_tree: &WidgetTree, new_tree: &WidgetTree, widget_id: WidgetId) {
        let old_entry = old_tree.get(widget_id);
        let new_entry = new_tree.get(widget_id);

        match (old_entry, new_entry) {
            (Some(old), Some(new)) => {
                if old.kind != new.kind {
                    self.pending_ops.push(ReconcileOp::Remove { widget_id });
                    self.pending_ops.push(ReconcileOp::Insert {
                        parent: old.parent.unwrap_or(WidgetId::default()),
                        widget_id,
                        node_id: new.node_id,
                        kind: new.kind,
                    });
                    return;
                }

                if old.parent != new.parent
                    && let Some(new_parent) = new.parent
                {
                    self.pending_ops.push(ReconcileOp::Move {
                        widget_id,
                        new_parent,
                    });
                }

                self.pending_ops.push(ReconcileOp::Update { widget_id });

                let old_children = old_tree.children(widget_id);
                let new_children = new_tree.children(widget_id);

                let mut old_idx = 0;
                let mut new_idx = 0;

                while old_idx < old_children.len() && new_idx < new_children.len() {
                    let old_child = old_children[old_idx];
                    let new_child = new_children[new_idx];

                    if old_child == new_child {
                        self.diff_subtree(old_tree, new_tree, old_child);
                        old_idx += 1;
                        new_idx += 1;
                    } else if new_tree.get(old_child).is_some() {
                        self.diff_subtree(old_tree, new_tree, old_child);
                        old_idx += 1;
                    } else if old_tree.get(new_child).is_some() {
                        self.diff_subtree(old_tree, new_tree, new_child);
                        new_idx += 1;
                    } else {
                        self.diff_subtree(old_tree, new_tree, old_child);
                        old_idx += 1;
                        new_idx += 1;
                    }
                }

                while old_idx < old_children.len() {
                    let child = old_children[old_idx];
                    if new_tree.get(child).is_none() {
                        self.pending_ops
                            .push(ReconcileOp::Remove { widget_id: child });
                    }
                    old_idx += 1;
                }

                while new_idx < new_children.len() {
                    let child = new_children[new_idx];
                    if old_tree.get(child).is_none()
                        && let Some(entry) = new_tree.get(child)
                    {
                        self.pending_ops.push(ReconcileOp::Insert {
                            parent: widget_id,
                            widget_id: child,
                            node_id: entry.node_id,
                            kind: entry.kind,
                        });
                    }
                    new_idx += 1;
                }
            }
            (None, Some(new)) => {
                self.pending_ops.push(ReconcileOp::Insert {
                    parent: new.parent.unwrap_or(WidgetId::default()),
                    widget_id,
                    node_id: new.node_id,
                    kind: new.kind,
                });
            }
            (Some(_old), None) => {
                self.pending_ops.push(ReconcileOp::Remove { widget_id });
            }
            (None, None) => {}
        }
    }

    pub fn ops(&self) -> &[ReconcileOp] {
        &self.pending_ops
    }

    pub fn clear(&mut self) {
        self.pending_ops.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bettertui_engine::tree::NodeArena;
    use bettertui_engine::tree::NodeKind;
    use bettertui_engine::tree::RenderNode;

    fn make_tree_with_entries() -> (WidgetTree, NodeArena) {
        let mut tree = WidgetTree::new();
        let mut arena = NodeArena::new();

        let root_nid = arena.insert(RenderNode::new(NodeKind::Box));
        let child1_nid = arena.insert(RenderNode::new(NodeKind::Text));
        let child2_nid = arena.insert(RenderNode::new(NodeKind::Box));

        let root_wid = WidgetId(root_nid);
        let child1_wid = WidgetId(child1_nid);
        let child2_wid = WidgetId(child2_nid);

        tree.insert(root_wid, root_nid, "Box");
        tree.insert(child1_wid, child1_nid, "Text");
        tree.insert(child2_wid, child2_nid, "Box");
        tree.set_parent(child1_wid, root_wid);
        tree.set_parent(child2_wid, root_wid);

        (tree, arena)
    }

    fn get_root_wid(tree: &WidgetTree) -> WidgetId {
        tree.iter()
            .find(|(_, entry)| entry.parent.is_none())
            .map(|(k, _)| *k)
            .unwrap()
    }

    #[test]
    fn reconciler_new() {
        let reconciler = Reconciler::new();
        assert!(reconciler.ops().is_empty());
    }

    #[test]
    fn reconciler_same_tree() {
        let (tree1, _a1) = make_tree_with_entries();
        let (tree2, _a2) = make_tree_with_entries();

        let mut reconciler = Reconciler::new();
        let root_wid = get_root_wid(&tree1);
        let ops = reconciler.reconcile(&tree1, &tree2, root_wid);

        let updates: Vec<_> = ops
            .iter()
            .filter(|op| matches!(op, ReconcileOp::Update { .. }))
            .collect();
        assert!(!updates.is_empty());
    }

    #[test]
    fn reconciler_added_child() {
        let (tree1, _a1) = make_tree_with_entries();
        let (mut tree2, mut a2) = make_tree_with_entries();

        let new_nid = a2.insert(RenderNode::new(NodeKind::Text));
        let new_wid = WidgetId(new_nid);
        tree2.insert(new_wid, new_nid, "Text");
        let root_wid = get_root_wid(&tree2);
        tree2.set_parent(new_wid, root_wid);

        let mut reconciler = Reconciler::new();
        let ops = reconciler.reconcile(&tree1, &tree2, root_wid);

        let inserts: Vec<_> = ops
            .iter()
            .filter(|op| matches!(op, ReconcileOp::Insert { .. }))
            .collect();
        assert!(!inserts.is_empty());
    }

    #[test]
    fn reconciler_removed_child() {
        let (tree1, _a1) = make_tree_with_entries();
        let (mut tree2, _a2) = make_tree_with_entries();

        let root_wid = get_root_wid(&tree2);
        let children = tree2.children(root_wid);
        if let Some(&child) = children.first() {
            tree2.remove(child);
        }

        let mut reconciler = Reconciler::new();
        let ops = reconciler.reconcile(&tree1, &tree2, root_wid);

        let removes: Vec<_> = ops
            .iter()
            .filter(|op| matches!(op, ReconcileOp::Remove { .. }))
            .collect();
        assert!(!removes.is_empty());
    }

    #[test]
    fn reconciler_clear() {
        let mut reconciler = Reconciler::new();
        reconciler.pending_ops.push(ReconcileOp::Update {
            widget_id: WidgetId(NodeId::default()),
        });
        reconciler.clear();
        assert!(reconciler.ops().is_empty());
    }

    #[test]
    fn reconcile_op_equality() {
        let id = WidgetId(NodeId::default());
        let op1 = ReconcileOp::Update { widget_id: id };
        let op2 = ReconcileOp::Update { widget_id: id };
        assert_eq!(op1, op2);
    }
}
