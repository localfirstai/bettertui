//! Widget framework: Widget trait, built-in widgets, context, reconciliation, and theming.

pub mod app;
pub mod badge_widget;
pub mod box_widget;
pub mod button_widget;
pub mod callback_types;
pub mod chat;
pub mod code_widget;
pub mod container;
pub mod context;
pub mod flex_widget;
pub mod grid_widget;
pub mod heading_widget;
pub mod input_widget;
pub mod label_widget;
pub mod markdown;
pub mod modal_widget;
pub mod pipeline;
pub mod progress_widget;
pub mod prompt_composer;
pub mod reconcile;
pub mod registry;
pub mod scroll_area;
pub mod separator_widget;
pub mod spacer_widget;
pub mod spinner_widget;
pub mod stack_widget;
pub mod tabs_widget;
pub mod text_widget;
pub mod textarea_widget;
pub mod theme;
pub mod tooltip_widget;
pub mod tree;

pub use app::AppState;
pub use badge_widget::{BadgeVariant, BadgeWidget};
pub use box_widget::BoxWidget;
pub use button_widget::{ButtonVariant, ButtonWidget};
pub use chat::{ChatState, ChatStatus, ChatView, Message, Role, StatusBar, ThinkingIndicator};
pub use code_widget::CodeWidget;
pub use container::ContainerWidget;
pub use context::WidgetContext;
pub use flex_widget::FlexWidget;
pub use grid_widget::GridWidget;
pub use heading_widget::{HeadingLevel, HeadingWidget};
pub use input_widget::InputWidget;
pub use label_widget::LabelWidget;
pub use markdown::{InlineNode, MarkdownNode, MarkdownRenderer, Parser as MarkdownParser};
pub use modal_widget::ModalWidget;
pub use pipeline::Pipeline;
pub use progress_widget::ProgressWidget;
pub use prompt_composer::{ComposerState, PromptComposer};
pub use reconcile::{ReconcileOp, Reconciler};
pub use registry::WidgetRegistry;
pub use scroll_area::ScrollAreaWidget;
pub use separator_widget::SeparatorWidget;
pub use spacer_widget::SpacerWidget;
pub use spinner_widget::{SpinnerType, SpinnerWidget};
pub use stack_widget::{StackChild, StackWidget};
pub use tabs_widget::{TabItem, TabsWidget};
pub use text_widget::TextWidget;
pub use textarea_widget::TextareaWidget;
pub use theme::{SpacingToken, Theme, ThemeToken};
pub use tooltip_widget::TooltipWidget;
pub use tree::WidgetTree;

use crate::input::Event;
use crate::input::EventResult;
use crate::tree::node_id::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WidgetId(pub NodeId);

impl WidgetId {
    pub fn node_id(self) -> NodeId {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WidgetLifecycle {
    Create,
    Mount,
    Update,
    Destroy,
}

pub trait Widget: Send + Sync {
    fn kind(&self) -> &'static str;

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId;

    fn update(&self, _id: WidgetId, _ctx: &mut WidgetContext) {}

    fn handle_event(&self, _id: WidgetId, _ctx: &mut WidgetContext, _event: &Event) -> EventResult {
        EventResult::Ignored
    }

    fn destroy(&self, _id: WidgetId, _ctx: &mut WidgetContext) {}
}

pub struct WidgetHost {
    tree: WidgetTree,
    registry: WidgetRegistry,
    widgets: Vec<Box<dyn Widget>>,
    widget_map: std::collections::HashMap<WidgetId, usize>,
}

impl Default for WidgetHost {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetHost {
    pub fn new() -> Self {
        Self {
            tree: WidgetTree::new(),
            registry: WidgetRegistry::new(),
            widgets: Vec::new(),
            widget_map: std::collections::HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        kind: &'static str,
        factory: impl Fn() -> Box<dyn Widget> + Send + Sync + 'static,
    ) {
        self.registry.register(kind, factory);
    }

    pub fn mount(&mut self, widget: Box<dyn Widget>, ctx: &mut WidgetContext) -> WidgetId {
        let node_id = widget.create(ctx);
        let kind = widget.kind();
        let idx = self.widgets.len();
        self.widgets.push(widget);
        self.tree.insert(node_id, node_id.node_id(), kind);
        self.widget_map.insert(node_id, idx);
        node_id
    }

    pub fn unmount(&mut self, widget_id: WidgetId, ctx: &mut WidgetContext) {
        if let Some(&idx) = self.widget_map.get(&widget_id) {
            self.widgets[idx].destroy(widget_id, ctx);
            self.tree.remove(widget_id);
            self.widget_map.remove(&widget_id);
        }
    }

    pub fn handle_event(
        &mut self,
        widget_id: WidgetId,
        ctx: &mut WidgetContext,
        event: &Event,
    ) -> EventResult {
        if let Some(&idx) = self.widget_map.get(&widget_id) {
            self.widgets[idx].handle_event(widget_id, ctx, event)
        } else {
            EventResult::Ignored
        }
    }

    pub fn update(&mut self, widget_id: WidgetId, ctx: &mut WidgetContext) {
        if let Some(&idx) = self.widget_map.get(&widget_id) {
            self.widgets[idx].update(widget_id, ctx);
        }
    }

    pub fn tree(&self) -> &WidgetTree {
        &self.tree
    }

    pub fn tree_mut(&mut self) -> &mut WidgetTree {
        &mut self.tree
    }

    pub fn registry(&self) -> &WidgetRegistry {
        &self.registry
    }

    pub fn widget_count(&self) -> usize {
        self.widgets.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::FocusManager;
    use crate::scheduler::Scheduler;
    use crate::tree::arena::NodeArena;

    struct TestWidget;

    impl Widget for TestWidget {
        fn kind(&self) -> &'static str {
            "Test"
        }

        fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
            let id = ctx.make_box(
                crate::layout::types::LayoutProps::default(),
                crate::tree::style::Style::default(),
            );
            WidgetId(id)
        }
    }

    struct EventCaptureWidget {
        last_event: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    }

    impl Widget for EventCaptureWidget {
        fn kind(&self) -> &'static str {
            "EventCapture"
        }

        fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
            let id = ctx.make_box(
                crate::layout::types::LayoutProps::default(),
                crate::tree::style::Style::default(),
            );
            WidgetId(id)
        }

        fn handle_event(
            &self,
            _id: WidgetId,
            _ctx: &mut WidgetContext,
            event: &Event,
        ) -> EventResult {
            let name = match event {
                Event::Key(_) => "key",
                Event::Mouse(_) => "mouse",
                _ => "other",
            };
            *self.last_event.lock().unwrap() = Some(name.to_string());
            EventResult::Consumed
        }
    }

    fn make_host() -> (WidgetHost, NodeArena, FocusManager, Scheduler, Theme) {
        (
            WidgetHost::new(),
            NodeArena::new(),
            FocusManager::new(),
            Scheduler::new(),
            Theme::default(),
        )
    }

    #[test]
    fn host_new() {
        let (host, _, _, _, _) = make_host();
        assert_eq!(host.widget_count(), 0);
    }

    #[test]
    fn host_mount() {
        let (mut host, mut arena, mut focus, mut sched, theme) = make_host();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let wid = host.mount(Box::new(TestWidget), &mut ctx);
        assert_eq!(host.widget_count(), 1);
        assert!(host.tree().get(wid).is_some());
    }

    #[test]
    fn host_unmount() {
        let (mut host, mut arena, mut focus, mut sched, theme) = make_host();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let wid = host.mount(Box::new(TestWidget), &mut ctx);
        host.unmount(wid, &mut ctx);
        assert_eq!(host.widget_count(), 1);
        assert!(host.tree().get(wid).is_none());
    }

    #[test]
    fn host_handle_event() {
        let (mut host, mut arena, mut focus, mut sched, theme) = make_host();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };

        let last = std::sync::Arc::new(std::sync::Mutex::new(None));
        let widget = EventCaptureWidget {
            last_event: last.clone(),
        };
        let wid = host.mount(Box::new(widget), &mut ctx);

        let event = Event::Key(crate::input::KeyEvent::new(
            crate::input::Key::Enter,
            wid.node_id(),
        ));
        let result = host.handle_event(wid, &mut ctx, &event);
        assert_eq!(result, EventResult::Consumed);
        assert_eq!(last.lock().unwrap().as_deref(), Some("key"));
    }

    #[test]
    fn widget_id_default() {
        let wid = WidgetId::default();
        assert_eq!(wid.node_id(), NodeId::default());
    }

    #[test]
    fn widget_id_equality() {
        let w1 = WidgetId(NodeId::default());
        let w2 = WidgetId(NodeId::default());
        assert_eq!(w1, w2);
    }

    #[test]
    fn lifecycle_variants() {
        assert_eq!(WidgetLifecycle::Create, WidgetLifecycle::Create);
        assert_eq!(WidgetLifecycle::Mount, WidgetLifecycle::Mount);
        assert_eq!(WidgetLifecycle::Update, WidgetLifecycle::Update);
        assert_eq!(WidgetLifecycle::Destroy, WidgetLifecycle::Destroy);
        assert_ne!(WidgetLifecycle::Create, WidgetLifecycle::Destroy);
    }
}
