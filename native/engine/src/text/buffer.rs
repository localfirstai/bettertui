use ropey::Rope;

#[derive(Debug, Clone)]
pub struct TextBuffer {
    rope: Rope,
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl TextBuffer {
    pub fn new() -> Self {
        Self { rope: Rope::new() }
    }

    pub fn with_text(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
        }
    }

    pub fn insert_char(&mut self, pos: usize, ch: char) {
        self.rope.insert_char(pos, ch);
    }

    pub fn insert_str(&mut self, pos: usize, s: &str) {
        self.rope.insert(pos, s);
    }

    pub fn delete_char(&mut self, pos: usize) {
        if pos < self.rope.len_chars() {
            self.rope.remove(pos..pos + 1);
        }
    }

    pub fn delete_range(&mut self, start: usize, end: usize) {
        if start < end && end <= self.rope.len_chars() {
            self.rope.remove(start..end);
        }
    }

    pub fn char_at(&self, pos: usize) -> char {
        if pos < self.rope.len_chars() {
            self.rope.char(pos)
        } else {
            '\0'
        }
    }

    pub fn substring(&self, start: usize, end: usize) -> String {
        if start < end && end <= self.rope.len_chars() {
            self.rope.slice(start..end).to_string()
        } else {
            String::new()
        }
    }

    pub fn line(&self, line: usize) -> Option<String> {
        if line < self.rope.len_lines() {
            Some(
                self.rope
                    .line(line)
                    .to_string()
                    .trim_end_matches('\n')
                    .to_string(),
            )
        } else {
            None
        }
    }

    pub fn line_length(&self, line: usize) -> Option<usize> {
        if line < self.rope.len_lines() {
            Some(self.rope.line(line).len_chars())
        } else {
            None
        }
    }

    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn char_count(&self) -> usize {
        self.rope.len_chars()
    }

    pub fn word_count(&self) -> usize {
        let text = self.rope.to_string();
        text.split_whitespace().count()
    }

    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    pub fn clear(&mut self) {
        self.rope = Rope::new();
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    pub fn line_to_char(&self, line: usize) -> usize {
        self.rope.line_to_char(line)
    }

    pub fn char_to_line(&self, char_idx: usize) -> usize {
        self.rope.char_to_line(char_idx)
    }

    pub fn word_boundary_left(&self, pos: usize) -> usize {
        if pos == 0 {
            return 0;
        }

        let text = self.to_string();
        let chars: Vec<char> = text.chars().collect();
        let mut i = pos;

        while i > 0 && chars[i - 1].is_whitespace() {
            i -= 1;
        }

        while i > 0 && !chars[i - 1].is_whitespace() {
            i -= 1;
        }

        i
    }

    pub fn word_boundary_right(&self, pos: usize) -> usize {
        let text = self.to_string();
        let chars: Vec<char> = text.chars().collect();
        let mut i = pos;

        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }

        while i < chars.len() && !chars[i].is_whitespace() {
            i += 1;
        }

        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_buffer_new() {
        let buffer = TextBuffer::new();
        assert!(buffer.is_empty());
    }

    #[test]
    fn text_buffer_default() {
        let buffer = TextBuffer::default();
        assert!(buffer.is_empty());
    }

    #[test]
    fn text_buffer_with_text() {
        let buffer = TextBuffer::with_text("hello");
        assert_eq!(buffer.char_count(), 5);
    }

    #[test]
    fn text_buffer_insert_char() {
        let mut buffer = TextBuffer::new();
        buffer.insert_char(0, 'a');
        buffer.insert_char(1, 'b');
        assert_eq!(buffer.to_string(), "ab");
    }

    #[test]
    fn text_buffer_insert_str() {
        let mut buffer = TextBuffer::new();
        buffer.insert_str(0, "hello");
        assert_eq!(buffer.to_string(), "hello");
    }

    #[test]
    fn text_buffer_delete_char() {
        let mut buffer = TextBuffer::with_text("abc");
        buffer.delete_char(1);
        assert_eq!(buffer.to_string(), "ac");
    }

    #[test]
    fn text_buffer_delete_range() {
        let mut buffer = TextBuffer::with_text("hello");
        buffer.delete_range(1, 3);
        assert_eq!(buffer.to_string(), "hlo");
    }

    #[test]
    fn text_buffer_char_at() {
        let buffer = TextBuffer::with_text("hello");
        assert_eq!(buffer.char_at(0), 'h');
        assert_eq!(buffer.char_at(4), 'o');
    }

    #[test]
    fn text_buffer_substring() {
        let buffer = TextBuffer::with_text("hello");
        assert_eq!(buffer.substring(1, 3), "el");
    }

    #[test]
    fn text_buffer_line() {
        let buffer = TextBuffer::with_text("line1\nline2\nline3");
        assert_eq!(buffer.line(0), Some("line1".to_string()));
        assert_eq!(buffer.line(1), Some("line2".to_string()));
        assert_eq!(buffer.line(2), Some("line3".to_string()));
    }

    #[test]
    fn text_buffer_line_count() {
        let buffer = TextBuffer::with_text("line1\nline2\nline3");
        assert_eq!(buffer.line_count(), 3);
    }

    #[test]
    fn text_buffer_word_count() {
        let buffer = TextBuffer::with_text("hello world foo");
        assert_eq!(buffer.word_count(), 3);
    }

    #[test]
    fn text_buffer_word_boundary() {
        let buffer = TextBuffer::with_text("hello world foo");
        assert_eq!(buffer.word_boundary_left(7), 6);
        assert_eq!(buffer.word_boundary_right(6), 11);
    }
}
