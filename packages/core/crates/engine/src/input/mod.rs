//! Input runtime: manages keyboard, mouse, and clipboard input state and event queue.

mod clipboard;
mod event;
mod keyboard;
mod mouse;

pub use clipboard::{ClipboardAction, ClipboardInput};
pub use event::{InputEvent, InputEventType};
pub use keyboard::{KeyAction, KeyModifiers, KeyboardInput};
pub use mouse::{MouseButton, MouseEvent, MouseInput};

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct InputRuntime {
    events: VecDeque<InputEvent>,
    clipboard: ClipboardState,
    mouse_state: MouseState,
    keyboard_state: KeyboardState,
}

#[derive(Debug, Clone)]
pub struct ClipboardState {
    pub content: Option<String>,
    pub selection: Option<String>,
    pub primary: Option<String>,
}

impl Default for ClipboardState {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipboardState {
    pub fn new() -> Self {
        Self {
            content: None,
            selection: None,
            primary: None,
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.content = Some(content);
    }

    pub fn get_content(&self) -> Option<&str> {
        self.content.as_deref()
    }

    pub fn set_selection(&mut self, selection: String) {
        self.selection = Some(selection);
    }

    pub fn get_selection(&self) -> Option<&str> {
        self.selection.as_deref()
    }

    pub fn set_primary(&mut self, primary: String) {
        self.primary = Some(primary);
    }

    pub fn get_primary(&self) -> Option<&str> {
        self.primary.as_deref()
    }

    pub fn clear(&mut self) {
        self.content = None;
        self.selection = None;
        self.primary = None;
    }
}

#[derive(Debug, Clone)]
pub struct MouseState {
    pub position: (u16, u16),
    pub buttons: MouseButton,
    pub modifiers: KeyModifiers,
    pub scroll_direction: Option<ScrollDirection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Default for MouseState {
    fn default() -> Self {
        Self::new()
    }
}

impl MouseState {
    pub fn new() -> Self {
        Self {
            position: (0, 0),
            buttons: MouseButton::empty(),
            modifiers: KeyModifiers::empty(),
            scroll_direction: None,
        }
    }

    pub fn set_position(&mut self, x: u16, y: u16) {
        self.position = (x, y);
    }

    pub fn set_buttons(&mut self, buttons: MouseButton) {
        self.buttons = buttons;
    }

    pub fn set_modifiers(&mut self, modifiers: KeyModifiers) {
        self.modifiers = modifiers;
    }

    pub fn set_scroll(&mut self, direction: ScrollDirection) {
        self.scroll_direction = Some(direction);
    }

    pub fn clear_scroll(&mut self) {
        self.scroll_direction = None;
    }
}

#[derive(Debug, Clone)]
pub struct KeyboardState {
    pub modifiers: KeyModifiers,
    pub kitty_keyboard: bool,
    pub bracketed_paste: bool,
    pub focus_events: bool,
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyboardState {
    pub fn new() -> Self {
        Self {
            modifiers: KeyModifiers::empty(),
            kitty_keyboard: false,
            bracketed_paste: false,
            focus_events: false,
        }
    }

    pub fn with_kitty_keyboard(mut self, enabled: bool) -> Self {
        self.kitty_keyboard = enabled;
        self
    }

    pub fn with_bracketed_paste(mut self, enabled: bool) -> Self {
        self.bracketed_paste = enabled;
        self
    }

    pub fn with_focus_events(mut self, enabled: bool) -> Self {
        self.focus_events = enabled;
        self
    }

    pub fn set_modifiers(&mut self, modifiers: KeyModifiers) {
        self.modifiers = modifiers;
    }

    pub fn press_key(&mut self, key: char, modifiers: KeyModifiers) -> KeyboardInput {
        KeyboardInput {
            key,
            modifiers,
            action: KeyAction::Press,
        }
    }

    pub fn release_key(&mut self, key: char, modifiers: KeyModifiers) -> KeyboardInput {
        KeyboardInput {
            key,
            modifiers,
            action: KeyAction::Release,
        }
    }
}

impl Default for InputRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl InputRuntime {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            clipboard: ClipboardState::new(),
            mouse_state: MouseState::new(),
            keyboard_state: KeyboardState::new(),
        }
    }

    pub fn with_kitty_keyboard(mut self, enabled: bool) -> Self {
        self.keyboard_state = self.keyboard_state.with_kitty_keyboard(enabled);
        self
    }

    pub fn with_bracketed_paste(mut self, enabled: bool) -> Self {
        self.keyboard_state = self.keyboard_state.with_bracketed_paste(enabled);
        self
    }

    pub fn with_focus_events(mut self, enabled: bool) -> Self {
        self.keyboard_state = self.keyboard_state.with_focus_events(enabled);
        self
    }

    pub fn push_event(&mut self, event: InputEvent) {
        self.events.push_back(event);
    }

    pub fn poll_event(&mut self) -> Option<InputEvent> {
        self.events.pop_front()
    }

    pub fn events(&self) -> &VecDeque<InputEvent> {
        &self.events
    }

    pub fn clipboard(&self) -> &ClipboardState {
        &self.clipboard
    }

    pub fn clipboard_mut(&mut self) -> &mut ClipboardState {
        &mut self.clipboard
    }

    pub fn mouse_state(&self) -> &MouseState {
        &self.mouse_state
    }

    pub fn mouse_state_mut(&mut self) -> &mut MouseState {
        &mut self.mouse_state
    }

    pub fn keyboard_state(&self) -> &KeyboardState {
        &self.keyboard_state
    }

    pub fn keyboard_state_mut(&mut self) -> &mut KeyboardState {
        &mut self.keyboard_state
    }

    pub fn handle_keyboard_input(&mut self, input: KeyboardInput) {
        let event = InputEvent {
            event_type: InputEventType::Keyboard(input),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        };
        self.push_event(event);
    }

    pub fn handle_mouse_input(&mut self, input: MouseInput) {
        let event = InputEvent {
            event_type: InputEventType::Mouse(input),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        };
        self.push_event(event);
    }

    pub fn handle_clipboard_input(&mut self, input: ClipboardInput) {
        match input.action {
            ClipboardAction::Copy => {
                self.clipboard.set_content(input.data.clone());
            }
            ClipboardAction::Paste => {
                // Paste action will be handled by the consumer
            }
            ClipboardAction::Cut => {
                self.clipboard.set_content(input.data.clone());
            }
        }

        let event = InputEvent {
            event_type: InputEventType::Clipboard(input),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        };
        self.push_event(event);
    }

    pub fn clear(&mut self) {
        self.events.clear();
        self.clipboard.clear();
        self.mouse_state = MouseState::new();
        self.keyboard_state = KeyboardState::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_runtime_new() {
        let runtime = InputRuntime::new();
        assert!(runtime.events().is_empty());
    }

    #[test]
    fn input_runtime_default() {
        let runtime = InputRuntime::default();
        assert!(runtime.events().is_empty());
    }

    #[test]
    fn input_runtime_with_kitty_keyboard() {
        let runtime = InputRuntime::new().with_kitty_keyboard(true);
        assert!(runtime.keyboard_state().kitty_keyboard);
    }

    #[test]
    fn input_runtime_with_bracketed_paste() {
        let runtime = InputRuntime::new().with_bracketed_paste(true);
        assert!(runtime.keyboard_state().bracketed_paste);
    }

    #[test]
    fn input_runtime_with_focus_events() {
        let runtime = InputRuntime::new().with_focus_events(true);
        assert!(runtime.keyboard_state().focus_events);
    }

    #[test]
    fn input_runtime_push_poll_event() {
        let mut runtime = InputRuntime::new();
        let event = InputEvent {
            event_type: InputEventType::Resize(80, 24),
            timestamp: 0,
        };
        runtime.push_event(event);
        assert!(!runtime.events().is_empty());
        let polled = runtime.poll_event();
        assert!(polled.is_some());
    }

    #[test]
    fn input_runtime_handle_keyboard_input() {
        let mut runtime = InputRuntime::new();
        let input = KeyboardInput::new('a', KeyModifiers::empty());
        runtime.handle_keyboard_input(input);
        assert_eq!(runtime.events().len(), 1);
    }

    #[test]
    fn input_runtime_handle_mouse_input() {
        let mut runtime = InputRuntime::new();
        let input = MouseInput::new(0, 0, MouseButton::empty());
        runtime.handle_mouse_input(input);
        assert_eq!(runtime.events().len(), 1);
    }

    #[test]
    fn input_runtime_handle_clipboard_input() {
        let mut runtime = InputRuntime::new();
        let input = ClipboardInput::new(ClipboardAction::Copy, "hello".to_string());
        runtime.handle_clipboard_input(input);
        assert_eq!(runtime.events().len(), 1);
        assert_eq!(runtime.clipboard().get_content(), Some("hello"));
    }

    #[test]
    fn input_runtime_clear() {
        let mut runtime = InputRuntime::new();
        let event = InputEvent {
            event_type: InputEventType::Resize(80, 24),
            timestamp: 0,
        };
        runtime.push_event(event);
        runtime.clear();
        assert!(runtime.events().is_empty());
    }

    #[test]
    fn clipboard_state_new() {
        let state = ClipboardState::new();
        assert!(state.get_content().is_none());
    }

    #[test]
    fn clipboard_state_set_get() {
        let mut state = ClipboardState::new();
        state.set_content("hello".to_string());
        assert_eq!(state.get_content(), Some("hello"));
    }

    #[test]
    fn mouse_state_new() {
        let state = MouseState::new();
        assert_eq!(state.position, (0, 0));
    }

    #[test]
    fn keyboard_state_new() {
        let state = KeyboardState::new();
        assert!(!state.kitty_keyboard);
    }
}
