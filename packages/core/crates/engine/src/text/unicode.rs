use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthChar;
use unicode_width::UnicodeWidthStr;

pub fn display_width(s: &str) -> usize {
    if !s.contains('\x1b') {
        return s.width();
    }
    let mut width = 0;
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        if bytes[i] == 0x1b {
            i += 1;
            if i < len && bytes[i] == b'[' {
                i += 1;
                while i < len {
                    let b = bytes[i];
                    i += 1;
                    if (0x40..=0x7e).contains(&b) {
                        break;
                    }
                }
            } else if i < len && bytes[i] == b']' {
                i += 1;
                while i < len {
                    if bytes[i] == 0x07 {
                        i += 1;
                        break;
                    }
                    if bytes[i] == 0x1b && i + 1 < len && bytes[i + 1] == b'\\' {
                        i += 2;
                        break;
                    }
                    i += 1;
                }
            }
            continue;
        }

        if bytes[i] >= 0x20 && bytes[i] <= 0x7e {
            width += 1;
            i += 1;
            continue;
        }

        if let Some(c) = s[i..].chars().next() {
            width += c.width().unwrap_or(0);
            i += c.len_utf8();
        } else {
            i += 1;
        }
    }

    width
}

pub fn grapheme_width(g: &str) -> usize {
    if g.contains('\x1b') {
        return display_width(g);
    }
    g.width()
}

pub fn char_width(c: char) -> u16 {
    c.width().unwrap_or(0) as u16
}

pub fn grapheme_clusters(s: &str) -> Vec<&str> {
    s.graphemes(true).collect()
}

pub fn grapheme_count(s: &str) -> usize {
    s.graphemes(true).count()
}

pub fn display_width_to_byte_offset(s: &str, target_col: usize) -> usize {
    let mut col = 0;
    for (byte_offset, g) in s.grapheme_indices(true) {
        let w = g.width();
        if col + w > target_col {
            return byte_offset;
        }
        col += w;
    }
    s.len()
}

pub fn byte_offset_to_display_width(s: &str, byte_offset: usize) -> usize {
    let mut col = 0;
    for (bo, g) in s.grapheme_indices(true) {
        if bo >= byte_offset {
            break;
        }
        col += g.width();
    }
    col
}

pub fn truncate_to_width(s: &str, max_width: usize) -> &str {
    let mut col = 0;
    for (byte_offset, g) in s.grapheme_indices(true) {
        let w = g.width();
        if col + w > max_width {
            return &s[..byte_offset];
        }
        col += w;
    }
    s
}

pub fn truncate_with_ellipsis(s: &str, max_width: usize) -> String {
    let ellipsis = "\u{2026}";
    let ellipsis_width = ellipsis.width();
    if max_width < ellipsis_width {
        return String::new();
    }
    let available = max_width - ellipsis_width;
    if display_width(s) <= available {
        return s.to_string();
    }
    let mut col = 0;
    let mut cut = 0;
    for (byte_offset, g) in s.grapheme_indices(true) {
        let w = g.width();
        if col + w > available {
            break;
        }
        col += w;
        cut = byte_offset + g.len();
    }
    let mut result = s[..cut].to_string();
    result.push_str(ellipsis);
    result
}

pub fn is_wide_char(c: char) -> bool {
    char_width(c) == 2
}

pub fn is_emoji(c: char) -> bool {
    matches!(c as u32,
        0x231A..=0x231B | 0x23E9..=0x23F3 | 0x23F8..=0x23FA |
        0x25FD..=0x25FE | 0x2614..=0x2615 | 0x2648..=0x2653 |
        0x267F | 0x2693 | 0x26A1 | 0x26AA..=0x26AB |
        0x26BD..=0x26BE | 0x26C4..=0x26C5 | 0x26CE | 0x26D4 |
        0x26EA | 0x26F2..=0x26F3 | 0x26F5 | 0x26FA | 0x26FD |
        0x2702 | 0x2705 | 0x2708..=0x270D | 0x270F |
        0x2712 | 0x2714 | 0x2716 | 0x271D | 0x2721 |
        0x2728 | 0x2733..=0x2734 | 0x2744 | 0x2747 | 0x274C |
        0x274E | 0x2753..=0x2755 | 0x2757 | 0x2763..=0x2764 |
        0x2795..=0x2797 | 0x27A1 | 0x27B0 | 0x27BF |
        0x2934..=0x2935 | 0x2B05..=0x2B07 | 0x2B1B..=0x2B1C |
        0x2B50 | 0x2B55 | 0x3030 | 0x303D | 0x3297 | 0x3299 |
        0x1F000..=0x1FFFF
    )
}

pub fn is_nerd_font_glyph(c: char) -> bool {
    match c as u32 {
        0xE000..=0xE0FF | 0xEE00..=0xEEFF => true,
        0xF500..=0xFDFF => true,
        0xFE000..=0xFE02F => true,
        0xFE300..=0xFE4FF => true,
        0x1F600..=0x1F64F => false,
        _ => c.width().unwrap_or(0) == 2 && is_emoji(c),
    }
}

pub fn is_box_drawing(c: char) -> bool {
    matches!(c as u32, 0x2500..=0x25FF)
}

pub fn is_powerline(c: char) -> bool {
    matches!(c as u32, 0xE0A0..=0xE0D4)
}

pub fn is_zero_width(c: char) -> bool {
    c.width().unwrap_or(0) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_width() {
        assert_eq!(display_width("hello"), 5);
        assert_eq!(display_width(""), 0);
    }

    #[test]
    fn ansi_width() {
        assert_eq!(display_width("\x1b[38;2;255;255;255mhello\x1b[0m"), 5);
        assert_eq!(display_width("\x1b[1m\x1b[31mWORLD\x1b[0m"), 5);
    }

    #[test]
    fn cjk_width() {
        assert_eq!(display_width("\u{4e2d}\u{6587}"), 4);
        assert_eq!(char_width('\u{4e2d}'), 2);
    }

    #[test]
    fn emoji_width() {
        assert_eq!(char_width('\u{231A}'), 2);
    }

    #[test]
    fn grapheme_emoji_sequence() {
        let s = "a\u{301}";
        let clusters = grapheme_clusters(s);
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0], "a\u{301}");
        assert_eq!(grapheme_width(clusters[0]), 1);
    }

    #[test]
    fn grapheme_flag_emoji() {
        let s = "\u{1F1FA}\u{1F1F8}";
        let clusters = grapheme_clusters(s);
        assert_eq!(clusters.len(), 1);
    }

    #[test]
    fn zwj_emoji() {
        let s = "\u{1F468}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}";
        let clusters = grapheme_clusters(s);
        assert_eq!(clusters.len(), 1);
    }

    #[test]
    fn display_width_to_byte_offset_works() {
        let s = "ab\u{4e2d}cd";
        assert_eq!(display_width_to_byte_offset(s, 0), 0);
        assert_eq!(display_width_to_byte_offset(s, 2), 2);
        assert_eq!(display_width_to_byte_offset(s, 4), 5);
    }

    #[test]
    fn byte_offset_to_display_width_works() {
        let s = "ab\u{4e2d}cd";
        assert_eq!(byte_offset_to_display_width(s, 0), 0);
        assert_eq!(byte_offset_to_display_width(s, 2), 2);
        assert_eq!(byte_offset_to_display_width(s, 3), 4);
    }

    #[test]
    fn truncate_to_width_ascii() {
        assert_eq!(truncate_to_width("hello world", 5), "hello");
    }

    #[test]
    fn truncate_to_width_cjk() {
        assert_eq!(truncate_to_width("\u{4e2d}\u{6587}test", 4), "\u{4e2d}\u{6587}");
    }

    #[test]
    fn truncate_with_ellipsis_ascii() {
        let r = truncate_with_ellipsis("hello world", 8);
        assert_eq!(r, "hello w\u{2026}", "got {r:?}");
    }

    #[test]
    fn truncate_with_ellipsis_fits() {
        let r = truncate_with_ellipsis("hi", 8);
        assert_eq!(r, "hi");
    }

    #[test]
    fn truncate_with_ellipsis_too_narrow() {
        let r = truncate_with_ellipsis("hi", 0);
        assert_eq!(r, "");
    }

    #[test]
    fn truncate_with_ellipsis_cjk() {
        let r = truncate_with_ellipsis("\u{4e2d}\u{6587}test", 5);
        assert_eq!(r, "\u{4e2d}\u{6587}\u{2026}", "got {r:?}");
    }

    #[test]
    fn is_wide_char_works() {
        assert!(!is_wide_char('a'));
        assert!(is_wide_char('\u{4e2d}'));
        assert!(is_wide_char('\u{231A}'));
    }

    #[test]
    fn is_box_drawing_works() {
        assert!(is_box_drawing('\u{2500}'));
        assert!(is_box_drawing('\u{2550}'));
        assert!(!is_box_drawing('a'));
    }

    #[test]
    fn is_powerline_works() {
        assert!(is_powerline('\u{E0B0}'));
        assert!(!is_powerline('a'));
    }

    #[test]
    fn is_zero_width_works() {
        assert!(is_zero_width('\u{200D}'));
        assert!(!is_zero_width('a'));
    }

    #[test]
    fn grapheme_count_works() {
        assert_eq!(grapheme_count("hello"), 5);
        assert_eq!(grapheme_count("\u{1F1FA}\u{1F1F8}"), 1);
    }

    #[test]
    fn mixed_width_truncation() {
        let s = "a\u{4e2d}b\u{6587}c";
        assert_eq!(display_width(s), 7);
        let clipped = truncate_to_width(s, 5);
        assert_eq!(display_width(clipped), 4);
    }

    #[test]
    fn ellipsis_replaces_last_char() {
        let r = truncate_with_ellipsis("\u{4e2d}\u{6587}test", 5);
        assert_eq!(r, "\u{4e2d}\u{6587}\u{2026}", "got {r:?}");
    }
}
