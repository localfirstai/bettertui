//! Selection manager for tree-level selection state.
//!
//! Tracks which nodes are selected, supports single and multi-select modes,
//! and provides range selection for list-like widgets.

use crate::tree::NodeId;
use std::collections::HashSet;

/// Selection mode determines how selections behave.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionMode {
    /// Only one node can be selected at a time.
    #[default]
    Single,
    /// Multiple nodes can be selected independently.
    Multi,
    /// Shift+click selects a range between anchor and target.
    Range,
}

/// Tracks selection state across the widget tree.
#[derive(Debug, Clone)]
pub struct SelectionManager {
    /// Currently selected node IDs.
    selected: HashSet<NodeId>,
    /// The anchor node for range selection.
    anchor: Option<NodeId>,
    /// The current selection mode.
    mode: SelectionMode,
    /// Maximum number of selections allowed (0 = unlimited).
    max_selections: usize,
}

impl Default for SelectionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectionManager {
    /// Creates a new SelectionManager with single-select mode.
    pub fn new() -> Self {
        Self {
            selected: HashSet::new(),
            anchor: None,
            mode: SelectionMode::Single,
            max_selections: 0,
        }
    }

    /// Sets the selection mode.
    pub fn with_mode(mut self, mode: SelectionMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the maximum number of selections (0 = unlimited).
    pub fn with_max_selections(mut self, max: usize) -> Self {
        self.max_selections = max;
        self
    }

    /// Returns the current selection mode.
    pub fn mode(&self) -> SelectionMode {
        self.mode
    }

    /// Selects a node. Returns true if the selection changed.
    pub fn select(&mut self, id: NodeId) -> bool {
        match self.mode {
            SelectionMode::Single => {
                if self.selected.contains(&id) {
                    return false;
                }
                self.selected.clear();
                self.selected.insert(id);
                self.anchor = Some(id);
                true
            }
            SelectionMode::Multi | SelectionMode::Range => {
                if self.max_selections > 0 && self.selected.len() >= self.max_selections {
                    return false;
                }
                self.anchor = Some(id);
                self.selected.insert(id)
            }
        }
    }

    /// Deselects a node. Returns true if the selection changed.
    pub fn deselect(&mut self, id: &NodeId) -> bool {
        let removed = self.selected.remove(id);
        if removed && self.anchor == Some(*id) {
            self.anchor = self.selected.iter().next().copied();
        }
        removed
    }

    /// Toggles selection on a node. Returns the new selection state.
    pub fn toggle(&mut self, id: NodeId) -> bool {
        if self.selected.contains(&id) {
            self.deselect(&id);
            false
        } else {
            self.select(id);
            true
        }
    }

    /// Selects a range from anchor to the given node (inclusive).
    /// The nodes must be provided in order for the range to work.
    pub fn select_range(&mut self, nodes: &[NodeId], target: NodeId) -> bool {
        if self.mode == SelectionMode::Single {
            return self.select(target);
        }

        let anchor = match self.anchor {
            Some(a) => a,
            None => {
                return self.select(target);
            }
        };

        let start_idx = nodes.iter().position(|n| *n == anchor);
        let end_idx = nodes.iter().position(|n| *n == target);

        match (start_idx, end_idx) {
            (Some(start), Some(end)) => {
                let (lo, hi) = if start <= end {
                    (start, end)
                } else {
                    (end, start)
                };
                let prev_len = self.selected.len();
                for node in &nodes[lo..=hi] {
                    self.selected.insert(*node);
                }
                self.selected.len() != prev_len
            }
            _ => self.select(target),
        }
    }

    /// Clears all selections.
    pub fn clear(&mut self) -> bool {
        let was_empty = self.selected.is_empty();
        self.selected.clear();
        self.anchor = None;
        !was_empty
    }

    /// Returns true if the given node is selected.
    pub fn is_selected(&self, id: &NodeId) -> bool {
        self.selected.contains(id)
    }

    /// Returns the number of selected nodes.
    pub fn len(&self) -> usize {
        self.selected.len()
    }

    /// Returns true if no nodes are selected.
    pub fn is_empty(&self) -> bool {
        self.selected.is_empty()
    }

    /// Returns the currently selected node IDs.
    pub fn selected(&self) -> impl Iterator<Item = &NodeId> {
        self.selected.iter()
    }

    /// Returns the currently selected node IDs as a Vec.
    pub fn selected_ids(&self) -> Vec<NodeId> {
        self.selected.iter().copied().collect()
    }

    /// Returns the anchor node (last explicitly selected node).
    pub fn anchor(&self) -> Option<NodeId> {
        self.anchor
    }

    /// Returns true if the selection is at the maximum capacity.
    pub fn is_full(&self) -> bool {
        self.max_selections > 0 && self.selected.len() >= self.max_selections
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ids(n: usize) -> Vec<NodeId> {
        let mut arena = crate::tree::NodeArena::new();
        (0..n)
            .map(|_| arena.insert(crate::tree::RenderNode::new(crate::tree::NodeKind::Box)))
            .collect()
    }

    #[test]
    fn single_select() {
        let ids = make_ids(3);
        let mut mgr = SelectionManager::new().with_mode(SelectionMode::Single);
        assert!(mgr.select(ids[0]));
        assert!(mgr.is_selected(&ids[0]));
        assert_eq!(mgr.len(), 1);
        // Selecting a new node clears the old one
        assert!(mgr.select(ids[1]));
        assert!(!mgr.is_selected(&ids[0]));
        assert!(mgr.is_selected(&ids[1]));
        assert_eq!(mgr.len(), 1);
    }

    #[test]
    fn multi_select() {
        let ids = make_ids(3);
        let mut mgr = SelectionManager::new().with_mode(SelectionMode::Multi);
        assert!(mgr.select(ids[0]));
        assert!(mgr.select(ids[1]));
        assert!(mgr.select(ids[2]));
        assert_eq!(mgr.len(), 3);
    }

    #[test]
    fn deselect() {
        let ids = make_ids(2);
        let mut mgr = SelectionManager::new();
        mgr.select(ids[0]);
        assert!(mgr.deselect(&ids[0]));
        assert!(mgr.is_empty());
    }

    #[test]
    fn toggle() {
        let ids = make_ids(1);
        let mut mgr = SelectionManager::new();
        mgr.toggle(ids[0]);
        assert!(mgr.is_selected(&ids[0]));
        mgr.toggle(ids[0]);
        assert!(!mgr.is_selected(&ids[0]));
    }

    #[test]
    fn clear() {
        let ids = make_ids(3);
        let mut mgr = SelectionManager::new().with_mode(SelectionMode::Multi);
        mgr.select(ids[0]);
        mgr.select(ids[1]);
        assert!(mgr.clear());
        assert!(mgr.is_empty());
    }

    #[test]
    fn max_selections() {
        let ids = make_ids(3);
        let mut mgr = SelectionManager::new()
            .with_mode(SelectionMode::Multi)
            .with_max_selections(2);
        assert!(mgr.select(ids[0]));
        assert!(mgr.select(ids[1]));
        assert!(!mgr.select(ids[2])); // at max
        assert!(mgr.is_full());
    }

    #[test]
    fn anchor_tracks_last_select() {
        let ids = make_ids(2);
        let mut mgr = SelectionManager::new().with_mode(SelectionMode::Multi);
        mgr.select(ids[0]);
        assert_eq!(mgr.anchor(), Some(ids[0]));
        mgr.select(ids[1]);
        assert_eq!(mgr.anchor(), Some(ids[1]));
    }

    #[test]
    fn range_select() {
        let ids = make_ids(5);
        let mut mgr = SelectionManager::new().with_mode(SelectionMode::Range);
        mgr.select(ids[0]);
        mgr.select_range(&ids, ids[3]);
        // ids[0] through ids[3] should be selected
        assert!(mgr.is_selected(&ids[0]));
        assert!(mgr.is_selected(&ids[1]));
        assert!(mgr.is_selected(&ids[2]));
        assert!(mgr.is_selected(&ids[3]));
        assert!(!mgr.is_selected(&ids[4]));
    }
}
