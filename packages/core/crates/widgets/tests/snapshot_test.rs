//! Insta snapshot tests for widget rendering.
//!
//! These tests create widget trees, render them using the engine's Renderer,
//! and capture the rendered FrameBuffer output as insta snapshots.

use bettertui_engine::input::FocusManager;
use bettertui_engine::render::Renderer;
use bettertui_engine::scheduler::Scheduler;
use bettertui_engine::tree::NodeArena;

use bettertui_widgets::{
    BoxWidget, ButtonWidget, ContainerWidget, FlexWidget, GridWidget, LabelWidget, ModalWidget,
    ProgressWidget, SeparatorWidget, SpacerWidget, SpinnerWidget, StackWidget, TabsWidget, Theme,
    TooltipWidget, WidgetContext, WidgetHost,
};

const WIDTH: u16 = 40;
const HEIGHT: u16 = 10;

fn setup() -> (WidgetHost, NodeArena, FocusManager, Scheduler, Theme) {
    (
        WidgetHost::new(),
        NodeArena::new(),
        FocusManager::new(),
        Scheduler::new(),
        Theme::default(),
    )
}

fn render(host: &mut WidgetHost, arena: &mut NodeArena) -> Renderer {
    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    // Sync widget tree into the arena using the pipeline
    let pipeline = bettertui_widgets::Pipeline::new();
    pipeline.sync_arena(host.tree(), arena);
    pipeline.build_render_tree(host.tree(), arena);

    renderer.render_full(arena);
    renderer
}

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
        terminal_size: (WIDTH, HEIGHT),
        theme,
    }
}

#[test]
fn snapshot_box_widget() {
    let (mut host, mut arena, mut focus, mut sched, theme) = setup();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);
    host.mount(Box::new(BoxWidget::new()), &mut ctx);
    let renderer = render(&mut host, &mut arena);
    insta::assert_debug_snapshot!(renderer.framebuffer());
}

#[test]
fn snapshot_label_widget_hello() {
    let (mut host, mut arena, mut focus, mut sched, theme) = setup();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);
    host.mount(Box::new(LabelWidget::new("Hello World")), &mut ctx);
    let renderer = render(&mut host, &mut arena);
    insta::assert_debug_snapshot!(renderer.framebuffer());
}

#[test]
fn snapshot_button_widget() {
    let (mut host, mut arena, mut focus, mut sched, theme) = setup();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);
    host.mount(Box::new(ButtonWidget::new("Click Me")), &mut ctx);
    let renderer = render(&mut host, &mut arena);
    insta::assert_debug_snapshot!(renderer.framebuffer());
}

#[test]
fn snapshot_container_widget() {
    let (mut host, mut arena, mut focus, mut sched, theme) = setup();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);
    host.mount(Box::new(ContainerWidget::new()), &mut ctx);
    let renderer = render(&mut host, &mut arena);
    insta::assert_debug_snapshot!(renderer.framebuffer());
}

#[test]
fn snapshot_flex_widget() {
    let (mut host, mut arena, mut focus, mut sched, theme) = setup();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);
    host.mount(Box::new(FlexWidget::new()), &mut ctx);
    let renderer = render(&mut host, &mut arena);
    insta::assert_debug_snapshot!(renderer.framebuffer());
}

#[test]
fn snapshot_spacer_widget() {
    let (mut host, mut arena, mut focus, mut sched, theme) = setup();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);
    host.mount(Box::new(SpacerWidget::new()), &mut ctx);
    let renderer = render(&mut host, &mut arena);
    insta::assert_debug_snapshot!(renderer.framebuffer());
}

#[test]
fn snapshot_separator_widget() {
    let (mut host, mut arena, mut focus, mut sched, theme) = setup();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);
    host.mount(Box::new(SeparatorWidget::new()), &mut ctx);
    let renderer = render(&mut host, &mut arena);
    insta::assert_debug_snapshot!(renderer.framebuffer());
}

#[test]
fn snapshot_spinner_widget() {
    let (mut host, mut arena, mut focus, mut sched, theme) = setup();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);
    host.mount(Box::new(SpinnerWidget::new()), &mut ctx);
    let renderer = render(&mut host, &mut arena);
    insta::assert_debug_snapshot!(renderer.framebuffer());
}

#[test]
fn snapshot_tooltip_widget() {
    let (mut host, mut arena, mut focus, mut sched, theme) = setup();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);
    host.mount(Box::new(TooltipWidget::new("this is a tooltip")), &mut ctx);
    let renderer = render(&mut host, &mut arena);
    insta::assert_debug_snapshot!(renderer.framebuffer());
}

#[test]
fn snapshot_progress_widget() {
    let (mut host, mut arena, mut focus, mut sched, theme) = setup();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);
    host.mount(Box::new(ProgressWidget::new()), &mut ctx);
    let renderer = render(&mut host, &mut arena);
    insta::assert_debug_snapshot!(renderer.framebuffer());
}

#[test]
fn snapshot_tabs_widget() {
    let (mut host, mut arena, mut focus, mut sched, theme) = setup();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);
    host.mount(Box::new(TabsWidget::new()), &mut ctx);
    let renderer = render(&mut host, &mut arena);
    insta::assert_debug_snapshot!(renderer.framebuffer());
}

#[test]
fn snapshot_stack_widget() {
    let (mut host, mut arena, mut focus, mut sched, theme) = setup();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);
    host.mount(Box::new(StackWidget::new()), &mut ctx);
    let renderer = render(&mut host, &mut arena);
    insta::assert_debug_snapshot!(renderer.framebuffer());
}

#[test]
fn snapshot_grid_widget() {
    let (mut host, mut arena, mut focus, mut sched, theme) = setup();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);
    host.mount(Box::new(GridWidget::new()), &mut ctx);
    let renderer = render(&mut host, &mut arena);
    insta::assert_debug_snapshot!(renderer.framebuffer());
}

#[test]
fn snapshot_modal_widget() {
    let (mut host, mut arena, mut focus, mut sched, theme) = setup();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);
    host.mount(Box::new(ModalWidget::new()), &mut ctx);
    let renderer = render(&mut host, &mut arena);
    insta::assert_debug_snapshot!(renderer.framebuffer());
}

#[test]
fn snapshot_nested_container_label() {
    let (mut host, mut arena, mut focus, mut sched, theme) = setup();
    let mut ctx = make_ctx(&mut arena, &mut focus, &mut sched, &theme);

    let parent = host.mount(Box::new(ContainerWidget::new()), &mut ctx);
    let child = host.mount(Box::new(LabelWidget::new("Nested")), &mut ctx);
    host.tree_mut().set_parent(child, parent);

    let renderer = render(&mut host, &mut arena);
    insta::assert_debug_snapshot!(renderer.framebuffer());
}
