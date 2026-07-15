use bettertui_engine::input::Event;
use bettertui_engine::input::EventResult;
use bettertui_engine::taffy::LayoutProps;
use bettertui_engine::tree::Style;

use crate::{Widget, WidgetContext, WidgetId};

/// Heading widget for section titles (h1-h6).
///
/// Renders text with appropriate styling for document headings.
pub struct HeadingWidget {
    pub level: HeadingLevel,
    pub content: Box<str>,
    pub style: Style,
    pub layout: LayoutProps,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HeadingLevel {
    #[default]
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl HeadingLevel {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
        }
    }
}

impl Default for HeadingWidget {
    fn default() -> Self {
        Self {
            level: HeadingLevel::default(),
            content: Box::from(""),
            style: Style::default(),
            layout: LayoutProps::default(),
        }
    }
}

impl HeadingWidget {
    pub fn new(content: impl Into<Box<str>>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    pub fn h1(content: impl Into<Box<str>>) -> Self {
        Self {
            level: HeadingLevel::H1,
            content: content.into(),
            style: Style {
                bold: Some(true),
                ..Style::default()
            },
            ..Default::default()
        }
    }

    pub fn h2(content: impl Into<Box<str>>) -> Self {
        Self {
            level: HeadingLevel::H2,
            content: content.into(),
            style: Style {
                bold: Some(true),
                ..Style::default()
            },
            ..Default::default()
        }
    }

    pub fn h3(content: impl Into<Box<str>>) -> Self {
        Self {
            level: HeadingLevel::H3,
            content: content.into(),
            style: Style {
                bold: Some(true),
                ..Style::default()
            },
            ..Default::default()
        }
    }

    pub fn with_level(mut self, level: HeadingLevel) -> Self {
        self.level = level;
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

impl Widget for HeadingWidget {
    fn kind(&self) -> &'static str {
        "Heading"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let mut style = self.style;
        style.bold = Some(true);

        let node = bettertui_engine::tree::RenderNode {
            kind: bettertui_engine::tree::NodeKind::Text,
            text: Some(self.content.clone()),
            style,
            layout: self.layout,
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
    use bettertui_engine::tree::NodeKind;

    fn make_ctx() -> (NodeArena, FocusManager, Scheduler, Theme) {
        (
            NodeArena::new(),
            FocusManager::new(),
            Scheduler::new(),
            Theme::default(),
        )
    }

    #[test]
    fn heading_widget_kind() {
        let w = HeadingWidget::new("Title");
        assert_eq!(w.kind(), "Heading");
    }

    #[test]
    fn heading_widget_h1() {
        let w = HeadingWidget::h1("Title");
        assert_eq!(w.level, HeadingLevel::H1);
        assert!(w.style.bold.expect("Node missing from arena"));
        assert_eq!(w.content.as_ref(), "Title");
    }

    #[test]
    fn heading_widget_h2() {
        let w = HeadingWidget::h2("Subtitle");
        assert_eq!(w.level, HeadingLevel::H2);
    }

    #[test]
    fn heading_widget_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = HeadingWidget::h1("Hello");
        let id = w.create(&mut ctx);
        let node = ctx
            .arena
            .get(id.node_id())
            .expect("Node missing from arena");
        assert_eq!(node.kind, NodeKind::Text);
        assert_eq!(node.text.as_deref(), Some("Hello"));
        assert!(node.style.bold.expect("Node missing from arena"));
    }

    #[test]
    fn heading_level_to_str() {
        assert_eq!(HeadingLevel::H1.to_str(), "h1");
        assert_eq!(HeadingLevel::H6.to_str(), "h6");
    }
}
