//! Mouse input types.

use bitflags::bitflags;

use super::KeyModifiers;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MouseButtons: u8 {
        const LEFT = 0b0001;
        const RIGHT = 0b0010;
        const MIDDLE = 0b0100;
        const EXTRA1 = 0b1000;
        const EXTRA2 = 0b10000;
    }
}

impl Default for MouseButtons {
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
    pub buttons: MouseButtons,
    pub modifiers: KeyModifiers,
    pub event_type: MouseEventType,
    pub scroll_delta: Option<(i16, i16)>,
}

impl MouseInput {
    pub fn new(x: u16, y: u16, buttons: MouseButtons) -> Self {
        Self { x, y, buttons, modifiers: KeyModifiers::empty(), event_type: MouseEventType::Press, scroll_delta: None }
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

    pub fn press(x: u16, y: u16, buttons: MouseButtons) -> Self {
        Self::new(x, y, buttons).with_event_type(MouseEventType::Press)
    }

    pub fn release(x: u16, y: u16, buttons: MouseButtons) -> Self {
        Self::new(x, y, buttons).with_event_type(MouseEventType::Release)
    }

    pub fn move_to(x: u16, y: u16) -> Self {
        Self::new(x, y, MouseButtons::empty()).with_event_type(MouseEventType::Move)
    }

    pub fn scroll(x: u16, y: u16, delta_x: i16, delta_y: i16) -> Self {
        Self::new(x, y, MouseButtons::empty())
            .with_event_type(MouseEventType::Scroll)
            .with_scroll_delta(delta_x, delta_y)
    }

    pub fn drag(x: u16, y: u16, buttons: MouseButtons) -> Self {
        Self::new(x, y, buttons).with_event_type(MouseEventType::Drag)
    }

    pub fn drop(x: u16, y: u16, buttons: MouseButtons) -> Self {
        Self::new(x, y, buttons).with_event_type(MouseEventType::Drop)
    }

    pub fn is_left_button(&self) -> bool {
        self.buttons.contains(MouseButtons::LEFT)
    }

    pub fn is_right_button(&self) -> bool {
        self.buttons.contains(MouseButtons::RIGHT)
    }

    pub fn is_middle_button(&self) -> bool {
        self.buttons.contains(MouseButtons::MIDDLE)
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
        Self::new(0, 0, MouseButtons::empty())
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MouseInputEvent {
    pub input: MouseInput,
    pub timestamp: u64,
}

impl MouseInputEvent {
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
