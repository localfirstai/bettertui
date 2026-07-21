//! Clipboard input types.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardAction {
    Copy,
    Paste,
    Cut,
}

#[derive(Debug, Clone)]
pub struct ClipboardInput {
    pub data: String,
    pub action: ClipboardAction,
}

impl ClipboardInput {
    pub fn new(action: ClipboardAction, data: String) -> Self {
        Self { data, action }
    }

    pub fn copy(data: String) -> Self {
        Self::new(ClipboardAction::Copy, data)
    }

    pub fn paste(data: String) -> Self {
        Self::new(ClipboardAction::Paste, data)
    }

    pub fn cut(data: String) -> Self {
        Self::new(ClipboardAction::Cut, data)
    }

    pub fn is_copy(&self) -> bool {
        self.action == ClipboardAction::Copy
    }

    pub fn is_paste(&self) -> bool {
        self.action == ClipboardAction::Paste
    }

    pub fn is_cut(&self) -> bool {
        self.action == ClipboardAction::Cut
    }
}

impl Default for ClipboardInput {
    fn default() -> Self {
        Self::new(ClipboardAction::Copy, String::new())
    }
}
