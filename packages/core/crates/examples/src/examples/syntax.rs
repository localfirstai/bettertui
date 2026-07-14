use std::io::{self, Write};

use bettertui_engine::syntax::SyntaxHighlighter;
use bettertui_terminal::Terminal;

pub fn run(terminal: &mut Terminal) -> io::Result<()> {
    let mut out = io::stdout();
    terminal.clear()?;
    terminal.move_cursor(0, 0)?;

    writeln!(out, "\x1b[1;97m━━━ Syntax: Tree-Sitter Highlighting ━━━\x1b[0m\n")?;

    let mut highlighter = SyntaxHighlighter::new();

    // Rust
    writeln!(out, "\x1b[33m[1]\x1b[0m Rust syntax highlighting...")?;
    let rust = r#"fn main() {
    let msg = "Hello, BetterTUI!";
    println!("{msg}");
    for i in 0..10 {
        if i % 2 == 0 {
            println!("even: {i}");
        }
    }
}
"#;
    if let Some(lines) = highlighter.highlight(rust, "rust") {
        for line in &lines {
            for seg in &line.segments {
                let bold = if seg.style.bold.unwrap_or(false) { "BOLD" } else { "    " };
                let fg = match &seg.style.fg {
                    Some(c) => format!("fg:{c:?}"),
                    None => "default".into(),
                };
                writeln!(out, "  [{bold} {fg:<30}] {}", seg.text)?;
            }
        }
        writeln!(out, "  ({} lines highlighted)", lines.len())?;
    } else {
        writeln!(out, "  (highlighting not available)")?;
    }

    // TypeScript
    writeln!(out, "\n\x1b[33m[2]\x1b[0m TypeScript syntax highlighting...")?;
    let ts = r#"interface User {
  name: string;
  age: number;
}
function greet(user: User): string {
  return `Hello, ${user.name}!`;
}
const alice: User = { name: "Alice", age: 30 };
console.log(greet(alice));
"#;
    if let Some(lines) = highlighter.highlight(ts, "typescript") {
        for line in &lines {
            for seg in &line.segments {
                let b = if seg.style.bold.unwrap_or(false) { "B" } else { " " };
                let i = if seg.style.italic.unwrap_or(false) { "I" } else { " " };
                let fg = match &seg.style.fg {
                    Some(bettertui_engine::tree::Color::Rgb { r, g, b }) => format!("#{r:02X}{g:02X}{b:02X}"),
                    _ => "default".into(),
                };
                writeln!(out, "  [{b}{i} fg:{fg:<9}] {}", seg.text)?;
            }
        }
        writeln!(out, "  ({} lines highlighted)", lines.len())?;
    } else {
        writeln!(out, "  (highlighting not available)")?;
    }

    // Python
    writeln!(out, "\n\x1b[33m[3]\x1b[0m Python syntax highlighting...")?;
    let py = r#"import sys

class Calculator:
    def add(self, a: int, b: int) -> int:
        return a + b

    def divide(self, a: int, b: int) -> float:
        if b == 0:
            raise ValueError("Cannot divide by zero")
        return a / b

calc = Calculator()
result = calc.add(40, 2)
print(f"The answer is {result}")
"#;
    if let Some(lines) = highlighter.highlight(py, "python") {
        for (i, line) in lines.iter().enumerate() {
            let text: String = line.segments.iter().map(|s| s.text.as_str()).collect();
            writeln!(out, "  [{:>2}] {}", i + 1, text)?;
        }
        writeln!(out, "  ({} lines highlighted)", lines.len())?;
    } else {
        writeln!(out, "  (highlighting not available)")?;
    }

    writeln!(out, "\n\x1b[2;90mPress any key to return to menu...\x1b[0m")?;
    out.flush()?;
    wait_for_any_key(terminal)
}

fn wait_for_any_key(terminal: &mut Terminal) -> io::Result<()> {
    loop {
        if let Some(bettertui_terminal::TerminalEvent::Key(_)) = terminal.poll_event(std::time::Duration::from_millis(100))? { return Ok(()) }
    }
}
