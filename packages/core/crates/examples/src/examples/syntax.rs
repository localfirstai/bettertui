//! Syntax highlighting example: Tree-sitter integration.
//!
//! Demonstrates:
//! - `SyntaxHighlighter` for code highlighting
//! - Language detection and styling
//! - Rendering highlighted code with BetterTUI

use std::io::{self, Write};

use bettertui_engine::engine::Engine;
use bettertui_engine::render::{AnsiBackend, Renderer};
use bettertui_engine::syntax::SyntaxHighlighter;
use bettertui_engine::tree::{Color, NamedColor, NodeKind, Style};
use bettertui_terminal::Terminal;

pub fn run(terminal: &mut Terminal) -> io::Result<()> {
    let mut out = io::stdout();
    terminal.clear()?;
    terminal.move_cursor(0, 0)?;

    let mut engine = Engine::new();
    let root = engine.arena().root();

    let title = engine.create_node(NodeKind::Text);
    engine.set_text(title, "Syntax: Tree-Sitter Highlighting");
    engine.set_style(title, Style::new().fg(Color::Named(NamedColor::BrightWhite)).bold(true));
    engine.append_child(root, title).unwrap();

    let spacer = engine.create_node(NodeKind::Text);
    engine.set_text(spacer, "");
    engine.append_child(root, spacer).unwrap();

    let mut highlighter = SyntaxHighlighter::new();

    let section1 = engine.create_node(NodeKind::Text);
    engine.set_text(section1, "[1] Rust syntax highlighting...");
    engine.set_style(section1, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section1).unwrap();

    let rust_code = r#"fn main() {
    let msg = "Hello, BetterTUI!";
    println!("{msg}");
}"#;

    if let Some(lines) = highlighter.highlight(rust_code, "rust") {
        for line in &lines {
            let mut text = String::new();
            for seg in &line.segments {
                text.push_str(&seg.text);
            }
            let n = engine.create_node(NodeKind::Text);
            engine.set_text(n, format!("  {}", text));
            if let Some(fg) = line.segments.first().and_then(|s| s.style.fg) {
                engine.set_style(n, Style::new().fg(fg));
            }
            engine.append_child(root, n).unwrap();
        }
        let count = engine.create_node(NodeKind::Text);
        engine.set_text(count, format!("  ({} lines highlighted)", lines.len()));
        engine.set_style(count, Style::new().fg(Color::Named(NamedColor::BrightBlack)));
        engine.append_child(root, count).unwrap();
    } else {
        let n = engine.create_node(NodeKind::Text);
        engine.set_text(n, "  (highlighting not available)");
        engine.append_child(root, n).unwrap();
    }

    let spacer2 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer2, "");
    engine.append_child(root, spacer2).unwrap();

    let section2 = engine.create_node(NodeKind::Text);
    engine.set_text(section2, "[2] TypeScript syntax highlighting...");
    engine.set_style(section2, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section2).unwrap();

    let ts_code = r#"interface User {
  name: string;
  age: number;
}
function greet(user: User): string {
  return `Hello, ${user.name}!`;
}"#;

    if let Some(lines) = highlighter.highlight(ts_code, "typescript") {
        for line in &lines {
            let mut text = String::new();
            for seg in &line.segments {
                text.push_str(&seg.text);
            }
            let n = engine.create_node(NodeKind::Text);
            engine.set_text(n, format!("  {}", text));
            engine.append_child(root, n).unwrap();
        }
        let count = engine.create_node(NodeKind::Text);
        engine.set_text(count, format!("  ({} lines highlighted)", lines.len()));
        engine.set_style(count, Style::new().fg(Color::Named(NamedColor::BrightBlack)));
        engine.append_child(root, count).unwrap();
    } else {
        let n = engine.create_node(NodeKind::Text);
        engine.set_text(n, "  (highlighting not available)");
        engine.append_child(root, n).unwrap();
    }

    let spacer3 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer3, "");
    engine.append_child(root, spacer3).unwrap();

    let section3 = engine.create_node(NodeKind::Text);
    engine.set_text(section3, "[3] Python syntax highlighting...");
    engine.set_style(section3, Style::new().fg(Color::Named(NamedColor::Yellow)));
    engine.append_child(root, section3).unwrap();

    let py_code = r#"import sys

class Calculator:
    def add(self, a: int, b: int) -> int:
        return a + b

calc = Calculator()
result = calc.add(40, 2)
print(f"The answer is {result}")"#;

    if let Some(lines) = highlighter.highlight(py_code, "python") {
        for (i, line) in lines.iter().enumerate() {
            let text: String = line.segments.iter().map(|s| s.text.as_str()).collect();
            let n = engine.create_node(NodeKind::Text);
            engine.set_text(n, format!("  [{:>2}] {}", i + 1, text));
            engine.append_child(root, n).unwrap();
        }
        let count = engine.create_node(NodeKind::Text);
        engine.set_text(count, format!("  ({} lines highlighted)", lines.len()));
        engine.set_style(count, Style::new().fg(Color::Named(NamedColor::BrightBlack)));
        engine.append_child(root, count).unwrap();
    } else {
        let n = engine.create_node(NodeKind::Text);
        engine.set_text(n, "  (highlighting not available)");
        engine.append_child(root, n).unwrap();
    }

    let spacer4 = engine.create_node(NodeKind::Text);
    engine.set_text(spacer4, "");
    engine.append_child(root, spacer4).unwrap();

    let hint = engine.create_node(NodeKind::Text);
    engine.set_text(hint, "Press any key to return to menu...");
    engine.set_style(hint, Style { fg: Some(Color::Named(NamedColor::BrightBlack)), dim: Some(true), ..Style::new() });
    engine.append_child(root, hint).unwrap();

    engine.begin_frame();
    engine.commit_frame();

    let mut renderer = Renderer::new(80, 24);
    renderer.set_backend(Box::new(AnsiBackend::new()));
    let frame = renderer.render_full(engine.arena_mut());
    out.write_all(&frame.output_data)?;
    out.flush()?;

    wait_for_any_key(terminal)
}

fn wait_for_any_key(terminal: &mut Terminal) -> io::Result<()> {
    loop {
        if let Some(bettertui_terminal::TerminalEvent::Key(_)) =
            terminal.poll_event(std::time::Duration::from_millis(100))?
        {
            return Ok(());
        }
    }
}
