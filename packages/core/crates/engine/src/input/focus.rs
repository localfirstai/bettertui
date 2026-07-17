//! Focus management: scope, traversal, and state tracking.

use crate::tree::NodeId;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusEventType {
    Focus,
    Blur,
    FocusIn,
    FocusOut,
}

#[derive(Debug, Clone)]
pub struct FocusEvent_ {
    pub node_id: NodeId,
    pub event_type: FocusEventType,
}

impl FocusEvent_ {
    pub fn new(node_id: NodeId, event_type: FocusEventType) -> Self {
        Self { node_id, event_type }
    }

    pub fn is_focus(&self) -> bool {
        self.event_type == FocusEventType::Focus
    }

    pub fn is_blur(&self) -> bool {
        self.event_type == FocusEventType::Blur
    }

    pub fn is_focus_in(&self) -> bool {
        self.event_type == FocusEventType::FocusIn
    }

    pub fn is_focus_out(&self) -> bool {
        self.event_type == FocusEventType::FocusOut
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FocusScopeType {
    Window,
    Panel,
    Modal,
    Popup,
    Tooltip,
}

#[derive(Debug, Clone)]
pub struct FocusScope {
    pub id: NodeId,
    pub scope_type: FocusScopeType,
    pub modal: bool,
    pub trap_focus: bool,
}

impl FocusScope {
    pub fn new(id: NodeId, scope_type: FocusScopeType) -> Self {
        Self { id, scope_type, modal: false, trap_focus: false }
    }

    pub fn with_modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    pub fn with_trap_focus(mut self, trap_focus: bool) -> Self {
        self.trap_focus = trap_focus;
        self
    }

    pub fn is_modal(&self) -> bool {
        self.modal
    }

    pub fn traps_focus(&self) -> bool {
        self.trap_focus
    }
}

impl Default for FocusScope {
    fn default() -> Self {
        Self::new(NodeId::default(), FocusScopeType::Window)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusDirection {
    Forward,
    Backward,
    Up,
    Down,
    Left,
    Right,
    First,
    Last,
}

pub struct FocusTraversal;

impl FocusTraversal {
    pub fn next(manager: &FocusManager) -> Option<NodeId> {
        let focusable = manager.focusable_nodes();
        if focusable.is_empty() {
            return None;
        }

        let current = manager.focused();
        if let Some(current_id) = current {
            if let Some(pos) = focusable.iter().position(|&id| id == current_id) {
                let next_pos = (pos + 1) % focusable.len();
                Some(focusable[next_pos])
            } else {
                Some(focusable[0])
            }
        } else {
            Some(focusable[0])
        }
    }

    pub fn previous(manager: &FocusManager) -> Option<NodeId> {
        let focusable = manager.focusable_nodes();
        if focusable.is_empty() {
            return None;
        }

        let current = manager.focused();
        if let Some(current_id) = current {
            if let Some(pos) = focusable.iter().position(|&id| id == current_id) {
                let prev_pos = if pos == 0 { focusable.len() - 1 } else { pos - 1 };
                Some(focusable[prev_pos])
            } else {
                Some(focusable[focusable.len() - 1])
            }
        } else {
            Some(focusable[focusable.len() - 1])
        }
    }

    pub fn first(manager: &FocusManager) -> Option<NodeId> {
        let focusable = manager.focusable_nodes();
        focusable.into_iter().next()
    }

    pub fn last(manager: &FocusManager) -> Option<NodeId> {
        let focusable = manager.focusable_nodes();
        focusable.into_iter().last()
    }

    pub fn traverse(manager: &FocusManager, direction: FocusDirection) -> Option<NodeId> {
        match direction {
            FocusDirection::Forward => Self::next(manager),
            FocusDirection::Backward => Self::previous(manager),
            FocusDirection::First => Self::first(manager),
            FocusDirection::Last => Self::last(manager),
            FocusDirection::Up | FocusDirection::Down | FocusDirection::Left | FocusDirection::Right => {
                Self::next(manager)
            }
        }
    }
}

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
        Self { nodes: HashMap::new(), focused: None, previous: None, scopes: Vec::new(), tab_order: Vec::new() }
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

    pub fn focus(&mut self, node_id: NodeId) -> Option<FocusEvent_> {
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
            events.push(FocusEvent_ { node_id: old_id.node_id(), event_type: FocusEventType::Blur });
        }

        if let Some(state) = self.nodes.get_mut(&node_id) {
            state.focused = Some(FocusId::new(node_id));
            events.push(FocusEvent_ { node_id, event_type: FocusEventType::Focus });
        }

        events.first().cloned()
    }

    pub fn blur(&mut self, node_id: NodeId) -> Option<FocusEvent_> {
        if self.focused.map(|f| f.node_id()) == Some(node_id) {
            self.focused = None;
            if let Some(state) = self.nodes.get_mut(&node_id) {
                state.focused = None;
            }
            Some(FocusEvent_ { node_id, event_type: FocusEventType::Blur })
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

    pub fn restore(&mut self) -> Option<FocusEvent_> {
        if let Some(prev) = self.previous { self.focus(prev.node_id()) } else { None }
    }

    pub fn is_focusable(&self, node_id: NodeId) -> bool {
        self.nodes.get(&node_id).is_some_and(|state| state.is_focusable())
    }

    pub fn is_focused(&self, node_id: NodeId) -> bool {
        self.focused.map(|f| f.node_id()) == Some(node_id)
    }

    pub fn focusable_nodes(&self) -> Vec<NodeId> {
        self.nodes.iter().filter(|(_, state)| state.is_focusable()).map(|(id, _)| *id).collect()
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
        self.tab_order.sort_by_key(|id| self.nodes.get(id).map_or(0, |state| state.tab_index));
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FocusId(pub NodeId);

impl FocusId {
    pub fn new(node_id: NodeId) -> Self {
        Self(node_id)
    }

    pub fn node_id(&self) -> NodeId {
        self.0
    }
}

#[allow(clippy::derivable_impls)]
impl Default for FocusId {
    fn default() -> Self {
        Self(NodeId::default())
    }
}

#[derive(Debug, Clone)]
pub struct FocusState {
    pub focused: Option<FocusId>,
    pub previous: Option<FocusId>,
    pub scope: FocusScope,
    pub tab_index: i32,
    pub focusable: bool,
}

impl Default for FocusState {
    fn default() -> Self {
        Self::new()
    }
}

impl FocusState {
    pub fn new() -> Self {
        Self { focused: None, previous: None, scope: FocusScope::default(), tab_index: 0, focusable: true }
    }

    pub fn with_focusable(focusable: bool) -> Self {
        Self { focusable, ..Self::new() }
    }

    pub fn with_tab_index(tab_index: i32) -> Self {
        Self { tab_index, ..Self::new() }
    }

    pub fn is_focusable(&self) -> bool {
        self.focusable
    }

    pub fn is_focused(&self) -> bool {
        self.focused.is_some()
    }
}
