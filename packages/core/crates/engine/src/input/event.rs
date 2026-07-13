use super::{ClipboardInput, KeyboardInput, MouseInput};

#[derive(Debug, Clone)]
pub enum InputEventType {
    Keyboard(KeyboardInput),
    Mouse(MouseInput),
    Clipboard(ClipboardInput),
    Resize(u16, u16),
    Focus,
    Blur,
    Paste(String),
}

#[derive(Debug, Clone)]
pub struct InputEvent {
    pub event_type: InputEventType,
    pub timestamp: u64,
}

impl InputEvent {
    pub fn new(event_type: InputEventType) -> Self {
        Self {
            event_type,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }

    pub fn keyboard(input: KeyboardInput) -> Self {
        Self::new(InputEventType::Keyboard(input))
    }

    pub fn mouse(input: MouseInput) -> Self {
        Self::new(InputEventType::Mouse(input))
    }

    pub fn clipboard(input: ClipboardInput) -> Self {
        Self::new(InputEventType::Clipboard(input))
    }

    pub fn resize(width: u16, height: u16) -> Self {
        Self::new(InputEventType::Resize(width, height))
    }

    pub fn focus() -> Self {
        Self::new(InputEventType::Focus)
    }

    pub fn blur() -> Self {
        Self::new(InputEventType::Blur)
    }

    pub fn paste(data: String) -> Self {
        Self::new(InputEventType::Paste(data))
    }

    pub fn is_keyboard(&self) -> bool {
        matches!(self.event_type, InputEventType::Keyboard(_))
    }

    pub fn is_mouse(&self) -> bool {
        matches!(self.event_type, InputEventType::Mouse(_))
    }

    pub fn is_clipboard(&self) -> bool {
        matches!(self.event_type, InputEventType::Clipboard(_))
    }

    pub fn is_resize(&self) -> bool {
        matches!(self.event_type, InputEventType::Resize(_, _))
    }

    pub fn is_focus(&self) -> bool {
        matches!(self.event_type, InputEventType::Focus)
    }

    pub fn is_blur(&self) -> bool {
        matches!(self.event_type, InputEventType::Blur)
    }

    pub fn is_paste(&self) -> bool {
        matches!(self.event_type, InputEventType::Paste(_))
    }
}

impl Default for InputEvent {
    fn default() -> Self {
        Self::new(InputEventType::Focus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_event_new() {
        let event = InputEvent::new(InputEventType::Focus);
        assert!(event.is_focus());
    }

    #[test]
    fn input_event_default() {
        let event = InputEvent::default();
        assert!(event.is_focus());
    }

    #[test]
    fn input_event_keyboard() {
        let input = KeyboardInput::new('a', super::super::keyboard::KeyModifiers::empty());
        let event = InputEvent::keyboard(input);
        assert!(event.is_keyboard());
    }

    #[test]
    fn input_event_mouse() {
        let input = MouseInput::new(0, 0, super::super::mouse::MouseButton::empty());
        let event = InputEvent::mouse(input);
        assert!(event.is_mouse());
    }

    #[test]
    fn input_event_clipboard() {
        let input = ClipboardInput::copy("hello".to_string());
        let event = InputEvent::clipboard(input);
        assert!(event.is_clipboard());
    }

    #[test]
    fn input_event_resize() {
        let event = InputEvent::resize(80, 24);
        assert!(event.is_resize());
    }

    #[test]
    fn input_event_focus() {
        let event = InputEvent::focus();
        assert!(event.is_focus());
    }

    #[test]
    fn input_event_blur() {
        let event = InputEvent::blur();
        assert!(event.is_blur());
    }

    #[test]
    fn input_event_paste() {
        let event = InputEvent::paste("hello".to_string());
        assert!(event.is_paste());
    }
}
