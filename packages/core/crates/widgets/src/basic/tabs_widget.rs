use bettertui_engine::input::{Event, EventResult, Key};
use bettertui_engine::layout::LayoutProps;
use bettertui_engine::tree::Style;

use crate::callback_types::IndexChangeCallback;
use crate::{Widget, WidgetContext, WidgetId};

/// Tabs widget for tabbed navigation.
///
/// Renders a tab bar with selectable tabs and content panels.
#[derive(Default)]
pub struct TabsWidget {
    pub tabs: Vec<TabItem>,
    pub active_index: usize,
    pub disabled: bool,
    pub style: Style,
    pub layout: LayoutProps,
    pub on_change: Option<IndexChangeCallback>,
}

#[derive(Debug, Clone)]
pub struct TabItem {
    pub label: Box<str>,
    pub id: Option<Box<str>>,
}

impl TabItem {
    pub fn new(label: impl Into<Box<str>>) -> Self {
        Self {
            label: label.into(),
            id: None,
        }
    }

    pub fn with_id(mut self, id: impl Into<Box<str>>) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl TabsWidget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_tabs(mut self, tabs: Vec<TabItem>) -> Self {
        self.tabs = tabs;
        self
    }

    pub fn with_active(mut self, index: usize) -> Self {
        self.active_index = index;
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

    pub fn on_change(mut self, handler: impl Fn(usize) + Send + Sync + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    pub fn active_tab(&self) -> Option<&TabItem> {
        self.tabs.get(self.active_index)
    }
}

impl Widget for TabsWidget {
    fn kind(&self) -> &'static str {
        "Tabs"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let display_text = self
            .tabs
            .iter()
            .enumerate()
            .map(|(i, tab)| {
                if i == self.active_index {
                    format!("[{}]", tab.label)
                } else {
                    format!(" {}", tab.label)
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        let node = bettertui_engine::tree::RenderNode {
            kind: bettertui_engine::tree::NodeKind::Tab,
            text: Some(Box::from(display_text)),
            style: self.style,
            layout: self.layout,
            ..bettertui_engine::tree::RenderNode::default()
        };
        let id = ctx.insert_node(node);
        ctx.set_focusable(id, true);
        WidgetId(id)
    }

    fn handle_event(&self, _id: WidgetId, ctx: &mut WidgetContext, event: &Event) -> EventResult {
        if self.disabled {
            return EventResult::Ignored;
        }

        match event {
            Event::Key(key_event) => {
                let mut new_index = self.active_index;

                match key_event.key {
                    Key::ArrowLeft => {
                        new_index = new_index.saturating_sub(1);
                    }
                    Key::ArrowRight => {
                        if new_index < self.tabs.len().saturating_sub(1) {
                            new_index += 1;
                        }
                    }
                    Key::Home => {
                        new_index = 0;
                    }
                    Key::End => {
                        new_index = self.tabs.len().saturating_sub(1);
                    }
                    _ => return EventResult::Ignored,
                }

                if new_index != self.active_index {
                    if let Some(ref handler) = self.on_change {
                        handler(new_index);
                    }
                    ctx.request_frame();
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
        (
            NodeArena::new(),
            FocusManager::new(),
            Scheduler::new(),
            Theme::default(),
        )
    }

    #[test]
    fn tabs_widget_kind() {
        let w = TabsWidget::new();
        assert_eq!(w.kind(), "Tabs");
    }

    #[test]
    fn tabs_widget_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let tabs = vec![
            TabItem::new("Tab 1"),
            TabItem::new("Tab 2"),
            TabItem::new("Tab 3"),
        ];
        let w = TabsWidget::new().with_tabs(tabs);
        let id = w.create(&mut ctx);
        let node = ctx.arena.get(id.node_id()).unwrap();
        assert_eq!(node.kind, NodeKind::Tab);
    }

    #[test]
    fn tabs_widget_tab_count() {
        let tabs = vec![TabItem::new("Tab 1"), TabItem::new("Tab 2")];
        let w = TabsWidget::new().with_tabs(tabs);
        assert_eq!(w.tab_count(), 2);
    }

    #[test]
    fn tabs_widget_active_tab() {
        let tabs = vec![TabItem::new("Tab 1"), TabItem::new("Tab 2")];
        let w = TabsWidget::new().with_tabs(tabs).with_active(1);
        assert_eq!(w.active_tab().unwrap().label.as_ref(), "Tab 2");
    }

    #[test]
    fn tab_item_with_id() {
        let tab = TabItem::new("Tab").with_id("tab-1");
        assert_eq!(tab.id.as_deref(), Some("tab-1"));
    }

    #[test]
    fn tabs_widget_disabled() {
        let w = TabsWidget::new().with_disabled(true);
        assert!(w.disabled);
    }
}
