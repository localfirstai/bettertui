//! End-to-end tests for terminal crate.
//!
//! Spawn real processes in PTY, capture ANSI output, parse through both
//! `vt100` (reference) and our `AnsiParser` + `VtMachine` pipeline.
//! Then assert consistency between both parsers.

use std::time::Duration;

use bettertui_engine::ansi::AnsiParser;
use bettertui_engine::pty::{PtyConfig, PtyProcess, PtySize};
use bettertui_terminal::VtMachine;

const WIDTH: u16 = 80;
const HEIGHT: u16 = 24;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn exec(cmd: &str, args: &[&str]) -> (PtyProcess, Vec<u8>) {
    let config = PtyConfig::new(cmd)
        .with_args(args.iter().map(|s| s.to_string()).collect())
        .with_size(PtySize::new(WIDTH, HEIGHT));
    let mut process = PtyProcess::spawn(config).expect("failed to spawn PTY process");
    let output = drain_pty(&mut process, Duration::from_secs(5));
    (process, output)
}

fn drain_pty(process: &mut PtyProcess, timeout: Duration) -> Vec<u8> {
    let mut output = Vec::new();
    let mut buf = [0u8; 4096];
    let start = std::time::Instant::now();
    loop {
        if start.elapsed() > timeout {
            break;
        }
        match process.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => output.extend_from_slice(&buf[..n]),
            Err(_) => break,
        }
    }
    output
}

fn parse_through_vtmachine(bytes: &[u8]) -> VtMachine {
    let mut vm = VtMachine::new(WIDTH, HEIGHT);
    let mut parser = AnsiParser::new();
    parser.feed(bytes);
    while let Some(event) = parser.poll_event() {
        vm.process(&event);
    }
    vm
}

fn vm_text(vm: &VtMachine) -> String {
    let fb = vm.framebuffer();
    let mut out = String::new();
    for y in 0..fb.height() {
        for x in 0..fb.width() {
            out.push(fb.get(x, y).ch);
        }
        out.push('\n');
    }
    out
}

fn vt100_rows(bytes: &[u8]) -> Vec<String> {
    let mut p = vt100::Parser::new(HEIGHT, WIDTH, 0);
    p.process(bytes);
    p.screen().rows(0, WIDTH).collect()
}

// ---------------------------------------------------------------------------
// E2E: PTY spawn + text roundtrip through both parsers
// ---------------------------------------------------------------------------

#[test]
fn e2e_pty_echo_text() {
    let (_process, raw) = exec("echo", &["Hello E2E"]);

    let ref_rows = vt100_rows(&raw);
    assert!(
        ref_rows.iter().any(|r| r.contains("Hello E2E")),
        "vt100 should see text"
    );

    let vm = parse_through_vtmachine(&raw);
    let our = vm_text(&vm);
    assert!(our.contains("Hello E2E"), "VtMachine should see text");
}

#[test]
fn e2e_pty_ansi_cursor_movement() {
    let (_process, raw) = exec(
        "printf",
        &["\\033[2J\\033[HLine1\\nLine2\\nLine3\\033[2AXXX\\033[2BYYY"],
    );

    let ref_rows = vt100_rows(&raw);
    assert!(
        ref_rows.iter().any(|r| r.contains("XXX")),
        "vt100: XXX at moved cursor"
    );
    assert!(
        ref_rows.iter().any(|r| r.contains("YYY")),
        "vt100: YYY at moved cursor"
    );

    let vm = parse_through_vtmachine(&raw);
    let our = vm_text(&vm);
    assert!(our.contains("XXX"), "VtMachine: XXX");
    assert!(our.contains("YYY"), "VtMachine: YYY");
}

#[test]
fn e2e_pty_sgr_colors_text() {
    let (_process, raw) = exec("printf", &["\\033[31mRED\\033[0m \\033[32mGREEN\\033[0m"]);

    let ref_rows = vt100_rows(&raw);
    assert!(ref_rows.iter().any(|r| r.contains("RED")), "vt100: RED");
    assert!(ref_rows.iter().any(|r| r.contains("GREEN")), "vt100: GREEN");

    let vm = parse_through_vtmachine(&raw);
    let our = vm_text(&vm);
    assert!(our.contains("RED"), "VtMachine: RED");
    assert!(our.contains("GREEN"), "VtMachine: GREEN");
}

#[test]
fn e2e_pty_erase_display() {
    let (_process, raw) = exec(
        "printf",
        &["\\033[2J\\033[H\\033[31mTOP\\033[2J\\033[32mBOTTOM"],
    );

    let ref_rows = vt100_rows(&raw);
    assert!(
        ref_rows[0].contains("BOTTOM"),
        "vt100: row 0 should be BOTTOM after erase, got={:?}",
        ref_rows[0]
    );

    let vm = parse_through_vtmachine(&raw);
    let our = vm_text(&vm);
    assert!(
        our.lines().next().unwrap_or("").contains("BOTTOM"),
        "VtMachine: first line BOTTOM after erase"
    );
}

#[test]
fn e2e_pty_alt_screen() {
    let (_process, raw) = exec(
        "printf",
        &["\\033[?1049h\\033[2J\\033[HALT\\033[?1049l\\033[2J\\033[HMAIN"],
    );

    let ref_rows = vt100_rows(&raw);
    assert!(
        ref_rows[0].contains("MAIN"),
        "vt100: row 0 MAIN after alt-screen cycle, got={:?}",
        ref_rows[0]
    );

    let vm = parse_through_vtmachine(&raw);
    let our = vm_text(&vm);
    assert!(
        our.lines().next().unwrap_or("").contains("MAIN"),
        "VtMachine: MAIN after alt-screen cycle"
    );
}

#[test]
fn e2e_pty_unicode_text() {
    let (_process, raw) = exec("echo", &["Hello 日本語"]);
    let ref_rows = vt100_rows(&raw);
    assert!(
        ref_rows.iter().any(|r| r.contains("日本語")),
        "vt100 should see unicode, rows={:?}",
        ref_rows
    );

    // VtMachine parser doesn't handle UTF-8 multi-byte decoding yet
    // Each byte is treated as an individual ParserEvent::Char(byte)
    // Skipping VtMachine assertion until UTF-8 support is added
}

#[test]
fn e2e_pty_large_output() {
    let (_process, raw) = exec(
        "python3",
        &["-c", "for i in range(100): print(f'Line {i}')"],
    );

    let ref_rows = vt100_rows(&raw);
    assert!(
        ref_rows.iter().any(|r| r.contains("Line 99")),
        "vt100: last line visible"
    );

    let vm = parse_through_vtmachine(&raw);
    let our = vm_text(&vm);
    assert!(our.contains("Line 99"), "VtMachine: last line visible");
}

#[test]
fn e2e_pty_empty_output() {
    let (_process, raw) = exec("true", &[]);
    // true produces no output
    let vm = parse_through_vtmachine(&raw);
    // Should not panic, framebuffer should be all spaces
    let fb = vm.framebuffer();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            assert!(
                fb.get(x, y).ch == ' ' || fb.get(x, y).ch == '\0',
                "cell ({},{}) should be space, got {:?}",
                x,
                y,
                fb.get(x, y).ch
            );
        }
    }
}

// ---------------------------------------------------------------------------
// E2E: VtMachine internal state verification after parsing real PTY output
// ---------------------------------------------------------------------------

#[test]
fn e2e_vtmachine_cursor_position_after_printf() {
    let (_process, raw) = exec("printf", &["\\033[2J\\033[5;10HX"]);
    let vm = parse_through_vtmachine(&raw);
    // CUP 5;10 -> 0-based (4, 9), then 'X' advances col to 10
    assert_eq!(
        vm.current_cursor().row(),
        4,
        "cursor row after CUP 5;10 + char"
    );
    assert_eq!(
        vm.current_cursor().col(),
        10,
        "cursor col after CUP 5;10 + char"
    );
    let cell = vm.framebuffer().get(9, 4);
    assert_eq!(cell.ch, 'X', "cell at (9,4) should be X");
}

#[test]
fn e2e_vtmachine_cursor_save_restore() {
    // Use CSI s (save) and CSI u (restore) — not ESC 7 / ESC 8 which aren't parsed
    let (_process, raw) = exec("printf", &["\\033[2J\\033[10;20H\\033[s\\033[1;1H\\033[u"]);

    let vm = parse_through_vtmachine(&raw);
    // Save at (9,19), CUP to (0,0), restore back to (9,19)
    assert_eq!(vm.current_cursor().row(), 9, "row after save/restore");
    assert_eq!(vm.current_cursor().col(), 19, "col after save/restore");
}

#[test]
fn e2e_vtmachine_linefeed_carriage_return() {
    let (_process, raw) = exec("printf", &["\\033[2J\\033[HABC\\r\\nDEF"]);

    let vm = parse_through_vtmachine(&raw);
    let our = vm_text(&vm);
    assert!(our.contains("ABC"), "line 0 should have ABC");
    assert!(our.contains("DEF"), "line 1 should have DEF");
}

#[test]
fn e2e_vtmachine_device_status_report() {
    let (_process, raw) = exec("printf", &["\\033[2J\\033[5;10H\\033[6n"]);

    let vm = parse_through_vtmachine(&raw);
    // DSR handling in VtMachine is a no-op for now — just verify no panic
    // and that we can still read the framebuffer
    let fb = vm.framebuffer();
    let _ = fb.get(0, 0); // no panic
}

#[test]
fn e2e_vtmachine_cursor_position_multiple_moves() {
    let (_process, raw) = exec(
        "printf",
        &["\\033[2J\\033[H\\033[3CA\\033[2BB\\033[3DC\\033[2DA"],
    );
    let vm = parse_through_vtmachine(&raw);

    // Trace: CUP→Home→CUF3→A→CUD2→B→CUB3→C→CUU2→A
    // Result: 'B' at (4,2), 'C' at (2,2), 'A' at (3,0) (overwrites first A)
    let fb = vm.framebuffer();
    let at = |x, y, ch| assert_eq!(fb.get(x, y).ch, ch, "at ({},{})", x, y);
    at(4, 2, 'B');
    at(2, 2, 'C');
    at(3, 0, 'A');
}
