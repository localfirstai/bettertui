#![allow(clippy::drop_non_drop)]

use std::io::{self, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use bettertui_engine::input::FocusManager;
use bettertui_engine::render::{AnsiBackend, Renderer};
use bettertui_engine::scheduler::Scheduler;
use bettertui_engine::tree::{Color, NamedColor, NodeKind, RenderNode, Style};
use bettertui_engine::tree::NodeArena;
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
            && let Some(&first_child) = node.children.first() {
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

    writeln!(out, "\x1b[1;97m━━━ Widgets: WidgetHost, Lifecycle, and Rendering ━━━\x1b[0m\n")?;

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
    writeln!(out, "\x1b[33m[1]\x1b[0m Mounted {} widgets", host.widget_count())?;
    drop(ctx);

    // Render
    writeln!(out, "\n\x1b[33m[2]\x1b[0m Rendering widget tree...")?;
    let mut renderer = Renderer::new(80, 6);
    renderer.set_backend(Box::new(AnsiBackend::new()));
    let frame = renderer.render_full(&mut arena);
    writeln!(out, "  Output ({} bytes):", frame.output_data.len())?;
    writeln!(out, "{}", String::from_utf8_lossy(&frame.output_data).trim_end())?;

    // Update
    writeln!(out, "\n\x1b[33m[3]\x1b[0m Updating via shared AtomicU64...")?;
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
    drop(ctx2);
    let frame2 = renderer.render(&mut arena);
    writeln!(out, "  After update (Alpha+1, Beta+10):")?;
    writeln!(out, "{}", String::from_utf8_lossy(&frame2.output_data).trim_end())?;

    // Unmount
    writeln!(out, "\n\x1b[33m[4]\x1b[0m Unmounting Beta...")?;
    let mut focus3 = FocusManager::new();
    let mut ctx3 = WidgetContext {
        arena: &mut arena,
        focus_manager: &mut focus3,
        scheduler: &mut scheduler,
        terminal_size: (80, 24),
        theme: &theme,
    };
    host.unmount(w2, &mut ctx3);
    drop(ctx3);
    writeln!(out, "  Widgets remaining: {}", host.widget_count())?;

    writeln!(out, "\n\x1b[33m[5]\x1b[0m Widget tree structure...")?;
    for (wid, entry) in host.tree().iter() {
        let parent = entry.parent.map(|p| format!("{p:?}")).unwrap_or_else(|| "root".into());
        writeln!(out, "  Widget {wid:?} (kind={}) parent={parent}", entry.kind)?;
    }

    writeln!(out, "\n\x1b[2;90mPress any key to return to menu...\x1b[0m")?;
    out.flush()?;
    wait_for_any_key(terminal)
}

fn wait_for_any_key(terminal: &mut Terminal) -> io::Result<()> {
    loop {
        if let Some(bettertui_terminal::TerminalEvent::Key(_)) = terminal.poll_event(std::time::Duration::from_millis(100))? { return Ok(()) }
    }
}
