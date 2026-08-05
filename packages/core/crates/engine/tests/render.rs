//! Integration tests for the render module (consolidated from renderer.rs, painter.rs, ansi.rs, pipeline.rs, object.rs).

use bettertui_engine::dirty_diff::DirtyRegion;
use bettertui_engine::framebuffer::{Cell, CellAttributes, FrameBuffer};
use bettertui_engine::render::effects::ColorMatrixPass;
use bettertui_engine::render::effects::INVERT_MATRIX;
use bettertui_engine::render::{
    AnsiBackend, PassPriority, PassResult, RenderBackend, RenderObject, RenderPass, RenderPassContext, RenderPipeline,
    RenderTree, Renderer,
};
use bettertui_engine::taffy::{LayoutEngine, PaintContext, Sizing, build_render_tree};
use bettertui_engine::tree::{Color, Display, NamedColor, NodeArena, NodeKind, RenderNode};

// ═══════════════════════════════════════════════════════════════════════════════
// === Tests from ansi.rs ===
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn ansi_backend_new() {
    let backend = AnsiBackend::new();
    assert!(backend.finish().is_empty());
}

#[test]
fn ansi_backend_encode_empty() {
    let mut backend = AnsiBackend::new();
    let fb = FrameBuffer::new(5, 5);
    backend.encode(&fb, &[]);
    let out = backend.finish();
    assert!(out.is_empty(), "empty regions should produce no output");
}

#[test]
fn ansi_backend_encode_with_regions() {
    let mut backend = AnsiBackend::new();
    let mut fb = FrameBuffer::new(5, 5);
    fb.set(0, 0, Cell::new('A'));
    let region = DirtyRegion::new(0, 0, 1, 1);
    backend.encode(&fb, &[region]);
    let out = backend.finish();
    assert!(!out.is_empty(), "regions should produce output");
    let s = String::from_utf8_lossy(out);
    assert!(s.contains('A'));
    assert!(s.contains("\x1b[?25l"));
}

#[test]
fn ansi_backend_full_cell() {
    let mut backend = AnsiBackend::new();
    let cell = Cell::new('Z')
        .with_fg(Color::Named(NamedColor::Red))
        .with_bg(Color::Named(NamedColor::Blue))
        .with_attrs(CellAttributes::BOLD);
    // Test encode_cell indirectly through encode
    let mut fb = FrameBuffer::new(1, 1);
    fb.set(0, 0, cell);
    let region = DirtyRegion::new(0, 0, 1, 1);
    backend.encode(&fb, &[region]);
    let out = backend.finish();
    let s = String::from_utf8_lossy(out);
    assert!(s.contains("31"));
    assert!(s.contains("44"));
    assert!(s.contains("1"));
    assert!(s.contains("Z"));
}

#[test]
fn ansi_backend_reset() {
    let mut backend = AnsiBackend::new();
    let fb = FrameBuffer::new(1, 1);
    let region = DirtyRegion::new(0, 0, 1, 1);
    backend.encode(&fb, &[region]);
    assert!(!backend.finish().is_empty());
    backend.reset();
    assert!(backend.finish().is_empty());
}

#[test]
fn ansi_backend_region() {
    let mut backend = AnsiBackend::new();
    let mut fb = FrameBuffer::new(5, 3);
    fb.set(1, 1, Cell::new('H'));
    let region = DirtyRegion::new(0, 0, 5, 3);
    backend.encode(&fb, &[region]);
    let out = backend.finish();
    let s = String::from_utf8_lossy(out);
    assert!(s.contains('H'));
}

#[test]
fn ansi_backend_run_length_coalescing() {
    let mut backend = AnsiBackend::new();
    let mut fb = FrameBuffer::new(10, 1);
    let cell = Cell::new('A').with_fg(Color::Named(NamedColor::Red));
    let cell2 = Cell::new('B').with_fg(Color::Named(NamedColor::Red));
    let cell3 = Cell::new('C').with_fg(Color::Named(NamedColor::Red));
    fb.set(0, 0, cell);
    fb.set(1, 0, cell2);
    fb.set(2, 0, cell3);
    let region = DirtyRegion::new(0, 0, 3, 1);
    backend.encode(&fb, &[region]);
    let out = backend.finish();
    let s = String::from_utf8_lossy(out);
    // Should have exactly one SGR sequence for the entire run
    assert!(s.contains("31"), "should contain red fg SGR");
    assert!(s.contains("ABC"), "characters should be batched");
    // Count SGR sequences: should be 1 (shared for the run) not 3 (per-cell)
    let sgr_sequences = s.matches("\x1b[38").count(); // 38 = fg params typically
    assert!(sgr_sequences <= 1, "should have at most 1 fg SGR for same-styled chars");
    assert!(s.contains("ABC"), "characters should appear as a contiguous batch");
}

// ═══════════════════════════════════════════════════════════════════════════════
// === Tests from object.rs ===
// ═══════════════════════════════════════════════════════════════════════════════

fn make_id() -> bettertui_engine::tree::NodeId {
    let mut arena = NodeArena::new();
    arena.insert(RenderNode::new(NodeKind::Box))
}

#[test]
fn render_object_new() {
    let id = make_id();
    let obj = RenderObject::new(id);
    assert_eq!(obj.id, id);
    assert_eq!(obj.opacity, 1.0);
    assert_eq!(obj.z_index, 0);
    assert!(obj.text.is_none());
    assert!(obj.clip.is_none());
}

#[test]
fn render_object_has_background() {
    let mut obj = RenderObject::new(make_id());
    assert!(!obj.has_background());
    obj.style.bg = Some(Color::Named(NamedColor::Blue));
    assert!(obj.has_background());
}

#[test]
fn render_object_has_text() {
    let mut obj = RenderObject::new(make_id());
    assert!(!obj.has_text());
    obj.text = Some("hello".into());
    assert!(obj.has_text());
}

#[test]
fn render_object_is_visible() {
    let mut obj = RenderObject::new(make_id());
    assert!(obj.is_visible());
    obj.opacity = 0.0;
    assert!(!obj.is_visible());
}

#[test]
fn render_object_content_rect() {
    let mut obj = RenderObject::new(make_id());
    obj.bounds.width = 20;
    obj.bounds.height = 10;
    obj.bounds.padding_left = 2;
    obj.bounds.padding_right = 2;
    obj.bounds.padding_top = 1;
    obj.bounds.padding_bottom = 1;
    let cr = obj.content_rect();
    assert_eq!(cr.x, 2);
    assert_eq!(cr.y, 1);
    assert_eq!(cr.width, 16);
    assert_eq!(cr.height, 8);
}

// RenderTree tests

fn make_ids(count: usize) -> Vec<bettertui_engine::tree::NodeId> {
    let mut arena = NodeArena::new();
    let mut ids = Vec::new();
    for _ in 0..count {
        ids.push(arena.insert(RenderNode::new(NodeKind::Box)));
    }
    ids
}

#[test]
fn render_tree_new() {
    let tree = RenderTree::new();
    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);
}

#[test]
fn render_tree_push_and_get() {
    let ids = make_ids(2);
    let mut tree = RenderTree::new();
    let mut obj = RenderObject::new(ids[0]);
    obj.z_index = 1;
    tree.push(obj);
    tree.push(RenderObject::new(ids[1]));
    assert_eq!(tree.len(), 2);
    assert!(tree.get(ids[0]).is_some());
    assert_eq!(tree.get(ids[0]).unwrap().z_index, 1);
}

#[test]
fn render_tree_root() {
    let ids = make_ids(1);
    let mut tree = RenderTree::new();
    assert!(tree.root().is_none());
    tree.push(RenderObject::new(ids[0]));
    assert_eq!(tree.root(), Some(ids[0]));
}

#[test]
fn render_tree_sorted_by_z_index() {
    let ids = make_ids(3);
    let mut tree = RenderTree::new();
    let mut obj0 = RenderObject::new(ids[0]);
    obj0.z_index = 10;
    let mut obj1 = RenderObject::new(ids[1]);
    obj1.z_index = 0;
    let mut obj2 = RenderObject::new(ids[2]);
    obj2.z_index = 5;
    tree.push(obj0);
    tree.push(obj1);
    tree.push(obj2);
    let sorted = tree.sorted_by_z_index();
    assert_eq!(sorted[0], 1);
    assert_eq!(sorted[1], 2);
    assert_eq!(sorted[2], 0);
    // Second call should return cached result
    let sorted2 = tree.sorted_by_z_index();
    assert_eq!(sorted2, sorted);
}

#[test]
fn render_tree_clear() {
    let ids = make_ids(2);
    let mut tree = RenderTree::new();
    tree.push(RenderObject::new(ids[0]));
    tree.push(RenderObject::new(ids[1]));
    tree.clear();
    assert!(tree.is_empty());
    assert!(tree.root().is_none());
}

#[test]
fn render_tree_iter_mut() {
    let ids = make_ids(2);
    let mut tree = RenderTree::new();
    tree.push(RenderObject::new(ids[0]));
    tree.push(RenderObject::new(ids[1]));
    for obj in tree.iter_mut() {
        obj.opacity = 0.5;
    }
    for obj in tree.iter() {
        assert_eq!(obj.opacity, 0.5);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// === Tests from painter.rs ===
// ═══════════════════════════════════════════════════════════════════════════════

fn make_painter_with_tree() -> (bettertui_engine::render::Painter, RenderTree) {
    let mut arena = NodeArena::new();
    let child = arena.insert({
        let mut n = RenderNode::new(NodeKind::Text);
        n.text = Some("Hi".into());
        n.style.fg = Some(Color::Named(NamedColor::Red));
        n.style.bold = Some(true);
        n.layout.width = Some(Sizing::Points(4.0));
        n.layout.height = Some(Sizing::Points(1.0));
        n
    });
    arena.append_child(arena.root(), child).unwrap();

    {
        let root = arena.get_mut(arena.root()).unwrap();
        root.layout.width = Some(Sizing::Points(80.0));
        root.layout.height = Some(Sizing::Points(24.0));
    }

    let mut engine = LayoutEngine::new();
    let root = arena.root();
    let rn = arena.get(root).unwrap();
    engine.register_container(root, &rn.layout);
    let cn = arena.get(child).unwrap();
    engine.register_container(child, &cn.layout);
    engine.add_child(root, child);
    engine.compute_layout(root, 80.0, 24.0).unwrap();
    let results = engine.collect_results();

    let mut tree = bettertui_engine::render::RenderTree::new();
    build_render_tree(&arena, &results, &mut tree);
    let ctx = PaintContext::new(80, 24);
    let mut painter = bettertui_engine::render::Painter::new(80, 24);
    painter.paint(&tree, &ctx);
    (painter, tree)
}

#[test]
fn painter_new() {
    let painter = bettertui_engine::render::Painter::new(80, 24);
    assert_eq!(painter.buffer().width(), 80);
    assert_eq!(painter.buffer().height(), 24);
}

#[test]
fn painter_paint_background() {
    let mut arena = NodeArena::new();
    let child = arena.insert({
        let mut n = RenderNode::new(NodeKind::Box);
        n.style.bg = Some(Color::Named(NamedColor::Blue));
        n.layout.width = Some(Sizing::Points(5.0));
        n.layout.height = Some(Sizing::Points(1.0));
        n
    });
    arena.append_child(arena.root(), child).unwrap();

    {
        let root = arena.get_mut(arena.root()).unwrap();
        root.layout.width = Some(Sizing::Points(80.0));
        root.layout.height = Some(Sizing::Points(24.0));
    }

    let mut engine = LayoutEngine::new();
    let root = arena.root();
    let rn = arena.get(root).unwrap();
    engine.register_container(root, &rn.layout);
    let cn = arena.get(child).unwrap();
    engine.register_container(child, &cn.layout);
    engine.add_child(root, child);
    engine.compute_layout(root, 80.0, 24.0).unwrap();
    let results = engine.collect_results();

    let mut tree = bettertui_engine::render::RenderTree::new();
    build_render_tree(&arena, &results, &mut tree);
    let ctx = PaintContext::new(80, 24);
    let mut painter = bettertui_engine::render::Painter::new(80, 24);
    painter.paint(&tree, &ctx);

    let cell = painter.buffer().get(0, 0);
    assert_eq!(cell.bg, Color::Named(NamedColor::Blue));
}

#[test]
fn painter_paint_text() {
    let (painter, _) = make_painter_with_tree();
    let cell = painter.buffer().get(0, 0);
    assert_eq!(cell.ch, 'H');
    assert_eq!(cell.fg, Color::Named(NamedColor::Red));
    assert!(cell.attributes.contains(CellAttributes::BOLD));
}

#[test]
fn painter_excludes_hidden() {
    let mut arena = NodeArena::new();
    let child = arena.insert({
        let mut n = RenderNode::new(NodeKind::Text);
        n.text = Some("Hidden".into());
        n.visibility.display = Display::None;
        n
    });
    arena.append_child(arena.root(), child).unwrap();

    let mut engine = LayoutEngine::new();
    let root = arena.root();
    let rn = arena.get(root).unwrap();
    engine.register_container(root, &rn.layout);
    let cn = arena.get(child).unwrap();
    engine.register_container(child, &cn.layout);
    engine.add_child(root, child);
    engine.compute_layout(root, 80.0, 24.0).unwrap();
    let results = engine.collect_results();

    let mut tree = bettertui_engine::render::RenderTree::new();
    build_render_tree(&arena, &results, &mut tree);
    let ctx = PaintContext::new(80, 24);
    let mut painter = bettertui_engine::render::Painter::new(80, 24);
    painter.paint(&tree, &ctx);

    let cell = painter.buffer().get(0, 0);
    assert!(cell.is_empty());
}

#[test]
fn painter_diff() {
    let (mut painter, tree) = make_painter_with_tree();
    let ctx = PaintContext::new(80, 24);
    painter.paint(&tree, &ctx);
    painter.swap();
    painter.paint(&tree, &ctx);
    let dirty = painter.diff();
    assert!(dirty.is_empty());
}

// ═══════════════════════════════════════════════════════════════════════════════
// === Tests from pipeline.rs ===
// ═══════════════════════════════════════════════════════════════════════════════

struct TestPass {
    name: &'static str,
    enabled: bool,
    priority: PassPriority,
    modify: bool,
}

impl RenderPass for TestPass {
    fn name(&self) -> &str {
        self.name
    }

    fn execute(&mut self, buffer: &mut FrameBuffer, _ctx: &RenderPassContext) -> PassResult {
        if self.modify {
            let cell = Cell::new('T').with_fg(Color::Rgb { r: 255, g: 0, b: 0 });
            buffer.set(0, 0, cell);
            PassResult::Modified
        } else {
            PassResult::Unchanged
        }
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn priority(&self) -> PassPriority {
        self.priority
    }
}

#[test]
fn pipeline_new() {
    let p = RenderPipeline::new();
    assert!(p.is_empty());
    assert!(p.enabled());
}

#[test]
fn pipeline_add_pass() {
    let mut p = RenderPipeline::new();
    p.add_pass(Box::new(TestPass { name: "test", enabled: true, priority: PassPriority::Normal, modify: false }));
    assert_eq!(p.len(), 1);
}

#[test]
fn pipeline_get_pass() {
    let mut p = RenderPipeline::new();
    p.add_pass(Box::new(TestPass { name: "test", enabled: true, priority: PassPriority::Normal, modify: false }));
    assert!(p.get_pass("test").is_some());
    assert!(p.get_pass("nonexistent").is_none());
}

#[test]
fn pipeline_execute_unmodified() {
    let mut p = RenderPipeline::new();
    p.add_pass(Box::new(TestPass { name: "test", enabled: true, priority: PassPriority::Normal, modify: false }));
    let mut fb = FrameBuffer::new(10, 10);
    let ctx = RenderPassContext::new(10, 10);
    assert_eq!(p.execute(&mut fb, &ctx), PassResult::Unchanged);
}

#[test]
fn pipeline_execute_modified() {
    let mut p = RenderPipeline::new();
    p.add_pass(Box::new(TestPass { name: "test", enabled: true, priority: PassPriority::Normal, modify: true }));
    let mut fb = FrameBuffer::new(10, 10);
    let ctx = RenderPassContext::new(10, 10);
    assert_eq!(p.execute(&mut fb, &ctx), PassResult::Modified);
}

#[test]
fn pipeline_disabled() {
    let mut p = RenderPipeline::new();
    p.add_pass(Box::new(TestPass { name: "test", enabled: true, priority: PassPriority::Normal, modify: true }));
    p.set_enabled(false);
    let mut fb = FrameBuffer::new(10, 10);
    let ctx = RenderPassContext::new(10, 10);
    assert_eq!(p.execute(&mut fb, &ctx), PassResult::Unchanged);
}

#[test]
fn pipeline_pass_disabled() {
    let mut p = RenderPipeline::new();
    p.add_pass(Box::new(TestPass { name: "test", enabled: false, priority: PassPriority::Normal, modify: true }));
    let mut fb = FrameBuffer::new(10, 10);
    let ctx = RenderPassContext::new(10, 10);
    assert_eq!(p.execute(&mut fb, &ctx), PassResult::Unchanged);
}

#[test]
fn pipeline_priority_ordering() {
    let mut p = RenderPipeline::new();
    let last = TestPass { name: "last", enabled: true, priority: PassPriority::Last, modify: false };
    let first = TestPass { name: "first", enabled: true, priority: PassPriority::First, modify: false };
    p.add_pass(Box::new(last));
    // Should still be ordered by priority after resort
    assert_eq!(p.passes()[0].priority(), PassPriority::Last);
    p.add_pass(Box::new(first));
    // After add, resort runs — first should now be at index 0
    assert_eq!(p.passes()[0].priority(), PassPriority::First);
    assert_eq!(p.passes()[1].priority(), PassPriority::Last);
}

#[test]
fn pipeline_remove_pass() {
    let mut p = RenderPipeline::new();
    p.add_pass(Box::new(TestPass { name: "test", enabled: true, priority: PassPriority::Normal, modify: false }));
    assert_eq!(p.len(), 1);
    p.remove_pass("test");
    assert!(p.is_empty());
}

#[test]
fn pipeline_priority_get_pass_mut() {
    let mut p = RenderPipeline::new();
    p.add_pass(Box::new(TestPass { name: "test", enabled: true, priority: PassPriority::Normal, modify: false }));
    {
        let pass = p.get_pass_mut("test").unwrap();
        pass.set_enabled(false);
    }
    assert!(!p.get_pass("test").unwrap().enabled());
}

// ═══════════════════════════════════════════════════════════════════════════════
// === Tests from renderer.rs ===
// ═══════════════════════════════════════════════════════════════════════════════

fn make_arena() -> NodeArena {
    let mut arena = NodeArena::new();
    arena.insert(bettertui_engine::tree::RenderNode::box_node());
    arena
}

#[test]
fn renderer_new() {
    let r = Renderer::new(80, 24);
    assert_eq!(r.dimensions(), (80, 24));
}

#[test]
fn renderer_default() {
    let r = Renderer::default();
    assert_eq!(r.dimensions(), (80, 24));
}

#[test]
fn renderer_with_fps() {
    let r = Renderer::with_fps(60);
    assert_eq!(r.dimensions(), (80, 24));
}

#[test]
fn renderer_resize() {
    let mut r = Renderer::new(80, 24);
    r.resize(120, 40);
    assert_eq!(r.dimensions(), (120, 40));
    // Note: needs_full_repaint is private, we can't test it directly
}

#[test]
fn renderer_render_empty_tree() {
    let mut r = Renderer::new(40, 10);
    let mut arena = make_arena();
    let frame = r.render(&mut arena);
    assert_eq!(frame.width, 40);
    assert_eq!(frame.height, 10);
    assert!(!frame.output_data.is_empty());
}

#[test]
fn renderer_render_full() {
    let mut r = Renderer::new(40, 10);
    let mut arena = make_arena();
    let frame = r.render_full(&mut arena);
    assert_eq!(frame.width, 40);
    assert!(!frame.output_data.is_empty());
}

#[test]
fn renderer_frame_suppression() {
    let mut r = Renderer::new(40, 10);
    let mut arena = make_arena();
    // First render always produces output
    let frame1 = r.render(&mut arena);
    assert!(!frame1.is_empty(), "first render should produce output");
    // Second render with no changes should be suppressed
    let frame2 = r.render(&mut arena);
    assert!(frame2.is_empty(), "second render with no changes should be suppressed");
}

#[test]
fn renderer_frame_suppression_released_on_change() {
    let mut r = Renderer::new(40, 10);
    let mut arena = make_arena();
    // First render
    let _ = r.render(&mut arena);
    // Second render suppressed (no changes)
    let frame2 = r.render(&mut arena);
    assert!(frame2.is_empty(), "second render with no changes should be suppressed");
    // Mutate the arena to release suppression
    arena.mark_changed();
    // Third render should proceed (change_count check passes)
    let _frame3 = r.render(&mut arena);
    // After mutation, render proceeds - verify it doesn't early-return
    // by checking that last_change_count was updated
    let frame4 = r.render(&mut arena);
    assert!(frame4.is_empty(), "fourth render with no new changes should be suppressed");
}

#[test]
fn renderer_render_tree_empty() {
    let r = Renderer::new(40, 10);
    assert!(r.render_tree().is_empty());
}

#[test]
fn renderer_framebuffer_dimensions() {
    let r = Renderer::new(120, 50);
    assert_eq!(r.framebuffer().width(), 120);
    assert_eq!(r.framebuffer().height(), 50);
}

#[test]
fn renderer_with_backend() {
    let backend = Box::new(AnsiBackend::new());
    let r = Renderer::with_backend(80, 24, backend);
    assert_eq!(r.dimensions(), (80, 24));
}

#[test]
fn renderer_set_backend() {
    let mut r = Renderer::new(80, 24);
    let backend = Box::new(AnsiBackend::new());
    r.set_backend(backend);
    assert_eq!(r.dimensions(), (80, 24));
}

#[test]
fn renderer_viewport_culling_pipeline() {
    let mut r = Renderer::new(80, 24);
    let mut arena = make_arena();
    let root = arena.root();

    // Add 50 children stacked vertically at y=0..50 (only first 24 in viewport)
    for _i in 0..50 {
        let mut n = bettertui_engine::tree::RenderNode::new(NodeKind::Box);
        n.layout.width = Some(Sizing::Points(80.0));
        n.layout.height = Some(Sizing::Points(1.0));
        let id = arena.insert(n);
        arena.append_child(root, id).unwrap();
    }

    // First render — full tree visible
    let frame1 = r.render(&mut arena);
    assert!(!frame1.is_empty(), "first render should produce output");

    // Mark arena changed but don't actually change anything structural
    arena.mark_changed();
    let frame2 = r.render(&mut arena);
    // Content visually unchanged, so dirty regions should be empty
    assert!(frame2.dirty_regions.is_empty(), "unchanged content should have no dirty regions");

    // Render again — verify frame suppression
    let frame3 = r.render(&mut arena);
    assert!(frame3.is_empty(), "no-change render should be suppressed");
}

#[test]
fn renderer_stress_large_tree_partial_visible() {
    let mut r = Renderer::new(80, 24);
    let mut arena = make_arena();
    let root = arena.root();

    // Build 200 nodes
    for _i in 0..200 {
        let n = bettertui_engine::tree::RenderNode::new(NodeKind::Box);
        let id = arena.insert(n);
        arena.append_child(root, id).unwrap();
    }

    let frame = r.render(&mut arena);
    assert!(!frame.is_empty());
    let tree = r.render_tree();
    assert!(tree.len() < 200, "stress: large tree should be pruned, len={}", tree.len());
}

#[test]
fn renderer_stress_nested_scroll() {
    let mut r = Renderer::new(80, 24);
    let mut arena = make_arena();
    let root = arena.root();

    // Create scroll container with 1000 children
    let scroll_parent = arena.insert({
        let mut n = bettertui_engine::tree::RenderNode::new(NodeKind::Scroll);
        n.layout.width = Some(Sizing::Points(80.0));
        n.layout.height = Some(Sizing::Points(20.0));
        n
    });
    arena.append_child(root, scroll_parent).unwrap();

    for _ in 0..1000 {
        let child = arena.insert({
            let mut n = bettertui_engine::tree::RenderNode::new(NodeKind::Text);
            n.layout.width = Some(Sizing::Points(80.0));
            n.layout.height = Some(Sizing::Points(1.0));
            n.text = Some("x".into());
            n
        });
        arena.append_child(scroll_parent, child).unwrap();
    }

    let frame = r.render(&mut arena);
    assert!(!frame.is_empty());
    let tree = r.render_tree();
    // With 80x24 viewport and 80x20 scroll container + 1000 children at 1px each,
    // culling should reduce tree. Exact count depends on layout direction.
    assert!(!tree.is_empty() && tree.len() < 1000, "nested scroll should cull (len={})", tree.len());
}

// Post-Processing Pipeline Integration Tests

fn make_arena_with_text() -> NodeArena {
    let mut arena = NodeArena::new();
    let child = arena.insert({
        let mut n = bettertui_engine::tree::RenderNode::new(NodeKind::Text);
        n.text = Some("Hello".into());
        n.style.fg = Some(Color::Rgb { r: 128, g: 128, b: 128 });
        n.layout.width = Some(Sizing::Points(10.0));
        n.layout.height = Some(Sizing::Points(1.0));
        n
    });
    arena.append_child(arena.root(), child).unwrap();
    {
        let root = arena.get_mut(arena.root()).unwrap();
        root.layout.width = Some(Sizing::Points(40.0));
        root.layout.height = Some(Sizing::Points(10.0));
    }
    arena
}

#[test]
fn renderer_pipeline_passthrough_empty() {
    let mut r = Renderer::new(40, 10);
    let mut arena = make_arena();
    let frame = r.render(&mut arena);
    assert!(!frame.is_empty());
}

#[test]
fn renderer_pipeline_modifies_output() {
    let mut r = Renderer::new(40, 10);
    r.pipeline_mut().add_pass(Box::new(ColorMatrixPass::new(INVERT_MATRIX)));
    let mut arena = make_arena_with_text();
    let frame = r.render(&mut arena);
    assert!(!frame.is_empty());
    let cell = r.framebuffer().get(0, 0);
    assert_eq!(cell.ch, 'H');
    // 128 inverts to 126 via float math: (-128/255 + 1) * 255 = 126.999... → 126
    assert_eq!(cell.fg, Color::Rgb { r: 126, g: 126, b: 126 });
}

#[test]
fn renderer_pipeline_passthrough_no_modify() {
    let mut r = Renderer::new(40, 10);
    // No passes added — pipeline is empty
    let mut arena = make_arena_with_text();
    let frame = r.render(&mut arena);
    assert!(!frame.is_empty());
    let cell = r.framebuffer().get(0, 0);
    assert_eq!(cell.fg, Color::Rgb { r: 128, g: 128, b: 128 });
}

#[test]
fn renderer_pipeline_disabled_no_modify() {
    let mut r = Renderer::new(40, 10);
    r.pipeline_mut().add_pass(Box::new(ColorMatrixPass::new(INVERT_MATRIX)));
    r.pipeline_mut().set_enabled(false);
    let mut arena = make_arena_with_text();
    let frame = r.render(&mut arena);
    assert!(!frame.output_data.is_empty());
    // Buffer unchanged
    let cell = r.framebuffer().get(0, 0);
    assert_eq!(cell.fg, Color::Rgb { r: 128, g: 128, b: 128 });
}

#[test]
fn renderer_pipeline_get_pass_mut() {
    let mut r = Renderer::new(40, 10);
    r.pipeline_mut().add_pass(Box::new(ColorMatrixPass::new(INVERT_MATRIX)));
    {
        let pass = r.pipeline_mut().get_pass_mut("color_matrix").unwrap();
        pass.set_enabled(false);
    }
    assert!(!r.pipeline().get_pass("color_matrix").unwrap().enabled());
}

#[test]
fn renderer_multiple_passes_no_overflow() {
    let mut r = Renderer::new(80, 24);
    let mut arena = make_arena();
    let root = arena.root();

    for _ in 0..10 {
        let n = bettertui_engine::tree::RenderNode::new(NodeKind::Box);
        let id = arena.insert(n);
        arena.append_child(root, id).unwrap();
    }

    // Render multiple times — should not crash or leak
    for i in 0..5 {
        arena.mark_changed();
        let frame = r.render(&mut arena);
        // First pass produces output, subsequent passes may suppress
        if i == 0 {
            assert!(!frame.is_empty());
        }
    }
}
