//! Widgets example: WidgetHost lifecycle and state management.
//!
//! Demonstrates:
//! - `WidgetHost` for managing widget lifecycle
//! - Custom `Widget` implementation
//! - `WidgetContext` for tree manipulation
//! - Shared state with `Arc<AtomicU64>`

use std::io::{self, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use bettertui_engine::input::FocusManager;
use bettertui_engine::render::{AnsiBackend, Renderer};
use bettertui_engine::scheduler::Scheduler;
use bettertui_engine::tree::{Color, NamedColor, NodeArena, NodeKind, RenderNode, Style};
use bettertui_terminal::Terminal;
use bettertui_widgets::context::WidgetContext;
use bettertui_widgets::theme::Theme;
use bettertui_widgets::{Widget, WidgetHost, WidgetId};

struct CounterWidget {
    label: &'static str,
    count: Arc<AtomicU64>,
}

impl CounterWidget {
    fn new(label: &'static str, count: Arc<AtomicU64>) -> Self {
        Self { label, count }
    }
}

impl Widget for CounterWidget {
    fn kind(&self) -> &'static str {
        "Counter"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let node_id = ctx.insert_node(RenderNode::new(NodeKind::Box));
        let text_id = ctx.arena.insert(RenderNode::new(NodeKind::Text));
        ctx.set_text(text_id, format!("{}: {}", self.label, self.count.load(Ordering::Relaxed)));
        ctx.set_style(text_id, Style::new().fg(Color::Named(NamedColor::BrightCyan)).bold(true));
        ctx.append_child(node_id, text_id);
        let root = ctx.arena.root();
        let _ = ctx.arena.append_child(root, node_id);
        WidgetId(node_id)
    }

    fn update(&self, id: WidgetId, ctx: &mut WidgetContext) {
        if let Some(node) = ctx.arena.get(id.node_id())
            && let Some(&first_child) = node.children.first()
        {
            ctx.set_text(first_child, format!("{}: {}", self.label, self.count.load(Ordering::Relaxed)));
        }
    }

    fn destroy(&self, id: WidgetId, ctx: &mut WidgetContext) {
        ctx.remove_subtree(id.node_id());
    }
}

pub fn run(terminal: &mut Terminal) -> io::Result<()> {
    let mut out = io::stdout();
    terminal.clear()?;
    terminal.move_cursor(0, 0)?;

    let mut engine = Engine::new_with_text("Widgets: WidgetHost, Lifecycle, and Rendering");
    engine.set_title_style(Style::new().fg(Color::Named(NamedColor::BrightWhite)).bold(true));

    engine.add_section("[1] Mounted widgets", Style::new().fg(Color::Named(NamedColor::Yellow)));

    let mut host = WidgetHost::new();
    host.register("Counter", || Box::new(CounterWidget::new("", Arc::new(AtomicU64::new(0)))));

    let count1 = Arc::new(AtomicU64::new(42));
    let count2 = Arc::new(AtomicU64::new(7));

    let mut arena = NodeArena::new();
    let mut scheduler = Scheduler::new();
    let theme = Theme::default();
    let mut focus = FocusManager::new();
    let mut ctx = WidgetContext {
        arena: &mut arena,
        focus_manager: &mut focus,
        scheduler: &mut scheduler,
        terminal_size: (80, 24),
        theme: &theme,
    };

    let w1 = host.mount(Box::new(CounterWidget::new("Alpha", count1.clone())), &mut ctx);
    let w2 = host.mount(Box::new(CounterWidget::new("Beta", count2.clone())), &mut ctx);

    engine.add_text(format!("    Mounted {} widgets", host.widget_count()));

    engine.add_section("[2] Rendering widget tree...", Style::new().fg(Color::Named(NamedColor::Yellow)));

    let mut renderer = Renderer::new(80, 6);
    renderer.set_backend(Box::new(AnsiBackend::new()));
    let frame = renderer.render_full(&mut arena);
    engine.add_text(format!("    Output ({} bytes)", frame.output_data.len()));

    engine.add_section("[3] Updating via shared AtomicU64...", Style::new().fg(Color::Named(NamedColor::Yellow)));

    count1.fetch_add(1, Ordering::Relaxed);
    count2.fetch_add(10, Ordering::Relaxed);
    let mut focus2 = FocusManager::new();
    let mut ctx2 = WidgetContext {
        arena: &mut arena,
        focus_manager: &mut focus2,
        scheduler: &mut scheduler,
        terminal_size: (80, 24),
        theme: &theme,
    };
    host.update(w1, &mut ctx2);
    host.update(w2, &mut ctx2);

    let _frame2 = renderer.render(&mut arena);
    engine.add_text("    After update (Alpha+1, Beta+10)");

    engine.add_section("[4] Unmounting Beta...", Style::new().fg(Color::Named(NamedColor::Yellow)));

    let mut focus3 = FocusManager::new();
    let mut ctx3 = WidgetContext {
        arena: &mut arena,
        focus_manager: &mut focus3,
        scheduler: &mut scheduler,
        terminal_size: (80, 24),
        theme: &theme,
    };
    host.unmount(w2, &mut ctx3);

    engine.add_text(format!("    Widgets remaining: {}", host.widget_count()));

    engine.add_section("[5] Widget tree structure...", Style::new().fg(Color::Named(NamedColor::Yellow)));

    for (wid, entry) in host.tree().iter() {
        let parent = entry.parent.map(|p| format!("{:?}", p)).unwrap_or_else(|| "root".into());
        engine.add_text(format!("    Widget {:?} (kind={}) parent={}", wid, entry.kind, parent));
    }

    engine.add_hint("Press any key to return to menu...");

    let output = engine.render();
    out.write_all(&output)?;
    out.flush()?;

    wait_for_any_key(terminal)
}

struct Engine {
    nodes: Vec<(String, Option<Style>)>,
}

impl Engine {
    fn new_with_text(title: &str) -> Self {
        let nodes = vec![
            (title.to_string(), Some(Style::new().fg(Color::Named(NamedColor::BrightWhite)).bold(true))),
            (String::new(), None),
        ];
        Self { nodes }
    }

    fn set_title_style(&mut self, style: Style) {
        if let Some((_, s)) = self.nodes.first_mut() {
            *s = Some(style);
        }
    }

    fn add_section(&mut self, text: &str, style: Style) {
        self.nodes.push(("".to_string(), None));
        self.nodes.push((text.to_string(), Some(style)));
    }

    fn add_text(&mut self, text: impl Into<String>) {
        self.nodes.push((text.into(), None));
    }

    fn add_hint(&mut self, text: &str) {
        self.nodes.push(("".to_string(), None));
        self.nodes.push((
            text.to_string(),
            Some(Style { fg: Some(Color::Named(NamedColor::BrightBlack)), dim: Some(true), ..Style::new() }),
        ));
    }

    fn render(self) -> Vec<u8> {
        let mut engine = bettertui_engine::engine::Engine::new();
        let root = engine.arena().root();

        for (text, style) in self.nodes {
            let n = engine.create_node(NodeKind::Text);
            engine.set_text(n, text);
            if let Some(s) = style {
                engine.set_style(n, s);
            }
            let _ = engine.append_child(root, n);
        }

        engine.begin_frame();
        engine.commit_frame();

        let mut renderer = Renderer::new(80, 24);
        renderer.set_backend(Box::new(AnsiBackend::new()));
        renderer.render_full(engine.arena_mut()).output_data
    }
}

fn wait_for_any_key(terminal: &mut Terminal) -> io::Result<()> {
    loop {
        if let Some(bettertui_terminal::TerminalEvent::Key(_)) =
            terminal.poll_event(std::time::Duration::from_millis(100))?
        {
            return Ok(());
        }
    }
}
