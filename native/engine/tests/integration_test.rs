use bettertui_engine::ansi::AnsiEncoder;
use bettertui_engine::dirty_diff::{DirtyDiff, DirtyRegion};
use bettertui_engine::framebuffer::{Cell, CellAttributes, FrameBuffer};
use bettertui_engine::layout::LayoutTreeSync;
use bettertui_engine::painter::Painter;
use bettertui_engine::render_object::{PaintContext, PaintFlags, RenderObject, RenderTree};
use bettertui_engine::renderer::Renderer;
use bettertui_engine::scheduler::Scheduler;
use bettertui_engine::tree::NodeId;
use bettertui_engine::tree::arena::NodeArena;

fn make_single_node_arena() -> NodeArena {
    let mut arena = NodeArena::new();
    arena.insert(bettertui_engine::tree::RenderNode::box_node());
    arena
}

fn make_parent_child_arena() -> NodeArena {
    let mut arena = NodeArena::new();
    let root = arena.insert(bettertui_engine::tree::RenderNode::box_node());
    let child1 = arena.insert(bettertui_engine::tree::RenderNode::text("hello"));
    let child2 = arena.insert(bettertui_engine::tree::RenderNode::text("world"));
    arena.append_child(root, child1).unwrap();
    arena.append_child(root, child2).unwrap();
    arena
}

fn make_nested_arena() -> NodeArena {
    let mut arena = NodeArena::new();
    let root = arena.insert(bettertui_engine::tree::RenderNode::box_node());
    let outer = arena.insert(bettertui_engine::tree::RenderNode::box_node());
    let inner = arena.insert(bettertui_engine::tree::RenderNode::text("deep"));
    arena.append_child(root, outer).unwrap();
    arena.append_child(outer, inner).unwrap();
    arena
}

#[test]
fn integration_full_pipeline_single_node() {
    let mut renderer = Renderer::new(80, 24);
    let arena = make_single_node_arena();
    let frame = renderer.render_full(&arena);
    assert_eq!(frame.width, 80);
    assert_eq!(frame.height, 24);
    assert!(!frame.output_data.is_empty());
    assert!(!frame.dirty_regions.is_empty());
}

#[test]
fn integration_full_pipeline_parent_child() {
    let mut renderer = Renderer::new(80, 24);
    let arena = make_parent_child_arena();
    let frame = renderer.render_full(&arena);
    assert!(!frame.output_data.is_empty());
    assert!(!frame.dirty_regions.is_empty());
}

#[test]
fn integration_full_pipeline_nested() {
    let mut renderer = Renderer::new(80, 24);
    let arena = make_nested_arena();
    let frame = renderer.render_full(&arena);
    assert!(!frame.output_data.is_empty());
}

#[test]
fn integration_double_render_reduces_dirty() {
    let mut renderer = Renderer::new(80, 24);
    let arena = make_parent_child_arena();
    let frame1 = renderer.render_full(&arena);
    let frame2 = renderer.render(&arena);
    assert!(!frame1.output_data.is_empty());
    assert!(!frame2.output_data.is_empty());
}

#[test]
fn integration_framebuffer_copy_from() {
    let mut a = FrameBuffer::new(4, 3);
    a.fill_rect(0, 0, 4, 3, Cell::new('x'));
    let mut a2 = FrameBuffer::new(4, 3);
    a2.copy_from(&a);
    for y in 0..3 {
        for x in 0..4 {
            assert_eq!(a2.get(x, y).ch, 'x');
        }
    }
}

#[test]
fn integration_dirty_diff_empty_buffers() {
    let mut diff = DirtyDiff::new();
    let a = FrameBuffer::new(10, 10);
    let b = FrameBuffer::new(10, 10);
    let regions = diff.compute(&a, &b, 1);
    assert!(regions.is_empty());
}

#[test]
fn integration_dirty_diff_different_buffers() {
    let mut diff = DirtyDiff::new();
    let mut a = FrameBuffer::new(10, 10);
    let b = FrameBuffer::new(10, 10);
    a.fill_rect(2, 2, 3, 3, Cell::new('*'));
    let regions = diff.compute(&a, &b, 1);
    assert!(!regions.is_empty());
}

#[test]
fn integration_ansi_encoder_full_frame() {
    let mut encoder = AnsiEncoder::new();
    let mut buf = FrameBuffer::new(20, 5);
    buf.write_str(
        0,
        0,
        "Hello World",
        bettertui_engine::tree::color::Color::Default,
        bettertui_engine::tree::color::Color::Default,
    );
    let full_region = DirtyRegion::new(0, 0, 20, 5);
    encoder.encode(&buf, &[full_region]);
    let output = encoder.finish();
    assert!(!output.is_empty());
    assert!(output.windows(1).any(|w| w == b"\x1b"));
}

#[test]
fn integration_painter_renders_text() {
    let mut tree = RenderTree::new();
    let obj = RenderObject::new(NodeId::default());
    tree.push(obj);
    let mut painter = Painter::new(40, 10);
    let ctx = PaintContext::new(40, 10);
    painter.paint(&tree, &ctx);
    assert_eq!(painter.buffer().width(), 40);
    assert_eq!(painter.buffer().height(), 10);
}

#[test]
fn integration_scheduler_coalescing() {
    let mut sched = Scheduler::with_fps(60);
    assert_eq!(
        sched.status(),
        bettertui_engine::scheduler::FrameStatus::Idle
    );
    sched.request_frame();
    assert_eq!(
        sched.status(),
        bettertui_engine::scheduler::FrameStatus::Pending
    );
}

#[test]
fn integration_layout_sync_compute() {
    let mut sync = LayoutTreeSync::new();
    let arena = make_parent_child_arena();
    sync.sync_full(&arena);
    let root = arena.root();
    for (id, _) in arena.iter() {
        let children = arena.children(id);
        if !children.is_empty() {
            sync.sync_children(&arena, id);
        }
    }
    let result = sync.compute(root, 80, 24);
    assert!(result.is_ok());
    assert!(!sync.results().is_empty());
}

#[test]
fn integration_renderer_resize_invalidates() {
    let mut renderer = Renderer::new(80, 24);
    let arena = make_single_node_arena();
    renderer.render_full(&arena);
    renderer.resize(100, 30);
    assert_eq!(renderer.dimensions(), (100, 30));
    let frame = renderer.render_full(&arena);
    assert_eq!(frame.width, 100);
    assert_eq!(frame.height, 30);
}

#[test]
fn integration_anime_sgr_chaining() {
    let mut encoder = AnsiEncoder::new();
    let mut buf = FrameBuffer::new(10, 1);
    buf.write_str(
        0,
        0,
        "AB",
        bettertui_engine::tree::color::Color::Default,
        bettertui_engine::tree::color::Color::Default,
    );
    let region = DirtyRegion::new(0, 0, 10, 1);
    encoder.encode(&buf, &[region]);
    let output = encoder.finish();
    assert!(!output.is_empty());
    let sgr_count = output.windows(2).filter(|w| w == b"\x1b[").count();
    assert!(sgr_count > 0);
}

#[test]
fn integration_dirty_region_merge() {
    let r1 = DirtyRegion::new(0, 0, 5, 5);
    let r2 = DirtyRegion::new(5, 0, 5, 5);
    assert!(r1.can_merge_horizontal(&r2));
    let merged = r1.merge(&r2);
    assert_eq!(merged.width, 10);
    assert_eq!(merged.height, 5);
}

#[test]
fn integration_framebuffer_resize() {
    let mut buf = FrameBuffer::new(10, 10);
    buf.fill_rect(0, 0, 10, 10, Cell::new('A'));
    buf.resize(20, 20);
    assert_eq!(buf.width(), 20);
    assert_eq!(buf.height(), 20);
}

#[test]
fn integration_paint_flags_bitflags() {
    let flags = PaintFlags::BACKGROUND | PaintFlags::TEXT;
    assert!(flags.contains(PaintFlags::BACKGROUND));
    assert!(flags.contains(PaintFlags::TEXT));
    assert!(!flags.contains(PaintFlags::SCROLLBAR));
}

#[test]
fn integration_cell_attributes_bitflags() {
    let attrs = CellAttributes::BOLD | CellAttributes::ITALIC;
    assert!(attrs.contains(CellAttributes::BOLD));
    assert!(attrs.contains(CellAttributes::ITALIC));
    assert!(!attrs.contains(CellAttributes::UNDERLINE));
}
