use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct KeyModifiers: u8 {
        const SHIFT = 0b0001;
        const CONTROL = 0b0010;
        const ALT = 0b0100;
        const SUPER = 0b1000;
    }
}

impl Default for KeyModifiers {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyAction {
    Press,
    Release,
    Repeat,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    Char(char),
    Enter,
    Esc,
    Backspace,
    Tab,
    Space,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
    Insert,
    F(u8),
    Modifier(KeyModifiers),
    Media(MediaKey),
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKey {
    PlayPause,
    NextTrack,
    PreviousTrack,
    Stop,
    VolumeUp,
    VolumeDown,
    Mute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardInput {
    pub key: char,
    pub modifiers: KeyModifiers,
    pub action: KeyAction,
}

impl KeyboardInput {
    pub fn new(key: char, modifiers: KeyModifiers) -> Self {
        Self {
            key,
            modifiers,
            action: KeyAction::Press,
        }
    }

    pub fn with_action(mut self, action: KeyAction) -> Self {
        self.action = action;
        self
    }

    pub fn press(key: char, modifiers: KeyModifiers) -> Self {
        Self::new(key, modifiers).with_action(KeyAction::Press)
    }

    pub fn release(key: char, modifiers: KeyModifiers) -> Self {
        Self::new(key, modifiers).with_action(KeyAction::Release)
    }

    pub fn repeat(key: char, modifiers: KeyModifiers) -> Self {
        Self::new(key, modifiers).with_action(KeyAction::Repeat)
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

    pub fn is_super(&self) -> bool {
        self.modifiers.contains(KeyModifiers::SUPER)
    }

    pub fn is_modifier_only(&self) -> bool {
        matches!(
            self.key,
            '\x00' | '\x1b' | '\x08' | '\x09' | '\x0d' | '\x0a'
        )
    }

    pub fn to_display_string(&self) -> String {
        let mut result = String::new();

        if self.is_ctrl() {
            result.push_str("Ctrl+");
        }
        if self.is_shift() {
            result.push_str("Shift+");
        }
        if self.is_alt() {
            result.push_str("Alt+");
        }
        if self.is_super() {
            result.push_str("Super+");
        }

        match self.key {
            '\x00' => result.push_str("Null"),
            '\x1b' => result.push_str("Esc"),
            '\x08' => result.push_str("Backspace"),
            '\x09' => result.push_str("Tab"),
            '\x0d' => result.push_str("Enter"),
            '\x0a' => result.push_str("LineFeed"),
            ' ' => result.push_str("Space"),
            c => result.push(c),
        }

        result
    }
}

impl Default for KeyboardInput {
    fn default() -> Self {
        Self::new('\0', KeyModifiers::empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_modifiers_empty() {
        let mods = KeyModifiers::empty();
        assert!(!mods.contains(KeyModifiers::SHIFT));
        assert!(!mods.contains(KeyModifiers::CONTROL));
        assert!(!mods.contains(KeyModifiers::ALT));
        assert!(!mods.contains(KeyModifiers::SUPER));
    }

    #[test]
    fn key_modifiers_default() {
        let mods = KeyModifiers::default();
        assert!(mods.is_empty());
    }

    #[test]
    fn keyboard_input_new() {
        let input = KeyboardInput::new('a', KeyModifiers::empty());
        assert_eq!(input.key, 'a');
        assert_eq!(input.modifiers, KeyModifiers::empty());
        assert_eq!(input.action, KeyAction::Press);
    }

    #[test]
    fn keyboard_input_default() {
        let input = KeyboardInput::default();
        assert_eq!(input.key, '\0');
    }

    #[test]
    fn keyboard_input_press() {
        let input = KeyboardInput::press('a', KeyModifiers::CONTROL);
        assert!(input.is_ctrl());
    }

    #[test]
    fn keyboard_input_release() {
        let input = KeyboardInput::release('a', KeyModifiers::SHIFT);
        assert!(input.is_shift());
    }

    #[test]
    fn keyboard_input_repeat() {
        let input = KeyboardInput::repeat('a', KeyModifiers::ALT);
        assert!(input.is_alt());
    }

    #[test]
    fn keyboard_input_is_ctrl() {
        let input = KeyboardInput::new('a', KeyModifiers::CONTROL);
        assert!(input.is_ctrl());
    }

    #[test]
    fn keyboard_input_is_shift() {
        let input = KeyboardInput::new('a', KeyModifiers::SHIFT);
        assert!(input.is_shift());
    }

    #[test]
    fn keyboard_input_is_alt() {
        let input = KeyboardInput::new('a', KeyModifiers::ALT);
        assert!(input.is_alt());
    }

    #[test]
    fn keyboard_input_is_super() {
        let input = KeyboardInput::new('a', KeyModifiers::SUPER);
        assert!(input.is_super());
    }

    #[test]
    fn keyboard_input_to_string() {
        let input = KeyboardInput::new('a', KeyModifiers::CONTROL);
        assert_eq!(input.to_display_string(), "Ctrl+a");
    }

    #[test]
    fn keyboard_input_to_string_shift() {
        let input = KeyboardInput::new('A', KeyModifiers::SHIFT);
        assert_eq!(input.to_display_string(), "Shift+A");
    }
}
