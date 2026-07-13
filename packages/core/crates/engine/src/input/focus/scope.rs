use crate::tree::node_id::NodeId;

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
        Self {
            id,
            scope_type,
            modal: false,
            trap_focus: false,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_scope_new() {
        let scope = FocusScope::new(NodeId::default(), FocusScopeType::Window);
        assert_eq!(scope.scope_type, FocusScopeType::Window);
        assert!(!scope.is_modal());
        assert!(!scope.traps_focus());
    }

    #[test]
    fn focus_scope_default() {
        let scope = FocusScope::default();
        assert_eq!(scope.scope_type, FocusScopeType::Window);
    }

    #[test]
    fn focus_scope_with_modal() {
        let scope = FocusScope::new(NodeId::default(), FocusScopeType::Modal).with_modal(true);
        assert!(scope.is_modal());
    }

    #[test]
    fn focus_scope_with_trap_focus() {
        let scope = FocusScope::new(NodeId::default(), FocusScopeType::Panel).with_trap_focus(true);
        assert!(scope.traps_focus());
    }
}
