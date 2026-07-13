use bettertui_engine::syntax::*;
use bettertui_engine::tree::{Color, Style};

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
    let result = sh.highlight("const x = 1;", "jsx");
    assert!(result.is_some(), "jsx should resolve to javascript");

    let result = sh.highlight("echo hello", "zsh");
    assert!(result.is_some(), "zsh should resolve to bash");

    let result = sh.highlight(".a { color: red; }", "scss");
    assert!(result.is_some(), "scss should resolve to css");
}

#[test]
fn theme_github_dark_has_keywords() {
    let theme = SyntaxTheme::github_dark();
    assert!(theme.get("keyword").is_some());
    assert!(theme.get("keyword.control").is_some());
}

#[test]
fn theme_falls_back_to_parent_scope() {
    let theme = SyntaxTheme::github_dark();
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
