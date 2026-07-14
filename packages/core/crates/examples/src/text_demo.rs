use bettertui_engine::text::{SearchOptions, TextEngine};

use crate::util;

pub fn run() {
    util::heading("Text Demo: TextEngine Buffer, Cursor, and Search");

    // ── Basic text buffer ──
    println!("[1] Creating TextEngine and inserting text...");
    let mut te = TextEngine::with_text("Hello, BetterTUI!");
    println!("  Text: \"{}\"", te.text());
    println!("  Char count: {}, Line count: {}", te.char_count(), te.line_count());

    // ── Cursor movement ──
    println!("\n[2] Cursor operations...");
    te.cursor_mut().set_position(7);
    te.insert_str(" Native");
    println!("  After insert at pos 7: \"{}\"", te.text());
    println!("  Cursor position: {}", te.cursor().position());

    te.cursor_mut().set_position(0);
    te.insert_str(">> ");
    println!("  After prepend:       \"{}\"", te.text());

    // ── Delete operations ──
    println!("\n[3] Delete operations...");
    let mut te2 = TextEngine::with_text("This is some text to edit");
    te2.cursor_mut().set_position(15);
    te2.delete_char();
    te2.delete_char();
    te2.delete_char();
    te2.delete_char();
    te2.delete_char();
    println!("  After deleting 'text ': \"{}\"", te2.text());

    // ── Multi-line ──
    println!("\n[4] Multi-line editing...");
    let te3 = TextEngine::with_text("Line 1\nLine 2\nLine 3");
    println!("  Lines:");
    for i in 0..te3.line_count() {
        if let Some(line) = te3.line(i) {
            println!("    [{i}] \"{line}\"");
        }
    }
    println!("  Total lines: {}", te3.line_count());

    // ── Search ──
    println!("\n[5] Search...");
    let mut te4 = TextEngine::with_text("The quick brown fox jumps over the lazy dog. The fox is quick.");
    let results = te4.search("fox", SearchOptions::default());
    println!("  Found 'fox' {} times", results.len());
    for r in &results {
        println!("    at chars {}-{} (line {}, col {})",
            r.range.start, r.range.end, r.line, r.column);
    }
    let case_opts = SearchOptions { case_sensitive: true, ..SearchOptions::default() };
    let results2 = te4.search("The", case_opts);
    println!("  Found 'The' (case-sensitive) {} times", results2.len());

    // ── Unicode ──
    println!("\n[6] Unicode support...");
    let te5 = TextEngine::with_text("日本語 & 中文 & English 🔥");
    println!("  Text: \"{}\"", te5.text());
    println!("  Char count: {}", te5.char_count());
    println!("  Display width: {}", bettertui_engine::text::display_width(&te5.text()));
    println!("  Grapheme count: {}", bettertui_engine::text::grapheme_count(&te5.text()));

    println!();
}
