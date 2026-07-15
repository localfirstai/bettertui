//! Integration tests for VT100/VTxxx terminal emulation state machine.
//!
//! These tests exercise the **public API** of the vt module types.

use bettertui_engine::ansi::AnsiParser;
use bettertui_engine::framebuffer::CellAttributes;
use bettertui_engine::input::{KeyAction, KeyModifiers};
use bettertui_engine::tree::{Color, NamedColor};
use bettertui_terminal::{Cursor, KittyKeyEvent, Pen, PrivateMode, ScreenBuffer, TerminalMode, VtMachine};

// =============================================================================
// Cursor Tests
// =============================================================================

#[test]
fn cursor_new() {
    let c = Cursor::new();
    assert_eq!(c.position(), (0, 0));
    assert!(c.visible());
}

#[test]
fn cursor_move_up() {
    let mut c = Cursor::new();
    c.set_position(5, 5);
    c.move_up(2);
    assert_eq!(c.row(), 3);
}

#[test]
fn cursor_move_up_saturating() {
    let mut c = Cursor::new();
    c.move_up(5);
    assert_eq!(c.row(), 0);
}

#[test]
fn cursor_move_down() {
    let mut c = Cursor::new();
    c.move_down(3, 24);
    assert_eq!(c.row(), 3);
}

#[test]
fn cursor_move_down_clamp() {
    let mut c = Cursor::new();
    c.set_position(20, 0);
    c.move_down(10, 24);
    assert_eq!(c.row(), 23);
}

#[test]
fn cursor_move_left_right() {
    let mut c = Cursor::new();
    c.set_position(0, 10);
    c.move_left(3);
    assert_eq!(c.col(), 7);
    c.move_right(5, 80);
    assert_eq!(c.col(), 12);
}

#[test]
fn cursor_move_to_column() {
    let mut c = Cursor::new();
    c.move_to_column(5);
    assert_eq!(c.col(), 4);
}

#[test]
fn cursor_move_to() {
    let mut c = Cursor::new();
    c.move_to(10, 20);
    assert_eq!(c.row(), 9);
    assert_eq!(c.col(), 19);
}

#[test]
fn cursor_save_restore() {
    let mut c = Cursor::new();
    c.set_position(5, 10);
    c.save_position();
    c.set_position(0, 0);
    c.restore_position();
    assert_eq!(c.position(), (5, 10));
}

#[test]
fn cursor_carriage_return() {
    let mut c = Cursor::new();
    c.set_position(5, 20);
    c.carriage_return();
    assert_eq!(c.col(), 0);
    assert_eq!(c.row(), 5);
}

#[test]
fn cursor_tab() {
    let mut c = Cursor::new();
    c.set_position(0, 3);
    c.tab(&[8, 16, 24]);
    assert_eq!(c.col(), 8);
}

#[test]
fn cursor_tab_next_stop() {
    let mut c = Cursor::new();
    c.set_position(0, 12);
    c.tab(&[8, 16, 24]);
    assert_eq!(c.col(), 16);
}

#[test]
fn cursor_tab_default() {
    let mut c = Cursor::new();
    c.set_position(0, 5);
    c.tab(&[]);
    assert_eq!(c.col(), 8);
}

#[test]
fn cursor_backspace() {
    let mut c = Cursor::new();
    c.set_position(0, 5);
    c.backspace();
    assert_eq!(c.col(), 4);
}

#[test]
fn cursor_backspace_saturating() {
    let mut c = Cursor::new();
    c.backspace();
    assert_eq!(c.col(), 0);
}

// =============================================================================
// TerminalMode Tests
// =============================================================================

#[test]
fn default_modes() {
    let m = TerminalMode::default();
    assert!(m.auto_wrap());
    assert!(m.cursor_visible());
    assert!(m.cursor_blinking());
    assert!(!m.alt_screen());
    assert!(!m.bracketed_paste());
}

#[test]
fn mode_set_reset() {
    let mut m = TerminalMode::default();
    m.insert(TerminalMode::BRACKETED_PASTE);
    assert!(m.bracketed_paste());
    m.remove(TerminalMode::BRACKETED_PASTE);
    assert!(!m.bracketed_paste());
}

#[test]
fn mode_toggle() {
    let mut m = TerminalMode::default();
    m.insert(TerminalMode::INSERT);
    assert!(m.is_insert());
    m.remove(TerminalMode::INSERT);
    assert!(!m.is_insert());
}

#[test]
fn private_mode_from_code() {
    assert_eq!(PrivateMode::from_code(25), Some(PrivateMode::CursorVisible));
    assert_eq!(PrivateMode::from_code(2004), Some(PrivateMode::BracketedPaste));
    assert_eq!(PrivateMode::from_code(9999), None);
}

#[test]
fn private_mode_to_terminal() {
    let tm = PrivateMode::CursorVisible.to_terminal_mode();
    assert_eq!(tm, TerminalMode::VISIBLE_CURSOR);
}

// =============================================================================
// ScreenBuffer Tests
// =============================================================================

fn make_pen() -> Pen {
    Pen::default()
}

#[test]
fn screen_buffer_new() {
    let sb = ScreenBuffer::new(80, 24);
    assert_eq!(sb.width(), 80);
    assert_eq!(sb.height(), 24);
}

#[test]
fn screen_write_char() {
    let mut sb = ScreenBuffer::new(10, 5);
    let pen = make_pen();
    sb.write_char(2, 3, 'X', &pen);
    assert_eq!(sb.buffer().get(3, 2).ch, 'X');
}

#[test]
fn screen_scroll_up() {
    let mut sb = ScreenBuffer::new(10, 5);
    let pen = make_pen();
    sb.write_char(0, 0, 'A', &pen);
    sb.write_char(1, 0, 'B', &pen);
    sb.scroll_up(1, &pen);
    assert_eq!(sb.buffer().get(0, 0).ch, 'B');
    assert_eq!(sb.buffer().get(0, 4).ch, ' ');
}

#[test]
fn screen_scroll_down() {
    let mut sb = ScreenBuffer::new(10, 5);
    let pen = make_pen();
    sb.write_char(3, 0, 'X', &pen);
    sb.scroll_down(2, &pen);
    assert_eq!(sb.buffer().get(0, 0).ch, ' ');
    assert_eq!(sb.buffer().get(0, 0).ch, ' ');
}

#[test]
fn screen_erase_in_display_cursor_to_end() {
    let mut sb = ScreenBuffer::new(10, 5);
    let pen = make_pen();
    sb.write_char(2, 0, 'X', &pen);
    sb.write_char(3, 0, 'Y', &pen);
    sb.erase_in_display(0, 2, 0, &pen);
    assert_eq!(sb.buffer().get(0, 2).ch, ' ');
    assert_eq!(sb.buffer().get(0, 3).ch, ' ');
}

#[test]
fn screen_erase_in_display_beginning_to_cursor() {
    let mut sb = ScreenBuffer::new(10, 5);
    let pen = make_pen();
    sb.write_char(0, 0, 'A', &pen);
    sb.write_char(1, 0, 'B', &pen);
    sb.erase_in_display(1, 1, 5, &pen);
    assert_eq!(sb.buffer().get(0, 0).ch, ' ');
    assert_eq!(sb.buffer().get(5, 1).ch, ' ');
}

#[test]
fn screen_erase_in_display_entire() {
    let mut sb = ScreenBuffer::new(10, 5);
    let pen = make_pen();
    sb.write_char(0, 0, 'A', &pen);
    sb.write_char(4, 9, 'Z', &pen);
    sb.erase_in_display(2, 0, 0, &pen);
    assert!(sb.buffer().get(0, 0).is_empty());
    assert!(sb.buffer().get(9, 4).is_empty());
}

#[test]
fn screen_insert_lines() {
    let mut sb = ScreenBuffer::new(10, 5);
    let pen = make_pen();
    sb.write_char(1, 0, 'A', &pen);
    sb.write_char(2, 0, 'B', &pen);
    sb.insert_lines(1, 2, &pen);
    assert_eq!(sb.buffer().get(0, 1).ch, ' ');
    assert_eq!(sb.buffer().get(0, 3).ch, 'A');
    assert_eq!(sb.buffer().get(0, 4).ch, 'B');
}

#[test]
fn screen_delete_lines() {
    let mut sb = ScreenBuffer::new(10, 5);
    let pen = make_pen();
    sb.write_char(0, 0, 'A', &pen);
    sb.write_char(1, 0, 'B', &pen);
    sb.write_char(2, 0, 'C', &pen);
    sb.delete_lines(0, 2, &pen);
    assert_eq!(sb.buffer().get(0, 0).ch, 'C');
}

#[test]
fn screen_insert_chars() {
    let mut sb = ScreenBuffer::new(10, 5);
    let pen = make_pen();
    sb.write_char(0, 5, 'A', &pen);
    sb.insert_chars(0, 2, 3, &pen);
    assert_eq!(sb.buffer().get(2, 0).ch, ' ');
    assert_eq!(sb.buffer().get(8, 0).ch, 'A');
}

#[test]
fn screen_delete_chars() {
    let mut sb = ScreenBuffer::new(10, 5);
    let pen = make_pen();
    sb.write_char(0, 2, 'D', &pen);
    sb.write_char(0, 5, 'E', &pen);
    sb.delete_chars(0, 2, 3, &pen);
    assert_eq!(sb.buffer().get(2, 0).ch, 'E');
}

#[test]
fn screen_tab_stops() {
    let mut sb = ScreenBuffer::new(80, 24);
    assert!(sb.tab_stops().contains(&8));
    assert!(sb.tab_stops().contains(&16));
    sb.set_tab_stop(5);
    assert!(sb.tab_stops().contains(&5));
    sb.clear_tab_stop(8);
    assert!(!sb.tab_stops().contains(&8));
}

#[test]
fn screen_scrollback() {
    let mut sb = ScreenBuffer::new(10, 3);
    let pen = make_pen();
    sb.write_char(0, 0, 'A', &pen);
    sb.write_char(0, 1, 'B', &pen);
    sb.write_char(0, 2, 'C', &pen);
    sb.scroll_up(2, &pen);
    assert!(sb.scrollback_len() == 2);
}

// =============================================================================
// VtMachine Tests
// =============================================================================

#[test]
fn machine_new() {
    let m = VtMachine::new(80, 24);
    // Use public accessors instead of direct field access
    assert_eq!(m.current_screen().width(), 80);
    assert_eq!(m.current_screen().height(), 24);
    assert!(m.current_modes().auto_wrap());
}

#[test]
fn machine_write_text() {
    let mut m = VtMachine::new(10, 5);
    let mut p = AnsiParser::new();
    p.feed(b"Hello");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    assert_eq!(m.framebuffer().get(0, 0).ch, 'H');
    assert_eq!(m.framebuffer().get(4, 0).ch, 'o');
}

#[test]
fn machine_newline() {
    let mut m = VtMachine::new(10, 5);
    let mut p = AnsiParser::new();
    p.feed(b"AB\nCD");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    assert_eq!(m.framebuffer().get(0, 0).ch, 'A');
    assert_eq!(m.framebuffer().get(1, 0).ch, 'B');
    assert_eq!(m.framebuffer().get(0, 1).ch, 'C');
    assert_eq!(m.framebuffer().get(1, 1).ch, 'D');
}

#[test]
fn machine_cursor_movement() {
    let mut m = VtMachine::new(10, 5);
    let mut p = AnsiParser::new();
    p.feed(b"\x1b[3;4HX");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    assert_eq!(m.framebuffer().get(3, 2).ch, 'X');
}

#[test]
fn machine_sgr_colors() {
    let mut m = VtMachine::new(10, 5);
    let mut p = AnsiParser::new();
    p.feed(b"\x1b[31mR\x1b[0m");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    assert_eq!(m.framebuffer().get(0, 0).ch, 'R');
    assert_eq!(m.framebuffer().get(0, 0).fg, Color::Named(NamedColor::Red));
}

#[test]
fn machine_sgr_bold() {
    let mut m = VtMachine::new(10, 5);
    let mut p = AnsiParser::new();
    p.feed(b"\x1b[1mB");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    let cell = m.framebuffer().get(0, 0);
    assert_eq!(cell.ch, 'B');
    assert!(cell.attributes.contains(CellAttributes::BOLD));
}

#[test]
fn machine_erase_display() {
    let mut m = VtMachine::new(10, 5);
    let mut p = AnsiParser::new();
    p.feed(b"AB\x1b[2J");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    assert!(m.framebuffer().get(0, 0).is_empty());
    assert!(m.framebuffer().get(1, 0).is_empty());
}

#[test]
fn machine_scroll() {
    let mut m = VtMachine::new(10, 3);
    let mut p = AnsiParser::new();
    p.feed(b"Line1\nLine2\nLine3\nLine4");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    assert_eq!(m.framebuffer().get(0, 0).ch, 'L');
    assert_eq!(m.framebuffer().get(4, 0).ch, '2');
    assert_eq!(m.framebuffer().get(0, 1).ch, 'L');
    assert_eq!(m.framebuffer().get(4, 1).ch, '3');
    assert_eq!(m.framebuffer().get(0, 2).ch, 'L');
    assert_eq!(m.framebuffer().get(4, 2).ch, '4');
}

#[test]
fn machine_reset() {
    let mut m = VtMachine::new(80, 24);
    let mut p = AnsiParser::new();
    p.feed(b"\x1b[31mX");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    p.feed(b"\x1bc");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    let cell = m.framebuffer().get(0, 0);
    assert_eq!(cell.fg, Color::Default);
}

#[test]
fn machine_osc_title() {
    let mut m = VtMachine::new(80, 24);
    let mut p = AnsiParser::new();
    p.feed(b"\x1b]2;My Terminal\x07");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    assert_eq!(m.title(), "My Terminal");
}

#[test]
fn machine_carriage_return() {
    let mut m = VtMachine::new(10, 5);
    let mut p = AnsiParser::new();
    p.feed(b"Hello\rX");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    assert_eq!(m.framebuffer().get(0, 0).ch, 'X');
    assert_eq!(m.framebuffer().get(1, 0).ch, 'e');
}

#[test]
fn machine_tab() {
    let mut m = VtMachine::new(20, 5);
    let mut p = AnsiParser::new();
    p.feed(b"\tX");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    assert_eq!(m.framebuffer().get(8, 0).ch, 'X');
}

#[test]
fn machine_alternate_screen() {
    let mut m = VtMachine::new(10, 5);
    let mut p = AnsiParser::new();
    p.feed(b"Main");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    assert_eq!(m.framebuffer().get(0, 0).ch, 'M');

    let mut p2 = AnsiParser::new();
    p2.feed(b"\x1b[?1049hAlt");
    while let Some(event) = p2.poll_event() {
        m.process(&event);
    }
    assert!(m.current_modes().alt_screen());
    assert_eq!(m.framebuffer().get(0, 0).ch, 'A');

    let mut p3 = AnsiParser::new();
    p3.feed(b"\x1b[?1049l");
    while let Some(event) = p3.poll_event() {
        m.process(&event);
    }
    assert!(!m.current_modes().alt_screen());
    assert_eq!(m.framebuffer().get(0, 0).ch, 'M');
}

#[test]
fn kitty_key_event_to_keyboard_input_press() {
    let ev = KittyKeyEvent {
        keycode: 97,
        modifiers: 0,
        event_type: bettertui_engine::ansi::KittyEventType::Press,
        associated_text: None,
    };
    let ki = ev.to_keyboard_input();
    assert_eq!(ki.key, 'a');
    assert_eq!(ki.modifiers, KeyModifiers::empty());
    assert_eq!(ki.action, KeyAction::Press);
}

#[test]
fn kitty_key_event_to_keyboard_input_modifiers() {
    let ev = KittyKeyEvent {
        keycode: 65,
        modifiers: 1 | 4, // Shift=1, Ctrl=4
        event_type: bettertui_engine::ansi::KittyEventType::Repeat,
        associated_text: None,
    };
    let ki = ev.to_keyboard_input();
    assert_eq!(ki.key, 'A');
    assert!(ki.modifiers.contains(KeyModifiers::SHIFT));
    assert!(ki.modifiers.contains(KeyModifiers::CONTROL));
    assert!(!ki.modifiers.contains(KeyModifiers::ALT));
    assert!(!ki.modifiers.contains(KeyModifiers::SUPER));
    assert_eq!(ki.action, KeyAction::Repeat);
}

#[test]
fn kitty_key_event_to_keyboard_input_alt_super() {
    let ev = KittyKeyEvent {
        keycode: 98,
        modifiers: 2 | 8, // Alt=2, Super=8
        event_type: bettertui_engine::ansi::KittyEventType::Release,
        associated_text: None,
    };
    let ki = ev.to_keyboard_input();
    assert_eq!(ki.key, 'b');
    assert!(!ki.modifiers.contains(KeyModifiers::SHIFT));
    assert!(!ki.modifiers.contains(KeyModifiers::CONTROL));
    assert!(ki.modifiers.contains(KeyModifiers::ALT));
    assert!(ki.modifiers.contains(KeyModifiers::SUPER));
    assert_eq!(ki.action, KeyAction::Release);
}

#[test]
fn kitty_key_event_to_keyboard_input_unknown_type() {
    let ev = KittyKeyEvent {
        keycode: 99,
        modifiers: 0,
        event_type: bettertui_engine::ansi::KittyEventType::Unknown,
        associated_text: None,
    };
    let ki = ev.to_keyboard_input();
    assert_eq!(ki.key, 'c');
    assert_eq!(ki.action, KeyAction::Press);
}

#[test]
fn kitty_key_event_to_keyboard_input_invalid_keycode() {
    let ev = KittyKeyEvent {
        keycode: 0x110000, // beyond valid Unicode
        modifiers: 0,
        event_type: bettertui_engine::ansi::KittyEventType::Press,
        associated_text: None,
    };
    let ki = ev.to_keyboard_input();
    assert_eq!(ki.key, '\0');
}

#[test]
fn machine_true_color_sgr() {
    let mut m = VtMachine::new(10, 5);
    let mut p = AnsiParser::new();
    p.feed(b"\x1b[38;2;255;128;64mC");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    assert_eq!(m.framebuffer().get(0, 0).fg, Color::Rgb { r: 255, g: 128, b: 64 });
}
