use crate::events::Event;
use crate::events::types::EventResult;
use crate::tree::style::Style;

use super::{Widget, WidgetContext, WidgetId};

pub struct TextWidget {
    pub content: Box<str>,
    pub style: Style,
}

impl TextWidget {
    pub fn new(content: impl Into<Box<str>>) -> Self {
        Self {
            content: content.into(),
            style: Style::default(),
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn bold(content: impl Into<Box<str>>) -> Self {
        Self {
            content: content.into(),
            style: Style {
                bold: Some(true),
                ..Style::default()
            },
        }
    }

    pub fn colored(content: impl Into<Box<str>>, fg: crate::tree::color::Color) -> Self {
        Self {
            content: content.into(),
            style: Style {
                fg: Some(fg),
                ..Style::default()
            },
        }
    }
}

impl Widget for TextWidget {
    fn kind(&self) -> &'static str {
        "Text"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let id = ctx.make_text(self.content.as_ref(), self.style);
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
    use crate::tree::color::{Color, NamedColor};
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
        assert_eq!(node.kind, crate::tree::node_kind::NodeKind::Text);
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
