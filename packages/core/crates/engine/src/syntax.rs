use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use tree_sitter::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor};

use crate::tree::{Color, Style};

// ============================================================================
// Public Exports
// ============================================================================

pub use segment::{HighlightedLine, StyledSegment};
pub use syntax_highlighter::SyntaxHighlighter;
pub use theme::{SyntaxTheme, ThemePreset, ThemeScope};

mod syntax_highlighter {
    use super::*;

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

        pub fn resolve_language(&self, input: &str) -> Option<&str> {
            let lower = input
                .split_whitespace()
                .next()
                .unwrap_or(input)
                .to_lowercase();

            if self.grammars.contains_key(&lower) {
                return self
                    .grammars
                    .keys()
                    .find(|k| *k == &lower)
                    .map(|s| s.as_str());
            }

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
                fg: Some(crate::tree::Color::rgb(230, 237, 243)),
                ..Style::default()
            }
        }

        pub fn specificity(name: &str) -> usize {
            name.chars().filter(|&c| c == '.').count()
        }

        fn resolve_cascade(
            active: &[&CaptureMeta],
            theme: &SyntaxTheme,
            default_style: &Style,
        ) -> ResolvedStyle {
            for cm in active {
                if cm.conceal.is_some() {
                    return ResolvedStyle::Concealed(cm.conceal.clone().unwrap_or_default());
                }
            }

            if active.is_empty() {
                return ResolvedStyle::Styled(*default_style);
            }

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

            boundaries.sort_by(|a, b| {
                a.offset
                    .cmp(&b.offset)
                    .then_with(|| match (a.btype, b.btype) {
                        (BoundaryType::End, BoundaryType::Start) => Ordering::Less,
                        (BoundaryType::Start, BoundaryType::End) => Ordering::Greater,
                        _ => Ordering::Equal,
                    })
            });

            let default_style = Self::default_text_style();
            let mut segments: Vec<StyledSegment> = Vec::new();
            let mut active: Vec<usize> = Vec::new();
            let mut pos = 0;

            for boundary in &boundaries {
                if boundary.offset > pos {
                    let active_refs: Vec<&CaptureMeta> =
                        active.iter().map(|&i| &captures[i]).collect();

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
                        }
                    }
                }

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

            if pos < code.len() {
                let text = &code[pos..];
                if !text.is_empty() {
                    segments.push(StyledSegment::new(text, default_style));
                }
            }

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
}

mod segment {
    use super::*;

    /// A single highlighted segment of code with its visual style.
    #[derive(Debug, Clone)]
    pub struct StyledSegment {
        pub text: String,
        pub style: Style,
    }

    impl StyledSegment {
        pub fn new(text: impl Into<String>, style: Style) -> Self {
            Self {
                text: text.into(),
                style,
            }
        }
    }

    /// A line of code composed of potentially multiple styled segments.
    #[derive(Debug, Clone)]
    pub struct HighlightedLine {
        pub segments: Vec<StyledSegment>,
    }

    impl HighlightedLine {
        pub fn new(segments: Vec<StyledSegment>) -> Self {
            Self { segments }
        }

        pub fn plain(text: impl Into<String>, style: Style) -> Self {
            Self {
                segments: vec![StyledSegment::new(text, style)],
            }
        }

        pub fn text(&self) -> String {
            self.segments.iter().map(|s| s.text.as_str()).collect()
        }
    }
}

mod theme {
    use super::*;

    /// A single scope-to-style mapping, matching OpenTUI's `ThemeTokenStyle`.
    pub struct ThemeScope {
        pub scopes: Vec<String>,
        pub fg: Option<Color>,
        pub bg: Option<Color>,
        pub bold: Option<bool>,
        pub italic: Option<bool>,
        pub underline: Option<bool>,
        pub dim: Option<bool>,
    }

    /// Built-in theme presets.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ThemePreset {
        GitHubDark,
        GitHubLight,
    }

    /// Maps tree-sitter capture names to terminal styles.
    ///
    /// Uses a GitHub Dark-inspired theme by default.
    /// Supports theme bridging via `convert_from_theme()`.
    pub struct SyntaxTheme {
        mappings: HashMap<String, Style>,
    }

    impl Default for SyntaxTheme {
        fn default() -> Self {
            Self::github_dark()
        }
    }

    impl SyntaxTheme {
        /// GitHub Dark theme colors
        pub fn github_dark() -> Self {
            let mut m = HashMap::new();
            m.insert(
                "keyword".into(),
                Style {
                    fg: Some(Color::rgb(255, 123, 114)),
                    bold: Some(true),
                    ..Style::default()
                },
            );
            m.insert(
                "keyword.operator".into(),
                Style {
                    fg: Some(Color::rgb(255, 123, 114)),
                    ..Style::default()
                },
            );
            m.insert(
                "keyword.control".into(),
                Style {
                    fg: Some(Color::rgb(255, 123, 114)),
                    bold: Some(true),
                    ..Style::default()
                },
            );
            m.insert(
                "string".into(),
                Style {
                    fg: Some(Color::rgb(165, 214, 255)),
                    ..Style::default()
                },
            );
            m.insert(
                "string.special".into(),
                Style {
                    fg: Some(Color::rgb(165, 214, 255)),
                    italic: Some(true),
                    ..Style::default()
                },
            );
            m.insert(
                "comment".into(),
                Style {
                    fg: Some(Color::rgb(139, 148, 158)),
                    italic: Some(true),
                    ..Style::default()
                },
            );
            m.insert(
                "type".into(),
                Style {
                    fg: Some(Color::rgb(255, 166, 87)),
                    ..Style::default()
                },
            );
            m.insert(
                "type.builtin".into(),
                Style {
                    fg: Some(Color::rgb(255, 166, 87)),
                    bold: Some(true),
                    ..Style::default()
                },
            );
            m.insert(
                "function".into(),
                Style {
                    fg: Some(Color::rgb(210, 168, 255)),
                    ..Style::default()
                },
            );
            m.insert(
                "function.method".into(),
                Style {
                    fg: Some(Color::rgb(210, 168, 255)),
                    ..Style::default()
                },
            );
            m.insert(
                "function.builtin".into(),
                Style {
                    fg: Some(Color::rgb(210, 168, 255)),
                    bold: Some(true),
                    ..Style::default()
                },
            );
            m.insert(
                "number".into(),
                Style {
                    fg: Some(Color::rgb(121, 192, 255)),
                    ..Style::default()
                },
            );
            m.insert(
                "constant".into(),
                Style {
                    fg: Some(Color::rgb(121, 192, 255)),
                    ..Style::default()
                },
            );
            m.insert(
                "constant.builtin".into(),
                Style {
                    fg: Some(Color::rgb(121, 192, 255)),
                    bold: Some(true),
                    ..Style::default()
                },
            );
            m.insert(
                "variable".into(),
                Style {
                    fg: Some(Color::rgb(230, 237, 243)),
                    ..Style::default()
                },
            );
            m.insert(
                "variable.parameter".into(),
                Style {
                    fg: Some(Color::rgb(230, 237, 243)),
                    italic: Some(true),
                    ..Style::default()
                },
            );
            m.insert(
                "punctuation".into(),
                Style {
                    fg: Some(Color::rgb(139, 148, 158)),
                    ..Style::default()
                },
            );
            m.insert(
                "punctuation.delimiter".into(),
                Style {
                    fg: Some(Color::rgb(139, 148, 158)),
                    ..Style::default()
                },
            );
            m.insert(
                "punctuation.bracket".into(),
                Style {
                    fg: Some(Color::rgb(139, 148, 158)),
                    ..Style::default()
                },
            );
            m.insert(
                "operator".into(),
                Style {
                    fg: Some(Color::rgb(255, 123, 114)),
                    ..Style::default()
                },
            );
            m.insert(
                "attribute".into(),
                Style {
                    fg: Some(Color::rgb(255, 166, 87)),
                    ..Style::default()
                },
            );
            m.insert(
                "property".into(),
                Style {
                    fg: Some(Color::rgb(121, 192, 255)),
                    ..Style::default()
                },
            );
            m.insert(
                "tag".into(),
                Style {
                    fg: Some(Color::rgb(123, 188, 123)),
                    ..Style::default()
                },
            );
            m.insert(
                "label".into(),
                Style {
                    fg: Some(Color::rgb(210, 168, 255)),
                    ..Style::default()
                },
            );
            m.insert(
                "include".into(),
                Style {
                    fg: Some(Color::rgb(255, 123, 114)),
                    ..Style::default()
                },
            );
            m.insert(
                "embedded".into(),
                Style {
                    fg: Some(Color::rgb(139, 148, 158)),
                    italic: Some(true),
                    ..Style::default()
                },
            );
            m.insert(
                "markup.heading.1".into(),
                Style {
                    fg: Some(Color::rgb(210, 168, 255)),
                    bold: Some(true),
                    ..Style::default()
                },
            );
            m.insert(
                "markup.heading.2".into(),
                Style {
                    fg: Some(Color::rgb(210, 168, 255)),
                    bold: Some(true),
                    ..Style::default()
                },
            );
            m.insert(
                "markup.heading".into(),
                Style {
                    fg: Some(Color::rgb(210, 168, 255)),
                    bold: Some(true),
                    ..Style::default()
                },
            );
            m.insert(
                "markup.raw.block".into(),
                Style {
                    fg: Some(Color::rgb(165, 214, 255)),
                    ..Style::default()
                },
            );
            m.insert(
                "markup.link.url".into(),
                Style {
                    fg: Some(Color::rgb(165, 214, 255)),
                    underline: Some(true),
                    ..Style::default()
                },
            );
            m.insert(
                "markup.link.label".into(),
                Style {
                    fg: Some(Color::rgb(139, 148, 158)),
                    italic: Some(true),
                    ..Style::default()
                },
            );
            m.insert(
                "markup.list".into(),
                Style {
                    fg: Some(Color::rgb(255, 166, 87)),
                    ..Style::default()
                },
            );
            m.insert(
                "markup.quote".into(),
                Style {
                    fg: Some(Color::rgb(139, 148, 158)),
                    italic: Some(true),
                    ..Style::default()
                },
            );
            m.insert(
                "markup.list.unchecked".into(),
                Style {
                    fg: Some(Color::rgb(139, 148, 158)),
                    ..Style::default()
                },
            );
            m.insert(
                "markup.list.checked".into(),
                Style {
                    fg: Some(Color::rgb(123, 188, 123)),
                    ..Style::default()
                },
            );
            m.insert(
                "spell".into(),
                Style {
                    fg: Some(Color::rgb(230, 237, 243)),
                    underline: Some(true),
                    ..Style::default()
                },
            );
            Self { mappings: m }
        }

        /// Apply a theme preset.
        pub fn with_preset(preset: ThemePreset) -> Self {
            match preset {
                ThemePreset::GitHubDark => Self::github_dark(),
                ThemePreset::GitHubLight => Self::github_light(),
            }
        }

        /// Convert a list of theme scopes into the internal mapping.
        pub fn convert_from_theme(scopes: Vec<ThemeScope>) -> Self {
            let mut m = HashMap::new();
            for scope in scopes {
                let style = Style {
                    fg: scope.fg,
                    bg: scope.bg,
                    bold: scope.bold,
                    italic: scope.italic,
                    underline: scope.underline,
                    dim: scope.dim,
                    ..Style::default()
                };
                for name in scope.scopes {
                    m.insert(name, style);
                }
            }
            Self { mappings: m }
        }

        /// Get the style for a capture name, with fallback to parent scope.
        pub fn get(&self, capture: &str) -> Option<Style> {
            if let Some(style) = self.mappings.get(capture) {
                return Some(*style);
            }
            if let Some(dot_pos) = capture.rfind('.') {
                let parent = &capture[..dot_pos];
                if let Some(style) = self.mappings.get(parent) {
                    return Some(*style);
                }
            }
            None
        }

        /// GitHub Light theme (lighter colors for light backgrounds).
        pub fn github_light() -> Self {
            let mut m = HashMap::new();
            m.insert(
                "keyword".into(),
                Style {
                    fg: Some(Color::rgb(215, 58, 73)),
                    bold: Some(true),
                    ..Style::default()
                },
            );
            m.insert(
                "string".into(),
                Style {
                    fg: Some(Color::rgb(3, 47, 98)),
                    ..Style::default()
                },
            );
            m.insert(
                "comment".into(),
                Style {
                    fg: Some(Color::rgb(106, 115, 125)),
                    italic: Some(true),
                    ..Style::default()
                },
            );
            m.insert(
                "type".into(),
                Style {
                    fg: Some(Color::rgb(109, 66, 0)),
                    ..Style::default()
                },
            );
            m.insert(
                "function".into(),
                Style {
                    fg: Some(Color::rgb(111, 66, 193)),
                    ..Style::default()
                },
            );
            m.insert(
                "number".into(),
                Style {
                    fg: Some(Color::rgb(0, 92, 197)),
                    ..Style::default()
                },
            );
            m.insert(
                "variable".into(),
                Style {
                    fg: Some(Color::rgb(36, 41, 46)),
                    ..Style::default()
                },
            );
            m.insert(
                "punctuation".into(),
                Style {
                    fg: Some(Color::rgb(149, 157, 165)),
                    ..Style::default()
                },
            );
            m.insert(
                "tag".into(),
                Style {
                    fg: Some(Color::rgb(34, 134, 58)),
                    ..Style::default()
                },
            );
            Self { mappings: m }
        }

        /// Merge two styles (child overrides parent properties).
        pub fn merge(parent: &Style, child: &Style) -> Style {
            Style {
                fg: child.fg.or(parent.fg),
                bg: child.bg.or(parent.bg),
                bold: child.bold.or(parent.bold),
                italic: child.italic.or(parent.italic),
                underline: child.underline.or(parent.underline),
                dim: child.dim.or(parent.dim),
                strikethrough: child.strikethrough.or(parent.strikethrough),
                inverse: child.inverse.or(parent.inverse),
                ..Style::default()
            }
        }
    }
}

// ============================================================================
// Language Resolution Utilities
// ============================================================================

/// Resolve a Markdown code fence info string to a language key.
pub fn info_string_to_filetype(info_string: &str) -> Option<&'static str> {
    let token = info_string.split_whitespace().next()?;
    if token.is_empty() {
        return None;
    }
    let lower = token.to_lowercase();
    match lower.as_str() {
        "javascript" | "ecmascript" | "es" => Some("javascript"),
        "js" | "cjs" | "mjs" | "jsx" => Some("javascript"),
        "javascriptreact" => Some("javascriptreact"),
        "typescript" => Some("typescript"),
        "ts" | "cts" | "mts" => Some("ts"),
        "tsx" | "typescriptreact" => Some("tsx"),
        "rust" | "rs" => Some("rust"),
        "python" | "py" | "pyi" => Some("python"),
        "json" | "jsonc" | "geojson" => Some("json"),
        "html" | "htm" | "xhtml" => Some("html"),
        "css" | "scss" | "less" => Some("css"),
        "bash" | "sh" | "shell" | "zsh" | "ksh" => Some("bash"),
        _ => None,
    }
}

// ============================================================================
// Global Highlighter Instance
// ============================================================================

/// Global lazy-initialized syntax highlighter instance.
pub fn global_highlighter() -> &'static Mutex<SyntaxHighlighter> {
    static HIGHLIGHTER: OnceLock<Mutex<SyntaxHighlighter>> = OnceLock::new();
    HIGHLIGHTER.get_or_init(|| Mutex::new(SyntaxHighlighter::new()))
}
