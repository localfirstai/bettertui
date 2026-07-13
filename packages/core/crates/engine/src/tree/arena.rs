use std::fmt;

use slotmap::SlotMap;
use smallvec::SmallVec;

use super::{node_id::NodeId, node_kind::NodeKind, render_node::RenderNode, tree_error::TreeError};

/// Arena-allocated node storage backed by `slotmap::SlotMap`.
///
/// Provides O(1) insertion, O(1) access, O(1) removal.
/// Generational indices prevent use-after-free.
///
/// The arena maintains a **tree invariant**: every node has exactly one parent
/// (except root, which has none). Violations are caught at operation time.
pub struct NodeArena {
    nodes: SlotMap<NodeId, RenderNode>,
    root: NodeId,
    /// Incremented on every structural change (insert, remove, tree ops)
    generation: u64,
    /// Incremented on every change including property mutations via CommandProcessor
    change_count: u64,
}

impl Default for NodeArena {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeArena {
    /// Create a new arena with a root node.
    pub fn new() -> Self {
        let mut nodes = SlotMap::with_key();
        let root = nodes.insert(RenderNode {
            kind: NodeKind::Box,
            ..Default::default()
        });
        Self {
            nodes,
            root,
            generation: 0,
            change_count: 0,
        }
    }

    /// Mark arena as changed (for property mutations from CommandProcessor).
    pub fn mark_changed(&mut self) {
        self.change_count += 1;
    }

    /// Get the total number of changes since creation.
    pub fn change_count(&self) -> u64 {
        self.change_count
    }

    /// Insert a node into the arena. Returns its NodeId.
    pub fn insert(&mut self, node: RenderNode) -> NodeId {
        let id = self.nodes.insert(node);
        self.nodes[id].id = id;
        self.generation += 1;
        self.mark_changed();
        id
    }

    /// Get a reference to a node by ID.
    pub fn get(&self, id: NodeId) -> Option<&RenderNode> {
        self.nodes.get(id)
    }

    /// Get a mutable reference to a node by ID.
    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut RenderNode> {
        self.nodes.get_mut(id)
    }

    /// Remove a node from the arena, returning it.
    pub fn remove(&mut self, id: NodeId) -> Option<RenderNode> {
        if id == self.root {
            return None;
        }
        self.generation += 1;
        self.mark_changed();
        self.nodes.remove(id)
    }

    /// Check if a node exists in the arena.
    pub fn contains(&self, id: NodeId) -> bool {
        self.nodes.contains_key(id)
    }

    /// Number of nodes in the arena (including root).
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns true if the arena has no nodes (should never happen since root always exists).
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Remove all nodes from the arena, keeping only root.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.root = self.nodes.insert(RenderNode {
            kind: NodeKind::Box,
            ..Default::default()
        });
        self.nodes[self.root].id = self.root;
        self.generation += 1;
        self.mark_changed();
    }

    /// Get the root node ID.
    pub fn root(&self) -> NodeId {
        self.root
    }

    /// Get the current generation ( incremented on every mutation).
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Iterate all nodes as (NodeId, &RenderNode).
    pub fn iter(&self) -> impl Iterator<Item = (NodeId, &RenderNode)> {
        self.nodes.iter()
    }

    /// Iterate all nodes mutably as (NodeId, &mut RenderNode).
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (NodeId, &mut RenderNode)> {
        self.nodes.iter_mut()
    }

    /// Get direct children of a node.
    pub fn children(&self, id: NodeId) -> SmallVec<[NodeId; 4]> {
        self.nodes
            .get(id)
            .map(|n| n.children.clone())
            .unwrap_or_default()
    }

    /// Get descendants of a node in DFS order.
    pub fn descendants(&self, id: NodeId) -> Vec<NodeId> {
        let mut result = Vec::new();
        self.descendants_recursive(id, &mut result);
        result
    }

    fn descendants_recursive(&self, id: NodeId, result: &mut Vec<NodeId>) {
        if let Some(node) = self.nodes.get(id) {
            for &child in &node.children {
                result.push(child);
                self.descendants_recursive(child, result);
            }
        }
    }

    /// Get ancestors of a node from parent to root.
    pub fn ancestors(&self, id: NodeId) -> Vec<NodeId> {
        let mut result = Vec::new();
        let mut current = id;
        while let Some(node) = self.nodes.get(current) {
            if let Some(parent) = node.parent {
                result.push(parent);
                current = parent;
            } else {
                break;
            }
        }
        result
    }

    /// Count all descendants of a node (recursive).
    pub fn descendant_count(&self, id: NodeId) -> usize {
        let mut count = 0;
        if let Some(node) = self.nodes.get(id) {
            for &child in &node.children {
                count += 1;
                count += self.descendant_count(child);
            }
        }
        count
    }

    /// Compute the depth of a node (root = 0).
    pub fn depth(&self, id: NodeId) -> u32 {
        let mut depth = 0;
        let mut current = id;
        while let Some(node) = self.nodes.get(current) {
            if let Some(parent) = node.parent {
                depth += 1;
                current = parent;
            } else {
                break;
            }
        }
        depth
    }

    /// Check if `ancestor` is an ancestor of `descendant`.
    pub fn is_ancestor(&self, ancestor: NodeId, descendant: NodeId) -> bool {
        let mut current = descendant;
        while let Some(node) = self.nodes.get(current) {
            if let Some(parent) = node.parent {
                if parent == ancestor {
                    return true;
                }
                current = parent;
            } else {
                break;
            }
        }
        false
    }

    // ─── Tree Operations ───────────────────────────────────────────

    /// Append a child to a parent node.
    pub fn append_child(&mut self, parent: NodeId, child: NodeId) -> Result<(), TreeError> {
        if !self.contains(parent) {
            return Err(TreeError::NodeNotFound(parent));
        }
        if !self.contains(child) {
            return Err(TreeError::NodeNotFound(child));
        }
        if child == self.root {
            return Err(TreeError::InvalidOperation(
                "Cannot append root as child".into(),
            ));
        }
        if self.is_ancestor(child, parent) {
            return Err(TreeError::CycleDetected {
                node: child,
                ancestor: parent,
            });
        }

        // Detach child from current parent if any
        if let Some(_current_parent) = self.nodes[child].parent {
            self.detach(child);
        }

        self.nodes[child].parent = Some(parent);
        self.nodes[parent].children.push(child);
        self.generation += 1;
        self.mark_changed();
        Ok(())
    }

    /// Insert a child before a reference node.
    pub fn insert_before(&mut self, reference: NodeId, child: NodeId) -> Result<(), TreeError> {
        if !self.contains(reference) {
            return Err(TreeError::NodeNotFound(reference));
        }
        if !self.contains(child) {
            return Err(TreeError::NodeNotFound(child));
        }
        if child == self.root {
            return Err(TreeError::InvalidOperation(
                "Cannot insert root as child".into(),
            ));
        }
        if self.is_ancestor(child, reference) {
            return Err(TreeError::CycleDetected {
                node: child,
                ancestor: reference,
            });
        }

        let parent = self.nodes[reference]
            .parent
            .ok_or(TreeError::InvalidOperation(
                "Reference node has no parent".into(),
            ))?;

        // Detach child from current parent if any
        if let Some(_current_parent) = self.nodes[child].parent {
            self.detach(child);
        }

        // Find the index of the reference node in parent's children
        if let Some(parent_node) = self.nodes.get_mut(parent) {
            if let Some(idx) = parent_node.children.iter().position(|&id| id == reference) {
                parent_node.children.insert(idx, child);
            } else {
                return Err(TreeError::InvalidOperation(
                    "Reference node not found in parent's children".into(),
                ));
            }
        }

        self.nodes[child].parent = Some(parent);
        self.generation += 1;
        self.mark_changed();
        Ok(())
    }

    /// Move a node to a new parent.
    pub fn move_node(&mut self, node: NodeId, new_parent: NodeId) -> Result<(), TreeError> {
        if !self.contains(node) {
            return Err(TreeError::NodeNotFound(node));
        }
        if !self.contains(new_parent) {
            return Err(TreeError::NodeNotFound(new_parent));
        }
        if node == self.root {
            return Err(TreeError::InvalidOperation("Cannot move root".into()));
        }
        if self.is_ancestor(node, new_parent) {
            return Err(TreeError::CycleDetected {
                node,
                ancestor: new_parent,
            });
        }

        // Detach from current parent
        self.detach(node);

        // Append to new parent
        self.append_child(new_parent, node)
    }

    /// Replace one node with another.
    pub fn replace_node(&mut self, old: NodeId, new: NodeId) -> Result<(), TreeError> {
        if !self.contains(old) {
            return Err(TreeError::NodeNotFound(old));
        }
        if !self.contains(new) {
            return Err(TreeError::NodeNotFound(new));
        }
        if old == self.root {
            return Err(TreeError::InvalidOperation("Cannot replace root".into()));
        }
        if new == self.root {
            return Err(TreeError::InvalidOperation(
                "Cannot replace with root".into(),
            ));
        }

        let parent = self.nodes[old]
            .parent
            .ok_or(TreeError::InvalidOperation("Old node has no parent".into()))?;

        // Move all children from old to new
        let old_children: SmallVec<[NodeId; 4]> = self.nodes[old].children.clone();
        for &child in &old_children {
            self.nodes[child].parent = Some(new);
            self.nodes[new].children.push(child);
        }
        self.nodes[old].children.clear();

        // Replace old with new in parent's children
        if let Some(parent_node) = self.nodes.get_mut(parent)
            && let Some(idx) = parent_node.children.iter().position(|&id| id == old)
        {
            parent_node.children[idx] = new;
        }

        self.nodes[new].parent = Some(parent);
        self.nodes.remove(old);
        self.generation += 1;
        self.mark_changed();
        Ok(())
    }

    /// Remove a node and all its descendants from the arena.
    pub fn remove_subtree(&mut self, id: NodeId) {
        if id == self.root {
            // Don't remove root, just clear children
            let children: SmallVec<[NodeId; 4]> = self.nodes[self.root].children.clone();
            for child in children {
                self.remove_subtree_recursive(child);
            }
            self.nodes[self.root].children.clear();
            return;
        }
        self.remove_subtree_recursive(id);
        self.generation += 1;
        self.mark_changed();
    }

    fn remove_subtree_recursive(&mut self, id: NodeId) {
        let children: SmallVec<[NodeId; 4]> = self
            .nodes
            .get(id)
            .map(|n| n.children.clone())
            .unwrap_or_default();
        for child in children {
            self.remove_subtree_recursive(child);
        }
        self.nodes.remove(id);
    }

    /// Detach a node from its parent (but keep in arena).
    pub fn detach(&mut self, id: NodeId) {
        if id == self.root {
            return;
        }

        let parent = match self.nodes.get(id).and_then(|n| n.parent) {
            Some(p) => p,
            None => return,
        };

        // Remove from parent's children
        if let Some(parent_node) = self.nodes.get_mut(parent) {
            parent_node.children.retain(|c| *c != id);
        }

        // Clear parent reference
        if let Some(node) = self.nodes.get_mut(id) {
            node.parent = None;
        }

        self.generation += 1;
        self.mark_changed();
    }

    /// Validate tree invariants. Returns Ok(()) if valid.
    pub fn validate(&self) -> Result<(), TreeError> {
        // Check root exists and has no parent
        let root = self
            .nodes
            .get(self.root)
            .ok_or(TreeError::NodeNotFound(self.root))?;
        if root.parent.is_some() {
            return Err(TreeError::InvalidOperation("Root has parent".into()));
        }

        // Check all nodes have consistent parent-child relationships
        for (id, node) in &self.nodes {
            if id == self.root {
                continue;
            }

            // Check parent exists
            let parent_id = node.parent.ok_or(TreeError::InvalidOperation(format!(
                "Non-root node {id:?} has no parent"
            )))?;

            if !self.contains(parent_id) {
                return Err(TreeError::InvalidOperation(format!(
                    "Node {id:?} references non-existent parent {parent_id:?}"
                )));
            }

            // Check this node is in parent's children
            let parent_node = &self.nodes[parent_id];
            if !parent_node.children.contains(&id) {
                return Err(TreeError::InvalidOperation(format!(
                    "Node {id:?} claims parent {parent_id:?} but is not in parent's children"
                )));
            }
        }

        // Check all children exist in arena
        for (id, node) in &self.nodes {
            for &child in &node.children {
                if !self.contains(child) {
                    return Err(TreeError::InvalidOperation(format!(
                        "Node {id:?} references non-existent child {child:?}"
                    )));
                }
            }
        }

        Ok(())
    }

    /// Print the tree in a human-readable format for debugging.
    pub fn print_tree(&self) -> String {
        let mut output = String::new();
        self.print_node(self.root, &mut output, "", true);
        output
    }

    fn print_node(&self, id: NodeId, output: &mut String, prefix: &str, is_last: bool) {
        if let Some(node) = self.nodes.get(id) {
            let connector = if is_last { "└── " } else { "├── " };
            let kind_name = node.kind.name();
            let text_preview = node
                .text
                .as_ref()
                .map(|t| format!(" \"{}\"", t))
                .unwrap_or_default();
            output.push_str(&format!("{prefix}{connector}{kind_name}{text_preview}\n"));

            let child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            let child_count = node.children.len();
            for (i, &child) in node.children.iter().enumerate() {
                self.print_node(child, output, &child_prefix, i == child_count - 1);
            }
        }
    }
}

impl fmt::Debug for NodeArena {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeArena")
            .field("len", &self.len())
            .field("generation", &self.generation)
            .field("root", &self.root)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_arena() -> NodeArena {
        NodeArena::new()
    }

    #[test]
    fn arena_new_has_root() {
        let arena = create_test_arena();
        assert_eq!(arena.len(), 1);
        assert!(!arena.is_empty());
        assert!(arena.contains(arena.root()));
    }

    #[test]
    fn insert_node() {
        let mut arena = create_test_arena();
        let node = arena.insert(RenderNode::new(NodeKind::Text));
        assert!(arena.contains(node));
        assert_eq!(arena.len(), 2);
    }

    #[test]
    fn get_node() {
        let mut arena = create_test_arena();
        let node = arena.insert(RenderNode::new(NodeKind::Text));
        let retrieved = arena.get(node);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().kind, NodeKind::Text);
    }

    #[test]
    fn remove_node() {
        let mut arena = create_test_arena();
        let node = arena.insert(RenderNode::new(NodeKind::Text));
        let removed = arena.remove(node);
        assert!(removed.is_some());
        assert!(!arena.contains(node));
        assert_eq!(arena.len(), 1);
    }

    #[test]
    fn cannot_remove_root() {
        let mut arena = create_test_arena();
        let removed = arena.remove(arena.root());
        assert!(removed.is_none());
        assert_eq!(arena.len(), 1);
    }

    #[test]
    fn append_child() {
        let mut arena = create_test_arena();
        let child = arena.insert(RenderNode::new(NodeKind::Text));
        arena.append_child(arena.root(), child).unwrap();

        let root = arena.get(arena.root()).unwrap();
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0], child);

        let child_node = arena.get(child).unwrap();
        assert_eq!(child_node.parent, Some(arena.root()));
    }

    #[test]
    fn append_multiple_children() {
        let mut arena = create_test_arena();
        let c1 = arena.insert(RenderNode::new(NodeKind::Text));
        let c2 = arena.insert(RenderNode::new(NodeKind::Box));
        let c3 = arena.insert(RenderNode::new(NodeKind::Flex));

        arena.append_child(arena.root(), c1).unwrap();
        arena.append_child(arena.root(), c2).unwrap();
        arena.append_child(arena.root(), c3).unwrap();

        let children = arena.children(arena.root());
        assert_eq!(children.len(), 3);
        assert_eq!(children[0], c1);
        assert_eq!(children[1], c2);
        assert_eq!(children[2], c3);
    }

    #[test]
    fn insert_before() {
        let mut arena = create_test_arena();
        let c1 = arena.insert(RenderNode::new(NodeKind::Text));
        let c2 = arena.insert(RenderNode::new(NodeKind::Box));
        let c3 = arena.insert(RenderNode::new(NodeKind::Flex));

        arena.append_child(arena.root(), c1).unwrap();
        arena.append_child(arena.root(), c2).unwrap();
        arena.insert_before(c2, c3).unwrap();

        let children = arena.children(arena.root());
        assert_eq!(children.len(), 3);
        assert_eq!(children[0], c1);
        assert_eq!(children[1], c3);
        assert_eq!(children[2], c2);
    }

    #[test]
    fn move_node() {
        let mut arena = create_test_arena();
        let parent1 = arena.insert(RenderNode::new(NodeKind::Box));
        let parent2 = arena.insert(RenderNode::new(NodeKind::Box));
        let child = arena.insert(RenderNode::new(NodeKind::Text));

        arena.append_child(arena.root(), parent1).unwrap();
        arena.append_child(arena.root(), parent2).unwrap();
        arena.append_child(parent1, child).unwrap();

        arena.move_node(child, parent2).unwrap();

        assert!(arena.children(parent1).is_empty());
        assert_eq!(arena.children(parent2).len(), 1);
        assert_eq!(arena.children(parent2)[0], child);
    }

    #[test]
    fn detach_node() {
        let mut arena = create_test_arena();
        let child = arena.insert(RenderNode::new(NodeKind::Text));
        arena.append_child(arena.root(), child).unwrap();

        arena.detach(child);

        let root = arena.get(arena.root()).unwrap();
        assert!(root.children.is_empty());

        let child_node = arena.get(child).unwrap();
        assert!(child_node.parent.is_none());
    }

    #[test]
    fn remove_subtree() {
        let mut arena = create_test_arena();
        let parent = arena.insert(RenderNode::new(NodeKind::Box));
        let child1 = arena.insert(RenderNode::new(NodeKind::Text));
        let child2 = arena.insert(RenderNode::new(NodeKind::Text));
        let grandchild = arena.insert(RenderNode::new(NodeKind::Text));

        arena.append_child(arena.root(), parent).unwrap();
        arena.append_child(parent, child1).unwrap();
        arena.append_child(parent, child2).unwrap();
        arena.append_child(child1, grandchild).unwrap();

        arena.remove_subtree(parent);

        assert!(!arena.contains(parent));
        assert!(!arena.contains(child1));
        assert!(!arena.contains(child2));
        assert!(!arena.contains(grandchild));
    }

    #[test]
    fn replace_node() {
        let mut arena = create_test_arena();
        let old = arena.insert(RenderNode::new(NodeKind::Text));
        let new = arena.insert(RenderNode::new(NodeKind::Box));
        let child = arena.insert(RenderNode::new(NodeKind::Text));

        arena.append_child(arena.root(), old).unwrap();
        arena.append_child(old, child).unwrap();

        arena.replace_node(old, new).unwrap();

        assert!(!arena.contains(old));
        assert!(arena.contains(new));

        let root = arena.get(arena.root()).unwrap();
        assert!(root.children.contains(&new));

        let new_node = arena.get(new).unwrap();
        assert_eq!(new_node.parent, Some(arena.root()));
        assert!(new_node.children.contains(&child));
    }

    #[test]
    fn descendants() {
        let mut arena = create_test_arena();
        let a = arena.insert(RenderNode::new(NodeKind::Box));
        let b = arena.insert(RenderNode::new(NodeKind::Box));
        let c = arena.insert(RenderNode::new(NodeKind::Text));
        let d = arena.insert(RenderNode::new(NodeKind::Text));

        arena.append_child(arena.root(), a).unwrap();
        arena.append_child(a, b).unwrap();
        arena.append_child(a, c).unwrap();
        arena.append_child(b, d).unwrap();

        let descendants = arena.descendants(arena.root());
        assert_eq!(descendants.len(), 4);
        assert!(descendants.contains(&a));
        assert!(descendants.contains(&b));
        assert!(descendants.contains(&c));
        assert!(descendants.contains(&d));
    }

    #[test]
    fn ancestors() {
        let mut arena = create_test_arena();
        let a = arena.insert(RenderNode::new(NodeKind::Box));
        let b = arena.insert(RenderNode::new(NodeKind::Box));
        let c = arena.insert(RenderNode::new(NodeKind::Text));

        arena.append_child(arena.root(), a).unwrap();
        arena.append_child(a, b).unwrap();
        arena.append_child(b, c).unwrap();

        let ancestors = arena.ancestors(c);
        assert_eq!(ancestors.len(), 3);
        assert_eq!(ancestors[0], b);
        assert_eq!(ancestors[1], a);
        assert_eq!(ancestors[2], arena.root());
    }

    #[test]
    fn depth() {
        let mut arena = create_test_arena();
        assert_eq!(arena.depth(arena.root()), 0);

        let a = arena.insert(RenderNode::new(NodeKind::Box));
        let b = arena.insert(RenderNode::new(NodeKind::Box));

        arena.append_child(arena.root(), a).unwrap();
        arena.append_child(a, b).unwrap();

        assert_eq!(arena.depth(a), 1);
        assert_eq!(arena.depth(b), 2);
    }

    #[test]
    fn is_ancestor() {
        let mut arena = create_test_arena();
        let a = arena.insert(RenderNode::new(NodeKind::Box));
        let b = arena.insert(RenderNode::new(NodeKind::Box));
        let c = arena.insert(RenderNode::new(NodeKind::Text));

        arena.append_child(arena.root(), a).unwrap();
        arena.append_child(a, b).unwrap();
        arena.append_child(b, c).unwrap();

        assert!(arena.is_ancestor(arena.root(), a));
        assert!(arena.is_ancestor(a, c));
        assert!(arena.is_ancestor(arena.root(), c));
        assert!(!arena.is_ancestor(c, a));
        assert!(!arena.is_ancestor(a, arena.root()));
    }

    #[test]
    fn cycle_detection() {
        let mut arena = create_test_arena();
        let a = arena.insert(RenderNode::new(NodeKind::Box));
        let b = arena.insert(RenderNode::new(NodeKind::Box));

        arena.append_child(arena.root(), a).unwrap();
        arena.append_child(a, b).unwrap();

        // Try to make a a child of b (would create cycle)
        let result = arena.append_child(b, a);
        assert!(result.is_err());
    }

    #[test]
    fn validate_tree() {
        let mut arena = create_test_arena();
        let a = arena.insert(RenderNode::new(NodeKind::Box));
        let b = arena.insert(RenderNode::new(NodeKind::Text));

        arena.append_child(arena.root(), a).unwrap();
        arena.append_child(a, b).unwrap();

        assert!(arena.validate().is_ok());
    }

    #[test]
    fn print_tree_output() {
        let mut arena = create_test_arena();
        let a = arena.insert(RenderNode::new(NodeKind::Box));
        let b = arena.insert(RenderNode::text("Hello"));

        arena.append_child(arena.root(), a).unwrap();
        arena.append_child(a, b).unwrap();

        let output = arena.print_tree();
        assert!(output.contains("Box"));
        assert!(output.contains("Text"));
        assert!(output.contains("\"Hello\""));
    }

    #[test]
    fn descendant_count() {
        let mut arena = create_test_arena();
        let a = arena.insert(RenderNode::new(NodeKind::Box));
        let b = arena.insert(RenderNode::new(NodeKind::Box));
        let c = arena.insert(RenderNode::new(NodeKind::Text));

        arena.append_child(arena.root(), a).unwrap();
        arena.append_child(a, b).unwrap();
        arena.append_child(b, c).unwrap();

        assert_eq!(arena.descendant_count(arena.root()), 3);
        assert_eq!(arena.descendant_count(a), 2);
        assert_eq!(arena.descendant_count(b), 1);
        assert_eq!(arena.descendant_count(c), 0);
    }

    #[test]
    fn generation_increments() {
        let mut arena = create_test_arena();
        let gen0 = arena.generation();

        let node = arena.insert(RenderNode::new(NodeKind::Text));
        assert!(arena.generation() > gen0);

        let gen1 = arena.generation();
        arena.append_child(arena.root(), node).unwrap();
        assert!(arena.generation() > gen1);
    }

    #[test]
    fn clear_arena() {
        let mut arena = create_test_arena();
        let a = arena.insert(RenderNode::new(NodeKind::Box));
        let b = arena.insert(RenderNode::new(NodeKind::Text));

        arena.append_child(arena.root(), a).unwrap();
        arena.append_child(a, b).unwrap();

        arena.clear();

        assert_eq!(arena.len(), 1);
        assert!(arena.validate().is_ok());
    }
}
