use super::KeyModifiers;
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MouseButton: u8 {
        const LEFT = 0b0001;
        const RIGHT = 0b0010;
        const MIDDLE = 0b0100;
        const EXTRA1 = 0b1000;
        const EXTRA2 = 0b10000;
    }
}

impl Default for MouseButton {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventType {
    Press,
    Release,
    Move,
    Scroll,
    Drag,
    Drop,
}

#[derive(Debug, Clone)]
pub struct MouseInput {
    pub x: u16,
    pub y: u16,
    pub buttons: MouseButton,
    pub modifiers: KeyModifiers,
    pub event_type: MouseEventType,
    pub scroll_delta: Option<(i16, i16)>,
}

impl MouseInput {
    pub fn new(x: u16, y: u16, buttons: MouseButton) -> Self {
        Self {
            x,
            y,
            buttons,
            modifiers: KeyModifiers::empty(),
            event_type: MouseEventType::Press,
            scroll_delta: None,
        }
    }

    pub fn with_modifiers(mut self, modifiers: KeyModifiers) -> Self {
        self.modifiers = modifiers;
        self
    }

    pub fn with_event_type(mut self, event_type: MouseEventType) -> Self {
        self.event_type = event_type;
        self
    }

    pub fn with_scroll_delta(mut self, delta_x: i16, delta_y: i16) -> Self {
        self.scroll_delta = Some((delta_x, delta_y));
        self
    }

    pub fn press(x: u16, y: u16, buttons: MouseButton) -> Self {
        Self::new(x, y, buttons).with_event_type(MouseEventType::Press)
    }

    pub fn release(x: u16, y: u16, buttons: MouseButton) -> Self {
        Self::new(x, y, buttons).with_event_type(MouseEventType::Release)
    }

    pub fn move_to(x: u16, y: u16) -> Self {
        Self::new(x, y, MouseButton::empty()).with_event_type(MouseEventType::Move)
    }

    pub fn scroll(x: u16, y: u16, delta_x: i16, delta_y: i16) -> Self {
        Self::new(x, y, MouseButton::empty())
            .with_event_type(MouseEventType::Scroll)
            .with_scroll_delta(delta_x, delta_y)
    }

    pub fn drag(x: u16, y: u16, buttons: MouseButton) -> Self {
        Self::new(x, y, buttons).with_event_type(MouseEventType::Drag)
    }

    pub fn drop(x: u16, y: u16, buttons: MouseButton) -> Self {
        Self::new(x, y, buttons).with_event_type(MouseEventType::Drop)
    }

    pub fn is_left_button(&self) -> bool {
        self.buttons.contains(MouseButton::LEFT)
    }

    pub fn is_right_button(&self) -> bool {
        self.buttons.contains(MouseButton::RIGHT)
    }

    pub fn is_middle_button(&self) -> bool {
        self.buttons.contains(MouseButton::MIDDLE)
    }

    pub fn is_ctrl(&self) -> bool {
        self.modifiers.contains(KeyModifiers::CONTROL)
    }

    pub fn is_shift(&self) -> bool {
        self.modifiers.contains(KeyModifiers::SHIFT)
    }

    pub fn is_alt(&self) -> bool {
        self.modifiers.contains(KeyModifiers::ALT)
    }
}

impl Default for MouseInput {
    fn default() -> Self {
        Self::new(0, 0, MouseButton::empty())
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MouseEvent {
    pub input: MouseInput,
    pub timestamp: u64,
}

impl MouseEvent {
    #[allow(dead_code)]
    pub fn new(input: MouseInput) -> Self {
        Self {
            input,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mouse_button_empty() {
        let buttons = MouseButton::empty();
        assert!(!buttons.contains(MouseButton::LEFT));
        assert!(!buttons.contains(MouseButton::RIGHT));
        assert!(!buttons.contains(MouseButton::MIDDLE));
    }

    #[test]
    fn mouse_button_default() {
        let buttons = MouseButton::default();
        assert!(buttons.is_empty());
    }

    #[test]
    fn mouse_input_new() {
        let input = MouseInput::new(10, 20, MouseButton::LEFT);
        assert_eq!(input.x, 10);
        assert_eq!(input.y, 20);
        assert!(input.is_left_button());
    }

    #[test]
    fn mouse_input_default() {
        let input = MouseInput::default();
        assert_eq!(input.x, 0);
        assert_eq!(input.y, 0);
    }

    #[test]
    fn mouse_input_press() {
        let input = MouseInput::press(10, 20, MouseButton::LEFT);
        assert_eq!(input.event_type, MouseEventType::Press);
        assert!(input.is_left_button());
    }

    #[test]
    fn mouse_input_release() {
        let input = MouseInput::release(10, 20, MouseButton::RIGHT);
        assert_eq!(input.event_type, MouseEventType::Release);
        assert!(input.is_right_button());
    }

    #[test]
    fn mouse_input_move_to() {
        let input = MouseInput::move_to(10, 20);
        assert_eq!(input.event_type, MouseEventType::Move);
        assert!(input.buttons.is_empty());
    }

    #[test]
    fn mouse_input_scroll() {
        let input = MouseInput::scroll(10, 20, 0, 1);
        assert_eq!(input.event_type, MouseEventType::Scroll);
        assert_eq!(input.scroll_delta, Some((0, 1)));
    }

    #[test]
    fn mouse_input_drag() {
        let input = MouseInput::drag(10, 20, MouseButton::LEFT);
        assert_eq!(input.event_type, MouseEventType::Drag);
    }

    #[test]
    fn mouse_input_drop() {
        let input = MouseInput::drop(10, 20, MouseButton::LEFT);
        assert_eq!(input.event_type, MouseEventType::Drop);
    }

    #[test]
    fn mouse_input_is_left_button() {
        let input = MouseInput::new(0, 0, MouseButton::LEFT);
        assert!(input.is_left_button());
    }

    #[test]
    fn mouse_input_is_right_button() {
        let input = MouseInput::new(0, 0, MouseButton::RIGHT);
        assert!(input.is_right_button());
    }

    #[test]
    fn mouse_input_is_middle_button() {
        let input = MouseInput::new(0, 0, MouseButton::MIDDLE);
        assert!(input.is_middle_button());
    }

    #[test]
    fn mouse_input_is_ctrl() {
        let input =
            MouseInput::new(0, 0, MouseButton::empty()).with_modifiers(KeyModifiers::CONTROL);
        assert!(input.is_ctrl());
    }

    #[test]
    fn mouse_input_is_shift() {
        let input = MouseInput::new(0, 0, MouseButton::empty()).with_modifiers(KeyModifiers::SHIFT);
        assert!(input.is_shift());
    }

    #[test]
    fn mouse_input_is_alt() {
        let input = MouseInput::new(0, 0, MouseButton::empty()).with_modifiers(KeyModifiers::ALT);
        assert!(input.is_alt());
    }

    #[test]
    fn mouse_event_new() {
        let input = MouseInput::new(10, 20, MouseButton::LEFT);
        let event = MouseEvent::new(input);
        assert_eq!(event.input.x, 10);
        assert_eq!(event.input.y, 20);
        assert!(event.timestamp > 0);
    }
}
