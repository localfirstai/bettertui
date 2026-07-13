use super::events::{FocusEvent, FocusEventType};
use super::{FocusId, FocusScope, FocusState};
use crate::tree::NodeId;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FocusManager {
    nodes: HashMap<NodeId, FocusState>,
    focused: Option<FocusId>,
    previous: Option<FocusId>,
    scopes: Vec<FocusScope>,
    tab_order: Vec<NodeId>,
}

impl Default for FocusManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FocusManager {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            focused: None,
            previous: None,
            scopes: Vec::new(),
            tab_order: Vec::new(),
        }
    }

    pub fn register(&mut self, node_id: NodeId, state: FocusState) {
        self.nodes.insert(node_id, state);
        self.update_tab_order();
    }

    pub fn unregister(&mut self, node_id: NodeId) {
        self.nodes.remove(&node_id);
        if self.focused.map(|f| f.node_id()) == Some(node_id) {
            self.focused = None;
        }
        self.update_tab_order();
    }

    pub fn focus(&mut self, node_id: NodeId) -> Option<FocusEvent> {
        if !self.is_focusable(node_id) {
            return None;
        }

        let old_focused = self.focused;
        let new_focused = Some(FocusId::new(node_id));

        if old_focused == new_focused {
            return None;
        }

        self.previous = old_focused;
        self.focused = new_focused;

        let mut events = Vec::new();

        if let Some(old_id) = old_focused
            && let Some(state) = self.nodes.get_mut(&old_id.node_id())
        {
            state.focused = None;
            events.push(FocusEvent {
                node_id: old_id.node_id(),
                event_type: FocusEventType::Blur,
            });
        }

        if let Some(state) = self.nodes.get_mut(&node_id) {
            state.focused = Some(FocusId::new(node_id));
            events.push(FocusEvent {
                node_id,
                event_type: FocusEventType::Focus,
            });
        }

        events.first().cloned()
    }

    pub fn blur(&mut self, node_id: NodeId) -> Option<FocusEvent> {
        if self.focused.map(|f| f.node_id()) == Some(node_id) {
            self.focused = None;
            if let Some(state) = self.nodes.get_mut(&node_id) {
                state.focused = None;
            }
            Some(FocusEvent {
                node_id,
                event_type: FocusEventType::Blur,
            })
        } else {
            None
        }
    }

    pub fn focused(&self) -> Option<NodeId> {
        self.focused.map(|f| f.node_id())
    }

    pub fn previous(&self) -> Option<NodeId> {
        self.previous.map(|f| f.node_id())
    }

    pub fn restore(&mut self) -> Option<FocusEvent> {
        if let Some(prev) = self.previous {
            self.focus(prev.node_id())
        } else {
            None
        }
    }

    pub fn is_focusable(&self, node_id: NodeId) -> bool {
        self.nodes
            .get(&node_id)
            .is_some_and(|state| state.is_focusable())
    }

    pub fn is_focused(&self, node_id: NodeId) -> bool {
        self.focused.map(|f| f.node_id()) == Some(node_id)
    }

    pub fn focusable_nodes(&self) -> Vec<NodeId> {
        self.nodes
            .iter()
            .filter(|(_, state)| state.is_focusable())
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn tab_order(&self) -> &[NodeId] {
        &self.tab_order
    }

    fn update_tab_order(&mut self) {
        self.tab_order = self
            .nodes
            .iter()
            .filter(|(_, state)| state.is_focusable())
            .map(|(id, state)| (*id, state.tab_index))
            .collect::<Vec<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|(id, _)| id)
            .collect();
        self.tab_order
            .sort_by_key(|id| self.nodes.get(id).map_or(0, |state| state.tab_index));
    }

    pub fn push_scope(&mut self, scope: FocusScope) {
        self.scopes.push(scope);
    }

    pub fn pop_scope(&mut self) -> Option<FocusScope> {
        self.scopes.pop()
    }

    pub fn current_scope(&self) -> Option<&FocusScope> {
        self.scopes.last()
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.focused = None;
        self.previous = None;
        self.scopes.clear();
        self.tab_order.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_manager_new() {
        let manager = FocusManager::new();
        assert!(manager.focused().is_none());
    }

    #[test]
    fn focus_manager_default() {
        let manager = FocusManager::default();
        assert!(manager.focused().is_none());
    }

    #[test]
    fn focus_manager_register() {
        let mut manager = FocusManager::new();
        let node_id = NodeId::default();
        manager.register(node_id, FocusState::new());
        assert!(manager.is_focusable(node_id));
    }

    #[test]
    fn focus_manager_focus() {
        let mut manager = FocusManager::new();
        let node_id = NodeId::default();
        manager.register(node_id, FocusState::new());
        let event = manager.focus(node_id);
        assert!(event.is_some());
        assert_eq!(manager.focused(), Some(node_id));
    }

    #[test]
    fn focus_manager_blur() {
        let mut manager = FocusManager::new();
        let node_id = NodeId::default();
        manager.register(node_id, FocusState::new());
        manager.focus(node_id);
        let event = manager.blur(node_id);
        assert!(event.is_some());
        assert!(manager.focused().is_none());
    }

    #[test]
    fn focus_manager_restore() {
        let mut manager = FocusManager::new();
        let node1 = NodeId::default();
        let node2 = NodeId::default();
        manager.register(node1, FocusState::new());
        manager.register(node2, FocusState::new());
        manager.focus(node1);
        manager.focus(node2);
        manager.restore();
        assert_eq!(manager.focused(), Some(node1));
    }
}
