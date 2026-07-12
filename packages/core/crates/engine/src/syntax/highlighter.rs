use std::cmp::Ordering;
use std::collections::HashMap;

use tree_sitter::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor};

use crate::tree::style::Style;

use super::segment::{HighlightedLine, StyledSegment};
use super::theme::SyntaxTheme;

/// A language grammar with its highlight query.
struct Grammar {
    language: Language,
    query: Query,
}

/// Metadata for a single tree-sitter capture used during highlight resolution.
struct CaptureMeta {
    name: String,
    start: usize,
    end: usize,
    capture_index: usize,
    conceal: Option<String>,
}

/// Syntax highlighter using tree-sitter to parse and highlight source code.
///
/// Supports multiple languages with lazy-loaded parsers. Each language
/// grammar is compiled into the binary as a Rust crate dependency.
pub struct SyntaxHighlighter {
    grammars: HashMap<String, Grammar>,
    parsers: HashMap<String, Parser>,
    theme: SyntaxTheme,
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        let mut sh = Self {
            grammars: HashMap::new(),
            parsers: HashMap::new(),
            theme: SyntaxTheme::default(),
        };
        sh.register_builtin_languages();
        sh
    }

    fn register_builtin_languages(&mut self) {
        self.register_language(
            "javascript",
            tree_sitter_javascript::LANGUAGE.into(),
            tree_sitter_javascript::HIGHLIGHT_QUERY,
        );
        self.register_language(
            "js",
            tree_sitter_javascript::LANGUAGE.into(),
            tree_sitter_javascript::HIGHLIGHT_QUERY,
        );
        self.register_language(
            "javascriptreact",
            tree_sitter_javascript::LANGUAGE.into(),
            tree_sitter_javascript::HIGHLIGHT_QUERY,
        );
        self.register_language(
            "typescript",
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            tree_sitter_typescript::HIGHLIGHTS_QUERY,
        );
        self.register_language(
            "ts",
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            tree_sitter_typescript::HIGHLIGHTS_QUERY,
        );
        self.register_language(
            "tsx",
            tree_sitter_typescript::LANGUAGE_TSX.into(),
            tree_sitter_typescript::HIGHLIGHTS_QUERY,
        );
        self.register_language(
            "typescriptreact",
            tree_sitter_typescript::LANGUAGE_TSX.into(),
            tree_sitter_typescript::HIGHLIGHTS_QUERY,
        );
        self.register_language(
            "rust",
            tree_sitter_rust::LANGUAGE.into(),
            tree_sitter_rust::HIGHLIGHTS_QUERY,
        );
        self.register_language(
            "rs",
            tree_sitter_rust::LANGUAGE.into(),
            tree_sitter_rust::HIGHLIGHTS_QUERY,
        );
        self.register_language(
            "python",
            tree_sitter_python::LANGUAGE.into(),
            tree_sitter_python::HIGHLIGHTS_QUERY,
        );
        self.register_language(
            "py",
            tree_sitter_python::LANGUAGE.into(),
            tree_sitter_python::HIGHLIGHTS_QUERY,
        );
        self.register_language(
            "json",
            tree_sitter_json::LANGUAGE.into(),
            tree_sitter_json::HIGHLIGHTS_QUERY,
        );
        self.register_language(
            "html",
            tree_sitter_html::LANGUAGE.into(),
            tree_sitter_html::HIGHLIGHTS_QUERY,
        );
        self.register_language(
            "css",
            tree_sitter_css::LANGUAGE.into(),
            tree_sitter_css::HIGHLIGHTS_QUERY,
        );
        self.register_language(
            "bash",
            tree_sitter_bash::LANGUAGE.into(),
            tree_sitter_bash::HIGHLIGHT_QUERY,
        );
        self.register_language(
            "sh",
            tree_sitter_bash::LANGUAGE.into(),
            tree_sitter_bash::HIGHLIGHT_QUERY,
        );
        self.register_language(
            "shell",
            tree_sitter_bash::LANGUAGE.into(),
            tree_sitter_bash::HIGHLIGHT_QUERY,
        );
    }

    pub fn register_language(&mut self, name: &str, language: Language, query_source: &str) {
        if let Ok(query) = Query::new(&language, query_source) {
            self.grammars
                .insert(name.to_string(), Grammar { language, query });
        }
    }

    pub fn has_language(&self, language: &str) -> bool {
        self.grammars.contains_key(language)
    }

    /// Resolve a file extension or info string to a supported language key.
    /// Returns the matched language name or None.
    pub fn resolve_language(&self, input: &str) -> Option<&str> {
        let lower = input
            .split_whitespace()
            .next()
            .unwrap_or(input)
            .to_lowercase();

        // Direct match
        if self.grammars.contains_key(&lower) {
            return self
                .grammars
                .keys()
                .find(|k| *k == &lower)
                .map(|s| s.as_str());
        }

        // Extension-based lookup
        match lower.as_str() {
            "cjs" | "mjs" | "jsx" => Some("javascript"),
            "cts" | "mts" => Some("ts"),
            "tsx" => Some("tsx"),
            "pyi" => Some("python"),
            _ => None,
        }
    }

    fn default_text_style() -> Style {
        Style {
            fg: Some(crate::tree::color::Color::rgb(230, 237, 243)),
            ..Style::default()
        }
    }

    // ── Core highlight algorithm ──────────────────────────────────────────

    /// Number of dot-separated components in a capture name (higher = more specific).
    fn specificity(name: &str) -> usize {
        name.chars().filter(|&c| c == '.').count()
    }

    /// Resolve the cascaded style for a set of active captures.
    ///
    /// Follows OpenTUI's cascade: sort active captures by specificity ascending,
    /// then merge styles in order (more specific overrides less specific).
    /// Also handles conceal metadata.
    fn resolve_cascade(
        active: &[&CaptureMeta],
        theme: &SyntaxTheme,
        default_style: &Style,
    ) -> ResolvedStyle {
        // Check for conceal first — highest priority
        for cm in active {
            if cm.conceal.is_some() {
                return ResolvedStyle::Concealed(cm.conceal.clone().unwrap_or_default());
            }
        }

        if active.is_empty() {
            return ResolvedStyle::Styled(*default_style);
        }

        // Sort by specificity (ascending), then by capture index (ascending)
        let mut sorted: Vec<&&CaptureMeta> = active.iter().collect();
        sorted.sort_by(|a, b| {
            let s = Self::specificity(&a.name).cmp(&Self::specificity(&b.name));
            if s != Ordering::Equal {
                return s;
            }
            a.capture_index.cmp(&b.capture_index)
        });

        let mut merged = *default_style;
        for cm in sorted {
            if let Some(style) = theme.get(&cm.name) {
                merged = SyntaxTheme::merge(&merged, &style);
            }
        }

        ResolvedStyle::Styled(merged)
    }

    pub fn highlight(&mut self, code: &str, language: &str) -> Option<Vec<HighlightedLine>> {
        let resolved = info_string_to_filetype(language).unwrap_or(language);
        let grammar = self.grammars.get(resolved)?;

        let parser = self.parsers.entry(resolved.to_string()).or_insert_with(|| {
            let mut p = Parser::new();
            let _ = p.set_language(&grammar.language);
            p
        });

        let tree = parser.parse(code, None)?;
        let root = tree.root_node();
        let code_bytes = code.as_bytes();

        // ── 1. Collect captures with metadata ──────────────────────────────

        let mut cursor = QueryCursor::new();
        let mut query_matches = cursor.matches(&grammar.query, root, code_bytes);

        let mut captures: Vec<CaptureMeta> = Vec::new();
        while let Some(match_) = query_matches.next() {
            let settings = grammar.query.property_settings(match_.pattern_index);
            let mut conceal: Option<String> = None;

            for setting in settings {
                if setting.key.as_ref() == "conceal" {
                    conceal = setting.value.as_ref().map(|s| s.to_string());
                }
            }

            for capture in match_.captures {
                let name = grammar.query.capture_names()[capture.index as usize].to_string();
                let range = capture.node.byte_range();
                captures.push(CaptureMeta {
                    name,
                    start: range.start,
                    end: range.end,
                    capture_index: capture.index as usize,
                    conceal: conceal.clone(),
                });
            }
        }

        // ── 2. Build boundary events ──────────────────────────────────────

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum BoundaryType {
            End,
            Start,
        }

        #[derive(Debug)]
        struct Boundary {
            offset: usize,
            btype: BoundaryType,
            capture_index: usize,
        }

        let mut boundaries: Vec<Boundary> = captures
            .iter()
            .enumerate()
            .flat_map(|(i, cap)| {
                vec![
                    Boundary {
                        offset: cap.start,
                        btype: BoundaryType::Start,
                        capture_index: i,
                    },
                    Boundary {
                        offset: cap.end,
                        btype: BoundaryType::End,
                        capture_index: i,
                    },
                ]
            })
            .collect();

        // Sort: offset ascending, End before Start at same offset
        boundaries.sort_by(|a, b| {
            a.offset
                .cmp(&b.offset)
                .then_with(|| match (a.btype, b.btype) {
                    (BoundaryType::End, BoundaryType::Start) => Ordering::Less,
                    (BoundaryType::Start, BoundaryType::End) => Ordering::Greater,
                    _ => Ordering::Equal,
                })
        });

        // ── 3. Sweep-line ──────────────────────────────────────────────────

        let default_style = Self::default_text_style();
        let mut segments: Vec<StyledSegment> = Vec::new();
        let mut active: Vec<usize> = Vec::new(); // indices into captures[]
        let mut pos = 0;

        for boundary in &boundaries {
            if boundary.offset > pos {
                // Resolve style from active captures
                let active_refs: Vec<&CaptureMeta> = active.iter().map(|&i| &captures[i]).collect();

                match Self::resolve_cascade(&active_refs, &self.theme, &default_style) {
                    ResolvedStyle::Styled(style) => {
                        let text = &code[pos..boundary.offset];
                        if !text.is_empty() {
                            segments.push(StyledSegment::new(text, style));
                        }
                    }
                    ResolvedStyle::Concealed(replacement) => {
                        if !replacement.is_empty() {
                            segments.push(StyledSegment::new(&replacement, default_style));
                        }
                        // Empty replacement = skip text entirely
                    }
                }
            }

            // Update active set
            match boundary.btype {
                BoundaryType::Start => active.push(boundary.capture_index),
                BoundaryType::End => {
                    active.retain(|&i| i != boundary.capture_index);
                }
            }

            if boundary.offset > pos {
                pos = boundary.offset;
            }
        }

        // ── 4. Handle remaining text ───────────────────────────────────────

        if pos < code.len() {
            let text = &code[pos..];
            if !text.is_empty() {
                segments.push(StyledSegment::new(text, default_style));
            }
        }

        // ── 5. Merge and split into lines ──────────────────────────────────

        segments = Self::merge_adjacent_same_style(segments);
        Some(Self::split_into_lines(segments, &default_style))
    }

    fn merge_adjacent_same_style(segments: Vec<StyledSegment>) -> Vec<StyledSegment> {
        let mut merged: Vec<StyledSegment> = Vec::new();
        for seg in segments {
            if let Some(last) = merged.last_mut()
                && last.style == seg.style
            {
                last.text.push_str(&seg.text);
                continue;
            }
            merged.push(seg);
        }
        merged
    }

    fn split_into_lines(
        segments: Vec<StyledSegment>,
        default_style: &Style,
    ) -> Vec<HighlightedLine> {
        let mut lines: Vec<HighlightedLine> = Vec::new();
        let mut current_segments: Vec<StyledSegment> = Vec::new();
        let mut current_line_text = String::new();
        let mut ends_with_newline = false;

        for seg in segments {
            for ch in seg.text.chars() {
                if ch == '\n' {
                    ends_with_newline = true;
                    if current_segments.is_empty() {
                        current_segments.push(StyledSegment::new(
                            current_line_text.clone(),
                            *default_style,
                        ));
                    }
                    lines.push(HighlightedLine::new(std::mem::take(&mut current_segments)));
                    current_line_text.clear();
                } else {
                    if let Some(last) = current_segments.last_mut()
                        && last.style == seg.style
                    {
                        last.text.push(ch);
                        current_line_text.push(ch);
                        continue;
                    }
                    let mut s = StyledSegment::new(String::new(), seg.style);
                    s.text.push(ch);
                    current_segments.push(s);
                    current_line_text.push(ch);
                }
            }
        }

        if ends_with_newline
            || !current_segments.is_empty()
            || !current_line_text.is_empty()
            || lines.is_empty()
        {
            if current_segments.is_empty() {
                current_segments.push(StyledSegment::new(
                    current_line_text.clone(),
                    *default_style,
                ));
            }
            lines.push(HighlightedLine::new(current_segments));
        }

        lines
    }
}

/// Result of cascading style resolution for a set of active captures.
enum ResolvedStyle {
    Styled(Style),
    Concealed(String),
}

// ── Language resolution utilities ──────────────────────────────────────────

/// Resolve a Markdown code fence info string to a language key.
///
/// Mirrors OpenTUI's `infoStringToFiletype()`: takes the first token,
/// normalizes, and maps common variants.
pub fn info_string_to_filetype(info_string: &str) -> Option<&'static str> {
    let token = info_string.split_whitespace().next()?;
    if token.is_empty() {
        return None;
    }
    let lower = token.to_lowercase();
    match lower.as_str() {
        // JavaScript
        "javascript" | "ecmascript" | "es" => Some("javascript"),
        "js" | "cjs" | "mjs" | "jsx" => Some("javascript"),
        "javascriptreact" => Some("javascriptreact"),
        // TypeScript
        "typescript" => Some("typescript"),
        "ts" | "cts" | "mts" => Some("ts"),
        "tsx" | "typescriptreact" => Some("tsx"),
        // Rust
        "rust" | "rs" => Some("rust"),
        // Python
        "python" | "py" | "pyi" => Some("python"),
        // JSON
        "json" | "jsonc" | "geojson" => Some("json"),
        // HTML
        "html" | "htm" | "xhtml" => Some("html"),
        // CSS
        "css" | "scss" | "less" => Some("css"),
        // Bash/Shell
        "bash" | "sh" | "shell" | "zsh" | "ksh" => Some("bash"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlighter_new() {
        let sh = SyntaxHighlighter::new();
        assert!(sh.has_language("javascript"));
        assert!(sh.has_language("rust"));
        assert!(sh.has_language("python"));
        assert!(sh.has_language("json"));
        assert!(sh.has_language("html"));
        assert!(sh.has_language("css"));
    }

    #[test]
    fn has_language_unknown() {
        let sh = SyntaxHighlighter::new();
        assert!(!sh.has_language("foobar"));
    }

    #[test]
    fn highlight_rust_code() {
        let mut sh = SyntaxHighlighter::new();
        let code = "fn main() {\n    println!(\"hello\");\n}\n";
        let result = sh.highlight(code, "rust");
        assert!(result.is_some());
        let lines = result.unwrap();
        assert_eq!(lines.len(), 4);
        let first_line = &lines[0];
        assert!(!first_line.segments.is_empty());
    }

    #[test]
    fn highlight_javascript_code() {
        let mut sh = SyntaxHighlighter::new();
        let code = "const x = 42;\n";
        let result = sh.highlight(code, "javascript");
        assert!(result.is_some());
        let lines = result.unwrap();
        assert!(!lines.is_empty());
    }

    #[test]
    fn highlight_unknown_language() {
        let mut sh = SyntaxHighlighter::new();
        let result = sh.highlight("code", "unknown_lang");
        assert!(result.is_none());
    }

    #[test]
    fn highlight_empty_code() {
        let mut sh = SyntaxHighlighter::new();
        let result = sh.highlight("", "javascript");
        assert!(result.is_some());
        let lines = result.unwrap();
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].text(), "");
    }

    #[test]
    fn highlight_python_code() {
        let mut sh = SyntaxHighlighter::new();
        let code = "def hello():\n    print('world')\n";
        let result = sh.highlight(code, "python");
        assert!(result.is_some());
        let lines = result.unwrap();
        assert!(!lines.is_empty());
    }

    #[test]
    fn highlight_json() {
        let mut sh = SyntaxHighlighter::new();
        let code = "{\"key\": \"value\"}\n";
        let result = sh.highlight(code, "json");
        assert!(result.is_some());
        let lines = result.unwrap();
        assert!(!lines.is_empty());
    }

    #[test]
    fn specificity_counts_dots() {
        assert_eq!(SyntaxHighlighter::specificity("keyword"), 0);
        assert_eq!(SyntaxHighlighter::specificity("keyword.control"), 1);
        assert_eq!(SyntaxHighlighter::specificity("markup.heading.1"), 2);
    }

    #[test]
    fn info_string_js() {
        assert_eq!(info_string_to_filetype("javascript"), Some("javascript"));
        assert_eq!(info_string_to_filetype("js"), Some("javascript"));
        assert_eq!(info_string_to_filetype("jsx"), Some("javascript"));
    }

    #[test]
    fn info_string_rust() {
        assert_eq!(info_string_to_filetype("rust"), Some("rust"));
        assert_eq!(info_string_to_filetype("rs"), Some("rust"));
    }

    #[test]
    fn info_string_unknown() {
        assert_eq!(info_string_to_filetype("foobar"), None);
    }

    #[test]
    fn info_string_empty() {
        assert_eq!(info_string_to_filetype(""), None);
    }

    #[test]
    fn info_string_with_args() {
        assert_eq!(info_string_to_filetype("rust ignore me"), Some("rust"));
    }

    #[test]
    fn highlight_resolves_language_alias() {
        let mut sh = SyntaxHighlighter::new();
        // "jsx" is not a registered grammar key but info_string_to_filetype maps it to "javascript"
        let result = sh.highlight("const x = 1;", "jsx");
        assert!(result.is_some(), "jsx should resolve to javascript");

        // "zsh" maps to "bash"
        let result = sh.highlight("echo hello", "zsh");
        assert!(result.is_some(), "zsh should resolve to bash");

        // "scss" maps to "css"
        let result = sh.highlight(".a { color: red; }", "scss");
        assert!(result.is_some(), "scss should resolve to css");
    }
}
