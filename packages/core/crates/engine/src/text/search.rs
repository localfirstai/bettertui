use super::buffer::TextBuffer;
use super::selection::SelectionRange;
use regex::{Regex, RegexBuilder};

#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub case_sensitive: bool,
    pub whole_word: bool,
    pub regex: bool,
    pub wrap_around: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self { case_sensitive: false, whole_word: false, regex: false, wrap_around: true }
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub range: SelectionRange,
    pub line: usize,
    pub column: usize,
}

/// Compile a `Regex` from a pattern and the requested options.
///
/// - When `regex` is disabled the pattern is escaped so it matches literally.
/// - When `whole_word` is enabled the pattern is wrapped in word boundaries (`\b`).
/// - `case_sensitive` toggles the regex `i` flag.
///
/// Returns `None` when the pattern is empty or fails to compile (callers treat
/// this as "no matches" rather than panicking on user input).
fn compile(pattern: &str, options: &SearchOptions) -> Option<Regex> {
    if pattern.is_empty() {
        return None;
    }

    let body = if options.regex { pattern.to_string() } else { regex::escape(pattern) };
    let source = if options.whole_word { format!(r"\b(?:{body})\b") } else { body };

    RegexBuilder::new(&source).case_insensitive(!options.case_sensitive).build().ok()
}

/// Build a [`SearchResult`] for a byte range, computing 0-based line and column.
fn make_result(text: &str, start: usize, end: usize) -> SearchResult {
    let line = text[..start].matches('\n').count();
    let last_newline = text[..start].rfind('\n').map_or(0, |p| p + 1);
    let column = start - last_newline;
    SearchResult { range: SelectionRange::new(start, end), line, column }
}

#[derive(Debug, Clone)]
pub struct SearchResultIterator {
    text: String,
    regex: Option<Regex>,
    current_pos: usize,
    finished: bool,
}

impl SearchResultIterator {
    pub fn new(text: String, pattern: String, options: SearchOptions) -> Self {
        let regex = compile(&pattern, &options);
        let finished = regex.is_none();
        Self { text, regex, current_pos: 0, finished }
    }
}

impl Iterator for SearchResultIterator {
    type Item = SearchResult;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let regex = self.regex.as_ref()?;

        match regex.find_at(&self.text, self.current_pos) {
            Some(m) => {
                let (start, end) = (m.start(), m.end());
                // Guard against zero-width matches (e.g. `\b` alone) to avoid
                // looping forever on the same position.
                self.current_pos = if end > start { end } else { end + 1 };
                if self.current_pos > self.text.len() {
                    self.finished = true;
                }
                Some(make_result(&self.text, start, end))
            }
            None => {
                self.finished = true;
                None
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SearchEngine {
    #[allow(dead_code)]
    last_pattern: Option<String>,
    #[allow(dead_code)]
    last_options: Option<SearchOptions>,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self { last_pattern: None, last_options: None }
    }

    pub fn search(&self, buffer: &TextBuffer, pattern: &str, options: SearchOptions) -> Vec<SearchResult> {
        if pattern.is_empty() {
            return Vec::new();
        }
        let text = buffer.to_string();
        SearchResultIterator::new(text, pattern.to_string(), options).collect()
    }

    pub fn search_from(
        &self,
        buffer: &TextBuffer,
        pattern: &str,
        start: usize,
        options: SearchOptions,
    ) -> Option<SearchResult> {
        let text = buffer.to_string();
        if pattern.is_empty() || start > text.len() {
            return None;
        }
        let regex = compile(pattern, &options)?;
        regex.find_at(&text, start).map(|m| make_result(&text, m.start(), m.end()))
    }

    pub fn search_backward(
        &self,
        buffer: &TextBuffer,
        pattern: &str,
        start: usize,
        options: SearchOptions,
    ) -> Option<SearchResult> {
        let text = buffer.to_string();
        if pattern.is_empty() || start == 0 {
            return None;
        }
        let regex = compile(pattern, &options)?;
        // The last match whose end does not cross `start`.
        regex.find_iter(&text).take_while(|m| m.end() <= start).last().map(|m| make_result(&text, m.start(), m.end()))
    }

    pub fn count(&self, buffer: &TextBuffer, pattern: &str, options: SearchOptions) -> usize {
        self.search(buffer, pattern, options).len()
    }

    pub fn replace_all(
        &self,
        buffer: &mut TextBuffer,
        pattern: &str,
        replacement: &str,
        options: SearchOptions,
    ) -> usize {
        let results = self.search(buffer, pattern, options);
        let count = results.len();

        for result in results.into_iter().rev() {
            buffer.delete_range(result.range.start, result.range.end);
            buffer.insert_str(result.range.start, replacement);
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_options_default() {
        let options = SearchOptions::default();
        assert!(!options.case_sensitive);
        assert!(!options.whole_word);
        assert!(!options.regex);
        assert!(options.wrap_around);
    }

    #[test]
    fn search_result_new() {
        let result = SearchResult { range: SelectionRange::new(0, 5), line: 0, column: 0 };
        assert_eq!(result.range.start, 0);
        assert_eq!(result.range.end, 5);
    }

    #[test]
    fn search_engine_new() {
        let engine = SearchEngine::new();
        assert!(engine.last_pattern.is_none());
    }

    #[test]
    fn search_engine_search() {
        let buffer = TextBuffer::with_text("hello world hello");
        let engine = SearchEngine::new();
        let results = engine.search(&buffer, "hello", SearchOptions::default());
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn search_engine_search_from() {
        let buffer = TextBuffer::with_text("hello world hello");
        let engine = SearchEngine::new();
        let result = engine.search_from(&buffer, "hello", 6, SearchOptions::default());
        assert!(result.is_some());
        assert_eq!(result.unwrap().range.start, 12);
    }

    #[test]
    fn search_engine_search_backward() {
        let buffer = TextBuffer::with_text("hello world hello");
        let engine = SearchEngine::new();
        let result = engine.search_backward(&buffer, "hello", 12, SearchOptions::default());
        assert!(result.is_some());
        assert_eq!(result.unwrap().range.start, 0);
    }

    #[test]
    fn search_engine_count() {
        let buffer = TextBuffer::with_text("hello world hello");
        let engine = SearchEngine::new();
        let count = engine.count(&buffer, "hello", SearchOptions::default());
        assert_eq!(count, 2);
    }

    #[test]
    fn search_engine_replace_all() {
        let mut buffer = TextBuffer::with_text("hello world hello");
        let engine = SearchEngine::new();
        let count = engine.replace_all(&mut buffer, "hello", "hi", SearchOptions::default());
        assert_eq!(count, 2);
        assert_eq!(buffer.to_string(), "hi world hi");
    }

    // --- whole_word ---

    #[test]
    fn whole_word_excludes_substrings() {
        let buffer = TextBuffer::with_text("cat category cat");
        let engine = SearchEngine::new();
        let opts = SearchOptions { whole_word: true, ..Default::default() };
        let results = engine.search(&buffer, "cat", opts);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].range.start, 0);
        assert_eq!(results[1].range.start, 13);
    }

    #[test]
    fn whole_word_off_includes_substrings() {
        let buffer = TextBuffer::with_text("cat category cat");
        let engine = SearchEngine::new();
        let results = engine.search(&buffer, "cat", SearchOptions::default());
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn whole_word_backward() {
        let buffer = TextBuffer::with_text("cat category cat");
        let engine = SearchEngine::new();
        let opts = SearchOptions { whole_word: true, ..Default::default() };
        let result = engine.search_backward(&buffer, "cat", 16, opts);
        assert_eq!(result.unwrap().range.start, 13);
    }

    // --- regex ---

    #[test]
    fn regex_matches_pattern() {
        let buffer = TextBuffer::with_text("a1 b22 c333");
        let engine = SearchEngine::new();
        let opts = SearchOptions { regex: true, ..Default::default() };
        let results = engine.search(&buffer, r"\d+", opts);
        assert_eq!(results.len(), 3);
        assert_eq!(results[2].range.start, 8);
        assert_eq!(results[2].range.end, 11);
    }

    #[test]
    fn regex_literal_when_disabled() {
        let buffer = TextBuffer::with_text("a.b axb");
        let engine = SearchEngine::new();
        // `.` must be literal when regex is off, so only "a.b" matches.
        let results = engine.search(&buffer, "a.b", SearchOptions::default());
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].range.start, 0);
    }

    #[test]
    fn regex_with_whole_word() {
        let buffer = TextBuffer::with_text("foo1 foo foobar");
        let engine = SearchEngine::new();
        let opts = SearchOptions { regex: true, whole_word: true, ..Default::default() };
        let results = engine.search(&buffer, "foo", opts);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].range.start, 5);
    }

    #[test]
    fn invalid_regex_yields_no_matches() {
        let buffer = TextBuffer::with_text("hello");
        let engine = SearchEngine::new();
        let opts = SearchOptions { regex: true, ..Default::default() };
        let results = engine.search(&buffer, "(unclosed", opts);
        assert!(results.is_empty());
    }

    // --- case sensitivity ---

    #[test]
    fn case_insensitive_by_default() {
        let buffer = TextBuffer::with_text("Hello HELLO hello");
        let engine = SearchEngine::new();
        let results = engine.search(&buffer, "hello", SearchOptions::default());
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn case_sensitive_matches_exact() {
        let buffer = TextBuffer::with_text("Hello HELLO hello");
        let engine = SearchEngine::new();
        let opts = SearchOptions { case_sensitive: true, ..Default::default() };
        let results = engine.search(&buffer, "hello", opts);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].range.start, 12);
    }

    #[test]
    fn line_and_column_are_reported() {
        let buffer = TextBuffer::with_text("line0\nline1\nfind me");
        let engine = SearchEngine::new();
        let results = engine.search(&buffer, "find", SearchOptions::default());
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].line, 2);
        assert_eq!(results[0].column, 0);
    }

    #[test]
    fn replace_all_with_regex() {
        let mut buffer = TextBuffer::with_text("a1 b2 c3");
        let engine = SearchEngine::new();
        let opts = SearchOptions { regex: true, ..Default::default() };
        let count = engine.replace_all(&mut buffer, r"\d", "#", opts);
        assert_eq!(count, 3);
        assert_eq!(buffer.to_string(), "a# b# c#");
    }
}
