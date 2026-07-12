use crate::tree::color::Color;
use crate::tree::style::Style;
use std::collections::HashMap;

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
        // Keywords: pink/red
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
        // Strings: blue
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
        // Comments: gray, italic
        m.insert(
            "comment".into(),
            Style {
                fg: Some(Color::rgb(139, 148, 158)),
                italic: Some(true),
                ..Style::default()
            },
        );
        // Types: orange
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
        // Functions: purple
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
        // Numbers: blue
        m.insert(
            "number".into(),
            Style {
                fg: Some(Color::rgb(121, 192, 255)),
                ..Style::default()
            },
        );
        // Constants: blue
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
        // Variables: white/default
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
        // Punctuation: gray
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
        // Operators
        m.insert(
            "operator".into(),
            Style {
                fg: Some(Color::rgb(255, 123, 114)),
                ..Style::default()
            },
        );
        // Attributes/properties
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
        // Tags (HTML/JSX): green
        m.insert(
            "tag".into(),
            Style {
                fg: Some(Color::rgb(123, 188, 123)),
                ..Style::default()
            },
        );
        // Labels
        m.insert(
            "label".into(),
            Style {
                fg: Some(Color::rgb(210, 168, 255)),
                ..Style::default()
            },
        );
        // Includes/imports
        m.insert(
            "include".into(),
            Style {
                fg: Some(Color::rgb(255, 123, 114)),
                ..Style::default()
            },
        );
        // Embedded languages
        m.insert(
            "embedded".into(),
            Style {
                fg: Some(Color::rgb(139, 148, 158)),
                italic: Some(true),
                ..Style::default()
            },
        );
        // Markdown-specific groups
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
        // Spell (misspelled words)
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
    ///
    /// Mirrors OpenTUI's `convertThemeToStyles()`: each `ThemeScope` lists
    /// one or more scope names that should share the same visual style.
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
        // Try exact match first
        if let Some(style) = self.mappings.get(capture) {
            return Some(*style);
        }
        // Try parent scope (e.g., "keyword.control" -> "keyword")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_github_dark_has_keywords() {
        let theme = SyntaxTheme::github_dark();
        assert!(theme.get("keyword").is_some());
        assert!(theme.get("keyword.control").is_some());
    }

    #[test]
    fn theme_falls_back_to_parent_scope() {
        let theme = SyntaxTheme::github_dark();
        // "keyword.control.foo" should fall back to "keyword"
        let style = theme.get("keyword.control.foo");
        assert!(style.is_some());
        assert_eq!(style.unwrap().fg, Some(Color::rgb(255, 123, 114)));
    }

    #[test]
    fn theme_returns_none_for_unknown() {
        let theme = SyntaxTheme::github_dark();
        assert!(theme.get("nonexistent").is_none());
    }

    #[test]
    fn theme_github_light_has_keywords() {
        let theme = SyntaxTheme::github_light();
        assert!(theme.get("keyword").is_some());
        assert_eq!(
            theme.get("keyword").unwrap().fg,
            Some(Color::rgb(215, 58, 73))
        );
    }

    #[test]
    fn theme_with_preset_dark() {
        let theme = SyntaxTheme::with_preset(ThemePreset::GitHubDark);
        assert!(theme.get("keyword").is_some());
    }

    #[test]
    fn theme_with_preset_light() {
        let theme = SyntaxTheme::with_preset(ThemePreset::GitHubLight);
        assert!(theme.get("keyword").is_some());
    }

    #[test]
    fn theme_convert_from_scopes() {
        let scopes = vec![ThemeScope {
            scopes: vec!["keyword".into(), "keyword.control".into()],
            fg: Some(Color::rgb(255, 0, 0)),
            bg: None,
            bold: Some(true),
            italic: None,
            underline: None,
            dim: None,
        }];
        let theme = SyntaxTheme::convert_from_theme(scopes);
        assert_eq!(
            theme.get("keyword").unwrap().fg,
            Some(Color::rgb(255, 0, 0))
        );
        assert!(theme.get("keyword").unwrap().bold.unwrap());
        assert!(theme.get("keyword.control").is_some());
    }

    #[test]
    fn theme_markdown_groups() {
        let theme = SyntaxTheme::github_dark();
        assert!(theme.get("markup.heading.1").is_some());
        assert!(theme.get("markup.raw.block").is_some());
        assert!(theme.get("markup.link.url").is_some());
        assert!(theme.get("markup.list").is_some());
    }

    #[test]
    fn merge_child_overrides_parent() {
        let parent = Style {
            fg: Some(Color::rgb(255, 0, 0)),
            bold: Some(true),
            ..Style::default()
        };
        let child = Style {
            fg: Some(Color::rgb(0, 255, 0)),
            ..Style::default()
        };
        let merged = SyntaxTheme::merge(&parent, &child);
        assert_eq!(merged.fg, Some(Color::rgb(0, 255, 0)));
        assert_eq!(merged.bold, Some(true));
    }
}
