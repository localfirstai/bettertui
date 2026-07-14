pub mod engine;
pub mod layout;
pub mod styling;
pub mod text;
pub mod widgets;
pub mod syntax;
pub mod post_process;
pub mod terminal;

use bettertui_terminal::Terminal;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Engine,
    Layout,
    Styling,
    Text,
    Widgets,
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
            Category::Widgets => "WIDGETS",
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
        // Engine
        Example {
            name: "Engine",
            description: "Command protocol, tree building, validation, ANSI rendering",
            category: Category::Engine,
            run: engine::run,
        },
        // Layout
        Example {
            name: "Layout",
            description: "Flexbox column/row layouts with nested containers",
            category: Category::Layout,
            run: layout::run,
        },
        // Styling
        Example {
            name: "Styling",
            description: "Named colors, RGB true color, bold/italic/underline",
            category: Category::Styling,
            run: styling::run,
        },
        // Text
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
        // Widgets
        Example {
            name: "Widgets",
            description: "WidgetHost lifecycle: mount, update, unmount with shared state",
            category: Category::Widgets,
            run: widgets::run,
        },
        // Effects
        Example {
            name: "Post-Processing",
            description: "Scanlines, color matrix, vignette render effects pipeline",
            category: Category::Effects,
            run: post_process::run,
        },
        // Terminal
        Example {
            name: "Terminal",
            description: "Raw mode, alternate screen, event polling, border drawing",
            category: Category::Terminal,
            run: terminal::run,
        },
    ]
}


