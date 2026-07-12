//! Keybinding engine.
//!
//! Provides keymap management, chord sequences, and context-aware shortcuts.
//!
//! # Architecture
//!
//! The keymap system uses a layered approach similar to OpenTUI's `@opentui/keymap`:
//!
//! - **KeyBinding**: A single binding mapping a key sequence to a command
//! - **KeyLayer**: A collection of bindings with priority and conditions
//! - **Keymap**: The main manager that resolves bindings across layers
//! - **KeySequence**: Represents a sequence of keys (for chord bindings like `dd`)
//! - **KeyParser**: Parses key strings into Key+Modifiers combinations
//!
//! # Example
//!
//! ```rust
//! use bettertui_engine::keybinding::{Keymap, KeyBinding, KeyLayer};
//! use bettertui_engine::events::types::Key;
//!
//! let mut keymap = Keymap::new();
//!
//! // Add a simple binding
//! keymap.add_binding(KeyBinding::new("save", "ctrl+s", "Save file"));
//!
//! // Add a layer with conditional bindings
//! let mut layer = KeyLayer::new("vim-normal", 10);
//! layer.add_binding(KeyBinding::new("dd", "dd", "Delete line"));
//! keymap.add_layer(layer);
//! ```

use crate::events::types::{Key, KeyEvent, Modifiers};

#[cfg(test)]
mod tests;

// ─── Key Sequence ────────────────────────────────────────────────────────────

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
        Self {
            key,
            modifiers: Modifiers::NONE,
        }
    }

    pub fn with_ctrl(key: Key) -> Self {
        Self {
            key,
            modifiers: Modifiers {
                ctrl: true,
                ..Modifiers::NONE
            },
        }
    }

    pub fn with_shift(key: Key) -> Self {
        Self {
            key,
            modifiers: Modifiers {
                shift: true,
                ..Modifiers::NONE
            },
        }
    }

    pub fn with_alt(key: Key) -> Self {
        Self {
            key,
            modifiers: Modifiers {
                alt: true,
                ..Modifiers::NONE
            },
        }
    }

    /// Check if this combo matches a key event
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

    /// Check if this sequence starts with the given combo
    pub fn starts_with(&self, combo: &KeyCombo) -> bool {
        self.keys.first() == Some(combo)
    }

    /// Get the remaining sequence after removing the first key
    pub fn tail(&self) -> Self {
        Self {
            keys: self.keys[1..].to_vec(),
        }
    }
}

// ─── Key Parser ──────────────────────────────────────────────────────────────

/// Parses key strings into KeyCombo or KeySequence
///
/// Supported formats:
/// - Single keys: `a`, `enter`, `escape`, `f1`, `space`
/// - Modifiers: `ctrl+s`, `alt+x`, `shift+tab`
/// - Chords: `dd`, `gg`, `<leader>s`
/// - Complex: `ctrl+shift+k`
pub struct KeyParser;

impl KeyParser {
    /// Parse a key string into a KeyCombo
    pub fn parse_combo(s: &str) -> Result<KeyCombo, ParseError> {
        let s = s.trim().to_lowercase();
        let mut modifiers = Modifiers::NONE;
        let mut key_str = s.as_str();

        // Parse modifiers
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

    /// Parse a key string into a KeySequence (supports chord notation)
    pub fn parse_sequence(s: &str) -> Result<KeySequence, ParseError> {
        let s = s.trim();

        // Handle chord sequences like `dd`, `gg`
        if s.len() == 2 && !s.contains('+') && !s.contains('<') {
            let chars: Vec<char> = s.chars().collect();
            if chars[0] == chars[1] {
                // Repeated key = chord (e.g., `dd`)
                let combo = KeyCombo::plain(Key::Character(chars[0]));
                return Ok(KeySequence::chord(vec![combo.clone(), combo]));
            }
        }

        // Handle single key
        if !s.contains(',') {
            let combo = Self::parse_combo(s)?;
            return Ok(KeySequence::single(combo));
        }

        // Handle comma-separated sequences
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
                let num: u8 = s[1..]
                    .parse()
                    .map_err(|_| ParseError::InvalidKey(s.to_string()))?;
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

// ─── Key Binding ─────────────────────────────────────────────────────────────

/// A single key binding mapping a key sequence to a command
#[derive(Debug, Clone)]
pub struct KeyBinding {
    /// Unique identifier for this binding
    pub id: String,
    /// The key sequence that triggers this binding
    pub sequence: KeySequence,
    /// The command to execute
    pub command: String,
    /// Optional description for documentation/cheat sheets
    pub description: Option<String>,
    /// Optional condition that must be true for this binding to be active
    pub condition: Option<BindingCondition>,
    /// Whether this binding is enabled
    pub enabled: bool,
}

/// Condition for a binding to be active
#[derive(Debug, Clone)]
pub enum BindingCondition {
    /// Binding is active when a specific mode is active
    Mode(String),
    /// Binding is active when a specific focus scope is active
    FocusScope(String),
    /// Binding is active when a custom condition is met
    Custom(String),
}

impl KeyBinding {
    /// Create a new key binding
    pub fn new(id: impl Into<String>, key_str: &str, description: impl Into<String>) -> Self {
        let id_str = id.into();
        let sequence = KeyParser::parse_sequence(key_str).expect("Invalid key sequence");
        Self {
            command: id_str.clone(),
            id: id_str,
            sequence,
            description: Some(description.into()),
            condition: None,
            enabled: true,
        }
    }

    /// Create a binding with a custom command
    pub fn with_command(mut self, command: impl Into<String>) -> Self {
        self.command = command.into();
        self
    }

    /// Add a mode condition
    pub fn in_mode(mut self, mode: impl Into<String>) -> Self {
        self.condition = Some(BindingCondition::Mode(mode.into()));
        self
    }

    /// Add a focus scope condition
    pub fn in_focus_scope(mut self, scope: impl Into<String>) -> Self {
        self.condition = Some(BindingCondition::FocusScope(scope.into()));
        self
    }

    /// Disable this binding
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Check if this binding matches a key event
    pub fn matches(&self, event: &KeyEvent, current_mode: Option<&str>) -> bool {
        if !self.enabled {
            return false;
        }

        // Check condition
        if let Some(ref condition) = self.condition {
            match condition {
                BindingCondition::Mode(mode) => {
                    if current_mode.is_none_or(|m| m != mode) {
                        return false;
                    }
                }
                BindingCondition::FocusScope(_) => {
                    // Focus scope checking would need integration with focus manager
                    // For now, assume it matches
                }
                BindingCondition::Custom(_) => {
                    // Custom conditions would need external evaluation
                    // For now, assume it matches
                }
            }
        }

        // Check if the first key of the sequence matches
        // For both single-key and chord bindings, we match on the first key
        if !self.sequence.is_empty() {
            return self.sequence.keys[0].matches(event);
        }

        false
    }
}

// ─── Key Layer ───────────────────────────────────────────────────────────────

/// A collection of bindings with priority and optional conditions
#[derive(Debug, Clone)]
pub struct KeyLayer {
    /// Layer name for identification
    pub name: String,
    /// Priority (higher = checked first)
    pub priority: i32,
    /// Whether this layer is enabled
    pub enabled: bool,
    /// Bindings in this layer
    bindings: Vec<KeyBinding>,
}

impl KeyLayer {
    /// Create a new key layer
    pub fn new(name: impl Into<String>, priority: i32) -> Self {
        Self {
            name: name.into(),
            priority,
            enabled: true,
            bindings: Vec::new(),
        }
    }

    /// Add a binding to this layer
    pub fn add_binding(&mut self, binding: KeyBinding) {
        self.bindings.push(binding);
    }

    /// Remove a binding by ID
    pub fn remove_binding(&mut self, id: &str) -> bool {
        let len = self.bindings.len();
        self.bindings.retain(|b| b.id != id);
        self.bindings.len() < len
    }

    /// Get all bindings
    pub fn bindings(&self) -> &[KeyBinding] {
        &self.bindings
    }

    /// Find a binding that matches a key event
    pub fn find_binding(
        &self,
        event: &KeyEvent,
        current_mode: Option<&str>,
    ) -> Option<&KeyBinding> {
        if !self.enabled {
            return None;
        }
        self.bindings
            .iter()
            .find(|b| b.matches(event, current_mode))
    }

    /// Enable this layer
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable this layer
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

// ─── Keymap ──────────────────────────────────────────────────────────────────

/// The main keymap manager that resolves bindings across layers
#[derive(Debug)]
pub struct Keymap {
    /// All layers sorted by priority (highest first)
    layers: Vec<KeyLayer>,
    /// Current mode (e.g., "normal", "insert", "visual")
    current_mode: Option<String>,
    /// Pending chord sequence (for multi-key bindings)
    pending_sequence: Vec<KeyCombo>,
    /// Timeout for chord sequences (in milliseconds)
    chord_timeout_ms: u64,
    /// Command history for debugging
    command_history: Vec<String>,
}

impl Default for Keymap {
    fn default() -> Self {
        Self::new()
    }
}

impl Keymap {
    /// Create a new empty keymap
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            current_mode: None,
            pending_sequence: Vec::new(),
            chord_timeout_ms: 1000,
            command_history: Vec::new(),
        }
    }

    /// Add a layer to the keymap
    pub fn add_layer(&mut self, layer: KeyLayer) {
        self.layers.push(layer);
        self.layers.sort_by_key(|b| std::cmp::Reverse(b.priority));
    }

    /// Remove a layer by name
    pub fn remove_layer(&mut self, name: &str) -> bool {
        let len = self.layers.len();
        self.layers.retain(|l| l.name != name);
        self.layers.len() < len
    }

    /// Get a layer by name
    pub fn get_layer(&self, name: &str) -> Option<&KeyLayer> {
        self.layers.iter().find(|l| l.name == name)
    }

    /// Get a mutable layer by name
    pub fn get_layer_mut(&mut self, name: &str) -> Option<&mut KeyLayer> {
        self.layers.iter_mut().find(|l| l.name == name)
    }

    /// Add a binding to a specific layer (creates layer if it doesn't exist)
    pub fn add_binding_to_layer(&mut self, layer_name: &str, binding: KeyBinding, priority: i32) {
        if let Some(layer) = self.get_layer_mut(layer_name) {
            layer.add_binding(binding);
        } else {
            let mut layer = KeyLayer::new(layer_name, priority);
            layer.add_binding(binding);
            self.add_layer(layer);
        }
    }

    /// Add a simple binding to the default layer
    pub fn add_binding(&mut self, binding: KeyBinding) {
        self.add_binding_to_layer("default", binding, 0);
    }

    /// Set the current mode
    pub fn set_mode(&mut self, mode: impl Into<String>) {
        self.current_mode = Some(mode.into());
        self.pending_sequence.clear();
    }

    /// Get the current mode
    pub fn current_mode(&self) -> Option<&str> {
        self.current_mode.as_deref()
    }

    /// Clear the current mode
    pub fn clear_mode(&mut self) {
        self.current_mode = None;
        self.pending_sequence.clear();
    }

    /// Set chord timeout in milliseconds
    pub fn set_chord_timeout(&mut self, ms: u64) {
        self.chord_timeout_ms = ms;
    }

    /// Get chord timeout in milliseconds
    pub fn chord_timeout_ms(&self) -> u64 {
        self.chord_timeout_ms
    }

    /// Handle a key event and return the command to execute (if any)
    pub fn handle_event(&mut self, event: &KeyEvent) -> Option<String> {
        // If we have a pending sequence, check if this key continues it
        if !self.pending_sequence.is_empty() {
            let expected = &self.pending_sequence[0];
            if expected.matches(event) {
                self.pending_sequence.remove(0);
                if self.pending_sequence.is_empty() {
                    // Complete chord sequence matched - return the command from history
                    return self.last_binding_command();
                }
                return None;
            } else {
                // Sequence broken, clear pending
                self.pending_sequence.clear();
            }
        }

        // Search layers for a matching binding
        for layer in &self.layers {
            if let Some(binding) = layer.find_binding(event, self.current_mode.as_deref()) {
                if binding.sequence.len() == 1 {
                    // Single key binding - execute immediately
                    self.command_history.push(binding.command.clone());
                    return Some(binding.command.clone());
                } else {
                    // Chord binding - start tracking the remaining keys
                    self.pending_sequence = binding.sequence.keys[1..].to_vec();
                    // Store the command so we can return it when the chord completes
                    self.command_history.push(binding.command.clone());
                    return None;
                }
            }
        }

        None
    }

    /// Check if there's a pending chord sequence
    pub fn has_pending_sequence(&self) -> bool {
        !self.pending_sequence.is_empty()
    }

    /// Clear any pending chord sequence
    pub fn clear_pending_sequence(&mut self) {
        self.pending_sequence.clear();
    }

    /// Get the remaining keys in the pending sequence
    pub fn pending_keys(&self) -> &[KeyCombo] {
        &self.pending_sequence
    }

    /// Get command history
    pub fn command_history(&self) -> &[String] {
        &self.command_history
    }

    /// Clear command history
    pub fn clear_history(&mut self) {
        self.command_history.clear();
    }

    /// Get all active bindings (for cheat sheet generation)
    pub fn active_bindings(&self) -> Vec<(&KeyBinding, &str)> {
        let mut result = Vec::new();
        for layer in &self.layers {
            if !layer.enabled {
                continue;
            }
            for binding in layer.bindings() {
                if binding.enabled {
                    result.push((binding, layer.name.as_str()));
                }
            }
        }
        result
    }

    /// Get all bindings as a flat list (for serialization)
    pub fn all_bindings(&self) -> Vec<(&KeyBinding, &str)> {
        self.layers
            .iter()
            .flat_map(|layer| {
                layer
                    .bindings()
                    .iter()
                    .map(move |b| (b, layer.name.as_str()))
            })
            .collect()
    }

    /// Get the command from the last completed binding (used for chord sequences)
    fn last_binding_command(&self) -> Option<String> {
        self.command_history.last().cloned()
    }
}

// ─── Keymap Builder ──────────────────────────────────────────────────────────

/// Builder for creating keymaps with a fluent API
pub struct KeymapBuilder {
    keymap: Keymap,
}

impl KeymapBuilder {
    pub fn new() -> Self {
        Self {
            keymap: Keymap::new(),
        }
    }

    /// Add a binding
    pub fn binding(mut self, id: &str, keys: &str, desc: &str) -> Self {
        self.keymap.add_binding(KeyBinding::new(id, keys, desc));
        self
    }

    /// Add a binding to a specific layer
    pub fn binding_in_layer(
        mut self,
        layer: &str,
        priority: i32,
        id: &str,
        keys: &str,
        desc: &str,
    ) -> Self {
        self.keymap
            .add_binding_to_layer(layer, KeyBinding::new(id, keys, desc), priority);
        self
    }

    /// Set the initial mode
    pub fn mode(mut self, mode: &str) -> Self {
        self.keymap.set_mode(mode);
        self
    }

    /// Build the keymap
    pub fn build(self) -> Keymap {
        self.keymap
    }
}

impl Default for KeymapBuilder {
    fn default() -> Self {
        Self::new()
    }
}
