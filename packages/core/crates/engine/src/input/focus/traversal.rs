use super::FocusManager;
use crate::tree::NodeId;

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
                let prev_pos = if pos == 0 {
                    focusable.len() - 1
                } else {
                    pos - 1
                };
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
            FocusDirection::Up
            | FocusDirection::Down
            | FocusDirection::Left
            | FocusDirection::Right => {
                // For now, just use next/previous for directional navigation
                // A real implementation would use spatial navigation
                Self::next(manager)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::FocusState;
    use super::*;

    #[test]
    fn focus_traversal_next() {
        let mut manager = FocusManager::new();
        let node1 = NodeId::default();
        let node2 = NodeId::default();
        manager.register(node1, FocusState::new());
        manager.register(node2, FocusState::new());
        manager.focus(node1);
        let next = FocusTraversal::next(&manager);
        assert_eq!(next, Some(node2));
    }

    #[test]
    fn focus_traversal_previous() {
        let mut manager = FocusManager::new();
        let node1 = NodeId::default();
        let node2 = NodeId::default();
        manager.register(node1, FocusState::new());
        manager.register(node2, FocusState::new());
        manager.focus(node2);
        let prev = FocusTraversal::previous(&manager);
        assert_eq!(prev, Some(node1));
    }

    #[test]
    fn focus_traversal_first() {
        let mut manager = FocusManager::new();
        let node1 = NodeId::default();
        let node2 = NodeId::default();
        manager.register(node1, FocusState::new());
        manager.register(node2, FocusState::new());
        let first = FocusTraversal::first(&manager);
        assert_eq!(first, Some(node1));
    }

    #[test]
    fn focus_traversal_last() {
        let mut manager = FocusManager::new();
        let node1 = NodeId::default();
        let node2 = NodeId::default();
        manager.register(node1, FocusState::new());
        manager.register(node2, FocusState::new());
        let last = FocusTraversal::last(&manager);
        assert_eq!(last, Some(node2));
    }

    #[test]
    fn focus_traversal_empty() {
        let manager = FocusManager::new();
        assert!(FocusTraversal::next(&manager).is_none());
        assert!(FocusTraversal::previous(&manager).is_none());
        assert!(FocusTraversal::first(&manager).is_none());
        assert!(FocusTraversal::last(&manager).is_none());
    }
}
