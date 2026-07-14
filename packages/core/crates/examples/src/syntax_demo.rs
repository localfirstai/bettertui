use bettertui_engine::syntax::SyntaxHighlighter;

use crate::util;

pub fn run() {
    util::heading("Syntax Highlighting Demo: Tree-Sitter Integration");

    let mut highlighter = SyntaxHighlighter::new();

    // ── Rust ──
    println!("[1] Rust syntax highlighting...");
    let rust_code = r#"fn main() {
    let msg = "Hello, BetterTUI!";
    println!("{msg}");

    for i in 0..10 {
        if i % 2 == 0 {
            println!("even: {i}");
        }
    }
}
"#;
    if let Some(lines) = highlighter.highlight(rust_code, "rust") {
        for line in &lines {
            for segment in &line.segments {
                // Display style info alongside text
                let style_desc = if segment.style.bold.unwrap_or(false) {
                    "BOLD"
                } else {
                    "    "
                };
                let fg_desc = match segment.style.fg {
                    Some(c) => format!("fg:{c:?}"),
                    None => "default".into(),
                };
                println!("  [{style_desc} {fg_desc:<30}] {}", segment.text);
            }
        }
        println!("  ({} lines highlighted)", lines.len());
    } else {
        println!("  (highlighting not available)");
    }

    // ── TypeScript ──
    println!("\n[2] TypeScript syntax highlighting...");
    let ts_code = r#"interface User {
  name: string;
  age: number;
}

function greet(user: User): string {
  return `Hello, ${user.name}!`;
}

const alice: User = { name: "Alice", age: 30 };
console.log(greet(alice));
"#;
    if let Some(lines) = highlighter.highlight(ts_code, "typescript") {
        for line in &lines {
            for segment in &line.segments {
                let bold = if segment.style.bold.unwrap_or(false) {
                    "B"
                } else {
                    " "
                };
                let italic = if segment.style.italic.unwrap_or(false) {
                    "I"
                } else {
                    " "
                };
                let fg = match segment.style.fg {
                    Some(bettertui_engine::tree::Color::Rgb { r, g, b }) => {
                        format!("#{r:02X}{g:02X}{b:02X}")
                    }
                    _ => "default".into(),
                };
                println!("  [{bold}{italic} fg:{fg:<9}] {}", segment.text);
            }
        }
        println!("  ({} lines highlighted)", lines.len());
    } else {
        println!("  (highlighting not available)");
    }

    // ── Python ──
    println!("\n[3] Python syntax highlighting...");
    let py_code = r#"import sys

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
    if let Some(lines) = highlighter.highlight(py_code, "python") {
        for (i, line) in lines.iter().enumerate() {
            let plain_text: String = line.segments.iter().map(|s| s.text.as_str()).collect();
            println!("  [{:>2}] {}", i + 1, plain_text);
        }
        println!("  ({} lines highlighted)", lines.len());
    } else {
        println!("  (highlighting not available)");
    }

    println!();
}
