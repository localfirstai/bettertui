use super::types::{Message, Role};
use crate::Widget;
use crate::WidgetId;
use crate::context::WidgetContext;
use bettertui_engine::input::{Event, EventResult};
use bettertui_engine::taffy::{FlexDirection, LayoutProps};
use bettertui_engine::tree::Color;
use bettertui_engine::tree::Style;

pub struct ChatView {
    pub style: Style,
    pub message_style: Style,
    pub user_style: Style,
    pub assistant_style: Style,
    pub system_style: Style,
    pub separator_style: Style,
}

impl Default for ChatView {
    fn default() -> Self {
        Self {
            style: Style::default(),
            message_style: Style::default(),
            user_style: Style { fg: Some(Color::rgb(100, 180, 255)), ..Style::default() },
            assistant_style: Style { fg: Some(Color::rgb(100, 200, 100)), ..Style::default() },
            system_style: Style { fg: Some(Color::rgb(150, 150, 150)), italic: Some(true), ..Style::default() },
            separator_style: Style { fg: Some(Color::rgb(60, 60, 60)), ..Style::default() },
        }
    }
}

impl ChatView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_user_style(mut self, style: Style) -> Self {
        self.user_style = style;
        self
    }

    pub fn with_assistant_style(mut self, style: Style) -> Self {
        self.assistant_style = style;
        self
    }

    pub fn render_message(&self, msg: &Message, ctx: &mut WidgetContext) -> WidgetId {
        let style = match msg.role {
            Role::User => self.user_style,
            Role::Assistant => self.assistant_style,
            Role::System => self.system_style,
        };

        let prefix = match msg.role {
            Role::User => "You: ",
            Role::Assistant => "Assistant: ",
            Role::System => "System: ",
        };

        let mut text = String::from(prefix);
        text.push_str(msg.content.as_ref());

        let id = ctx.make_text(text.as_str(), style);
        WidgetId(id)
    }

    pub fn render_thinking(&self, msg: &Message, ctx: &mut WidgetContext) -> Option<WidgetId> {
        if let Some(thinking) = &msg.thinking {
            let style = Style { fg: Some(Color::rgb(120, 120, 120)), italic: Some(true), ..Style::default() };
            let mut text = String::from("Thinking: ");
            text.push_str(thinking);
            let id = ctx.make_text(text.as_str(), style);
            Some(WidgetId(id))
        } else {
            None
        }
    }

    pub fn render_separator(&self, ctx: &mut WidgetContext) -> WidgetId {
        let id = ctx.make_text("────────────────────────────────────────", self.separator_style);
        WidgetId(id)
    }
}

impl Widget for ChatView {
    fn kind(&self) -> &'static str {
        "ChatView"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let layout = LayoutProps { direction: FlexDirection::Column, ..Default::default() };
        let id = ctx.make_box(layout, self.style);
        WidgetId(id)
    }

    fn handle_event(&self, _id: WidgetId, _ctx: &mut WidgetContext, event: &Event) -> EventResult {
        match event {
            Event::Key(key_event) => match key_event.key {
                bettertui_engine::input::Key::ArrowUp => EventResult::Consumed,
                bettertui_engine::input::Key::ArrowDown => EventResult::Consumed,
                _ => EventResult::Ignored,
            },
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

    fn make_ctx() -> (NodeArena, FocusManager, Scheduler, Theme) {
        (NodeArena::new(), FocusManager::new(), Scheduler::new(), Theme::default())
    }

    #[test]
    fn chat_view_new() {
        let view = ChatView::new();
        assert_eq!(view.kind(), "ChatView");
    }

    #[test]
    fn chat_view_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };
        let view = ChatView::new();
        let id = view.create(&mut ctx);
        assert!(ctx.arena.contains(id.node_id()));
    }

    #[test]
    fn render_user_message() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };
        let view = ChatView::new();
        let msg = Message::user("Hello", 100);
        let id = view.render_message(&msg, &mut ctx);
        assert!(ctx.arena.contains(id.node_id()));
    }

    #[test]
    fn render_assistant_message() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };
        let view = ChatView::new();
        let msg = Message::assistant("Hi there", 200);
        let id = view.render_message(&msg, &mut ctx);
        assert!(ctx.arena.contains(id.node_id()));
    }

    #[test]
    fn render_thinking() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };
        let view = ChatView::new();
        let msg = Message::assistant("Answer", 300).with_thinking("Let me think...");
        let id = view.render_thinking(&msg, &mut ctx).expect("Node missing from arena");
        assert!(ctx.arena.contains(id.node_id()));
    }

    #[test]
    fn render_separator() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };
        let view = ChatView::new();
        let id = view.render_separator(&mut ctx);
        assert!(ctx.arena.contains(id.node_id()));
    }
}
