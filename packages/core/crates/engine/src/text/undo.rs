use super::selection::SelectionRange;

#[derive(Debug, Clone)]
pub enum UndoAction {
    InsertChar { pos: usize, ch: char },
    InsertStr { pos: usize, text: String },
    DeleteChar { pos: usize, ch: char },
    DeleteRange { range: SelectionRange, text: String },
}

#[derive(Debug, Clone)]
pub struct UndoManager {
    undo_stack: Vec<UndoAction>,
    redo_stack: Vec<UndoAction>,
    max_undo: usize,
}

impl Default for UndoManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UndoManager {
    pub fn new() -> Self {
        Self { undo_stack: Vec::new(), redo_stack: Vec::new(), max_undo: 1000 }
    }

    pub fn with_max_undo(max_undo: usize) -> Self {
        Self { undo_stack: Vec::new(), redo_stack: Vec::new(), max_undo }
    }

    pub fn push(&mut self, action: UndoAction) {
        self.undo_stack.push(action);
        self.redo_stack.clear();

        if self.undo_stack.len() > self.max_undo {
            self.undo_stack.remove(0);
        }
    }

    pub fn undo(&mut self) -> Option<UndoAction> {
        if let Some(action) = self.undo_stack.pop() {
            self.redo_stack.push(action.clone());
            Some(action)
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<UndoAction> {
        if let Some(action) = self.redo_stack.pop() {
            self.undo_stack.push(action.clone());
            Some(action)
        } else {
            None
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undo_manager_new() {
        let manager = UndoManager::new();
        assert!(!manager.can_undo());
        assert!(!manager.can_redo());
    }

    #[test]
    fn undo_manager_default() {
        let manager = UndoManager::default();
        assert!(!manager.can_undo());
        assert!(!manager.can_redo());
    }

    #[test]
    fn undo_manager_push_undo() {
        let mut manager = UndoManager::new();
        manager.push(UndoAction::InsertChar { pos: 0, ch: 'a' });
        assert!(manager.can_undo());
        assert!(!manager.can_redo());
    }

    #[test]
    fn undo_manager_undo() {
        let mut manager = UndoManager::new();
        manager.push(UndoAction::InsertChar { pos: 0, ch: 'a' });
        let action = manager.undo();
        assert!(action.is_some());
        assert!(!manager.can_undo());
        assert!(manager.can_redo());
    }

    #[test]
    fn undo_manager_redo() {
        let mut manager = UndoManager::new();
        manager.push(UndoAction::InsertChar { pos: 0, ch: 'a' });
        manager.undo();
        let action = manager.redo();
        assert!(action.is_some());
        assert!(manager.can_undo());
        assert!(!manager.can_redo());
    }

    #[test]
    fn undo_manager_clear() {
        let mut manager = UndoManager::new();
        manager.push(UndoAction::InsertChar { pos: 0, ch: 'a' });
        manager.clear();
        assert!(!manager.can_undo());
        assert!(!manager.can_redo());
    }

    #[test]
    fn undo_manager_max_undo() {
        let mut manager = UndoManager::with_max_undo(2);
        manager.push(UndoAction::InsertChar { pos: 0, ch: 'a' });
        manager.push(UndoAction::InsertChar { pos: 1, ch: 'b' });
        manager.push(UndoAction::InsertChar { pos: 2, ch: 'c' });
        assert_eq!(manager.undo_count(), 2);
    }
}
