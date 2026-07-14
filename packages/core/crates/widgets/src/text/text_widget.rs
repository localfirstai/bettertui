use bettertui_engine::input::Event;
use bettertui_engine::input::EventResult;
use bettertui_engine::text::TextAlign;
use bettertui_engine::tree::Style;

use crate::{Widget, WidgetContext, WidgetId};

pub struct TextWidget {
    pub content: Box<str>,
    pub style: Style,
    pub wrap: bool,
    pub align: TextAlign,
}

impl TextWidget {
    pub fn new(content: impl Into<Box<str>>) -> Self {
        Self {
            content: content.into(),
            style: Style::default(),
            wrap: false,
            align: TextAlign::Left,
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    pub fn bold(content: impl Into<Box<str>>) -> Self {
        Self {
            content: content.into(),
            style: Style {
                bold: Some(true),
                ..Style::default()
            },
            ..Self::default()
        }
    }

    pub fn colored(content: impl Into<Box<str>>, fg: bettertui_engine::tree::Color) -> Self {
        Self {
            content: content.into(),
            style: Style {
                fg: Some(fg),
                ..Style::default()
            },
            ..Self::default()
        }
    }
}

impl Default for TextWidget {
    fn default() -> Self {
        Self {
            content: Box::from(""),
            style: Style::default(),
            wrap: false,
            align: TextAlign::Left,
        }
    }
}

impl Widget for TextWidget {
    fn kind(&self) -> &'static str {
        "Text"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let node = bettertui_engine::tree::RenderNode {
            kind: bettertui_engine::tree::NodeKind::Text,
            text: Some(self.content.clone()),
            style: self.style,
            text_align: self.align,
            text_wrap: self.wrap,
            ..bettertui_engine::tree::RenderNode::default()
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
    use crate::theme::Theme;
    use bettertui_engine::input::FocusManager;
    use bettertui_engine::scheduler::Scheduler;
    use bettertui_engine::tree::NodeArena;
    use bettertui_engine::tree::{Color, NamedColor};

    fn make_ctx() -> (NodeArena, FocusManager, Scheduler, Theme) {
        (
            NodeArena::new(),
            FocusManager::new(),
            Scheduler::new(),
            Theme::default(),
        )
    }

    #[test]
    fn text_widget_kind() {
        let w = TextWidget::new("hello");
        assert_eq!(w.kind(), "Text");
    }

    #[test]
    fn text_widget_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = TextWidget::new("Hello World");
        let id = w.create(&mut ctx);
        let node = ctx.arena.get(id.node_id()).unwrap();
        assert_eq!(node.kind, bettertui_engine::tree::NodeKind::Text);
        assert_eq!(node.text.as_deref(), Some("Hello World"));
    }

    #[test]
    fn text_widget_bold() {
        let w = TextWidget::bold("Bold");
        assert!(w.style.bold.unwrap());
        assert_eq!(w.content.as_ref(), "Bold");
    }

    #[test]
    fn text_widget_colored() {
        let w = TextWidget::colored("Red", Color::Named(NamedColor::Red));
        assert_eq!(w.style.fg, Some(Color::Named(NamedColor::Red)));
    }

    #[test]
    fn text_widget_with_style() {
        let style = Style {
            italic: Some(true),
            ..Style::default()
        };
        let w = TextWidget::new("Italic").with_style(style);
        assert!(w.style.italic.unwrap());
    }
}
