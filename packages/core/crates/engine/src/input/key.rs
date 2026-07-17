//! Key types, modifiers, keyboard input, key combos, sequences, and parsing.

use crate::event_bus::{Key, KeyEvent, Modifiers};
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
        Self { key, modifiers, action: KeyAction::Press }
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
        matches!(self.key, '\x00' | '\x1b' | '\x08' | '\x09' | '\x0d' | '\x0a')
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

/// Represents a sequence of keys (for chord bindings like `dd`, `<leader>s`)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeySequence {
    pub keys: Vec<KeyCombo>,
}

/// A single key combination (key + modifiers)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyCombo {
    pub key: Key,
    pub modifiers: Modifiers,
}

impl KeyCombo {
    pub fn new(key: Key, modifiers: Modifiers) -> Self {
        Self { key, modifiers }
    }

    pub fn plain(key: Key) -> Self {
        Self { key, modifiers: Modifiers::NONE }
    }

    pub fn with_ctrl(key: Key) -> Self {
        Self { key, modifiers: Modifiers { ctrl: true, ..Modifiers::NONE } }
    }

    pub fn with_shift(key: Key) -> Self {
        Self { key, modifiers: Modifiers { shift: true, ..Modifiers::NONE } }
    }

    pub fn with_alt(key: Key) -> Self {
        Self { key, modifiers: Modifiers { alt: true, ..Modifiers::NONE } }
    }

    pub fn matches(&self, event: &KeyEvent) -> bool {
        self.key == event.key && self.modifiers == event.modifiers
    }
}

impl KeySequence {
    pub fn single(combo: KeyCombo) -> Self {
        Self { keys: vec![combo] }
    }

    pub fn chord(combos: Vec<KeyCombo>) -> Self {
        Self { keys: combos }
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn starts_with(&self, combo: &KeyCombo) -> bool {
        self.keys.first() == Some(combo)
    }

    pub fn tail(&self) -> Self {
        Self { keys: self.keys[1..].to_vec() }
    }
}

/// Parses key strings into KeyCombo or KeySequence
pub struct KeyParser;

impl KeyParser {
    pub fn parse_combo(s: &str) -> Result<KeyCombo, ParseError> {
        let s = s.trim().to_lowercase();
        let mut modifiers = Modifiers::NONE;
        let mut key_str = s.as_str();

        loop {
            if key_str.starts_with("ctrl+") {
                modifiers.ctrl = true;
                key_str = &key_str[5..];
            } else if key_str.starts_with("alt+") {
                modifiers.alt = true;
                key_str = &key_str[4..];
            } else if key_str.starts_with("shift+") {
                modifiers.shift = true;
                key_str = &key_str[6..];
            } else if key_str.starts_with("meta+") {
                modifiers.meta = true;
                key_str = &key_str[5..];
            } else {
                break;
            }
        }

        let key = Self::parse_key(key_str)?;
        Ok(KeyCombo::new(key, modifiers))
    }

    pub fn parse_sequence(s: &str) -> Result<KeySequence, ParseError> {
        let s = s.trim();

        if s.len() == 2 && !s.contains('+') && !s.contains('<') {
            let chars: Vec<char> = s.chars().collect();
            if chars[0] == chars[1] {
                let combo = KeyCombo::plain(Key::Character(chars[0]));
                return Ok(KeySequence::chord(vec![combo.clone(), combo]));
            }
        }

        if !s.contains(',') {
            let combo = Self::parse_combo(s)?;
            return Ok(KeySequence::single(combo));
        }

        let combos: Result<Vec<_>, _> = s.split(',').map(Self::parse_combo).collect();
        Ok(KeySequence::chord(combos?))
    }

    fn parse_key(s: &str) -> Result<Key, ParseError> {
        match s {
            "enter" | "return" | "cr" => Ok(Key::Enter),
            "escape" | "esc" => Ok(Key::Escape),
            "backspace" | "bs" => Ok(Key::Backspace),
            "delete" | "del" => Ok(Key::Delete),
            "tab" => Ok(Key::Tab),
            "space" | "sp" => Ok(Key::Space),
            "up" | "arrow_up" => Ok(Key::ArrowUp),
            "down" | "arrow_down" => Ok(Key::ArrowDown),
            "left" | "arrow_left" => Ok(Key::ArrowLeft),
            "right" | "arrow_right" => Ok(Key::ArrowRight),
            "home" => Ok(Key::Home),
            "end" => Ok(Key::End),
            "page_up" | "pgup" => Ok(Key::PageUp),
            "page_down" | "pgdn" => Ok(Key::PageDown),
            s if s.starts_with('f') && s.len() <= 3 => {
                let num: u8 = s[1..].parse().map_err(|_| ParseError::InvalidKey(s.to_string()))?;
                Ok(Key::F(num))
            }
            s if s.len() == 1 => Ok(Key::Character(s.chars().next().unwrap())),
            _ => Err(ParseError::InvalidKey(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidKey(String),
    InvalidModifier(String),
    InvalidSequence(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidKey(s) => write!(f, "Invalid key: {}", s),
            ParseError::InvalidModifier(s) => write!(f, "Invalid modifier: {}", s),
            ParseError::InvalidSequence(s) => write!(f, "Invalid sequence: {}", s),
        }
    }
}

impl std::error::Error for ParseError {}
