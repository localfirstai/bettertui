use crate::tree::Style;

use super::unicode;

#[derive(Debug, Clone, PartialEq)]
pub struct StyledSpan {
    pub text: String,
    pub style: Style,
}

impl StyledSpan {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), style: Style::default() }
    }

    pub fn styled(text: impl Into<String>, style: Style) -> Self {
        Self { text: text.into(), style }
    }

    pub fn display_width(&self) -> usize {
        unicode::display_width(&self.text)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct StyledText {
    pub spans: Vec<StyledSpan>,
}

impl StyledText {
    pub fn new() -> Self {
        Self { spans: Vec::new() }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self { spans: Vec::with_capacity(cap) }
    }

    pub fn push(&mut self, span: StyledSpan) {
        if let Some(last) = self.spans.last_mut()
            && last.style == span.style
        {
            last.text.push_str(&span.text);
            return;
        }
        self.spans.push(span);
    }

    pub fn push_text(&mut self, text: impl Into<String>) {
        let text = text.into();
        if text.is_empty() {
            return;
        }
        if let Some(last) = self.spans.last_mut()
            && last.style.is_empty()
        {
            last.text.push_str(&text);
            return;
        }
        self.spans.push(StyledSpan::new(text));
    }

    pub fn push_styled(&mut self, text: impl Into<String>, style: Style) {
        self.push(StyledSpan::styled(text, style));
    }

    pub fn is_empty(&self) -> bool {
        self.spans.is_empty() || self.spans.iter().all(|s| s.text.is_empty())
    }

    pub fn plain_text(&self) -> String {
        let capacity = self.spans.iter().map(|s| s.text.len()).sum();
        let mut result = String::with_capacity(capacity);
        for span in &self.spans {
            result.push_str(&span.text);
        }
        result
    }

    pub fn display_width(&self) -> usize {
        self.spans.iter().map(|s| s.display_width()).sum()
    }

    pub fn merge_adjacent_with_same_style(&mut self) {
        if self.spans.len() < 2 {
            return;
        }
        let mut merged = Vec::with_capacity(self.spans.len());
        let mut current = self.spans[0].clone();
        for span in self.spans.drain(1..) {
            if current.style == span.style {
                current.text.push_str(&span.text);
            } else {
                merged.push(current);
                current = span;
            }
        }
        merged.push(current);
        self.spans = merged;
    }

    pub fn split_at(&self, byte_offset: usize) -> (StyledText, StyledText) {
        let mut left = StyledText::new();
        let mut right = StyledText::new();
        let mut pos = 0;
        for span in &self.spans {
            let span_end = pos + span.text.len();
            if byte_offset <= pos {
                right.spans.push(span.clone());
            } else if byte_offset >= span_end {
                left.spans.push(span.clone());
            } else {
                let split_point = byte_offset - pos;
                let (l, r) = span.text.split_at(split_point);
                left.spans.push(StyledSpan::styled(l.to_string(), span.style));
                right.spans.push(StyledSpan::styled(r.to_string(), span.style));
            }
            pos = span_end;
        }
        (left, right)
    }

    pub fn subspan(&self, start: usize, end: usize) -> StyledText {
        let (_, after_left) = self.split_at(start);
        let (result, _) = after_left.split_at(end - start);
        result
    }

    pub fn truncate_to_width(&self, max_width: usize) -> StyledText {
        let mut result = StyledText::new();
        let mut col = 0;
        for span in &self.spans {
            if col >= max_width {
                break;
            }
            let remaining = max_width - col;
            let span_w = span.display_width();
            if span_w <= remaining {
                result.push(span.clone());
                col += span_w;
            } else {
                let truncated = unicode::truncate_to_width(&span.text, remaining);
                if !truncated.is_empty() {
                    result.push(StyledSpan::styled(truncated.to_string(), span.style));
                }
                break;
            }
        }
        result
    }
}

impl From<&str> for StyledText {
    fn from(s: &str) -> Self {
        let mut ss = StyledText::new();
        ss.push_text(s);
        ss
    }
}

impl From<String> for StyledText {
    fn from(s: String) -> Self {
        let mut ss = StyledText::new();
        ss.push_text(s);
        ss
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::{Color, NamedColor};

    #[test]
    fn styled_span_new() {
        let span = StyledSpan::new("hello");
        assert_eq!(span.text, "hello");
        assert!(span.style.is_empty());
    }

    #[test]
    fn styled_span_styled() {
        let style = Style { bold: Some(true), ..Style::default() };
        let span = StyledSpan::styled("bold", style);
        assert!(span.style.bold.unwrap());
    }

    #[test]
    fn styled_string_push_merges_adjacent() {
        let mut ss = StyledText::new();
        ss.push_text("hello ");
        ss.push_text("world");
        assert_eq!(ss.spans.len(), 1);
        assert_eq!(ss.plain_text(), "hello world");
    }

    #[test]
    fn styled_string_push_no_merge_different_style() {
        let mut ss = StyledText::new();
        ss.push_styled("bold", Style { bold: Some(true), ..Style::default() });
        ss.push_text("normal");
        assert_eq!(ss.spans.len(), 2);
    }

    #[test]
    fn styled_string_plain_text() {
        let mut ss = StyledText::new();
        ss.push_styled("hello", Style { fg: Some(Color::Named(NamedColor::Red)), ..Style::default() });
        ss.push_text(" world");
        assert_eq!(ss.plain_text(), "hello world");
    }

    #[test]
    fn styled_string_display_width() {
        let mut ss = StyledText::new();
        ss.push_text("hello");
        ss.push_text(" \u{4e2d}\u{6587}");
        assert_eq!(ss.display_width(), 10);
    }

    #[test]
    fn styled_string_is_empty() {
        let ss = StyledText::new();
        assert!(ss.is_empty());
    }

    #[test]
    fn styled_string_merge_adjacent() {
        let mut ss = StyledText::new();
        let style = Style { bold: Some(true), ..Style::default() };
        ss.push_styled("hello", style);
        ss.push_styled(" world", Style { bold: Some(true), ..Style::default() });
        ss.merge_adjacent_with_same_style();
        assert_eq!(ss.spans.len(), 1);
        assert_eq!(ss.plain_text(), "hello world");
    }

    #[test]
    fn styled_string_split_at() {
        let mut ss = StyledText::new();
        ss.push_styled("hello", Style { bold: Some(true), ..Style::default() });
        ss.push_text(" world");
        let (left, right) = ss.split_at(5);
        assert_eq!(left.plain_text(), "hello");
        assert_eq!(right.plain_text(), " world");
    }

    #[test]
    fn styled_string_subspan() {
        let mut ss = StyledText::new();
        ss.push_text("hello world");
        let sub = ss.subspan(0, 5);
        assert_eq!(sub.plain_text(), "hello");
    }

    #[test]
    fn styled_string_truncate_to_width() {
        let mut ss = StyledText::new();
        ss.push_text("hello world");
        let truncated = ss.truncate_to_width(5);
        assert_eq!(truncated.plain_text(), "hello");
    }

    #[test]
    fn styled_string_from_str() {
        let ss: StyledText = "hello".into();
        assert_eq!(ss.plain_text(), "hello");
    }

    #[test]
    fn styled_string_from_string() {
        let ss: StyledText = String::from("hello").into();
        assert_eq!(ss.plain_text(), "hello");
    }

    #[test]
    fn styled_string_with_capacity() {
        let ss = StyledText::with_capacity(10);
        assert!(ss.is_empty());
    }
}
