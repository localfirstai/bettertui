//! Focus management: focus state, manager, scope, traversal, and events.

mod events;
mod manager;
mod scope;
mod traversal;

pub use events::{FocusEvent, FocusEventType};
pub use manager::FocusManager;
pub use scope::FocusScope;
pub use traversal::{FocusDirection, FocusTraversal};

use crate::tree::node_id::NodeId;

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
        Self {
            focused: None,
            previous: None,
            scope: FocusScope::default(),
            tab_index: 0,
            focusable: true,
        }
    }

    pub fn with_focusable(focusable: bool) -> Self {
        Self {
            focusable,
            ..Self::new()
        }
    }

    pub fn with_tab_index(tab_index: i32) -> Self {
        Self {
            tab_index,
            ..Self::new()
        }
    }

    pub fn is_focusable(&self) -> bool {
        self.focusable
    }

    pub fn is_focused(&self) -> bool {
        self.focused.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_id_new() {
        let id = FocusId::new(NodeId::default());
        assert_eq!(id.node_id(), NodeId::default());
    }

    #[test]
    fn focus_id_default() {
        let id = FocusId::default();
        assert_eq!(id.node_id(), NodeId::default());
    }

    #[test]
    fn focus_state_new() {
        let state = FocusState::new();
        assert!(state.is_focusable());
        assert!(!state.is_focused());
    }

    #[test]
    fn focus_state_default() {
        let state = FocusState::default();
        assert!(state.is_focusable());
        assert!(!state.is_focused());
    }

    #[test]
    fn focus_state_with_focusable() {
        let state = FocusState::with_focusable(false);
        assert!(!state.is_focusable());
    }

    #[test]
    fn focus_state_with_tab_index() {
        let state = FocusState::with_tab_index(5);
        assert_eq!(state.tab_index, 5);
    }
}
