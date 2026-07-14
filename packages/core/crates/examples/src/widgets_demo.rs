use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use bettertui_engine::input::FocusManager;
use bettertui_engine::render::{AnsiBackend, Renderer};
use bettertui_engine::scheduler::Scheduler;
use bettertui_engine::tree::{Color, NamedColor, NodeKind, RenderNode, Style};
use bettertui_engine::tree::NodeArena;
use bettertui_widgets::context::WidgetContext;
use bettertui_widgets::theme::Theme;
use bettertui_widgets::{Widget, WidgetHost, WidgetId};

use crate::util;

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
        if let Some(node) = ctx.arena.get(id.node_id()) {
            if let Some(&first_child) = node.children.first() {
                ctx.set_text(first_child, format!("{}: {}", self.label, self.count.load(Ordering::Relaxed)));
            }
        }
    }

    fn destroy(&self, id: WidgetId, ctx: &mut WidgetContext) {
        ctx.remove_subtree(id.node_id());
    }
}

pub fn run() {
    util::heading("Widgets Demo: WidgetHost, Lifecycle, and Rendering");

    let mut host = WidgetHost::new();
    host.register("Counter", || Box::new(CounterWidget::new("", Arc::new(AtomicU64::new(0)))));

    let count1 = Arc::new(AtomicU64::new(42));
    let count2 = Arc::new(AtomicU64::new(7));

    // Use a single persistent arena and renderer across all operations
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

    println!("[1] Mounted {} widgets", host.widget_count());

    // Drop ctx so we can use arena directly
    drop(ctx);

    // ── Render ──
    println!("\n[2] Rendering widget tree...");
    let mut renderer = Renderer::new(80, 6);
    renderer.set_backend(Box::new(AnsiBackend::new()));
    let frame = renderer.render_full(&mut arena);
    let ansi = String::from_utf8_lossy(&frame.output_data);
    println!("  Output ({} bytes):", frame.output_data.len());
    println!("  ─────────────────────");
    println!("{ansi}");

    // ── Update via shared atomics ──
    println!("\n[3] Updating widget state via shared AtomicU64...");
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
    let ansi2 = String::from_utf8_lossy(&frame2.output_data);
    println!("  After update (Alpha+1, Beta+10):");
    println!("{ansi2}");

    // ── Unmount ──
    println!("\n[4] Unmounting widget...");
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
    println!("  Widgets remaining: {}", host.widget_count());

    // ── Tree structure ──
    println!("\n[5] Widget tree:");
    for (wid, entry) in host.tree().iter() {
        let parent = entry.parent.map(|p| format!("{p:?}")).unwrap_or_else(|| "root".into());
        println!("  Widget {wid:?} (kind={}) parent={parent}", entry.kind);
    }
}
