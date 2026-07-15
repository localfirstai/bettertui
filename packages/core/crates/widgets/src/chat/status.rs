use super::types::{ChatState, ChatStatus};
use crate::Widget;
use crate::WidgetId;
use crate::context::WidgetContext;
use bettertui_engine::input::{Event, EventResult};
use bettertui_engine::taffy::LayoutProps;
use bettertui_engine::tree::Color;
use bettertui_engine::tree::Style;

pub struct StatusBar {
    pub style: Style,
    pub idle_style: Style,
    pub thinking_style: Style,
    pub streaming_style: Style,
    pub error_style: Style,
}

impl Default for StatusBar {
    fn default() -> Self {
        Self {
            style: Style { fg: Some(Color::rgb(200, 200, 200)), ..Style::default() },
            idle_style: Style { fg: Some(Color::rgb(100, 200, 100)), ..Style::default() },
            thinking_style: Style { fg: Some(Color::rgb(200, 200, 100)), ..Style::default() },
            streaming_style: Style { fg: Some(Color::rgb(100, 200, 255)), ..Style::default() },
            error_style: Style { fg: Some(Color::rgb(255, 100, 100)), ..Style::default() },
        }
    }
}

impl StatusBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn render_status(&self, state: &ChatState, ctx: &mut WidgetContext) -> WidgetId {
        let status_style = match state.status {
            ChatStatus::Idle => self.idle_style,
            ChatStatus::Thinking => self.thinking_style,
            ChatStatus::Streaming => self.streaming_style,
            ChatStatus::Error => self.error_style,
        };

        let mut text = String::from(state.status.label());
        text.push_str(" | ");
        text.push_str(&format!("{} messages", state.messages.len()));

        let id = ctx.make_text(text.as_str(), status_style);
        WidgetId(id)
    }

    pub fn render_position(&self, state: &ChatState, ctx: &mut WidgetContext) -> WidgetId {
        let text = if state.messages.is_empty() {
            "No messages".to_string()
        } else {
            format!("Position: {}/{}", state.scroll_offset + 1, state.messages.len())
        };

        let id = ctx.make_text(text.as_str(), self.style);
        WidgetId(id)
    }
}

impl Widget for StatusBar {
    fn kind(&self) -> &'static str {
        "StatusBar"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let layout = LayoutProps::default();
        let id = ctx.make_box(layout, self.style);
        WidgetId(id)
    }

    fn handle_event(&self, _id: WidgetId, _ctx: &mut WidgetContext, _event: &Event) -> EventResult {
        EventResult::Ignored
    }
}

pub struct ThinkingIndicator {
    pub style: Style,
    pub dots: u8,
}

impl Default for ThinkingIndicator {
    fn default() -> Self {
        Self { style: Style { fg: Some(Color::rgb(200, 200, 100)), ..Style::default() }, dots: 3 }
    }
}

impl ThinkingIndicator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_dots(mut self, dots: u8) -> Self {
        self.dots = dots;
        self
    }

    pub fn render(&self, frame: u64, ctx: &mut WidgetContext) -> WidgetId {
        let dots = ".".repeat(((frame as u8) % (self.dots + 1)) as usize);
        let text = format!("Thinking{}", dots);
        let id = ctx.make_text(text.as_str(), self.style);
        WidgetId(id)
    }
}

impl Widget for ThinkingIndicator {
    fn kind(&self) -> &'static str {
        "ThinkingIndicator"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let layout = LayoutProps::default();
        let id = ctx.make_box(layout, self.style);
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

    fn make_ctx() -> (NodeArena, FocusManager, Scheduler, Theme) {
        (NodeArena::new(), FocusManager::new(), Scheduler::new(), Theme::default())
    }

    #[test]
    fn status_bar_new() {
        let bar = StatusBar::new();
        assert_eq!(bar.kind(), "StatusBar");
    }

    #[test]
    fn status_bar_create() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };
        let bar = StatusBar::new();
        let id = bar.create(&mut ctx);
        assert!(ctx.arena.contains(id.node_id()));
    }

    #[test]
    fn render_status_idle() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };
        let bar = StatusBar::new();
        let state = ChatState::new();
        let id = bar.render_status(&state, &mut ctx);
        assert!(ctx.arena.contains(id.node_id()));
    }

    #[test]
    fn render_status_thinking() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };
        let bar = StatusBar::new();
        let mut state = ChatState::new();
        state.set_status(ChatStatus::Thinking);
        let id = bar.render_status(&state, &mut ctx);
        assert!(ctx.arena.contains(id.node_id()));
    }

    #[test]
    fn render_position() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };
        let bar = StatusBar::new();
        let state = ChatState::new();
        let id = bar.render_position(&state, &mut ctx);
        assert!(ctx.arena.contains(id.node_id()));
    }

    #[test]
    fn thinking_indicator_new() {
        let indicator = ThinkingIndicator::new();
        assert_eq!(indicator.kind(), "ThinkingIndicator");
        assert_eq!(indicator.dots, 3);
    }

    #[test]
    fn thinking_indicator_render() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };
        let indicator = ThinkingIndicator::new();
        let id = indicator.render(0, &mut ctx);
        assert!(ctx.arena.contains(id.node_id()));
    }

    #[test]
    fn thinking_indicator_dots() {
        let (mut arena, mut focus, mut sched, theme) = make_ctx();
        let mut ctx = WidgetContext {
            arena: &mut arena,
            focus_manager: &mut focus,
            scheduler: &mut sched,
            terminal_size: (80, 24),
            theme: &theme,
        };
        let indicator = ThinkingIndicator::new().with_dots(5);
        assert_eq!(indicator.dots, 5);
        let id = indicator.render(2, &mut ctx);
        assert!(ctx.arena.contains(id.node_id()));
    }
}
