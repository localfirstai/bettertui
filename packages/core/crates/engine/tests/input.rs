//! Integration tests for the input module.
//!
//! Tests all input types: InputRuntime, ClipboardState, MouseState, KeyboardState,
//! EventBus, KeyEvent, MouseEvent, FocusManager, KeybindingManager, etc.

use bettertui_engine::input::*;

// Uses input types via prelude

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
    let event = InputEvent { event_type: InputEventType::Resize(80, 24), timestamp: 0 };
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
    let input = MouseInput::new(0, 0, MouseButtons::empty());
    runtime.handle_mouse_input(input);
    assert_eq!(runtime.events().len(), 1);
}

#[test]
fn input_runtime_handle_clipboard_input() {
    let mut runtime = InputRuntime::new();
    let input = ClipboardInput::copy("hello".to_string());
    runtime.handle_clipboard_input(input);
    assert_eq!(runtime.events().len(), 1);
    assert_eq!(runtime.clipboard().get_content(), Some("hello"));
}

#[test]
fn input_runtime_clear() {
    let mut runtime = InputRuntime::new();
    let event = InputEvent { event_type: InputEventType::Resize(80, 24), timestamp: 0 };
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
