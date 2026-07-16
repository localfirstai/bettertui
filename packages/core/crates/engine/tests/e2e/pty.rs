use std::time::Duration;

use bettertui_engine::pty::{PtyConfig, PtyProcess, PtySize};

/// Helper: read all output from a PTY process with a timeout.
fn read_pty_output(process: &mut PtyProcess, timeout: Duration) -> Vec<u8> {
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

#[test]
fn e2e_pty_spawn_echo_raw() {
    let mut process = PtyProcess::spawn(
        PtyConfig::new("echo").with_args(vec!["Hello PTY".to_string()]).with_size(PtySize::new(80, 24)),
    )
    .expect("failed to spawn echo via PTY");

    let output = read_pty_output(&mut process, Duration::from_secs(5));
    process.kill().ok();

    assert!(!output.is_empty(), "PTY output should not be empty");

    let output_str = String::from_utf8_lossy(&output);
    assert!(output_str.contains("Hello PTY"), "raw output should contain echo text");
}

#[test]
fn e2e_pty_echo_parsed_with_vt100() {
    let mut process = PtyProcess::spawn(
        PtyConfig::new("echo").with_args(vec!["Hello PTY".to_string()]).with_size(PtySize::new(80, 24)),
    )
    .expect("failed to spawn echo via PTY");

    let output = read_pty_output(&mut process, Duration::from_secs(5));
    process.kill().ok();

    // Parse raw PTY output through vt100 terminal emulator
    let mut parser = vt100::Parser::new(24, 80, 0);
    parser.process(&output);

    let screen = parser.screen();
    let contents = screen.contents();
    assert!(
        contents.contains("Hello PTY"),
        "vt100-parsed screen should contain echo text\n--- raw output ---\n{:?}\n--- vt100 contents ---\n{}",
        String::from_utf8_lossy(&output),
        contents,
    );
}

#[test]
fn e2e_pty_ansi_color_parsed_with_vt100() {
    // Use printf to emit ANSI-colored text
    let mut process = PtyProcess::spawn(
        PtyConfig::new("printf")
            .with_args(vec!["\\033[31mRed\\033[0m \\033[32mGreen\\033[0m".to_string()])
            .with_size(PtySize::new(80, 24)),
    )
    .expect("failed to spawn printf via PTY");

    let output = read_pty_output(&mut process, Duration::from_secs(5));
    process.kill().ok();

    // Parse through vt100
    let mut parser = vt100::Parser::new(24, 80, 0);
    parser.process(&output);

    let screen = parser.screen();
    let contents = screen.contents();

    // The parsed screen should contain the visible text (without escape codes)
    assert!(
        contents.contains("Red"),
        "vt100 should parse ANSI red text\n--- raw ---\n{:?}\n--- parsed ---\n{}",
        String::from_utf8_lossy(&output),
        contents,
    );
    assert!(contents.contains("Green"), "vt100 should parse ANSI green text");

    // The word "Red" at column 0 should have red foreground
    if let Some(cell) = screen.cell(0, 0) {
        assert_eq!(cell.fgcolor(), vt100::Color::Idx(1), "first cell should have red foreground");
    }

    // The word "Green" should have green foreground (after the space)
    // Find where "Green" starts in the parsed screen
    let green_col = contents.find("Green").unwrap_or(0) as u16;
    if let Some(cell) = screen.cell(0, green_col) {
        assert_eq!(cell.fgcolor(), vt100::Color::Idx(2), "Green text should have green foreground");
    }
}

#[test]
fn e2e_pty_resize() {
    let mut process =
        PtyProcess::spawn(PtyConfig::new("echo").with_args(vec!["test".to_string()]).with_size(PtySize::new(40, 10)))
            .expect("failed to spawn process");

    let initial = process.size();
    assert_eq!(initial.cols, 40);
    assert_eq!(initial.rows, 10);

    process.resize(PtySize::new(80, 24)).expect("resize should succeed");
    let resized = process.size();
    assert_eq!(resized.cols, 80);
    assert_eq!(resized.rows, 24);

    process.kill().ok();
}

#[test]
fn e2e_pty_process_lifecycle() {
    let mut process =
        PtyProcess::spawn(PtyConfig::new("true").with_size(PtySize::new(80, 24))).expect("failed to spawn true");

    assert!(process.is_running(), "process should be running initially");
    let status = process.wait().expect("wait should succeed");
    assert!(status.is_some());
    assert!(!process.is_running(), "process should not be running after exit");
}
