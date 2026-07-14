use std::io::{self, Write};

use bettertui_engine::text::{SearchOptions, TextEngine, display_width, grapheme_count};
use bettertui_terminal::Terminal;

pub fn run(terminal: &mut Terminal) -> io::Result<()> {
    let mut out = io::stdout();
    terminal.clear()?;
    terminal.move_cursor(0, 0)?;

    writeln!(out, "\x1b[1;97m━━━ Text: TextEngine Buffer, Cursor, and Search ━━━\x1b[0m\n")?;

    // Basic text buffer
    writeln!(out, "\x1b[33m[1]\x1b[0m Basic text buffer...")?;
    let mut te = TextEngine::with_text("Hello, BetterTUI!");
    writeln!(out, "  Text: \"{}\"", te.text())?;
    writeln!(out, "  Char count: {}, Line count: {}", te.char_count(), te.line_count())?;

    // Cursor operations
    writeln!(out, "\n\x1b[33m[2]\x1b[0m Cursor operations...")?;
    te.cursor_mut().set_position(7);
    te.insert_str(" Native");
    writeln!(out, "  After insert at pos 7: \"{}\"", te.text())?;
    te.cursor_mut().set_position(0);
    te.insert_str(">> ");
    writeln!(out, "  After prepend:       \"{}\"", te.text())?;

    // Delete
    writeln!(out, "\n\x1b[33m[3]\x1b[0m Delete operations...")?;
    let mut te2 = TextEngine::with_text("This is some text to edit");
    te2.cursor_mut().set_position(15);
    for _ in 0..5 { te2.delete_char(); }
    writeln!(out, "  After deleting 'text ': \"{}\"", te2.text())?;

    // Multi-line
    writeln!(out, "\n\x1b[33m[4]\x1b[0m Multi-line editing...")?;
    let te3 = TextEngine::with_text("Line 1\nLine 2\nLine 3");
    for i in 0..te3.line_count() {
        if let Some(line) = te3.line(i) {
            writeln!(out, "    [{i}] \"{line}\"")?;
        }
    }

    // Search
    writeln!(out, "\n\x1b[33m[5]\x1b[0m Search...")?;
    let mut te4 = TextEngine::with_text("The quick brown fox jumps over the lazy dog. The fox is quick.");
    let results = te4.search("fox", SearchOptions::default());
    writeln!(out, "  Found 'fox' {} times", results.len())?;
    for r in &results {
        writeln!(out, "    at chars {}-{} (line {}, col {})", r.range.start, r.range.end, r.line, r.column)?;
    }
    let case_opts = SearchOptions { case_sensitive: true, ..SearchOptions::default() };
    writeln!(out, "  Found 'The' (case-sensitive) {} times", te4.search("The", case_opts).len())?;

    // Unicode
    writeln!(out, "\n\x1b[33m[6]\x1b[0m Unicode support...")?;
    let te5 = TextEngine::with_text("日本語 & 中文 & English 🔥");
    writeln!(out, "  Text: \"{}\"", te5.text())?;
    writeln!(out, "  Char count: {}", te5.char_count())?;
    writeln!(out, "  Display width: {}", display_width(&te5.text()))?;
    writeln!(out, "  Grapheme count: {}", grapheme_count(&te5.text()))?;

    writeln!(out, "\n\x1b[2;90mPress any key to return to menu...\x1b[0m")?;
    out.flush()?;
    wait_for_any_key(terminal)
}

fn wait_for_any_key(terminal: &mut Terminal) -> io::Result<()> {
    loop {
        if let Some(bettertui_terminal::TerminalEvent::Key(_)) = terminal.poll_event(std::time::Duration::from_millis(100))? { return Ok(()) }
    }
}
