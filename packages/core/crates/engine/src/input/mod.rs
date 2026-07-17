//! Input system: event model, keyboard, mouse, clipboard, focus management, keybindings, and runtime state.

pub mod clipboard;
pub mod focus;
pub mod key;
pub mod keybinding;
pub mod mouse;
pub mod state;
pub mod types;

// Re-export all public types at crate level for backwards compatibility.
pub use clipboard::*;
pub use focus::*;
pub use key::*;
pub use keybinding::*;
pub use mouse::*;
pub use state::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_bus::{
        Event, EventPhase, Key, KeyEvent, Modifiers, MouseButton, MouseEvent, PasteEvent, ResizeEvent,
    };
    use crate::tree::NodeId;

    mod clipboard {
        use super::*;

        #[test]
        fn new_default() {
            let c = ClipboardInput::default();
            assert!(c.data.is_empty());
            assert_eq!(c.action, ClipboardAction::Copy);
        }

        #[test]
        fn copy_constructor() {
            let c = ClipboardInput::copy("hello".to_string());
            assert_eq!(c.data, "hello");
            assert!(c.is_copy());
            assert!(!c.is_paste());
            assert!(!c.is_cut());
        }

        #[test]
        fn paste_constructor() {
            let c = ClipboardInput::paste("world".to_string());
            assert_eq!(c.data, "world");
            assert!(c.is_paste());
        }

        #[test]
        fn cut_constructor() {
            let c = ClipboardInput::cut("data".to_string());
            assert_eq!(c.data, "data");
            assert!(c.is_cut());
        }

        #[test]
        fn action_ordering() {
            assert_ne!(ClipboardAction::Copy, ClipboardAction::Paste);
            assert_ne!(ClipboardAction::Copy, ClipboardAction::Cut);
        }
    }

    mod keyboard {
        use super::*;

        #[test]
        fn new_default() {
            let k = KeyboardInput::default();
            assert_eq!(k.key, '\0');
            assert!(k.modifiers.is_empty());
            assert_eq!(k.action, KeyAction::Press);
        }

        #[test]
        fn new_with_modifiers() {
            let k = KeyboardInput::new('a', KeyModifiers::CONTROL);
            assert!(k.is_ctrl());
            assert!(!k.is_shift());
        }

        #[test]
        fn modifier_flags() {
            let ctrl_alt = KeyModifiers::CONTROL | KeyModifiers::ALT;
            let k = KeyboardInput::new('x', ctrl_alt);
            assert!(k.is_ctrl());
            assert!(k.is_alt());
            assert!(!k.is_shift());
            assert!(!k.is_super());
        }

        #[test]
        fn action_variants() {
            let p = KeyboardInput::press('a', KeyModifiers::empty());
            assert_eq!(p.action, KeyAction::Press);

            let r = KeyboardInput::release('a', KeyModifiers::empty());
            assert_eq!(r.action, KeyAction::Release);

            let rr = KeyboardInput::repeat('a', KeyModifiers::empty());
            assert_eq!(rr.action, KeyAction::Repeat);
        }

        #[test]
        fn with_action_overrides() {
            let k = KeyboardInput::new('a', KeyModifiers::empty()).with_action(KeyAction::Repeat);
            assert_eq!(k.action, KeyAction::Repeat);
        }

        #[test]
        fn modifier_only_keys() {
            let null = KeyboardInput::new('\0', KeyModifiers::empty());
            assert!(null.is_modifier_only());

            let esc = KeyboardInput::new('\x1b', KeyModifiers::empty());
            assert!(esc.is_modifier_only());

            let normal = KeyboardInput::new('a', KeyModifiers::empty());
            assert!(!normal.is_modifier_only());
        }

        #[test]
        fn display_string_ctrl() {
            let k = KeyboardInput::new('c', KeyModifiers::CONTROL);
            assert_eq!(k.to_display_string(), "Ctrl+c");
        }

        #[test]
        fn display_string_ctrl_shift() {
            let k = KeyboardInput::new('C', KeyModifiers::CONTROL | KeyModifiers::SHIFT);
            assert_eq!(k.to_display_string(), "Ctrl+Shift+C");
        }

        #[test]
        fn display_string_special() {
            let esc = KeyboardInput::new('\x1b', KeyModifiers::empty());
            assert_eq!(esc.to_display_string(), "Esc");

            let space = KeyboardInput::new(' ', KeyModifiers::empty());
            assert_eq!(space.to_display_string(), "Space");
        }

        #[test]
        fn key_modifiers_bitflags() {
            let empty = KeyModifiers::empty();
            assert!(empty.is_empty());

            let all = KeyModifiers::all();
            assert!(all.contains(KeyModifiers::SHIFT));
            assert!(all.contains(KeyModifiers::CONTROL));
            assert!(all.contains(KeyModifiers::ALT));
            assert!(all.contains(KeyModifiers::SUPER));
        }
    }

    mod mouse {
        use super::*;

        #[test]
        fn new_default() {
            let m = MouseInput::default();
            assert_eq!(m.x, 0);
            assert_eq!(m.y, 0);
            assert!(m.buttons.is_empty());
        }

        #[test]
        fn new_with_position() {
            let m = MouseInput::new(10, 20, MouseButtons::LEFT);
            assert_eq!(m.x, 10);
            assert_eq!(m.y, 20);
            assert!(m.is_left_button());
        }

        #[test]
        fn button_checks() {
            let left = MouseInput::new(0, 0, MouseButtons::LEFT);
            assert!(left.is_left_button());
            assert!(!left.is_right_button());
            assert!(!left.is_middle_button());

            let right = MouseInput::new(0, 0, MouseButtons::RIGHT);
            assert!(right.is_right_button());

            let middle = MouseInput::new(0, 0, MouseButtons::MIDDLE);
            assert!(middle.is_middle_button());
        }

        #[test]
        fn with_modifiers() {
            let m = MouseInput::new(5, 5, MouseButtons::LEFT).with_modifiers(KeyModifiers::CONTROL);
            assert!(m.is_ctrl());
        }

        #[test]
        fn event_type_variants() {
            let press = MouseInput::press(1, 2, MouseButtons::LEFT);
            assert_eq!(press.event_type, MouseEventType::Press);

            let release = MouseInput::release(1, 2, MouseButtons::LEFT);
            assert_eq!(release.event_type, MouseEventType::Release);

            let move_ = MouseInput::move_to(3, 4);
            assert_eq!(move_.event_type, MouseEventType::Move);

            let scroll = MouseInput::scroll(5, 6, 0, -3);
            assert_eq!(scroll.event_type, MouseEventType::Scroll);
            assert_eq!(scroll.scroll_delta, Some((0, -3)));

            let drag = MouseInput::drag(7, 8, MouseButtons::LEFT);
            assert_eq!(drag.event_type, MouseEventType::Drag);

            let drop = MouseInput::drop(9, 10, MouseButtons::LEFT);
            assert_eq!(drop.event_type, MouseEventType::Drop);
        }

        #[test]
        fn mouse_buttons_bitflags() {
            let both = MouseButtons::LEFT | MouseButtons::RIGHT;
            assert!(both.contains(MouseButtons::LEFT));
            assert!(both.contains(MouseButtons::RIGHT));
            assert!(!both.contains(MouseButtons::MIDDLE));
        }
    }

    mod input_event {
        use super::*;

        #[test]
        fn keyboard_event() {
            let input = KeyboardInput::new('q', KeyModifiers::empty());
            let ev = InputEvent::keyboard(input);
            assert!(ev.is_keyboard());
            assert!(!ev.is_mouse());
            assert!(!ev.is_clipboard());
            assert!(!ev.is_resize());
            assert!(!ev.is_focus());
            assert!(!ev.is_blur());
            assert!(!ev.is_paste());
        }

        #[test]
        fn focus_blur_events() {
            let focus = InputEvent::focus();
            assert!(focus.is_focus());

            let blur = InputEvent::blur();
            assert!(blur.is_blur());
        }

        #[test]
        fn resize_event() {
            let r = InputEvent::resize(120, 40);
            assert!(r.is_resize());
            assert!(!r.is_focus());
        }

        #[test]
        fn paste_event() {
            let p = InputEvent::paste("hello".to_string());
            assert!(p.is_paste());
        }

        #[test]
        fn clipboard_event() {
            let c = InputEvent::clipboard(ClipboardInput::copy("data".to_string()));
            assert!(c.is_clipboard());
        }

        #[test]
        fn mouse_event() {
            let m = InputEvent::mouse(MouseInput::new(10, 20, MouseButtons::RIGHT));
            assert!(m.is_mouse());
        }

        #[test]
        fn default_is_focus() {
            assert!(InputEvent::default().is_focus());
        }

        #[test]
        fn timestamps_advance() {
            let e1 = InputEvent::focus();
            let e2 = InputEvent::focus();
            assert!(e2.timestamp >= e1.timestamp);
        }
    }

    mod key_code_enum {
        use super::*;

        #[test]
        fn variant_distinct() {
            assert_ne!(KeyCode::Enter, KeyCode::Esc);
            assert_ne!(KeyCode::Tab, KeyCode::Space);
        }

        #[test]
        fn f_keys() {
            let f1 = KeyCode::F(1);
            let f12 = KeyCode::F(12);
            assert_ne!(f1, f12);
        }

        #[test]
        fn media_keys() {
            let pp = MediaKey::PlayPause;
            let next = MediaKey::NextTrack;
            assert_ne!(pp, next);
        }
    }

    mod event_enum {
        use super::*;

        #[test]
        fn key_event_new() {
            let ke = KeyEvent::new(Key::Enter, NodeId::default());
            assert_eq!(ke.key, Key::Enter);
            assert!(ke.modifiers.is_empty());
            assert_eq!(ke.phase, EventPhase::Target);
        }

        #[test]
        fn key_event_with_modifiers() {
            let ke = KeyEvent::new(Key::Character('c'), NodeId::default())
                .with_modifiers(Modifiers { ctrl: true, ..Default::default() });
            assert_eq!(ke.key, Key::Character('c'));
            assert!(ke.modifiers.ctrl);
        }

        #[test]
        fn key_event_prevent_default() {
            let mut ke = KeyEvent::new(Key::Escape, NodeId::default());
            assert!(!ke.default_prevented);
            ke.prevent_default();
            assert!(ke.default_prevented);
        }

        #[test]
        fn mouse_event_new() {
            let pt = crate::tree::Point::new(5, 10);
            let me = MouseEvent::new(MouseButton::Left, pt, NodeId::default());
            assert_eq!(me.button, MouseButton::Left);
            assert_eq!(me.position, pt);
        }

        #[test]
        fn paste_event_new() {
            let pe = PasteEvent::new("content", NodeId::default());
            assert_eq!(&*pe.text, "content");
        }

        #[test]
        fn event_types() {
            let ke = KeyEvent::new(Key::Enter, NodeId::default());
            assert!(matches!(Event::Key(ke), Event::Key(_)));

            let me = MouseEvent::new(MouseButton::Left, crate::tree::Point::new(0, 0), NodeId::default());
            assert!(matches!(Event::Mouse(me), Event::Mouse(_)));
        }

        #[test]
        fn event_phase_default() {
            let ke = KeyEvent::new(Key::Enter, NodeId::default());
            assert_eq!(ke.phase, EventPhase::Target);
        }

        #[test]
        fn event_phase_set_get() {
            let ke = KeyEvent::new(Key::Enter, NodeId::default());
            let event = Event::Key(ke);
            assert_eq!(event.phase(), EventPhase::Target);

            let mut event = event;
            event.set_phase(EventPhase::Capture);
            assert_eq!(event.phase(), EventPhase::Capture);
        }

        #[test]
        fn event_phase_resize() {
            let event = Event::Resize(ResizeEvent::new(80, 24, 80, 24));
            assert_eq!(event.phase(), EventPhase::Target);
            let mut event = event;
            event.set_phase(EventPhase::Bubble);
            assert_eq!(event.phase(), EventPhase::Target);
        }

        #[test]
        fn event_target_returns_some() {
            let target = NodeId::default();
            let ke = KeyEvent::new(Key::Enter, target);
            let event = Event::Key(ke);
            assert!(event.target().is_some());
        }

        #[test]
        fn event_target_resize_is_none() {
            let event = Event::Resize(ResizeEvent::new(80, 24, 80, 24));
            assert!(event.target().is_none());
        }

        #[test]
        fn event_is_consumed_false_by_default() {
            let ke = KeyEvent::new(Key::Enter, NodeId::default());
            let event = Event::Key(ke);
            assert!(!event.is_consumed());
        }

        #[test]
        fn event_modifiers() {
            let mut ke = KeyEvent::new(Key::Tab, NodeId::default());
            ke.modifiers.shift = true;
            assert!(ke.modifiers.shift);
            assert!(!ke.modifiers.ctrl);
        }
    }

    mod modifiers {
        use super::*;

        #[test]
        fn new_empty() {
            let m = Modifiers::default();
            assert!(!m.ctrl);
            assert!(!m.shift);
            assert!(!m.alt);
            assert!(!m.meta);
        }

        #[test]
        fn is_empty_true() {
            let m = Modifiers::default();
            assert!(m.is_empty());
        }

        #[test]
        fn is_empty_false() {
            let m = Modifiers { ctrl: true, ..Default::default() };
            assert!(!m.is_empty());
        }
    }

    mod mouse_button_enum {
        use super::*;

        #[test]
        fn variants() {
            assert_ne!(MouseButton::Left, MouseButton::Right);
            assert_ne!(MouseButton::Left, MouseButton::Middle);
        }
    }

    mod event_phase_enum {
        use super::*;

        #[test]
        fn order() {
            assert_ne!(EventPhase::Capture, EventPhase::Target);
            assert_ne!(EventPhase::Target, EventPhase::Bubble);
        }
    }
}
