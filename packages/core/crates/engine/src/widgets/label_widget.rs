use crate::input::Event;
use crate::input::EventResult;
use crate::layout::types::LayoutProps;
use crate::text::TextAlign;
use crate::tree::style::Style;

use super::{Widget, WidgetContext, WidgetId};

/// Label widget for form labels.
///
/// Renders text with optional association to a form control.
pub struct LabelWidget {
    pub content: Box<str>,
    pub html_for: Option<WidgetId>,
    pub style: Style,
    pub layout: LayoutProps,
    pub wrap: bool,
    pub align: TextAlign,
}

impl Default for LabelWidget {
    fn default() -> Self {
        Self {
            content: Box::from(""),
            html_for: None,
            style: Style::default(),
            layout: LayoutProps::default(),
            wrap: false,
            align: TextAlign::Left,
        }
    }
}

impl LabelWidget {
    pub fn new(content: impl Into<Box<str>>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    pub fn with_for(mut self, target: WidgetId) -> Self {
        self.html_for = Some(target);
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

    pub fn with_wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }
}

impl Widget for LabelWidget {
    fn kind(&self) -> &'static str {
        "Label"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let node = crate::tree::render_node::RenderNode {
            kind: crate::tree::node_kind::NodeKind::Text,
            text: Some(self.content.clone()),
            style: self.style,
            layout: self.layout,
            text_align: self.align,
            text_wrap: self.wrap,
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
    use crate::input::FocusManager;
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
    fn label_widget_kind() {
        let w = LabelWidget::new("Name");
        assert_eq!(w.kind(), "Label");
    }

    #[test]
    fn label_widget_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = LabelWidget::new("Email");
        let id = w.create(&mut ctx);
        let node = ctx.arena.get(id.node_id()).unwrap();
        assert_eq!(node.kind, NodeKind::Text);
        assert_eq!(node.text.as_deref(), Some("Email"));
    }

    #[test]
    fn label_widget_with_for() {
        let target = WidgetId(crate::tree::node_id::NodeId::default());
        let w = LabelWidget::new("Name").with_for(target);
        assert_eq!(w.html_for, Some(target));
    }

    #[test]
    fn label_widget_with_style() {
        let style = Style {
            bold: Some(true),
            ..Style::default()
        };
        let w = LabelWidget::new("Label").with_style(style);
        assert!(w.style.bold.unwrap());
    }
}
