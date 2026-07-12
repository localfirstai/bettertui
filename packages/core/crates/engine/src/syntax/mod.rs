/// Syntax highlighting module using tree-sitter.
///
/// Provides a `SyntaxHighlighter` that can parse and colorize source code
/// in multiple languages. Used by `CodeWidget` and `MarkdownRenderer`.
use std::sync::{Mutex, OnceLock};

pub mod highlighter;
pub mod segment;
pub mod theme;

pub use highlighter::SyntaxHighlighter;
pub use segment::{HighlightedLine, StyledSegment};
pub use theme::SyntaxTheme;

/// Global lazy-initialized syntax highlighter instance.
/// Shared across CodeWidget, MarkdownRenderer, and native bindings.
pub fn global_highlighter() -> &'static Mutex<SyntaxHighlighter> {
    static HIGHLIGHTER: OnceLock<Mutex<SyntaxHighlighter>> = OnceLock::new();
    HIGHLIGHTER.get_or_init(|| Mutex::new(SyntaxHighlighter::new()))
}
