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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clipboard_input_new() {
        let input = ClipboardInput::new(ClipboardAction::Copy, "hello".to_string());
        assert!(input.is_copy());
        assert_eq!(input.data, "hello");
    }

    #[test]
    fn clipboard_input_default() {
        let input = ClipboardInput::default();
        assert!(input.is_copy());
        assert!(input.data.is_empty());
    }

    #[test]
    fn clipboard_input_copy() {
        let input = ClipboardInput::copy("hello".to_string());
        assert!(input.is_copy());
        assert_eq!(input.data, "hello");
    }

    #[test]
    fn clipboard_input_paste() {
        let input = ClipboardInput::paste("hello".to_string());
        assert!(input.is_paste());
        assert_eq!(input.data, "hello");
    }

    #[test]
    fn clipboard_input_cut() {
        let input = ClipboardInput::cut("hello".to_string());
        assert!(input.is_cut());
        assert_eq!(input.data, "hello");
    }

    #[test]
    fn clipboard_input_is_copy() {
        let input = ClipboardInput::copy("hello".to_string());
        assert!(input.is_copy());
    }

    #[test]
    fn clipboard_input_is_paste() {
        let input = ClipboardInput::paste("hello".to_string());
        assert!(input.is_paste());
    }

    #[test]
    fn clipboard_input_is_cut() {
        let input = ClipboardInput::cut("hello".to_string());
        assert!(input.is_cut());
    }
}
