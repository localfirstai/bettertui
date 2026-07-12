use crate::events::Event;
use crate::events::types::EventResult;
use crate::syntax::global_highlighter;
use crate::tree::color::Color;
use crate::tree::layout::{FlexDirection, LayoutProps};
use crate::tree::style::Style;

use super::{Widget, WidgetContext, WidgetId};

/// Code widget for displaying highlighted code blocks.
///
/// When a `language` is specified, uses tree-sitter to parse and colorize
/// the source code. Falls back to plain text styling if no language is set
/// or highlighting fails.
pub struct CodeWidget {
    pub content: Box<str>,
    pub language: Option<Box<str>>,
    pub inline: bool,
    pub style: Style,
    pub layout: LayoutProps,
}

impl Default for CodeWidget {
    fn default() -> Self {
        Self {
            content: Box::from(""),
            language: None,
            inline: false,
            style: Style {
                fg: Some(Color::Named(crate::tree::NamedColor::Cyan)),
                ..Style::default()
            },
            layout: LayoutProps::default(),
        }
    }
}

impl CodeWidget {
    pub fn new(content: impl Into<Box<str>>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    pub fn inline(content: impl Into<Box<str>>) -> Self {
        Self {
            content: content.into(),
            inline: true,
            ..Default::default()
        }
    }

    pub fn block(content: impl Into<Box<str>>, language: impl Into<Box<str>>) -> Self {
        Self {
            content: content.into(),
            language: Some(language.into()),
            inline: false,
            ..Default::default()
        }
    }

    pub fn with_language(mut self, language: impl Into<Box<str>>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn with_inline(mut self, inline: bool) -> Self {
        self.inline = inline;
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_layout(mut self, layout: LayoutProps) -> Self {
        self.layout = layout;
        self
    }

    /// Create the highlighted render nodes for code content.
    /// Returns the root node ID of the created tree.
    fn create_highlighted(
        &self,
        ctx: &mut WidgetContext,
        code: &str,
        language: &str,
        base_style: Style,
    ) -> WidgetId {
        let lines = {
            let mut hl = global_highlighter().lock().unwrap();
            hl.highlight(code, language)
        };

        if let Some(lines) = lines {
            // Create a Flex column container for the code
            let flex_layout = LayoutProps {
                direction: FlexDirection::Column,
                ..LayoutProps::default()
            };
            let flex_id = ctx.make_flex(flex_layout, base_style);

            for line in &lines {
                let line_text = line.text();
                if line.segments.len() <= 1 {
                    // Single-style line: create one Text node
                    let style = line
                        .segments
                        .first()
                        .map(|s| merge_with_base(&s.style, &base_style))
                        .unwrap_or(base_style);
                    let text_id = ctx.make_text(line_text.as_str(), style);
                    ctx.append_child(flex_id, text_id);
                } else {
                    // Multi-style line: create a Flex row with styled Text children
                    let row_layout = LayoutProps {
                        direction: FlexDirection::Row,
                        ..LayoutProps::default()
                    };
                    let row_id = ctx.make_flex(row_layout, base_style);
                    for seg in &line.segments {
                        let seg_style = merge_with_base(&seg.style, &base_style);
                        let text_id = ctx.make_text(seg.text.as_str(), seg_style);
                        ctx.append_child(row_id, text_id);
                    }
                    ctx.append_child(flex_id, row_id);
                }
            }

            WidgetId(flex_id)
        } else {
            // Fallback: render as plain text
            self.create_plain(ctx, code, base_style)
        }
    }

    /// Create a plain text node (fallback when highlighting is not available).
    fn create_plain(&self, ctx: &mut WidgetContext, code: &str, style: Style) -> WidgetId {
        let id = ctx.make_text(code, style);
        WidgetId(id)
    }
}

/// Merge a segment's style with the base code block style.
/// The segment's attributes override the base.
fn merge_with_base(segment_style: &Style, base: &Style) -> Style {
    Style {
        fg: segment_style.fg.or(base.fg),
        bg: base.bg.or(segment_style.bg),
        bold: segment_style.bold.or(base.bold),
        italic: segment_style.italic.or(base.italic),
        underline: segment_style.underline.or(base.underline),
        dim: segment_style.dim.or(base.dim),
        strikethrough: segment_style.strikethrough.or(base.strikethrough),
        ..Style::default()
    }
}

impl Widget for CodeWidget {
    fn kind(&self) -> &'static str {
        "Code"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let mut style = self.style;
        style.bold = Some(false);
        style.italic = Some(false);

        if self.inline {
            // Inline code: always plain text
            let id = ctx.make_text(self.content.as_ref(), style);
            return WidgetId(id);
        }

        if let Some(lang) = &self.language
            && !lang.as_ref().is_empty()
        {
            return self.create_highlighted(ctx, self.content.as_ref(), lang, style);
        }

        // No language or empty language: plain text
        self.create_plain(ctx, self.content.as_ref(), style)
    }

    fn handle_event(&self, _id: WidgetId, _ctx: &mut WidgetContext, _event: &Event) -> EventResult {
        EventResult::Ignored
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::focus::FocusManager;
    use crate::scheduler::Scheduler;
    use crate::tree::arena::NodeArena;
    use crate::tree::node_kind::NodeKind;
    use crate::widgets::theme::Theme;

    fn make_ctx() -> (NodeArena, FocusManager, Scheduler, Theme) {
        (
            NodeArena::new(),
            FocusManager::new(),
            Scheduler::new(),
            Theme::default(),
        )
    }

    #[test]
    fn code_widget_kind() {
        let w = CodeWidget::new("let x = 1;");
        assert_eq!(w.kind(), "Code");
    }

    #[test]
    fn code_widget_inline() {
        let w = CodeWidget::inline("x");
        assert!(w.inline);
        assert_eq!(w.content.as_ref(), "x");
    }

    #[test]
    fn code_widget_block() {
        let w = CodeWidget::block("fn main() {}", "rust");
        assert!(!w.inline);
        assert_eq!(w.language.as_deref(), Some("rust"));
    }

    #[test]
    fn code_widget_create_plain() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = CodeWidget::new("hello");
        let id = w.create(&mut ctx);
        let node = ctx.arena.get(id.node_id()).unwrap();
        assert_eq!(node.text.as_deref(), Some("hello"));
    }

    #[test]
    fn code_widget_with_language() {
        let w = CodeWidget::new("code").with_language("python");
        assert_eq!(w.language.as_deref(), Some("python"));
    }

    #[test]
    fn code_widget_create_highlighted() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = CodeWidget::block("fn hello() {}", "rust");
        let id = w.create(&mut ctx);
        // Highlighted code creates a Flex container with children
        let node = ctx.arena.get(id.node_id()).unwrap();
        // Should be a Flex container (from create_highlighted)
        assert_eq!(node.kind, NodeKind::Flex);
    }

    #[test]
    fn code_widget_inline_vs_block() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        // Inline code should create a plain Text node
        let inline_w = CodeWidget::inline("x");
        let inline_id = inline_w.create(&mut ctx);
        assert_eq!(
            ctx.arena.get(inline_id.node_id()).unwrap().kind,
            NodeKind::Text
        );
    }
}
