use crate::events::Event;
use crate::events::types::EventResult;
use crate::tree::color::{Color, NamedColor};
use crate::tree::layout::LayoutProps;
use crate::tree::style::Style;

use super::{Widget, WidgetContext, WidgetId};

/// Code widget for displaying code blocks.
///
/// Renders text with monospace font and syntax-appropriate styling.
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
                fg: Some(Color::Named(NamedColor::Cyan)),
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
}

impl Widget for CodeWidget {
    fn kind(&self) -> &'static str {
        "Code"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let mut style = self.style;
        style.bold = Some(false);
        style.italic = Some(false);

        let node = crate::tree::render_node::RenderNode {
            kind: crate::tree::node_kind::NodeKind::Text,
            text: Some(self.content.clone()),
            style,
            layout: self.layout,
            ..crate::tree::render_node::RenderNode::default()
        };
        let id = ctx.insert_node(node);
        WidgetId(id)
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
    fn code_widget_create() {
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
        assert_eq!(node.kind, NodeKind::Text);
        assert_eq!(node.text.as_deref(), Some("hello"));
    }

    #[test]
    fn code_widget_with_language() {
        let w = CodeWidget::new("code").with_language("python");
        assert_eq!(w.language.as_deref(), Some("python"));
    }
}
