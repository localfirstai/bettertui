use std::collections::HashMap;

use crate::tree::node_id::NodeId;

use super::WidgetId;

#[derive(Debug, Clone)]
pub struct WidgetEntry {
    pub widget_id: WidgetId,
    pub node_id: NodeId,
    pub parent: Option<WidgetId>,
    pub children: Vec<WidgetId>,
    pub kind: &'static str,
}

pub struct WidgetTree {
    entries: HashMap<WidgetId, WidgetEntry>,
    node_to_widget: HashMap<NodeId, WidgetId>,
    next_id: u64,
}

impl Default for WidgetTree {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetTree {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            node_to_widget: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn next_id(&mut self) -> WidgetId {
        self.next_id += 1;
        WidgetId(crate::tree::NodeId::default())
    }

    pub fn insert(&mut self, widget_id: WidgetId, node_id: NodeId, kind: &'static str) {
        let entry = WidgetEntry {
            widget_id,
            node_id,
            parent: None,
            children: Vec::new(),
            kind,
        };
        self.entries.insert(widget_id, entry);
        self.node_to_widget.insert(node_id, widget_id);
    }

    pub fn set_parent(&mut self, child: WidgetId, parent: WidgetId) {
        if let Some(entry) = self.entries.get_mut(&child) {
            entry.parent = Some(parent);
        }
        if let Some(entry) = self.entries.get_mut(&parent) {
            entry.children.push(child);
        }
    }

    pub fn get(&self, widget_id: WidgetId) -> Option<&WidgetEntry> {
        self.entries.get(&widget_id)
    }

    pub fn get_by_node(&self, node_id: NodeId) -> Option<&WidgetEntry> {
        self.node_to_widget
            .get(&node_id)
            .and_then(|wid| self.entries.get(wid))
    }

    pub fn node_id(&self, widget_id: WidgetId) -> Option<NodeId> {
        self.entries.get(&widget_id).map(|e| e.node_id)
    }

    pub fn widget_id(&self, node_id: NodeId) -> Option<WidgetId> {
        self.node_to_widget.get(&node_id).copied()
    }

    pub fn children(&self, widget_id: WidgetId) -> Vec<WidgetId> {
        self.entries
            .get(&widget_id)
            .map(|e| e.children.clone())
            .unwrap_or_default()
    }

    pub fn parent(&self, widget_id: WidgetId) -> Option<WidgetId> {
        self.entries.get(&widget_id).and_then(|e| e.parent)
    }

    pub fn remove(&mut self, widget_id: WidgetId) -> Option<WidgetEntry> {
        if let Some(entry) = self.entries.remove(&widget_id) {
            self.node_to_widget.remove(&entry.node_id);

            if let Some(parent_id) = entry.parent
                && let Some(parent) = self.entries.get_mut(&parent_id)
            {
                parent.children.retain(|c| *c != widget_id);
            }

            for child in &entry.children {
                self.remove(*child);
            }

            Some(entry)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&WidgetId, &WidgetEntry)> {
        self.entries.iter()
    }

    pub fn widget_ids(&self) -> Vec<WidgetId> {
        self.entries.keys().copied().collect()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.node_to_widget.clear();
        self.next_id = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_wid() -> WidgetId {
        WidgetId(NodeId::default())
    }

    #[test]
    fn tree_new() {
        let tree = WidgetTree::new();
        assert!(tree.is_empty());
    }

    #[test]
    fn tree_insert_and_get() {
        let mut tree = WidgetTree::new();
        let wid = make_wid();
        let nid = NodeId::default();
        tree.insert(wid, nid, "Box");
        assert_eq!(tree.len(), 1);
        assert!(tree.get(wid).is_some());
        assert_eq!(tree.get(wid).unwrap().kind, "Box");
    }

    #[test]
    fn tree_node_to_widget() {
        let mut tree = WidgetTree::new();
        let wid = make_wid();
        let nid = NodeId::default();
        tree.insert(wid, nid, "Text");
        assert_eq!(tree.widget_id(nid), Some(wid));
        assert_eq!(tree.node_id(wid), Some(nid));
    }

    #[test]
    fn tree_parent_child() {
        let mut tree = WidgetTree::new();
        let mut arena = crate::tree::arena::NodeArena::new();
        let parent_nid = arena.insert(crate::tree::render_node::RenderNode::new(
            crate::tree::node_kind::NodeKind::Box,
        ));
        let child_nid = arena.insert(crate::tree::render_node::RenderNode::new(
            crate::tree::node_kind::NodeKind::Text,
        ));
        let parent_wid = WidgetId(parent_nid);
        let child_wid = WidgetId(child_nid);

        tree.insert(parent_wid, parent_nid, "Box");
        tree.insert(child_wid, child_nid, "Text");
        tree.set_parent(child_wid, parent_wid);

        assert_eq!(tree.parent(child_wid), Some(parent_wid));
        assert_eq!(tree.children(parent_wid), vec![child_wid]);
    }

    #[test]
    fn tree_remove() {
        let mut tree = WidgetTree::new();
        let wid = make_wid();
        let nid = NodeId::default();
        tree.insert(wid, nid, "Box");
        tree.remove(wid);
        assert!(tree.is_empty());
        assert!(tree.widget_id(nid).is_none());
    }

    #[test]
    fn tree_remove_cascades() {
        let mut tree = WidgetTree::new();
        let mut arena = crate::tree::arena::NodeArena::new();
        let parent_nid = arena.insert(crate::tree::render_node::RenderNode::new(
            crate::tree::node_kind::NodeKind::Box,
        ));
        let child_nid = arena.insert(crate::tree::render_node::RenderNode::new(
            crate::tree::node_kind::NodeKind::Text,
        ));
        let parent_wid = WidgetId(parent_nid);
        let child_wid = WidgetId(child_nid);

        tree.insert(parent_wid, parent_nid, "Box");
        tree.insert(child_wid, child_nid, "Text");
        tree.set_parent(child_wid, parent_wid);

        tree.remove(parent_wid);
        assert!(tree.is_empty());
    }

    #[test]
    fn tree_clear() {
        let mut tree = WidgetTree::new();
        let mut arena = crate::tree::arena::NodeArena::new();
        let nid1 = arena.insert(crate::tree::render_node::RenderNode::new(
            crate::tree::node_kind::NodeKind::Box,
        ));
        let nid2 = arena.insert(crate::tree::render_node::RenderNode::new(
            crate::tree::node_kind::NodeKind::Text,
        ));
        tree.insert(WidgetId(nid1), nid1, "Box");
        tree.insert(WidgetId(nid2), nid2, "Text");
        tree.clear();
        assert!(tree.is_empty());
    }

    #[test]
    fn tree_iter() {
        let mut tree = WidgetTree::new();
        tree.insert(make_wid(), NodeId::default(), "Box");
        let count = tree.iter().count();
        assert_eq!(count, 1);
    }

    #[test]
    fn tree_widget_ids() {
        let mut tree = WidgetTree::new();
        let mut arena = crate::tree::arena::NodeArena::new();
        let nid1 = arena.insert(crate::tree::render_node::RenderNode::new(
            crate::tree::node_kind::NodeKind::Box,
        ));
        let nid2 = arena.insert(crate::tree::render_node::RenderNode::new(
            crate::tree::node_kind::NodeKind::Text,
        ));
        let w1 = WidgetId(nid1);
        let w2 = WidgetId(nid2);
        tree.insert(w1, nid1, "A");
        tree.insert(w2, nid2, "B");
        let ids = tree.widget_ids();
        assert_eq!(ids.len(), 2);
    }
}
