use crate::layout::types::{FlexDirection, LayoutProps};
use crate::syntax::global_highlighter;
use crate::tree::Color;
use crate::tree::NodeId;
use crate::tree::Style;
use crate::widgets::WidgetId;
use crate::widgets::context::WidgetContext;

use super::ast::{InlineNode, MarkdownNode};

pub struct MarkdownRenderer {
    pub base_style: Style,
    pub heading_style: Style,
    pub bold_style: Style,
    pub italic_style: Style,
    pub code_style: Style,
    pub code_block_style: Style,
    pub link_style: Style,
    pub quote_style: Style,
    pub rule_style: Style,
    pub indent: u16,
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        Self {
            base_style: Style::default(),
            heading_style: Style {
                fg: Some(Color::rgb(100, 200, 255)),
                bold: Some(true),
                ..Style::default()
            },
            bold_style: Style {
                bold: Some(true),
                ..Style::default()
            },
            italic_style: Style {
                italic: Some(true),
                ..Style::default()
            },
            code_style: Style {
                fg: Some(Color::rgb(200, 150, 255)),
                ..Style::default()
            },
            code_block_style: Style {
                fg: Some(Color::rgb(200, 200, 200)),
                bg: Some(Color::rgb(30, 30, 40)),
                ..Style::default()
            },
            link_style: Style {
                underline: Some(true),
                fg: Some(Color::rgb(100, 180, 255)),
                ..Style::default()
            },
            quote_style: Style {
                italic: Some(true),
                fg: Some(Color::rgb(150, 150, 150)),
                ..Style::default()
            },
            rule_style: Style {
                fg: Some(Color::rgb(80, 80, 80)),
                ..Style::default()
            },
            indent: 0,
        }
    }
}

impl MarkdownRenderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&self, nodes: &[MarkdownNode], ctx: &mut WidgetContext) -> WidgetId {
        let mut children = Vec::new();
        for node in nodes {
            if let Some(child) = self.render_node(node, ctx) {
                children.push(child);
            }
        }

        if children.is_empty() {
            let id = ctx.make_text("", self.base_style);
            return WidgetId(id);
        }

        if children.len() == 1 {
            return WidgetId(children[0]);
        }

        let layout = LayoutProps {
            direction: FlexDirection::Column,
            ..Default::default()
        };
        let flex_id = ctx.make_flex(layout, self.base_style);
        for child in children {
            ctx.append_child(flex_id, child);
        }
        WidgetId(flex_id)
    }

    fn render_node(&self, node: &MarkdownNode, ctx: &mut WidgetContext) -> Option<NodeId> {
        match node {
            MarkdownNode::Heading { level, content } => {
                let style = match level {
                    1 => Style {
                        fg: Some(Color::rgb(80, 180, 255)),
                        bold: Some(true),
                        ..Style::default()
                    },
                    2 => Style {
                        fg: Some(Color::rgb(100, 200, 255)),
                        bold: Some(true),
                        ..Style::default()
                    },
                    3 => Style {
                        fg: Some(Color::rgb(120, 210, 255)),
                        ..Style::default()
                    },
                    4 => Style {
                        fg: Some(Color::rgb(140, 220, 255)),
                        ..Style::default()
                    },
                    _ => Style {
                        fg: Some(Color::rgb(160, 230, 255)),
                        ..Style::default()
                    },
                };
                let text = self.render_inline(content);
                Some(ctx.make_text(text.as_str(), style))
            }
            MarkdownNode::Paragraph(content) => {
                let text = self.render_inline(content);
                Some(ctx.make_text(text.as_str(), self.base_style))
            }
            MarkdownNode::List { items, .. } => {
                let mut children = Vec::new();
                for item in items {
                    let prefix = "• ";
                    let mut text = String::from(prefix);
                    text.push_str(&self.render_inline(&item.content));
                    children.push(ctx.make_text(text.as_str(), self.base_style));
                }
                if children.len() == 1 {
                    return Some(children[0]);
                }
                let layout = LayoutProps {
                    direction: FlexDirection::Column,
                    ..Default::default()
                };
                let flex_id = ctx.make_flex(layout, self.base_style);
                for child in children {
                    ctx.append_child(flex_id, child);
                }
                Some(flex_id)
            }
            MarkdownNode::Blockquote(children) => {
                let mut rendered = Vec::new();
                for child in children {
                    if let Some(r) = self.render_node(child, ctx) {
                        rendered.push(r);
                    }
                }
                if rendered.is_empty() {
                    return Some(ctx.make_text("", self.quote_style));
                }
                if rendered.len() == 1 {
                    return Some(rendered[0]);
                }
                let layout = LayoutProps {
                    direction: FlexDirection::Column,
                    ..Default::default()
                };
                let flex_id = ctx.make_flex(layout, self.base_style);
                for child in rendered {
                    ctx.append_child(flex_id, child);
                }
                Some(flex_id)
            }
            MarkdownNode::CodeBlock { language, code } => {
                if let Some(lang) = language
                    && !lang.as_ref().is_empty()
                {
                    // Try syntax highlighting
                    let lines = {
                        let mut hl = global_highlighter().lock().unwrap();
                        hl.highlight(code, lang.as_ref())
                    };

                    if let Some(lines) = lines {
                        let layout = LayoutProps {
                            direction: FlexDirection::Column,
                            ..LayoutProps::default()
                        };
                        let flex_id = ctx.make_flex(layout, self.code_block_style);

                        for line in &lines {
                            if line.segments.len() <= 1 {
                                let style = merge_with_base(
                                    line.segments.first().map(|s| &s.style),
                                    &self.code_block_style,
                                );
                                let tid = ctx.make_text(line.text(), style);
                                ctx.append_child(flex_id, tid);
                            } else {
                                let row_layout = LayoutProps {
                                    direction: FlexDirection::Row,
                                    ..LayoutProps::default()
                                };
                                let row_id = ctx.make_flex(row_layout, self.code_block_style);
                                for seg in &line.segments {
                                    let seg_style =
                                        merge_with_base(Some(&seg.style), &self.code_block_style);
                                    let tid = ctx.make_text(seg.text.as_str(), seg_style);
                                    ctx.append_child(row_id, tid);
                                }
                                ctx.append_child(flex_id, row_id);
                            }
                        }
                        return Some(flex_id);
                    }
                }
                // Fallback: plain text with language label
                let mut text = String::new();
                if let Some(lang) = language {
                    text.push_str(lang.as_ref());
                    text.push('\n');
                }
                text.push_str(code);
                Some(ctx.make_text(text.as_str(), self.code_block_style))
            }
            MarkdownNode::Table { headers, rows } => {
                let mut text = String::new();
                if let Some(header_row) = headers.first() {
                    let cells: Vec<&str> = header_row.iter().map(|c| c.text_content()).collect();
                    text.push_str(&cells.join(" | "));
                    text.push('\n');
                    text.push_str(&"-".repeat(cells.iter().map(|c| c.len() + 2).sum::<usize>()));
                    text.push('\n');
                }
                for row in rows {
                    let cells: Vec<String> = row
                        .iter()
                        .map(|c| {
                            c.iter()
                                .map(|n| n.text_content().to_string())
                                .collect::<Vec<_>>()
                                .join("")
                        })
                        .collect();
                    let cell_refs: Vec<&str> = cells.iter().map(|s| s.as_str()).collect();
                    text.push_str(&cell_refs.join(" | "));
                    text.push('\n');
                }
                Some(ctx.make_text(text.as_str(), self.code_style))
            }
            MarkdownNode::HorizontalRule => {
                Some(ctx.make_text("────────────────────────────────────────", self.rule_style))
            }
            MarkdownNode::TaskList(items) => {
                let mut children = Vec::new();
                for item in items {
                    let checkbox = if item.checked { "☑ " } else { "☐ " };
                    let mut text = String::from(checkbox);
                    text.push_str(&self.render_inline(&item.content));
                    children.push(ctx.make_text(text.as_str(), self.base_style));
                }
                if children.len() == 1 {
                    return Some(children[0]);
                }
                let layout = LayoutProps {
                    direction: FlexDirection::Column,
                    ..Default::default()
                };
                let flex_id = ctx.make_flex(layout, self.base_style);
                for child in children {
                    ctx.append_child(flex_id, child);
                }
                Some(flex_id)
            }
        }
    }

    fn render_inline(&self, nodes: &[InlineNode]) -> String {
        let mut result = String::new();
        for node in nodes {
            match node {
                InlineNode::Text(s) => result.push_str(s),
                InlineNode::Code(s) => result.push_str(s),
                InlineNode::Bold(children) => {
                    let inner = self.render_inline(children);
                    result.push_str(&inner);
                }
                InlineNode::Italic(children) => {
                    let inner = self.render_inline(children);
                    result.push_str(&inner);
                }
                InlineNode::BoldItalic(children) => {
                    let inner = self.render_inline(children);
                    result.push_str(&inner);
                }
                InlineNode::Strikethrough(children) => {
                    let inner = self.render_inline(children);
                    result.push_str(&inner);
                }
                InlineNode::Link { text, url } => {
                    result.push_str(text.as_ref());
                    result.push_str(" (");
                    result.push_str(url.as_ref());
                    result.push(')');
                }
            }
        }
        result
    }
}

/// Merge a highlight segment style with a base code block style.
/// Segment attributes override the base.
fn merge_with_base(segment_style: Option<&Style>, base: &Style) -> Style {
    match segment_style {
        Some(s) => Style {
            fg: s.fg.or(base.fg),
            bg: base.bg.or(s.bg),
            bold: s.bold.or(base.bold),
            italic: s.italic.or(base.italic),
            underline: s.underline.or(base.underline),
            dim: s.dim.or(base.dim),
            strikethrough: s.strikethrough.or(base.strikethrough),
            ..Style::default()
        },
        None => *base,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::FocusManager;
    use crate::scheduler::Scheduler;
    use crate::tree::NodeArena;
    use crate::widgets::context::WidgetContext;
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
    fn renderer_new() {
        let renderer = MarkdownRenderer::new();
        assert_eq!(renderer.indent, 0);
    }

    #[test]
    fn render_empty() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };
        let renderer = MarkdownRenderer::new();
        let id = renderer.render(&[], &mut ctx);
        assert!(ctx.arena.contains(id.node_id()));
    }

    #[test]
    fn render_paragraph() {
        let nodes = vec![MarkdownNode::Paragraph(vec![InlineNode::Text(
            "Hello".into(),
        )])];
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };
        let renderer = MarkdownRenderer::new();
        let id = renderer.render(&nodes, &mut ctx);
        assert!(ctx.arena.contains(id.node_id()));
    }

    #[test]
    fn render_inline_text() {
        let renderer = MarkdownRenderer::new();
        let nodes = vec![InlineNode::Text("hello".into())];
        let result = renderer.render_inline(&nodes);
        assert_eq!(result, "hello");
    }

    #[test]
    fn render_inline_bold() {
        let renderer = MarkdownRenderer::new();
        let nodes = vec![InlineNode::Bold(vec![InlineNode::Text("bold".into())])];
        let result = renderer.render_inline(&nodes);
        assert_eq!(result, "bold");
    }

    #[test]
    fn render_inline_link() {
        let renderer = MarkdownRenderer::new();
        let nodes = vec![InlineNode::Link {
            text: "click".into(),
            url: "https://example.com".into(),
        }];
        let result = renderer.render_inline(&nodes);
        assert_eq!(result, "click (https://example.com)");
    }
}
