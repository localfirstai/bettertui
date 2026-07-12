use super::*;
use crate::events::types::{KeyEvent, Modifiers};
use crate::tree::node_id::NodeId;
use crate::tree::node_kind::NodeKind;
use crate::tree::render_node::RenderNode;

fn make_id() -> NodeId {
    let mut arena = crate::tree::arena::NodeArena::new();
    arena.insert(RenderNode::new(NodeKind::Box))
}

fn make_event(key: Key, modifiers: Modifiers) -> KeyEvent {
    KeyEvent::new(key, make_id()).with_modifiers(modifiers)
}

// ─── KeyCombo Tests ──────────────────────────────────────────────────────────

#[test]
fn keycombo_plain() {
    let combo = KeyCombo::plain(Key::Enter);
    assert_eq!(combo.key, Key::Enter);
    assert!(combo.modifiers.is_empty());
}

#[test]
fn keycombo_with_ctrl() {
    let combo = KeyCombo::with_ctrl(Key::Character('s'));
    assert_eq!(combo.key, Key::Character('s'));
    assert!(combo.modifiers.ctrl);
    assert!(!combo.modifiers.shift);
}

#[test]
fn keycombo_with_shift() {
    let combo = KeyCombo::with_shift(Key::Tab);
    assert_eq!(combo.key, Key::Tab);
    assert!(combo.modifiers.shift);
}

#[test]
fn keycombo_with_alt() {
    let combo = KeyCombo::with_alt(Key::Character('x'));
    assert_eq!(combo.key, Key::Character('x'));
    assert!(combo.modifiers.alt);
}

#[test]
fn keycombo_matches_event() {
    let combo = KeyCombo::with_ctrl(Key::Character('s'));
    let event = make_event(
        Key::Character('s'),
        Modifiers {
            ctrl: true,
            ..Modifiers::NONE
        },
    );
    assert!(combo.matches(&event));

    let event_no_mod = make_event(Key::Character('s'), Modifiers::NONE);
    assert!(!combo.matches(&event_no_mod));

    let event_wrong_key = make_event(
        Key::Character('x'),
        Modifiers {
            ctrl: true,
            ..Modifiers::NONE
        },
    );
    assert!(!combo.matches(&event_wrong_key));
}

// ─── KeySequence Tests ───────────────────────────────────────────────────────

#[test]
fn keysequence_single() {
    let seq = KeySequence::single(KeyCombo::plain(Key::Enter));
    assert_eq!(seq.len(), 1);
    assert!(!seq.is_empty());
}

#[test]
fn keysequence_chord() {
    let seq = KeySequence::chord(vec![
        KeyCombo::plain(Key::Character('d')),
        KeyCombo::plain(Key::Character('d')),
    ]);
    assert_eq!(seq.len(), 2);
    assert!(!seq.is_empty());
}

#[test]
fn keysequence_starts_with() {
    let seq = KeySequence::chord(vec![
        KeyCombo::plain(Key::Character('d')),
        KeyCombo::plain(Key::Character('d')),
    ]);
    assert!(seq.starts_with(&KeyCombo::plain(Key::Character('d'))));
    assert!(!seq.starts_with(&KeyCombo::plain(Key::Character('x'))));
}

#[test]
fn keysequence_tail() {
    let seq = KeySequence::chord(vec![
        KeyCombo::plain(Key::Character('d')),
        KeyCombo::plain(Key::Character('d')),
    ]);
    let tail = seq.tail();
    assert_eq!(tail.len(), 1);
    assert_eq!(tail.keys[0].key, Key::Character('d'));
}

// ─── KeyParser Tests ─────────────────────────────────────────────────────────

#[test]
fn parser_single_key() {
    let combo = KeyParser::parse_combo("enter").unwrap();
    assert_eq!(combo.key, Key::Enter);
    assert!(combo.modifiers.is_empty());
}

#[test]
fn parser_ctrl_key() {
    let combo = KeyParser::parse_combo("ctrl+s").unwrap();
    assert_eq!(combo.key, Key::Character('s'));
    assert!(combo.modifiers.ctrl);
}

#[test]
fn parser_alt_key() {
    let combo = KeyParser::parse_combo("alt+x").unwrap();
    assert_eq!(combo.key, Key::Character('x'));
    assert!(combo.modifiers.alt);
}

#[test]
fn parser_shift_key() {
    let combo = KeyParser::parse_combo("shift+tab").unwrap();
    assert_eq!(combo.key, Key::Tab);
    assert!(combo.modifiers.shift);
}

#[test]
fn parser_multiple_modifiers() {
    let combo = KeyParser::parse_combo("ctrl+shift+k").unwrap();
    assert_eq!(combo.key, Key::Character('k'));
    assert!(combo.modifiers.ctrl);
    assert!(combo.modifiers.shift);
}

#[test]
fn parser_function_key() {
    let combo = KeyParser::parse_combo("f1").unwrap();
    assert_eq!(combo.key, Key::F(1));

    let combo = KeyParser::parse_combo("f12").unwrap();
    assert_eq!(combo.key, Key::F(12));
}

#[test]
fn parser_special_keys() {
    assert_eq!(KeyParser::parse_combo("escape").unwrap().key, Key::Escape);
    assert_eq!(
        KeyParser::parse_combo("backspace").unwrap().key,
        Key::Backspace
    );
    assert_eq!(KeyParser::parse_combo("tab").unwrap().key, Key::Tab);
    assert_eq!(KeyParser::parse_combo("space").unwrap().key, Key::Space);
    assert_eq!(KeyParser::parse_combo("up").unwrap().key, Key::ArrowUp);
    assert_eq!(KeyParser::parse_combo("down").unwrap().key, Key::ArrowDown);
}

#[test]
fn parser_aliases() {
    assert_eq!(KeyParser::parse_combo("return").unwrap().key, Key::Enter);
    assert_eq!(KeyParser::parse_combo("esc").unwrap().key, Key::Escape);
    assert_eq!(KeyParser::parse_combo("bs").unwrap().key, Key::Backspace);
    assert_eq!(KeyParser::parse_combo("del").unwrap().key, Key::Delete);
    assert_eq!(KeyParser::parse_combo("pgup").unwrap().key, Key::PageUp);
    assert_eq!(KeyParser::parse_combo("pgdn").unwrap().key, Key::PageDown);
}

#[test]
fn parser_character_key() {
    let combo = KeyParser::parse_combo("a").unwrap();
    assert_eq!(combo.key, Key::Character('a'));

    let combo = KeyParser::parse_combo("z").unwrap();
    assert_eq!(combo.key, Key::Character('z'));
}

#[test]
fn parser_invalid_key() {
    assert!(KeyParser::parse_combo("invalid_key").is_err());
}

#[test]
fn parser_sequence_single() {
    let seq = KeyParser::parse_sequence("ctrl+s").unwrap();
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.keys[0].key, Key::Character('s'));
    assert!(seq.keys[0].modifiers.ctrl);
}

#[test]
fn parser_sequence_chord() {
    let seq = KeyParser::parse_sequence("dd").unwrap();
    assert_eq!(seq.len(), 2);
    assert_eq!(seq.keys[0].key, Key::Character('d'));
    assert_eq!(seq.keys[1].key, Key::Character('d'));
}

#[test]
fn parser_sequence_comma_separated() {
    let seq = KeyParser::parse_sequence("ctrl+x, ctrl+s").unwrap();
    assert_eq!(seq.len(), 2);
    assert_eq!(seq.keys[0].key, Key::Character('x'));
    assert!(seq.keys[0].modifiers.ctrl);
    assert_eq!(seq.keys[1].key, Key::Character('s'));
    assert!(seq.keys[1].modifiers.ctrl);
}

// ─── KeyBinding Tests ────────────────────────────────────────────────────────

#[test]
fn binding_creation() {
    let binding = KeyBinding::new("save", "ctrl+s", "Save file");
    assert_eq!(binding.id, "save");
    assert_eq!(binding.command, "save");
    assert_eq!(binding.description.as_deref(), Some("Save file"));
    assert!(binding.enabled);
    assert!(binding.condition.is_none());
}

#[test]
fn binding_with_command() {
    let binding = KeyBinding::new("save", "ctrl+s", "Save file").with_command("file.save");
    assert_eq!(binding.command, "file.save");
}

#[test]
fn binding_in_mode() {
    let binding = KeyBinding::new("delete_line", "dd", "Delete line").in_mode("normal");
    assert!(binding.condition.is_some());
    match &binding.condition {
        Some(BindingCondition::Mode(mode)) => assert_eq!(mode, "normal"),
        _ => panic!("Expected mode condition"),
    }
}

#[test]
fn binding_disabled() {
    let binding = KeyBinding::new("test", "ctrl+t", "Test").disabled();
    assert!(!binding.enabled);
}

#[test]
fn binding_matches_event() {
    let binding = KeyBinding::new("save", "ctrl+s", "Save file");
    let event = make_event(
        Key::Character('s'),
        Modifiers {
            ctrl: true,
            ..Modifiers::NONE
        },
    );
    assert!(binding.matches(&event, None));

    let event_wrong = make_event(Key::Character('x'), Modifiers::NONE);
    assert!(!binding.matches(&event_wrong, None));
}

#[test]
fn binding_matches_with_mode() {
    let binding = KeyBinding::new("delete_line", "dd", "Delete line").in_mode("normal");

    let event = make_event(Key::Character('d'), Modifiers::NONE);

    // Should match in normal mode
    assert!(binding.matches(&event, Some("normal")));

    // Should not match in insert mode
    assert!(!binding.matches(&event, Some("insert")));

    // Should not match with no mode
    assert!(!binding.matches(&event, None));
}

#[test]
fn binding_disabled_no_match() {
    let binding = KeyBinding::new("test", "ctrl+t", "Test").disabled();
    let event = make_event(
        Key::Character('t'),
        Modifiers {
            ctrl: true,
            ..Modifiers::NONE
        },
    );
    assert!(!binding.matches(&event, None));
}

// ─── KeyLayer Tests ──────────────────────────────────────────────────────────

#[test]
fn layer_creation() {
    let layer = KeyLayer::new("vim", 10);
    assert_eq!(layer.name, "vim");
    assert_eq!(layer.priority, 10);
    assert!(layer.enabled);
    assert!(layer.bindings().is_empty());
}

#[test]
fn layer_add_binding() {
    let mut layer = KeyLayer::new("vim", 10);
    layer.add_binding(KeyBinding::new("save", "ctrl+s", "Save"));
    assert_eq!(layer.bindings().len(), 1);
}

#[test]
fn layer_remove_binding() {
    let mut layer = KeyLayer::new("vim", 10);
    layer.add_binding(KeyBinding::new("save", "ctrl+s", "Save"));
    assert!(layer.remove_binding("save"));
    assert!(layer.bindings().is_empty());
    assert!(!layer.remove_binding("nonexistent"));
}

#[test]
fn layer_find_binding() {
    let mut layer = KeyLayer::new("vim", 10);
    layer.add_binding(KeyBinding::new("save", "ctrl+s", "Save"));

    let event = make_event(
        Key::Character('s'),
        Modifiers {
            ctrl: true,
            ..Modifiers::NONE
        },
    );

    assert!(layer.find_binding(&event, None).is_some());
    assert_eq!(layer.find_binding(&event, None).unwrap().id, "save");

    let event_wrong = make_event(Key::Character('x'), Modifiers::NONE);
    assert!(layer.find_binding(&event_wrong, None).is_none());
}

#[test]
fn layer_disabled_no_find() {
    let mut layer = KeyLayer::new("vim", 10);
    layer.add_binding(KeyBinding::new("save", "ctrl+s", "Save"));
    layer.disable();

    let event = make_event(
        Key::Character('s'),
        Modifiers {
            ctrl: true,
            ..Modifiers::NONE
        },
    );

    assert!(layer.find_binding(&event, None).is_none());
}

#[test]
fn layer_enable_disable() {
    let mut layer = KeyLayer::new("vim", 10);
    assert!(layer.enabled);
    layer.disable();
    assert!(!layer.enabled);
    layer.enable();
    assert!(layer.enabled);
}

// ─── Keymap Tests ────────────────────────────────────────────────────────────

#[test]
fn keymap_creation() {
    let keymap = Keymap::new();
    assert!(keymap.current_mode().is_none());
    assert!(!keymap.has_pending_sequence());
}

#[test]
fn keymap_add_binding() {
    let mut keymap = Keymap::new();
    keymap.add_binding(KeyBinding::new("save", "ctrl+s", "Save"));

    let event = make_event(
        Key::Character('s'),
        Modifiers {
            ctrl: true,
            ..Modifiers::NONE
        },
    );

    let cmd = keymap.handle_event(&event);
    assert_eq!(cmd, Some("save".to_string()));
}

#[test]
fn keymap_priority() {
    let mut keymap = Keymap::new();

    // Low priority layer
    let mut low = KeyLayer::new("low", 1);
    low.add_binding(KeyBinding::new("low_save", "ctrl+s", "Low Save"));
    keymap.add_layer(low);

    // High priority layer
    let mut high = KeyLayer::new("high", 10);
    high.add_binding(KeyBinding::new("high_save", "ctrl+s", "High Save"));
    keymap.add_layer(high);

    let event = make_event(
        Key::Character('s'),
        Modifiers {
            ctrl: true,
            ..Modifiers::NONE
        },
    );

    // High priority should win
    let cmd = keymap.handle_event(&event);
    assert_eq!(cmd, Some("high_save".to_string()));
}

#[test]
fn keymap_mode() {
    let mut keymap = Keymap::new();

    let mut normal = KeyLayer::new("normal", 10);
    normal.add_binding(KeyBinding::new("delete_line", "dd", "Delete line").in_mode("normal"));
    keymap.add_layer(normal);

    keymap.set_mode("normal");

    let event = make_event(Key::Character('d'), Modifiers::NONE);

    // Should match in normal mode
    let cmd = keymap.handle_event(&event);
    // This is a chord binding, so it should start tracking
    assert!(cmd.is_none());
    assert!(keymap.has_pending_sequence());

    // Complete the chord
    let cmd = keymap.handle_event(&event);
    assert_eq!(cmd, Some("delete_line".to_string()));
}

#[test]
fn keymap_chord() {
    let mut keymap = Keymap::new();
    keymap.add_binding(KeyBinding::new("delete_line", "dd", "Delete line"));

    let event = make_event(Key::Character('d'), Modifiers::NONE);

    // First 'd' starts the chord
    let cmd = keymap.handle_event(&event);
    assert!(cmd.is_none());
    assert!(keymap.has_pending_sequence());
    assert_eq!(keymap.pending_keys().len(), 1);

    // Second 'd' completes the chord
    let cmd = keymap.handle_event(&event);
    assert_eq!(cmd, Some("delete_line".to_string()));
    assert!(!keymap.has_pending_sequence());
}

#[test]
fn keymap_chord_broken() {
    let mut keymap = Keymap::new();
    keymap.add_binding(KeyBinding::new("delete_line", "dd", "Delete line"));

    let d_event = make_event(Key::Character('d'), Modifiers::NONE);
    let x_event = make_event(Key::Character('x'), Modifiers::NONE);

    // First 'd' starts the chord
    keymap.handle_event(&d_event);
    assert!(keymap.has_pending_sequence());

    // 'x' breaks the chord
    let cmd = keymap.handle_event(&x_event);
    assert!(cmd.is_none());
    assert!(!keymap.has_pending_sequence());
}

#[test]
fn keymap_remove_layer() {
    let mut keymap = Keymap::new();
    keymap.add_layer(KeyLayer::new("vim", 10));
    assert!(keymap.get_layer("vim").is_some());
    assert!(keymap.remove_layer("vim"));
    assert!(keymap.get_layer("vim").is_none());
    assert!(!keymap.remove_layer("nonexistent"));
}

#[test]
fn keymap_get_layer() {
    let mut keymap = Keymap::new();
    keymap.add_layer(KeyLayer::new("vim", 10));
    assert!(keymap.get_layer("vim").is_some());
    assert_eq!(keymap.get_layer("vim").unwrap().name, "vim");
}

#[test]
fn keymap_set_mode() {
    let mut keymap = Keymap::new();
    assert!(keymap.current_mode().is_none());
    keymap.set_mode("normal");
    assert_eq!(keymap.current_mode(), Some("normal"));
    keymap.clear_mode();
    assert!(keymap.current_mode().is_none());
}

#[test]
fn keymap_command_history() {
    let mut keymap = Keymap::new();
    keymap.add_binding(KeyBinding::new("save", "ctrl+s", "Save"));
    keymap.add_binding(KeyBinding::new("open", "ctrl+o", "Open"));

    let save_event = make_event(
        Key::Character('s'),
        Modifiers {
            ctrl: true,
            ..Modifiers::NONE
        },
    );
    let open_event = make_event(
        Key::Character('o'),
        Modifiers {
            ctrl: true,
            ..Modifiers::NONE
        },
    );

    keymap.handle_event(&save_event);
    keymap.handle_event(&open_event);

    assert_eq!(keymap.command_history(), &["save", "open"]);

    keymap.clear_history();
    assert!(keymap.command_history().is_empty());
}

#[test]
fn keymap_active_bindings() {
    let mut keymap = Keymap::new();
    keymap.add_binding(KeyBinding::new("save", "ctrl+s", "Save"));

    let mut vim = KeyLayer::new("vim", 10);
    vim.add_binding(KeyBinding::new("delete_line", "dd", "Delete line"));
    keymap.add_layer(vim);

    let bindings = keymap.active_bindings();
    assert_eq!(bindings.len(), 2);
}

#[test]
fn keymap_builder() {
    let keymap = KeymapBuilder::new()
        .binding("save", "ctrl+s", "Save file")
        .binding("open", "ctrl+o", "Open file")
        .mode("normal")
        .build();

    assert_eq!(keymap.current_mode(), Some("normal"));
    assert_eq!(keymap.active_bindings().len(), 2);
}

#[test]
fn keymap_builder_with_layer() {
    let keymap = KeymapBuilder::new()
        .binding("save", "ctrl+s", "Save file")
        .binding_in_layer("vim", 10, "delete_line", "dd", "Delete line")
        .build();

    assert_eq!(keymap.active_bindings().len(), 2);
    assert!(keymap.get_layer("vim").is_some());
}

#[test]
fn keymap_clear_pending() {
    let mut keymap = Keymap::new();
    keymap.add_binding(KeyBinding::new("delete_line", "dd", "Delete line"));

    let event = make_event(Key::Character('d'), Modifiers::NONE);
    keymap.handle_event(&event);
    assert!(keymap.has_pending_sequence());

    keymap.clear_pending_sequence();
    assert!(!keymap.has_pending_sequence());
}

#[test]
fn keymap_chord_timeout() {
    let mut keymap = Keymap::new();
    assert_eq!(keymap.chord_timeout_ms(), 1000);

    keymap.set_chord_timeout(500);
    assert_eq!(keymap.chord_timeout_ms(), 500);
}

#[test]
fn keymap_multiple_layers_same_key() {
    let mut keymap = Keymap::new();

    let mut layer1 = KeyLayer::new("layer1", 1);
    layer1.add_binding(KeyBinding::new("cmd1", "ctrl+s", "Command 1"));
    keymap.add_layer(layer1);

    let mut layer2 = KeyLayer::new("layer2", 2);
    layer2.add_binding(KeyBinding::new("cmd2", "ctrl+s", "Command 2"));
    keymap.add_layer(layer2);

    // Disabled layer should not match
    keymap.get_layer_mut("layer2").unwrap().disable();

    let event = make_event(
        Key::Character('s'),
        Modifiers {
            ctrl: true,
            ..Modifiers::NONE
        },
    );

    let cmd = keymap.handle_event(&event);
    assert_eq!(cmd, Some("cmd1".to_string()));
}
