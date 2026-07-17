//! Keybinding system: keymaps, layers, and command dispatch.

use super::key::{KeyCombo, KeyParser, KeySequence};
use crate::event_bus::KeyEvent;

/// A single key binding mapping a key sequence to a command
#[derive(Debug, Clone)]
pub struct KeyBinding {
    pub id: String,
    pub sequence: KeySequence,
    pub command: String,
    pub description: Option<String>,
    pub condition: Option<BindingCondition>,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub enum BindingCondition {
    Mode(String),
    FocusScope(String),
    Custom(String),
}

impl KeyBinding {
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

    pub fn with_command(mut self, command: impl Into<String>) -> Self {
        self.command = command.into();
        self
    }

    pub fn in_mode(mut self, mode: impl Into<String>) -> Self {
        self.condition = Some(BindingCondition::Mode(mode.into()));
        self
    }

    pub fn in_focus_scope(mut self, scope: impl Into<String>) -> Self {
        self.condition = Some(BindingCondition::FocusScope(scope.into()));
        self
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    pub fn matches(&self, event: &KeyEvent, current_mode: Option<&str>) -> bool {
        if !self.enabled {
            return false;
        }

        if let Some(ref condition) = self.condition {
            match condition {
                BindingCondition::Mode(mode) => {
                    if current_mode.is_none_or(|m| m != mode) {
                        return false;
                    }
                }
                BindingCondition::FocusScope(_) => {}
                BindingCondition::Custom(_) => {}
            }
        }

        if !self.sequence.is_empty() {
            return self.sequence.keys[0].matches(event);
        }

        false
    }
}

/// A collection of bindings with priority and optional conditions
#[derive(Debug, Clone)]
pub struct KeyLayer {
    pub name: String,
    pub priority: i32,
    pub enabled: bool,
    bindings: Vec<KeyBinding>,
}

impl KeyLayer {
    pub fn new(name: impl Into<String>, priority: i32) -> Self {
        Self { name: name.into(), priority, enabled: true, bindings: Vec::new() }
    }

    pub fn add_binding(&mut self, binding: KeyBinding) {
        self.bindings.push(binding);
    }

    pub fn remove_binding(&mut self, id: &str) -> bool {
        let len = self.bindings.len();
        self.bindings.retain(|b| b.id != id);
        self.bindings.len() < len
    }

    pub fn bindings(&self) -> &[KeyBinding] {
        &self.bindings
    }

    pub fn find_binding(&self, event: &KeyEvent, current_mode: Option<&str>) -> Option<&KeyBinding> {
        if !self.enabled {
            return None;
        }
        self.bindings.iter().find(|b| b.matches(event, current_mode))
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

/// The main keymap manager that resolves bindings across layers
#[derive(Debug)]
pub struct Keymap {
    layers: Vec<KeyLayer>,
    current_mode: Option<String>,
    pending_sequence: Vec<KeyCombo>,
    chord_timeout_ms: u64,
    command_history: Vec<String>,
}

impl Default for Keymap {
    fn default() -> Self {
        Self::new()
    }
}

impl Keymap {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            current_mode: None,
            pending_sequence: Vec::new(),
            chord_timeout_ms: 1000,
            command_history: Vec::new(),
        }
    }

    pub fn add_layer(&mut self, layer: KeyLayer) {
        self.layers.push(layer);
        self.layers.sort_by_key(|b| std::cmp::Reverse(b.priority));
    }

    pub fn remove_layer(&mut self, name: &str) -> bool {
        let len = self.layers.len();
        self.layers.retain(|l| l.name != name);
        self.layers.len() < len
    }

    pub fn get_layer(&self, name: &str) -> Option<&KeyLayer> {
        self.layers.iter().find(|l| l.name == name)
    }

    pub fn get_layer_mut(&mut self, name: &str) -> Option<&mut KeyLayer> {
        self.layers.iter_mut().find(|l| l.name == name)
    }

    pub fn add_binding_to_layer(&mut self, layer_name: &str, binding: KeyBinding, priority: i32) {
        if let Some(layer) = self.get_layer_mut(layer_name) {
            layer.add_binding(binding);
        } else {
            let mut layer = KeyLayer::new(layer_name, priority);
            layer.add_binding(binding);
            self.add_layer(layer);
        }
    }

    pub fn add_binding(&mut self, binding: KeyBinding) {
        self.add_binding_to_layer("default", binding, 0);
    }

    pub fn set_mode(&mut self, mode: impl Into<String>) {
        self.current_mode = Some(mode.into());
        self.pending_sequence.clear();
    }

    pub fn current_mode(&self) -> Option<&str> {
        self.current_mode.as_deref()
    }

    pub fn clear_mode(&mut self) {
        self.current_mode = None;
        self.pending_sequence.clear();
    }

    pub fn set_chord_timeout(&mut self, ms: u64) {
        self.chord_timeout_ms = ms;
    }

    pub fn chord_timeout_ms(&self) -> u64 {
        self.chord_timeout_ms
    }

    pub fn handle_event(&mut self, event: &KeyEvent) -> Option<String> {
        if !self.pending_sequence.is_empty() {
            let expected = &self.pending_sequence[0];
            if expected.matches(event) {
                self.pending_sequence.remove(0);
                if self.pending_sequence.is_empty() {
                    return self.last_binding_command();
                }
                return None;
            } else {
                self.pending_sequence.clear();
            }
        }

        for layer in &self.layers {
            if let Some(binding) = layer.find_binding(event, self.current_mode.as_deref()) {
                if binding.sequence.len() == 1 {
                    self.command_history.push(binding.command.clone());
                    return Some(binding.command.clone());
                } else {
                    self.pending_sequence = binding.sequence.keys[1..].to_vec();
                    self.command_history.push(binding.command.clone());
                    return None;
                }
            }
        }

        None
    }

    pub fn has_pending_sequence(&self) -> bool {
        !self.pending_sequence.is_empty()
    }

    pub fn clear_pending_sequence(&mut self) {
        self.pending_sequence.clear();
    }

    pub fn pending_keys(&self) -> &[KeyCombo] {
        &self.pending_sequence
    }

    pub fn command_history(&self) -> &[String] {
        &self.command_history
    }

    pub fn clear_history(&mut self) {
        self.command_history.clear();
    }

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

    pub fn all_bindings(&self) -> Vec<(&KeyBinding, &str)> {
        self.layers.iter().flat_map(|layer| layer.bindings().iter().map(move |b| (b, layer.name.as_str()))).collect()
    }

    fn last_binding_command(&self) -> Option<String> {
        self.command_history.last().cloned()
    }
}

/// Builder for creating keymaps with a fluent API
pub struct KeymapBuilder {
    keymap: Keymap,
}

impl KeymapBuilder {
    pub fn new() -> Self {
        Self { keymap: Keymap::new() }
    }

    pub fn binding(mut self, id: &str, keys: &str, desc: &str) -> Self {
        self.keymap.add_binding(KeyBinding::new(id, keys, desc));
        self
    }

    pub fn binding_in_layer(mut self, layer: &str, priority: i32, id: &str, keys: &str, desc: &str) -> Self {
        self.keymap.add_binding_to_layer(layer, KeyBinding::new(id, keys, desc), priority);
        self
    }

    pub fn mode(mut self, mode: &str) -> Self {
        self.keymap.set_mode(mode);
        self
    }

    pub fn build(self) -> Keymap {
        self.keymap
    }
}

impl Default for KeymapBuilder {
    fn default() -> Self {
        Self::new()
    }
}
