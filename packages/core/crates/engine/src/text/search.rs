use super::buffer::TextBuffer;
use super::selection::SelectionRange;

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

#[derive(Debug, Clone)]
pub struct SearchResultIterator {
    text: String,
    pattern: String,
    options: SearchOptions,
    current_pos: usize,
    finished: bool,
}

impl SearchResultIterator {
    pub fn new(text: String, pattern: String, options: SearchOptions) -> Self {
        Self { text, pattern, options, current_pos: 0, finished: false }
    }
}

impl Iterator for SearchResultIterator {
    type Item = SearchResult;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished || self.pattern.is_empty() {
            return None;
        }

        let text = &self.text[self.current_pos..];
        let found = if self.options.case_sensitive {
            text.find(&self.pattern)
        } else {
            text.to_lowercase().find(&self.pattern.to_lowercase())
        };

        match found {
            Some(offset) => {
                let start = self.current_pos + offset;
                let end = start + self.pattern.len();
                let line = self.text[..start].lines().count();
                let last_newline = self.text[..start].rfind('\n').map_or(0, |p| p + 1);
                let column = start - last_newline;

                self.current_pos = end;

                if self.current_pos >= self.text.len() {
                    self.finished = true;
                }

                Some(SearchResult { range: SelectionRange::new(start, end), line, column })
            }
            None => {
                self.finished = true;
                None
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchEngine {
    #[allow(dead_code)]
    last_pattern: Option<String>,
    #[allow(dead_code)]
    last_options: Option<SearchOptions>,
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
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
        let iterator = SearchResultIterator::new(text, pattern.to_string(), options);
        iterator.collect()
    }

    pub fn search_from(
        &self,
        buffer: &TextBuffer,
        pattern: &str,
        start: usize,
        options: SearchOptions,
    ) -> Option<SearchResult> {
        if pattern.is_empty() {
            return None;
        }

        let text = buffer.to_string();
        if start >= text.len() {
            return None;
        }

        let text_slice = &text[start..];
        let found = if options.case_sensitive {
            text_slice.find(pattern)
        } else {
            text_slice.to_lowercase().find(&pattern.to_lowercase())
        };

        found.map(|offset| {
            let absolute_start = start + offset;
            let absolute_end = absolute_start + pattern.len();
            let line = text[..absolute_start].lines().count();
            let last_newline = text[..absolute_start].rfind('\n').map_or(0, |p| p + 1);
            let column = absolute_start - last_newline;

            SearchResult { range: SelectionRange::new(absolute_start, absolute_end), line, column }
        })
    }

    pub fn search_backward(
        &self,
        buffer: &TextBuffer,
        pattern: &str,
        start: usize,
        options: SearchOptions,
    ) -> Option<SearchResult> {
        if pattern.is_empty() {
            return None;
        }

        let text = buffer.to_string();
        if start == 0 {
            return None;
        }

        let text_slice = &text[..start];
        let found = if options.case_sensitive {
            text_slice.rfind(pattern)
        } else {
            text_slice.to_lowercase().rfind(&pattern.to_lowercase())
        };

        found.map(|offset| {
            let absolute_start = offset;
            let absolute_end = absolute_start + pattern.len();
            let line = text[..absolute_start].lines().count();
            let last_newline = text[..absolute_start].rfind('\n').map_or(0, |p| p + 1);
            let column = absolute_start - last_newline;

            SearchResult { range: SelectionRange::new(absolute_start, absolute_end), line, column }
        })
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
}
