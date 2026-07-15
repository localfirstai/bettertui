use std::path::PathBuf;
use std::time::Duration;

use bettertui_engine::pty::{PtyConfig, PtyProcess, PtySize};

fn binary_path() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_BIN_EXE_layout_e2e"));
    assert!(path.exists(), "layout_e2e binary not found at {:?}", path);
    path
}

fn run_scenario(scenario: &str) -> Vec<u8> {
    let binary = binary_path();
    let config =
        PtyConfig::new(binary.to_str().unwrap()).with_args(vec![scenario.to_string()]).with_size(PtySize::new(80, 24));
    let mut process = PtyProcess::spawn(config).expect("failed to spawn layout_e2e PTY process");

    let mut output = Vec::new();
    let mut buf = [0u8; 4096];
    let start = std::time::Instant::now();
    loop {
        if start.elapsed() > Duration::from_secs(5) {
            break;
        }
        match process.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => output.extend_from_slice(&buf[..n]),
            Err(_) => break,
        }
    }
    process.kill().ok();
    output
}

fn parse_scenario(scenario: &str) -> vt100::Screen {
    let output = run_scenario(scenario);
    let mut parser = vt100::Parser::new(24, 80, 0);
    parser.process(&output);
    parser.screen().clone()
}

#[test]
fn e2e_layout_basic_text() {
    let screen = parse_scenario("basic");
    let contents = screen.contents();
    assert!(contents.contains("Hello Layout E2E"), "basic: should contain text, got:\n{contents}");
    assert!(contents.starts_with("Hello Layout E2E"), "basic: text should start at row 0, got:\n{contents}");
}

#[test]
fn e2e_layout_flex_row() {
    let screen = parse_scenario("flex-row");
    let contents = screen.contents();
    assert!(contents.contains("Left"), "flex-row: should contain Left, got:\n{contents}");
    assert!(contents.contains("Right"), "flex-row: should contain Right, got:\n{contents}");
    // Left and Right should be on same line (row layout)
    let all: Vec<String> = screen.rows(0, 80).collect();
    assert!(
        all[0].contains("Left") && all[0].contains("Right"),
        "flex-row: Left and Right should be on row 0, got:\n{}",
        all[0]
    );
}

#[test]
fn e2e_layout_flex_column() {
    let screen = parse_scenario("flex-column");
    // rows(start_col, width) iterates ALL rows with column subset
    let all: Vec<String> = screen.rows(0, 80).collect();
    assert!(all.len() > 1, "flex-column: expected at least 2 rows, got {}", all.len());
    assert!(all[0].contains("Top"), "flex-column: row 0 should contain Top, got:\n{}", all[0]);
    assert!(all[1].contains("Bottom"), "flex-column: row 1 should contain Bottom, got:\n{}", all[1]);
}

#[test]
fn e2e_layout_styled_text() {
    let screen = parse_scenario("styled");
    let contents = screen.contents();
    assert!(contents.contains("Red"), "styled: should contain Red, got:\n{contents}");
    assert!(contents.contains("Green"), "styled: should contain Green, got:\n{contents}");
    assert!(contents.contains("Blue"), "styled: should contain Blue, got:\n{contents}");
    // Each word's first cell should have the correct color
    let red_col = contents.find("Red").unwrap_or(0) as u16;
    let green_col = contents.find("Green").unwrap_or(0) as u16;
    let blue_col = contents.find("Blue").unwrap_or(0) as u16;

    if let Some(cell) = screen.cell(0, red_col) {
        assert_eq!(
            cell.fgcolor(),
            vt100::Color::Idx(9),
            "Red should have bright red (idx 9) foreground at col {red_col}"
        );
    }
    if let Some(cell) = screen.cell(0, green_col) {
        assert_eq!(
            cell.fgcolor(),
            vt100::Color::Idx(10),
            "Green should have bright green (idx 10) foreground at col {green_col}"
        );
    }
    if let Some(cell) = screen.cell(0, blue_col) {
        assert_eq!(
            cell.fgcolor(),
            vt100::Color::Idx(12),
            "Blue should have bright blue (idx 12) foreground at col {blue_col}"
        );
    }
}

#[test]
fn e2e_layout_nested_padding() {
    let screen = parse_scenario("nested");
    // rows() first arg is start COLUMN, not row. Collect all rows instead.
    let all: Vec<String> = screen.rows(0, 80).collect();
    assert!(all.len() > 2, "nested: expected at least 3 rows, got {}", all.len());
    // Padding: top=2, left=5 means text starts at row 2 (0-based), col 5
    assert!(
        all[2].contains("Indented"),
        "nested: row 2 should contain Indented (after top=2 padding), got:\n{}",
        all[2]
    );
    if let Some(cell) = screen.cell(2, 5) {
        assert_eq!(cell.contents(), "I", "nested: cell at (2,5) should be 'I', got {:?}", cell.contents());
    }
}

#[test]
fn e2e_layout_empty_frame() {
    let screen = parse_scenario("empty");
    let contents = screen.contents();
    // Empty tree should produce blank output (no text)
    let trimmed = contents.trim();
    assert!(trimmed.is_empty(), "empty: should be blank, got:\n{contents}");
}
