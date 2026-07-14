//! Integration tests for the widget framework: lifecycle, events, reconciliation, and pipeline.

use bettertui_engine::input::{Event, EventResult, FocusManager, Key, KeyEvent};
use bettertui_engine::layout::LayoutProps;
use bettertui_engine::scheduler::Scheduler;
use bettertui_engine::tree::{NodeArena, NodeKind, Style};

use bettertui_widgets::{
    BoxWidget, ButtonWidget, ContainerWidget, FlexWidget, GridWidget, LabelWidget, ModalWidget,
    Pipeline, ProgressWidget, ReconcileOp, Reconciler, SeparatorWidget, SpacerWidget, SpinnerType,
    SpinnerWidget, StackWidget, TabsWidget, Theme, TooltipWidget, Widget, WidgetContext,
    WidgetHost, WidgetId, WidgetTree,
};

fn make_ctx<'a>(
    arena: &'a mut NodeArena,
    focus: &'a mut FocusManager,
    sched: &'a mut Scheduler,
    theme: &'a Theme,
) -> WidgetContext<'a> {
    WidgetContext {
        arena,
        focus_manager: focus,
        scheduler: sched,
        terminal_size: (80, 24),
        theme,
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
fn widget_host_mount_multiple() {
    let (mut host, mut arena, mut focus, mut sched, theme) = make_host();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);

    let w1 = host.mount(Box::new(BoxWidget::new()), &mut ctx);
    let w2 = host.mount(Box::new(LabelWidget::new("two")), &mut ctx);
    assert_eq!(host.widget_count(), 2);
    assert!(host.tree().get(w1).is_some());
    assert!(host.tree().get(w2).is_some());
}

#[test]
fn widget_host_unmount_untracked_no_panic() {
    let (mut host, mut arena, mut focus, mut sched, theme) = make_host();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);

    let wid = WidgetId(NodeArena::new().root());
    host.unmount(wid, &mut ctx);
}

#[test]
fn widget_host_handle_event_unmounted() {
    let (mut host, mut arena, mut focus, mut sched, theme) = make_host();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);

    let event = Event::Key(KeyEvent::new(
        Key::Character('x'),
        WidgetId::default().node_id(),
    ));
    let result = host.handle_event(WidgetId::default(), &mut ctx, &event);
    assert_eq!(result, EventResult::Ignored);
}

#[test]
fn widget_host_update_untracked() {
    let (mut host, mut arena, mut focus, mut sched, theme) = make_host();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);

    host.update(WidgetId::default(), &mut ctx);
}

#[test]
fn widget_host_registry() {
    let mut host = WidgetHost::new();
    host.register("registered_test", || -> Box<dyn Widget> {
        Box::new(LabelWidget::new("x"))
    });
    assert!(host.registry().has("registered_test"));
}

#[test]
fn widget_host_tree_access() {
    let host = WidgetHost::new();
    assert!(host.tree().children(WidgetId::default()).is_empty());
}

struct LifecycleWidget {
    kind: &'static str,
}

impl Widget for LifecycleWidget {
    fn kind(&self) -> &'static str {
        self.kind
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        WidgetId(ctx.make_box(LayoutProps::default(), Style::default()))
    }
}

#[test]
fn widget_create_and_destroy() {
    let (mut host, mut arena, mut focus, mut sched, theme) = make_host();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);

    let wid = host.mount(Box::new(LifecycleWidget { kind: "LC" }), &mut ctx);
    assert!(ctx.arena.get(wid.node_id()).is_some());

    host.unmount(wid, &mut ctx);
}

#[test]
fn all_widget_kinds() {
    let cases: Vec<(&str, Box<dyn Widget>)> = vec![
        ("Box", Box::new(BoxWidget::new())),
        ("Label", Box::new(LabelWidget::new("test"))),
        ("Button", Box::new(ButtonWidget::new("click"))),
        ("Spacer", Box::new(SpacerWidget::new())),
        ("Separator", Box::new(SeparatorWidget::new())),
        ("Spinner", Box::new(SpinnerWidget::new())),
        ("Tooltip", Box::new(TooltipWidget::new("tip"))),
        ("Container", Box::new(ContainerWidget::new())),
        ("Flex", Box::new(FlexWidget::new())),
        ("Progress", Box::new(ProgressWidget::new())),
        ("Tabs", Box::new(TabsWidget::new())),
        ("Stack", Box::new(StackWidget::new())),
        ("Grid", Box::new(GridWidget::new())),
        ("Modal", Box::new(ModalWidget::new())),
    ];
    for (expected, widget) in cases {
        assert_eq!(widget.kind(), expected);
    }
}

#[test]
fn widget_create_all_types() {
    let (mut host, mut arena, mut focus, mut sched, theme) = make_host();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);

    let widgets: Vec<Box<dyn Widget>> = vec![
        Box::new(BoxWidget::new()),
        Box::new(LabelWidget::new("lbl")),
        Box::new(ButtonWidget::new("btn")),
        Box::new(SpacerWidget::new()),
        Box::new(SeparatorWidget::new()),
        Box::new(SpinnerWidget::new().with_type(SpinnerType::Arc)),
        Box::new(TooltipWidget::new("tip")),
        Box::new(ContainerWidget::new()),
        Box::new(FlexWidget::new()),
        Box::new(ProgressWidget::new()),
    ];

    for widget in widgets {
        let wid = host.mount(widget, &mut ctx);
        assert!(ctx.arena.get(wid.node_id()).is_some());
    }
    assert_eq!(host.widget_count(), 10);
}

#[test]
fn tabs_widget_create() {
    let (mut host, mut arena, mut focus, mut sched, theme) = make_host();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);

    let wid = host.mount(Box::new(TabsWidget::new()), &mut ctx);
    let node = ctx.arena.get(wid.node_id()).unwrap();
    assert_eq!(node.kind, NodeKind::Tab);
}

#[test]
fn stack_widget_create() {
    let (mut host, mut arena, mut focus, mut sched, theme) = make_host();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);

    let wid = host.mount(Box::new(StackWidget::new()), &mut ctx);
    let node = ctx.arena.get(wid.node_id()).unwrap();
    assert_eq!(node.kind, NodeKind::Box);
}

#[test]
fn modal_widget_create() {
    let (mut host, mut arena, mut focus, mut sched, theme) = make_host();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);

    let wid = host.mount(Box::new(ModalWidget::new()), &mut ctx);
    let node = ctx.arena.get(wid.node_id()).unwrap();
    assert_eq!(node.kind, NodeKind::Modal);
}

#[test]
fn grid_widget_create() {
    let (mut host, mut arena, mut focus, mut sched, theme) = make_host();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);

    let wid = host.mount(Box::new(GridWidget::new()), &mut ctx);
    let node = ctx.arena.get(wid.node_id()).unwrap();
    assert_eq!(node.kind, NodeKind::Flex);
}

#[test]
fn box_widget_no_label_empty_text() {
    let (mut host, mut arena, mut focus, mut sched, theme) = make_host();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);

    let wid = host.mount(Box::new(BoxWidget::new()), &mut ctx);
    let node = ctx.arena.get(wid.node_id()).unwrap();
    assert!(node.text.is_none());
}

#[test]
fn spinner_widget_no_label() {
    let (mut host, mut arena, mut focus, mut sched, theme) = make_host();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);

    let wid = host.mount(Box::new(SpinnerWidget::new()), &mut ctx);
    let node = ctx.arena.get(wid.node_id()).unwrap();
    assert!(!node.text.as_deref().unwrap_or("").is_empty());
}

#[test]
fn tooltip_widget_empty_content() {
    let (mut host, mut arena, mut focus, mut sched, theme) = make_host();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);

    let wid = host.mount(Box::new(TooltipWidget::new("")), &mut ctx);
    let node = ctx.arena.get(wid.node_id()).unwrap();
    assert!(node.text.as_deref().is_some());
}

#[test]
fn reconciler_added_child() {
    let (tree1, _a1) = make_widget_tree();
    let (mut tree2, mut a2) = make_widget_tree();

    let new_nid = a2.insert(bettertui_engine::tree::RenderNode::new(NodeKind::Text));
    let new_wid = WidgetId(new_nid);
    tree2.insert(new_wid, new_nid, "Text");
    let root_wid = get_root_wid(&tree2);
    tree2.set_parent(new_wid, root_wid);

    let mut reconciler = Reconciler::new();
    let ops = reconciler.reconcile(&tree1, &tree2, root_wid);

    assert!(
        ops.iter()
            .any(|op| matches!(op, ReconcileOp::Insert { .. }))
    );
}

#[test]
fn reconciler_removed_child() {
    let (tree1, _a1) = make_widget_tree();
    let (mut tree2, _a2) = make_widget_tree();

    let root_wid = get_root_wid(&tree2);
    let children = tree2.children(root_wid);
    if let Some(&child) = children.first() {
        tree2.remove(child);
    }

    let mut reconciler = Reconciler::new();
    let ops = reconciler.reconcile(&tree1, &tree2, root_wid);

    assert!(
        ops.iter()
            .any(|op| matches!(op, ReconcileOp::Remove { .. }))
    );
}

fn make_widget_tree() -> (WidgetTree, NodeArena) {
    let mut tree = WidgetTree::new();
    let mut arena = NodeArena::new();

    let root_nid = arena.insert(bettertui_engine::tree::RenderNode::new(NodeKind::Box));
    let child1_nid = arena.insert(bettertui_engine::tree::RenderNode::new(NodeKind::Text));
    let child2_nid = arena.insert(bettertui_engine::tree::RenderNode::new(NodeKind::Box));

    let root_wid = WidgetId(root_nid);
    let child1_wid = WidgetId(child1_nid);
    let child2_wid = WidgetId(child2_nid);

    tree.insert(root_wid, root_nid, "Box");
    tree.insert(child1_wid, child1_nid, "Text");
    tree.insert(child2_wid, child2_nid, "Box");
    tree.set_parent(child1_wid, root_wid);
    tree.set_parent(child2_wid, root_wid);

    (tree, arena)
}

fn get_root_wid(tree: &WidgetTree) -> WidgetId {
    tree.iter()
        .find(|(_, entry)| entry.parent.is_none())
        .map(|(k, _)| *k)
        .unwrap()
}

#[test]
fn pipeline_new() {
    let p = Pipeline::new();
    assert!(p.needs_render());
}

#[test]
fn pipeline_mark_clear_dirty() {
    let mut p = Pipeline::new();
    p.clear_dirty();
    assert!(!p.needs_render());
    p.mark_dirty();
    assert!(p.needs_render());
}

#[test]
fn pipeline_advance_generation() {
    let mut p = Pipeline::new();
    assert_eq!(p.advance_generation(), 1);
}

#[test]
fn pipeline_build_render_tree() {
    let (tree, arena) = make_widget_tree();
    let p = Pipeline::new();
    let roots = p.build_render_tree(&tree, &arena);
    assert_eq!(roots.len(), 1);
}

#[test]
fn pipeline_sync_arena() {
    let (tree, mut arena) = make_widget_tree();
    let p = Pipeline::new();
    p.sync_arena(&tree, &mut arena);
    assert!(arena.len() >= 2);
}

#[test]
fn all_widgets_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}

    assert_send_sync(&BoxWidget::new());
    assert_send_sync(&LabelWidget::new("x"));
    assert_send_sync(&ButtonWidget::new("x"));
    assert_send_sync(&SpacerWidget::new());
    assert_send_sync(&SeparatorWidget::new());
    assert_send_sync(&SpinnerWidget::new());
    assert_send_sync(&TooltipWidget::new("x"));
    assert_send_sync(&ContainerWidget::new());
    assert_send_sync(&FlexWidget::new());
    assert_send_sync(&StackWidget::new());
    assert_send_sync(&TabsWidget::new());
    assert_send_sync(&GridWidget::new());
    assert_send_sync(&ProgressWidget::new());
    assert_send_sync(&ModalWidget::new());
}
