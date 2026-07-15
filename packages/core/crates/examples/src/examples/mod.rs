//! Example demonstrations organized by category.
//!
//! Each module contains focused examples for a specific aspect of BetterTUI.

pub mod engine;
pub mod layout;
pub mod post_process;
pub mod styling;
pub mod syntax;
pub mod terminal;
pub mod text;
use bettertui_terminal::Terminal;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Engine,
    Layout,
    Styling,
    Text,

    Effects,
    Terminal,
}

impl Category {
    pub fn label(self) -> &'static str {
        match self {
            Category::Engine => "ENGINE",
            Category::Layout => "LAYOUT",
            Category::Styling => "STYLING",
            Category::Text => "TEXT",
            Category::Effects => "EFFECTS",
            Category::Terminal => "TERMINAL",
        }
    }
}

pub struct Example {
    pub name: &'static str,
    pub description: &'static str,
    pub category: Category,
    pub run: fn(&mut Terminal) -> io::Result<()>,
}

pub fn all() -> Vec<Example> {
    vec![
        Example {
            name: "Engine",
            description: "Command protocol, tree building, validation, ANSI rendering",
            category: Category::Engine,
            run: engine::run,
        },
        Example {
            name: "Layout",
            description: "Flexbox column/row layouts with nested containers",
            category: Category::Layout,
            run: layout::run,
        },
        Example {
            name: "Styling",
            description: "Named colors, RGB true color, bold/italic/underline",
            category: Category::Styling,
            run: styling::run,
        },
        Example {
            name: "Text Engine",
            description: "Buffer editing, cursor movement, multi-line, search, unicode",
            category: Category::Text,
            run: text::run,
        },
        Example {
            name: "Syntax Highlighting",
            description: "Tree-sitter highlighting for Rust, TypeScript, Python",
            category: Category::Text,
            run: syntax::run,
        },
        Example {
            name: "Post-Processing",
            description: "Scanlines, color matrix, vignette render effects pipeline",
            category: Category::Effects,
            run: post_process::run,
        },
        Example {
            name: "Terminal",
            description: "Raw mode, alternate screen, event polling, border drawing",
            category: Category::Terminal,
            run: terminal::run,
        },
    ]
}
