//! Keyboard handler for processing key events and managing keybindings.
//!
//! Translates raw key input into actionable commands, supports modifier combinations,
//! and provides a keymap for customizable keybindings.

use crate::events::types::Key;

/// A keybinding maps a key combination to a named action.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    /// The key that triggers this binding.
    pub key: Key,
    /// Whether Ctrl must be held.
    pub ctrl: bool,
    /// Whether Shift must be held.
    pub shift: bool,
    /// Whether Alt must be held.
    pub alt: bool,
}

impl KeyBinding {
    /// Creates a new keybinding with no modifiers.
    pub fn new(key: Key) -> Self {
        Self {
            key,
            ctrl: false,
            shift: false,
            alt: false,
        }
    }

    /// Adds Ctrl modifier.
    pub fn ctrl(mut self) -> Self {
        self.ctrl = true;
        self
    }

    /// Adds Shift modifier.
    pub fn shift(mut self) -> Self {
        self.shift = true;
        self
    }

    /// Adds Alt modifier.
    pub fn alt(mut self) -> Self {
        self.alt = true;
        self
    }
}

/// Result of processing a key event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyAction {
    /// The key matched a binding and produced this action name.
    Action(String),
    /// The key was a character input.
    Char(char),
    /// The key was not handled.
    Unhandled,
}

/// Processes keyboard input and maps key combinations to actions.
#[derive(Debug, Clone)]
pub struct KeyboardHandler {
    /// Registered keybindings: binding -> action name.
    bindings: Vec<(KeyBinding, String)>,
    /// Currently active modifier state.
    ctrl: bool,
    shift: bool,
    alt: bool,
    /// Whether kitty keyboard protocol is enabled.
    kitty_protocol: bool,
}

impl Default for KeyboardHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyboardHandler {
    /// Creates a new KeyboardHandler with no bindings.
    pub fn new() -> Self {
        Self {
            bindings: Vec::new(),
            ctrl: false,
            shift: false,
            alt: false,
            kitty_protocol: false,
        }
    }

    /// Registers a keybinding.
    pub fn bind(&mut self, binding: KeyBinding, action: impl Into<String>) {
        self.bindings.push((binding, action.into()));
    }

    /// Removes a keybinding.
    pub fn unbind(&mut self, binding: &KeyBinding) {
        self.bindings.retain(|(b, _)| b != binding);
    }

    /// Enables or disables kitty keyboard protocol.
    pub fn set_kitty_protocol(&mut self, enabled: bool) {
        self.kitty_protocol = enabled;
    }

    /// Returns whether kitty keyboard protocol is enabled.
    pub fn kitty_protocol(&self) -> bool {
        self.kitty_protocol
    }

    /// Updates modifier state.
    pub fn set_modifiers(&mut self, ctrl: bool, shift: bool, alt: bool) {
        self.ctrl = ctrl;
        self.shift = shift;
        self.alt = alt;
    }

    /// Returns current modifier state.
    pub fn modifiers(&self) -> (bool, bool, bool) {
        (self.ctrl, self.shift, self.alt)
    }

    /// Processes a key and returns the corresponding action.
    pub fn process_key(&self, key: &Key) -> KeyAction {
        // Check bindings in reverse order (last registered wins)
        for (binding, action) in self.bindings.iter().rev() {
            if binding.key == *key
                && binding.ctrl == self.ctrl
                && binding.shift == self.shift
                && binding.alt == self.alt
            {
                return KeyAction::Action(action.clone());
            }
        }

        // Default handling for character input
        match key {
            Key::Character(ch) => KeyAction::Char(*ch),
            _ => KeyAction::Unhandled,
        }
    }

    /// Returns the number of registered bindings.
    pub fn binding_count(&self) -> usize {
        self.bindings.len()
    }

    /// Clears all bindings.
    pub fn clear_bindings(&mut self) {
        self.bindings.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_binding() {
        let mut handler = KeyboardHandler::new();
        handler.bind(KeyBinding::new(Key::Character('q')), "quit");
        let action = handler.process_key(&Key::Character('q'));
        assert_eq!(action, KeyAction::Action("quit".to_string()));
    }

    #[test]
    fn ctrl_binding() {
        let mut handler = KeyboardHandler::new();
        handler.bind(KeyBinding::new(Key::Character('c')).ctrl(), "interrupt");
        handler.set_modifiers(true, false, false);
        let action = handler.process_key(&Key::Character('c'));
        assert_eq!(action, KeyAction::Action("interrupt".to_string()));
    }

    #[test]
    fn unhandled_key() {
        let handler = KeyboardHandler::new();
        let action = handler.process_key(&Key::Character('x'));
        assert_eq!(action, KeyAction::Char('x'));
    }

    #[test]
    fn unbound_modifier() {
        let mut handler = KeyboardHandler::new();
        handler.bind(KeyBinding::new(Key::Character('q')), "quit");
        handler.set_modifiers(true, false, false); // Ctrl held
        let action = handler.process_key(&Key::Character('q'));
        assert_eq!(action, KeyAction::Char('q')); // doesn't match
    }

    #[test]
    fn unbind() {
        let mut handler = KeyboardHandler::new();
        let binding = KeyBinding::new(Key::Character('q'));
        handler.bind(binding.clone(), "quit");
        assert_eq!(handler.binding_count(), 1);
        handler.unbind(&binding);
        assert_eq!(handler.binding_count(), 0);
    }

    #[test]
    fn clear_bindings() {
        let mut handler = KeyboardHandler::new();
        handler.bind(KeyBinding::new(Key::Character('a')), "a");
        handler.bind(KeyBinding::new(Key::Character('b')), "b");
        handler.clear_bindings();
        assert_eq!(handler.binding_count(), 0);
    }

    #[test]
    fn binding_chaining() {
        let binding = KeyBinding::new(Key::Character('s')).ctrl().shift();
        assert!(binding.ctrl);
        assert!(binding.shift);
        assert!(!binding.alt);
    }

    #[test]
    fn kitty_protocol_toggle() {
        let mut handler = KeyboardHandler::new();
        assert!(!handler.kitty_protocol());
        handler.set_kitty_protocol(true);
        assert!(handler.kitty_protocol());
    }

    #[test]
    fn last_binding_wins() {
        let mut handler = KeyboardHandler::new();
        handler.bind(KeyBinding::new(Key::Character('x')), "first");
        handler.bind(KeyBinding::new(Key::Character('x')), "second");
        let action = handler.process_key(&Key::Character('x'));
        assert_eq!(action, KeyAction::Action("second".to_string()));
    }
}
