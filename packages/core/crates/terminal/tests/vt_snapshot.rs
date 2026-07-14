//! Insta snapshot tests for VtMachine framebuffer output.
//!
//! Processes known ANSI sequences through the VT state machine and
//! captures the resulting FrameBuffer as insta snapshots.

use bettertui_engine::ansi::AnsiParser;
use bettertui_terminal::VtMachine;

#[test]
fn snapshot_vt_machine_empty() {
    let m = VtMachine::new(40, 10);
    insta::assert_debug_snapshot!(m.framebuffer());
}

#[test]
fn snapshot_vt_machine_hello_world() {
    let mut m = VtMachine::new(40, 5);
    let mut p = AnsiParser::new();
    p.feed(b"Hello, World!");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    insta::assert_debug_snapshot!(m.framebuffer());
}

#[test]
fn snapshot_vt_machine_multiple_lines() {
    let mut m = VtMachine::new(40, 5);
    let mut p = AnsiParser::new();
    p.feed(b"Line 1\nLine 2\nLine 3");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    insta::assert_debug_snapshot!(m.framebuffer());
}

#[test]
fn snapshot_vt_machine_colors_sgr() {
    let mut m = VtMachine::new(40, 5);
    let mut p = AnsiParser::new();
    p.feed(b"\x1b[31mRed\x1b[32mGreen\x1b[0mDefault");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    insta::assert_debug_snapshot!(m.framebuffer());
}

#[test]
fn snapshot_vt_machine_cursor_position() {
    let mut m = VtMachine::new(40, 10);
    let mut p = AnsiParser::new();
    p.feed(b"\x1b[5;10HX");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    insta::assert_debug_snapshot!(m.framebuffer());
}

#[test]
fn snapshot_vt_machine_erase_display() {
    let mut m = VtMachine::new(40, 5);
    let mut p = AnsiParser::new();
    p.feed(b"ABCDEFGHIJ\x1b[2J");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    insta::assert_debug_snapshot!(m.framebuffer());
}

#[test]
fn snapshot_vt_machine_scroll() {
    let mut m = VtMachine::new(40, 3);
    let mut p = AnsiParser::new();
    p.feed(b"A\nB\nC\nD\nE");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    insta::assert_debug_snapshot!(m.framebuffer());
}

#[test]
fn snapshot_vt_machine_carriage_return() {
    let mut m = VtMachine::new(40, 5);
    let mut p = AnsiParser::new();
    p.feed(b"Hello\rX");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    insta::assert_debug_snapshot!(m.framebuffer());
}

#[test]
fn snapshot_vt_machine_tab() {
    let mut m = VtMachine::new(40, 5);
    let mut p = AnsiParser::new();
    p.feed(b"\tX");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    insta::assert_debug_snapshot!(m.framebuffer());
}

#[test]
fn snapshot_vt_machine_true_color() {
    let mut m = VtMachine::new(40, 5);
    let mut p = AnsiParser::new();
    p.feed(b"\x1b[38;2;255;100;50mOrange\x1b[0m");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    insta::assert_debug_snapshot!(m.framebuffer());
}

#[test]
fn snapshot_vt_machine_bold_italic() {
    let mut m = VtMachine::new(40, 5);
    let mut p = AnsiParser::new();
    p.feed(b"\x1b[1mBold\x1b[3mItalic\x1b[0mNormal");
    while let Some(event) = p.poll_event() {
        m.process(&event);
    }
    insta::assert_debug_snapshot!(m.framebuffer());
}
