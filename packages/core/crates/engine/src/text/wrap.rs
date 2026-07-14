use unicode_segmentation::UnicodeSegmentation;

use super::unicode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrappedLine {
    pub byte_offset: usize,
    pub byte_len: usize,
    pub visual_width: u16,
}

pub enum WrapMode {
    Word,
    Char,
    WordOrChar,
}

pub fn wrap_text(text: &str, max_width: u16, mode: WrapMode) -> Vec<WrappedLine> {
    if max_width == 0 {
        return Vec::new();
    }
    match mode {
        WrapMode::Word => word_wrap(text, max_width),
        WrapMode::Char => char_wrap(text, max_width),
        WrapMode::WordOrChar => word_wrap_fallback(text, max_width),
    }
}

fn word_wrap(text: &str, max_width: u16) -> Vec<WrappedLine> {
    let mut lines: Vec<WrappedLine> = Vec::new();
    let mut text_offset = 0usize;
    for hard_line in text.split('\n') {
        if hard_line.is_empty() {
            lines.push(WrappedLine {
                byte_offset: text_offset,
                byte_len: 0,
                visual_width: 0,
            });
            text_offset += 1;
            continue;
        }
        let hard_line_start = text_offset;
        let mut line_start = 0usize;
        loop {
            let remaining = &text[hard_line_start + line_start..][..hard_line.len() - line_start];
            let remaining_width = unicode::display_width(remaining);
            if remaining_width <= max_width as usize {
                lines.push(WrappedLine {
                    byte_offset: hard_line_start + line_start,
                    byte_len: remaining.len(),
                    visual_width: remaining_width as u16,
                });
                break;
            }
            let break_at = find_word_break(
                &text[hard_line_start..hard_line_start + hard_line.len()],
                line_start,
                max_width,
            );
            lines.push(WrappedLine {
                byte_offset: hard_line_start + line_start,
                byte_len: break_at - line_start,
                visual_width: unicode::display_width(
                    &text[hard_line_start + line_start..hard_line_start + break_at],
                ) as u16,
            });
            line_start = break_at;
        }
        text_offset += hard_line.len() + 1;
    }
    lines
}

fn find_word_break(text: &str, start: usize, max_width: u16) -> usize {
    let remaining = &text[start..];
    let mut col = 0u16;
    let mut last_space_end = None;
    for (byte_offset, g) in remaining.grapheme_indices(true) {
        let w = unicode::grapheme_width(g) as u16;
        if col + w > max_width {
            return match last_space_end {
                Some(space_end) => start + space_end,
                None => start + byte_offset,
            };
        }
        if g == " " || g == "\t" {
            last_space_end = Some(byte_offset + g.len());
        }
        col += w;
    }
    start + remaining.len()
}

fn char_wrap(text: &str, max_width: u16) -> Vec<WrappedLine> {
    let mut lines: Vec<WrappedLine> = Vec::new();
    let mut text_offset = 0usize;
    for hard_line in text.split('\n') {
        let hard_line_start = text_offset;
        if hard_line.is_empty() {
            lines.push(WrappedLine {
                byte_offset: hard_line_start,
                byte_len: 0,
                visual_width: 0,
            });
            text_offset += 1;
            continue;
        }
        let line_bytes = &text[hard_line_start..hard_line_start + hard_line.len()];
        let mut line_start = 0usize;
        let mut col = 0u16;
        for (rel_offset, g) in line_bytes.grapheme_indices(true) {
            let w = unicode::grapheme_width(g) as u16;
            if col + w > max_width {
                lines.push(WrappedLine {
                    byte_offset: hard_line_start + line_start,
                    byte_len: rel_offset - line_start,
                    visual_width: col,
                });
                line_start = rel_offset;
                col = 0;
            }
            col += w;
        }
        let remaining = &text[hard_line_start + line_start..hard_line_start + hard_line.len()];
        if !remaining.is_empty() {
            lines.push(WrappedLine {
                byte_offset: hard_line_start + line_start,
                byte_len: remaining.len(),
                visual_width: unicode::display_width(remaining) as u16,
            });
        }
        text_offset += hard_line.len() + 1;
    }
    lines
}

fn word_wrap_fallback(text: &str, max_width: u16) -> Vec<WrappedLine> {
    if max_width < 3 {
        return char_wrap(text, max_width);
    }
    let word_lines = word_wrap(text, max_width);
    let mut result = Vec::with_capacity(word_lines.len());
    for line in &word_lines {
        let fragment = &text[line.byte_offset..][..line.byte_len];
        if !fragment.is_empty() && unicode::display_width(fragment) <= max_width as usize {
            result.push(line.clone());
        } else {
            let char_lines = char_wrap(fragment, max_width);
            let base_offset = line.byte_offset;
            for cl in &char_lines {
                result.push(WrappedLine {
                    byte_offset: base_offset + cl.byte_offset,
                    byte_len: cl.byte_len,
                    visual_width: cl.visual_width,
                });
            }
        }
    }
    result
}

#[allow(dead_code)]
pub fn compute_line_count(wrapped: &[WrappedLine]) -> u16 {
    wrapped.len() as u16
}

#[allow(dead_code)]
pub fn line_at(wrapped: &[WrappedLine], index: usize) -> Option<&WrappedLine> {
    wrapped.get(index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_wrap_simple() {
        let text = "hello world foo";
        let lines = word_wrap(text, 10);
        assert_eq!(lines.len(), 2, "got {lines:#?}");
        assert_eq!(&text[lines[0].byte_offset..][..lines[0].byte_len], "hello ");
        assert_eq!(
            &text[lines[1].byte_offset..][..lines[1].byte_len],
            "world foo"
        );
    }

    #[test]
    fn word_wrap_no_break_needed() {
        let lines = word_wrap("hello", 10);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].visual_width, 5);
    }

    #[test]
    fn word_wrap_empty() {
        let lines = word_wrap("", 10);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].byte_len, 0);
    }

    #[test]
    fn word_wrap_single_word_longer_than_width() {
        let lines = word_wrap("superlongword", 5);
        assert!(lines.len() >= 2);
        for line in &lines {
            let text = &"superlongword"[line.byte_offset..][..line.byte_len];
            assert!(unicode::display_width(text) <= 5);
        }
    }

    #[test]
    fn char_wrap_simple() {
        let lines = char_wrap("abcdefghij", 5);
        assert_eq!(lines.len(), 2);
        assert_eq!(
            &"abcdefghij"[lines[0].byte_offset..][..lines[0].byte_len],
            "abcde"
        );
        assert_eq!(
            &"abcdefghij"[lines[1].byte_offset..][..lines[1].byte_len],
            "fghij"
        );
    }

    #[test]
    fn char_wrap_cjk() {
        let text = "\u{4e2d}\u{6587}\u{5b57}\u{7b26}\u{4e32}";
        let lines = char_wrap(text, 4);
        assert_eq!(lines.len(), 3, "got {lines:#?}");
        assert_eq!(lines[0].visual_width, 4);
        assert_eq!(lines[1].visual_width, 4);
        assert_eq!(lines[2].visual_width, 2);
    }

    #[test]
    fn wrap_preserves_newlines() {
        let text = "hello\nworld";
        let lines = word_wrap(text, 10);
        assert_eq!(lines.len(), 2, "got {lines:#?}");
        assert_eq!(&text[lines[0].byte_offset..][..lines[0].byte_len], "hello");
        assert_eq!(&text[lines[1].byte_offset..][..lines[1].byte_len], "world");
    }

    #[test]
    fn wrap_empty_lines() {
        let lines = word_wrap("hello\n\nworld", 10);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[1].byte_len, 0);
    }

    #[test]
    fn word_wrap_with_cjk() {
        let text = "\u{4e2d}\u{6587}hello\u{4e16}\u{754c}";
        let lines = word_wrap(text, 6);
        assert!(lines.len() >= 2);
    }

    #[test]
    fn word_or_char_fallback() {
        let lines = word_wrap_fallback("superlongwordthatwontbreak", 5);
        assert!(lines.len() >= 3);
        for line in &lines {
            let text = &"superlongwordthatwontbreak"[line.byte_offset..][..line.byte_len];
            assert!(unicode::display_width(text) <= 5);
        }
    }

    #[test]
    fn compute_line_count_works() {
        let lines = word_wrap("a\nb\nc", 10);
        assert_eq!(compute_line_count(&lines), 3);
    }

    #[test]
    fn line_at_works() {
        let lines = word_wrap("hello world", 10);
        assert!(line_at(&lines, 0).is_some());
        assert!(line_at(&lines, 5).is_none());
    }

    #[test]
    fn word_wrap_tab_as_space() {
        let lines = word_wrap("hello\tworld", 10);
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn char_wrap_exact_fit() {
        let lines = char_wrap("abcde", 5);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].visual_width, 5);
    }
}
