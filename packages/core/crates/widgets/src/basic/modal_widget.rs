use bettertui_engine::input::{Event, EventResult, Key};
use bettertui_engine::taffy::{LayoutProps, Position};
use bettertui_engine::tree::Style;

use crate::{Widget, WidgetContext, WidgetId};

/// Modal widget for dialog overlays.
///
/// Renders a centered dialog with optional backdrop.
pub struct ModalWidget {
    pub title: Option<Box<str>>,
    pub closable: bool,
    pub style: Style,
    pub layout: LayoutProps,
    pub on_close: Option<Box<dyn Fn() + Send + Sync>>,
}

impl Default for ModalWidget {
    fn default() -> Self {
        Self { title: None, closable: true, style: Style::default(), layout: LayoutProps::default(), on_close: None }
    }
}

impl ModalWidget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<Box<str>>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_closable(mut self, closable: bool) -> Self {
        self.closable = closable;
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

    pub fn on_close(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_close = Some(Box::new(handler));
        self
    }
}

impl Widget for ModalWidget {
    fn kind(&self) -> &'static str {
        "Modal"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let mut layout = self.layout;
        layout.position = Position::Absolute;

        let display_text = self.title.as_ref().map(|t| format!(" {} ", t)).unwrap_or_default();

        let node = bettertui_engine::tree::RenderNode {
            kind: bettertui_engine::tree::NodeKind::Modal,
            text: Some(Box::from(display_text)),
            style: self.style,
            layout,
            ..bettertui_engine::tree::RenderNode::default()
        };
        let id = ctx.insert_node(node);
        ctx.set_focusable(id, true);
        WidgetId(id)
    }

    fn handle_event(&self, _id: WidgetId, _ctx: &mut WidgetContext, event: &Event) -> EventResult {
        if !self.closable {
            return EventResult::Ignored;
        }

        match event {
            Event::Key(key_event) => {
                if key_event.key == Key::Escape {
                    if let Some(ref handler) = self.on_close {
                        handler();
                    }
                    EventResult::Consumed
                } else {
                    EventResult::Ignored
                }
            }
            _ => EventResult::Ignored,
        }
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
        (NodeArena::new(), FocusManager::new(), Scheduler::new(), Theme::default())
    }

    #[test]
    fn modal_widget_kind() {
        let w = ModalWidget::new();
        assert_eq!(w.kind(), "Modal");
    }

    #[test]
    fn modal_widget_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let w = ModalWidget::new().with_title("Confirm");
        let id = w.create(&mut ctx);
        let node = ctx.arena.get(id.node_id()).expect("Node missing from arena");
        assert_eq!(node.kind, NodeKind::Modal);
        assert_eq!(node.text.as_deref(), Some(" Confirm "));
    }

    #[test]
    fn modal_widget_with_title() {
        let w = ModalWidget::new().with_title("Dialog");
        assert_eq!(w.title.as_deref(), Some("Dialog"));
    }

    #[test]
    fn modal_widget_not_closable() {
        let w = ModalWidget::new().with_closable(false);
        assert!(!w.closable);
    }
}
