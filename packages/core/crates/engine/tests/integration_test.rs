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
use bettertui_engine::tree::color::{Color, NamedColor};
use bettertui_engine::tree::visual::Display;

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
    let mut arena = make_single_node_arena();
    let frame = renderer.render_full(&mut arena);
    assert_eq!(frame.width, 80);
    assert_eq!(frame.height, 24);
    assert!(!frame.output_data.is_empty());
    assert!(!frame.dirty_regions.is_empty());
}

#[test]
fn integration_full_pipeline_parent_child() {
    let mut renderer = Renderer::new(80, 24);
    let mut arena = make_parent_child_arena();
    let frame = renderer.render_full(&mut arena);
    assert!(!frame.output_data.is_empty());
    assert!(!frame.dirty_regions.is_empty());
}

#[test]
fn integration_full_pipeline_nested() {
    let mut renderer = Renderer::new(80, 24);
    let mut arena = make_nested_arena();
    let frame = renderer.render_full(&mut arena);
    assert!(!frame.output_data.is_empty());
}

#[test]
fn integration_double_render_reduces_dirty() {
    let mut renderer = Renderer::new(80, 24);
    let mut arena = make_parent_child_arena();
    let frame1 = renderer.render_full(&mut arena);
    let frame2 = renderer.render(&mut arena);
    assert!(!frame1.output_data.is_empty());
    // Second render with no changes should be suppressed
    assert!(
        frame2.output_data.is_empty(),
        "second render with no changes should be suppressed"
    );
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
    let mut arena = make_single_node_arena();
    renderer.render_full(&mut arena);
    renderer.resize(100, 30);
    assert_eq!(renderer.dimensions(), (100, 30));
    let frame = renderer.render_full(&mut arena);
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

// ─── Phase 2 — Behavior Verification ───────────────────────────────────────

#[test]
fn p2_single_node_text_update_detection() {
    let mut diff = DirtyDiff::new();
    let mut prev = FrameBuffer::new(10, 5);
    let mut curr = FrameBuffer::new(10, 5);
    prev.swap();
    curr.swap();
    curr.set(3, 2, Cell::new('X'));
    let regions = diff.compute(&curr, &prev, 1);
    assert!(
        !regions.is_empty(),
        "single cell change must produce dirty regions"
    );
    assert!(regions[0].contains(3, 2), "region must cover changed cell");
}

#[test]
fn p2_text_change_region_covers_text_extent() {
    let mut diff = DirtyDiff::new();
    let mut prev = FrameBuffer::new(20, 5);
    let mut curr = FrameBuffer::new(20, 5);
    prev.swap();
    curr.swap();
    curr.write_str(2, 1, "Hello", Color::Default, Color::Default);
    let regions = diff.compute(&curr, &prev, 1);
    assert!(!regions.is_empty());
    assert!(regions[0].contains(2, 1), "must cover first char");
    assert!(regions[0].contains(6, 1), "must cover last char");
}

#[test]
fn p2_style_change_triggers_repaint() {
    let mut diff = DirtyDiff::new();
    let mut prev = FrameBuffer::new(10, 5);
    let mut curr = FrameBuffer::new(10, 5);
    prev.swap();
    curr.swap();
    curr.set(0, 0, Cell::new(' ').with_bg(Color::Named(NamedColor::Red)));
    let regions = diff.compute(&curr, &prev, 1);
    assert!(!regions.is_empty(), "bg color change must be dirty");
}

#[test]
fn p2_identical_frames_produce_empty_regions() {
    let mut diff = DirtyDiff::new();
    let mut buf = FrameBuffer::new(10, 10);
    buf.swap();
    buf.set(5, 5, Cell::new('A'));
    let regions = diff.compute(&buf, &buf, 1);
    assert!(regions.is_empty(), "same buffer diff must be empty");
}

#[test]
fn p2_full_repaint_covers_entire_area() {
    let mut diff = DirtyDiff::new();
    let regions = diff.compute_full_repaint(80, 24).to_vec();
    assert_eq!(regions.len(), 1);
    assert_eq!(regions[0], DirtyRegion::new(0, 0, 80, 24));
    assert_eq!(diff.total_area(), 80 * 24);
}

#[test]
fn p2_repeated_identical_render_no_extra_dirty() {
    let mut renderer = Renderer::new(40, 10);
    let mut arena = make_parent_child_arena();
    let _ = renderer.render_full(&mut arena);
    let frame2 = renderer.render(&mut arena);
    let frame3 = renderer.render(&mut arena);
    assert!(
        frame2.dirty_regions.is_empty(),
        "second identical render should have empty dirty"
    );
    assert!(
        frame3.dirty_regions.is_empty(),
        "third identical render should have empty dirty"
    );
}

#[test]
fn p2_single_node_update_after_identical_render() {
    let mut diff = DirtyDiff::new();
    let mut prev = FrameBuffer::new(10, 5);
    let mut curr = FrameBuffer::new(10, 5);
    prev.swap();
    curr.swap();

    let _ = diff.compute(&curr, &prev, 1);
    let r1 = diff.compute(&curr, &prev, 1);
    assert!(r1.is_empty(), "cached same gen: no change");

    curr.set(0, 0, Cell::new('Y'));
    let r2 = diff.compute(&curr, &prev, 2);
    assert!(!r2.is_empty(), "new gen after mutation must detect change");
}

#[test]
fn p2_large_subtree_update_dirty_region_contains_all() {
    let mut fb = FrameBuffer::new(50, 30);
    fb.swap();
    fb.fill_rect(5, 5, 30, 15, Cell::new('X'));

    let empty = FrameBuffer::new(50, 30);
    let mut diff = DirtyDiff::new();
    let regions = diff.compute(&fb, &empty, 1);
    assert!(!regions.is_empty());
    assert!(regions[0].x <= 5, "left edge must cover change start");
    assert!(regions[0].y <= 5, "top edge must cover change start");
}

#[test]
fn p2_dirty_region_does_not_overflow_bounds() {
    let mut diff = DirtyDiff::new();
    let mut prev = FrameBuffer::new(5, 5);
    let mut curr = FrameBuffer::new(5, 5);
    prev.swap();
    curr.swap();
    curr.set(4, 4, Cell::new('Z'));
    let regions = diff.compute(&curr, &prev, 1);
    if let Some(r) = regions.first() {
        assert!(r.right() <= 5, "region must not exceed fb width");
        assert!(r.bottom() <= 5, "region must not exceed fb height");
    }
}

#[test]
fn p2_multiple_cells_same_row_merged_into_single_region() {
    let mut prev = FrameBuffer::new(20, 10);
    let mut curr = FrameBuffer::new(20, 10);
    prev.swap();
    curr.swap();
    curr.set(2, 0, Cell::new('A'));
    curr.set(3, 0, Cell::new('B'));
    curr.set(4, 0, Cell::new('C'));
    let mut diff = DirtyDiff::new();
    let regions = diff.compute(&curr, &prev, 1);
    assert_eq!(regions.len(), 1, "adjacent cells on same row should merge");
    assert!(
        regions[0].width >= 3,
        "merged region must span all changed cells"
    );
}

#[test]
fn p2_full_paint_replaces_all_cells() {
    let mut painter = Painter::new(10, 5);
    let mut tree = RenderTree::new();
    let mut obj = RenderObject::new(NodeId::default());
    obj.style.bg = Some(Color::Named(NamedColor::Green));
    obj.bounds.width = 10;
    obj.bounds.height = 5;
    obj.flags = PaintFlags::BACKGROUND;
    tree.push(obj);

    let ctx = PaintContext::new(10, 5);
    painter.paint(&tree, &ctx);

    for y in 0..5 {
        for x in 0..10 {
            let cell = painter.buffer().get(x, y);
            assert_eq!(cell.bg, Color::Named(NamedColor::Green));
        }
    }
}

#[test]
fn p2_opacity_zero_hides_content() {
    let mut arena = NodeArena::new();
    let child = arena.insert({
        let mut n = bettertui_engine::tree::RenderNode::text("visible");
        n.style.fg = Some(Color::Named(NamedColor::Red));
        n
    });
    arena.append_child(arena.root(), child).unwrap();

    let mut renderer = Renderer::new(40, 10);
    let frame = renderer.render_full(&mut arena);
    assert!(
        !frame.output_data.is_empty(),
        "visible node must produce output"
    );

    let mut arena2 = NodeArena::new();
    let hidden = arena2.insert({
        let mut n = bettertui_engine::tree::RenderNode::text("hidden");
        n.style.fg = Some(Color::Named(NamedColor::Red));
        n
    });
    arena2.get_mut(arena2.root()).unwrap().visibility.display = Display::None;
    arena2.append_child(arena2.root(), hidden).unwrap();

    let mut renderer2 = Renderer::new(40, 10);
    let frame2 = renderer2.render_full(&mut arena2);
    let output_str = String::from_utf8_lossy(&frame2.output_data);
    assert!(
        !output_str.contains("hidden"),
        "hidden node text must not appear in output"
    );
}

#[test]
fn p2_dirty_diff_after_content_change() {
    let mut diff = DirtyDiff::new();
    let mut prev = FrameBuffer::new(10, 5);
    let mut curr = FrameBuffer::new(10, 5);
    prev.swap();
    curr.swap();
    curr.set(2, 2, Cell::new('A'));
    let r1 = diff.compute(&curr, &prev, 1);
    assert!(!r1.is_empty(), "first change must produce dirty regions");
    let r2 = diff.compute(&curr, &curr, 2);
    assert!(r2.is_empty(), "no change with new gen must be empty");
    curr.set(2, 2, Cell::new('B'));
    let r3 = diff.compute(&curr, &prev, 3);
    assert!(
        !r3.is_empty(),
        "different content must produce dirty regions"
    );
}

#[test]
fn p2_clip_bounds_intersect_correctly() {
    use bettertui_engine::render_object::ClipBounds;
    let outer = ClipBounds::new(0, 0, 80, 24);
    let inner = ClipBounds::new(10, 5, 20, 10);
    let clipped = outer.intersect(&inner).unwrap();
    assert_eq!(clipped.x, 10);
    assert_eq!(clipped.y, 5);
    assert_eq!(clipped.width, 20);
    assert_eq!(clipped.height, 10);
    let outside = ClipBounds::new(100, 100, 10, 10);
    assert!(
        outer.intersect(&outside).is_none(),
        "non-overlapping clips must produce None"
    );
}

#[test]
fn p2_dirty_region_merge_adjacent_horizontal() {
    let a = DirtyRegion::new(0, 0, 5, 1);
    let b = DirtyRegion::new(5, 0, 5, 1);
    assert!(a.can_merge_horizontal(&b));
    let merged = a.merge(&b);
    assert_eq!(merged.x, 0);
    assert_eq!(merged.width, 10);
    assert_eq!(merged.height, 1);
}

#[test]
fn p2_dirty_region_merge_adjacent_vertical() {
    let a = DirtyRegion::new(0, 0, 5, 3);
    let b = DirtyRegion::new(0, 3, 5, 3);
    assert!(a.can_merge_vertical(&b));
    let merged = a.merge(&b);
    assert_eq!(merged.y, 0);
    assert_eq!(merged.height, 6);
    assert_eq!(merged.width, 5);
}

#[test]
fn p2_dirty_region_no_merge_non_adjacent() {
    let a = DirtyRegion::new(0, 0, 5, 5);
    let b = DirtyRegion::new(10, 0, 5, 5);
    assert!(
        !a.can_merge_horizontal(&b),
        "non-adjacent horizontal should not merge"
    );
    assert!(
        !a.can_merge_vertical(&b),
        "non-adjacent vertical should not merge"
    );
}

#[test]
fn p2_empty_frame_produces_no_dirty() {
    let mut renderer = Renderer::new(10, 5);
    let mut arena = NodeArena::new();
    let frame = renderer.render_full(&mut arena);
    assert!(
        !frame.output_data.is_empty(),
        "first render full repaint must produce output"
    );
    let frame2 = renderer.render(&mut arena);
    assert!(
        frame2.dirty_regions.is_empty(),
        "identical frame should have no dirty regions"
    );
}

#[test]
fn p2_different_sized_buffers_diff_gracefully() {
    let mut diff = DirtyDiff::new();
    let mut small = FrameBuffer::new(5, 3);
    let mut large = FrameBuffer::new(10, 8);
    small.swap();
    large.swap();
    large.set(0, 0, Cell::new('X'));
    let regions = diff.compute(&large, &small, 1);
    assert!(
        !regions.is_empty(),
        "different sized buffers with change must produce diff"
    );
}

#[test]
fn p2_region_area_calculation() {
    let r = DirtyRegion::new(0, 0, 80, 24);
    assert_eq!(r.area(), 1920);
    let r2 = DirtyRegion::new(5, 5, 10, 10);
    assert_eq!(r2.area(), 100);
}

#[test]
fn p2_region_contains_edge() {
    let r = DirtyRegion::new(5, 5, 10, 10);
    assert!(r.contains(5, 5));
    assert!(r.contains(14, 14));
    assert!(!r.contains(15, 15));
    assert!(!r.contains(4, 5));
}
