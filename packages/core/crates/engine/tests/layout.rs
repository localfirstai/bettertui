//! Tests for the layout module.

use std::time::Instant;

use bettertui_engine::layout::{
    AlignItems, ClipBounds, Display, FlexDirection, Gap, JustifyContent, LayoutEngine, LayoutProps,
    LayoutResult, LayoutTreeSync, PaintBounds, PaintContext, PaintFlags, Position, PositionedChild,
    PrimaryAxis, RectValues, Sizing, Viewport, build_render_tree, build_render_tree_with_viewport,
    get_objects_in_viewport,
};
use bettertui_engine::render::RenderTree;
use bettertui_engine::tree::{
    Color, Display as NodeDisplay, NamedColor, NodeArena, NodeId, NodeKind, NodeState, Overflow,
    RenderNode,
};

// ============================================================================
// TYPES TESTS
// ============================================================================

#[test]
fn default_layout_props() {
    let props = LayoutProps::default();
    assert_eq!(props.display, Display::Flex);
    assert_eq!(props.position, Position::Relative);
    assert_eq!(props.direction, FlexDirection::Column);
    assert_eq!(props.justify, JustifyContent::FlexStart);
    assert_eq!(props.align, AlignItems::Stretch);
    assert_eq!(props.flex_grow, 0.0);
    assert_eq!(props.flex_shrink, 1.0);
}

#[test]
fn sizing_variants() {
    let fixed = Sizing::Points(100.0);
    let percent = Sizing::Percent(50.0);
    let auto = Sizing::Auto;
    assert_eq!(fixed, Sizing::Points(100.0));
    assert_eq!(percent, Sizing::Percent(50.0));
    assert_eq!(auto, Sizing::Auto);
}

#[test]
fn gap_uniform() {
    let gap = Gap::uniform(5.0);
    assert_eq!(gap.row, 5.0);
    assert_eq!(gap.column, 5.0);
}

#[test]
fn rect_values_uniform() {
    let rect = RectValues::uniform(10.0);
    assert_eq!(rect.top, Some(10.0));
    assert_eq!(rect.right, Some(10.0));
    assert_eq!(rect.bottom, Some(10.0));
    assert_eq!(rect.left, Some(10.0));
}

#[test]
fn rect_values_sides() {
    let rect = RectValues::sides(1.0, 2.0, 3.0, 4.0);
    assert_eq!(rect.top, Some(1.0));
    assert_eq!(rect.right, Some(2.0));
    assert_eq!(rect.bottom, Some(3.0));
    assert_eq!(rect.left, Some(4.0));
}

// ============================================================================
// PAINT TESTS
// ============================================================================

#[test]
fn viewport_new() {
    let v = Viewport::new(0, 0, 80, 24);
    assert_eq!(v.width, 80);
    assert_eq!(v.height, 24);
}

#[test]
fn viewport_contains_rect_inside() {
    let v = Viewport::new(0, 0, 80, 24);
    assert!(v.contains_rect(5, 5, 10, 10));
}

#[test]
fn viewport_contains_rect_outside() {
    let v = Viewport::new(0, 0, 80, 24);
    assert!(!v.contains_rect(100, 100, 10, 10));
}

#[test]
fn viewport_contains_rect_partial() {
    let v = Viewport::new(10, 10, 20, 20);
    assert!(v.contains_rect(5, 5, 20, 20));
    assert!(!v.contains_rect(5, 5, 5, 5));
}

#[test]
fn viewport_intersect_overlap() {
    let a = Viewport::new(0, 0, 10, 10);
    let b = Viewport::new(5, 5, 10, 10);
    let c = a.intersect(&b).unwrap();
    assert_eq!(c.x, 5);
    assert_eq!(c.y, 5);
    assert_eq!(c.width, 5);
    assert_eq!(c.height, 5);
}

#[test]
fn viewport_intersect_no_overlap() {
    let a = Viewport::new(0, 0, 5, 5);
    let b = Viewport::new(10, 10, 5, 5);
    assert!(a.intersect(&b).is_none());
}

#[test]
fn viewport_intersect_contained() {
    let outer = Viewport::new(0, 0, 80, 24);
    let inner = Viewport::new(10, 10, 20, 10);
    let c = outer.intersect(&inner).unwrap();
    assert_eq!(c, inner);
}

#[test]
fn viewport_offset_positive() {
    let v = Viewport::new(0, 0, 80, 24);
    let o = v.offset(10, 5);
    assert_eq!(o.x, 10);
    assert_eq!(o.y, 5);
    assert_eq!(o.width, 80);
    assert_eq!(o.height, 24);
}

#[test]
fn viewport_with_padding() {
    let v = Viewport::new(10, 20, 80, 24);
    let p = v.with_padding(5);
    assert_eq!(p.x, 5);
    assert_eq!(p.y, 15);
    assert_eq!(p.width, 90);
    assert_eq!(p.height, 34);
}

#[test]
fn viewport_with_padding_zero() {
    let v = Viewport::new(0, 0, 80, 24);
    let p = v.with_padding(0);
    assert_eq!(p, v);
}

#[test]
fn viewport_with_padding_saturate() {
    let v = Viewport::new(2, 3, 5, 5);
    let p = v.with_padding(10);
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0);
    assert_eq!(p.width, 25);
    assert_eq!(p.height, 25);
}

#[test]
fn viewport_offset_clamp_zero() {
    let v = Viewport::new(5, 5, 80, 24);
    let o = v.offset(-10, -10);
    assert_eq!(o.x, 0);
    assert_eq!(o.y, 0);
}

#[test]
fn paint_bounds_default() {
    let b = PaintBounds::default();
    assert_eq!(b.x, 0);
    assert_eq!(b.width, 0);
}

#[test]
fn paint_bounds_new() {
    let b = PaintBounds::new(5, 10, 20, 15);
    assert_eq!(b.x, 5);
    assert_eq!(b.y, 10);
    assert_eq!(b.width, 20);
    assert_eq!(b.height, 15);
}

#[test]
fn paint_bounds_with_padding() {
    let b = PaintBounds::new(0, 0, 20, 10).with_padding(2, 2, 1, 1);
    assert_eq!(b.padding_left, 2);
    assert_eq!(b.content_rect().width, 16);
    assert_eq!(b.content_rect().height, 8);
}

#[test]
fn paint_bounds_contains() {
    let b = PaintBounds::new(5, 5, 10, 10);
    assert!(b.contains(5, 5));
    assert!(b.contains(14, 14));
    assert!(!b.contains(4, 5));
    assert!(!b.contains(15, 15));
}

#[test]
fn paint_bounds_intersect() {
    let a = PaintBounds::new(0, 0, 10, 10);
    let b = PaintBounds::new(5, 5, 10, 10);
    let c = a.intersect(&b).unwrap();
    assert_eq!(c.x, 5);
    assert_eq!(c.y, 5);
    assert_eq!(c.width, 5);
    assert_eq!(c.height, 5);
}

#[test]
fn paint_bounds_no_intersect() {
    let a = PaintBounds::new(0, 0, 5, 5);
    let b = PaintBounds::new(10, 10, 5, 5);
    assert!(a.intersect(&b).is_none());
}

#[test]
fn clip_bounds_new() {
    let c = ClipBounds::new(0, 0, 80, 24);
    assert_eq!(c.width, 80);
    assert_eq!(c.height, 24);
}

#[test]
fn clip_bounds_intersect() {
    let a = ClipBounds::new(0, 0, 10, 10);
    let b = ClipBounds::new(5, 5, 10, 10);
    let c = a.intersect(&b).unwrap();
    assert_eq!(c.x, 5);
    assert_eq!(c.y, 5);
    assert_eq!(c.width, 5);
    assert_eq!(c.height, 5);
}

#[test]
fn paint_flags_bitflags() {
    let flags = PaintFlags::BACKGROUND | PaintFlags::TEXT;
    assert!(flags.contains(PaintFlags::BACKGROUND));
    assert!(flags.contains(PaintFlags::TEXT));
    assert!(!flags.contains(PaintFlags::BORDER));
}

#[test]
fn paint_context_clip_stack() {
    let mut ctx = PaintContext::new(80, 24);
    assert!(ctx.current_clip().is_none());
    ctx.push_clip(ClipBounds::new(0, 0, 80, 24));
    assert!(ctx.current_clip().is_some());
    ctx.pop_clip();
    assert!(ctx.current_clip().is_none());
}

#[test]
fn paint_context_visibility() {
    let mut ctx = PaintContext::new(80, 24);
    let bounds = PaintBounds::new(5, 5, 10, 10);
    assert!(ctx.is_visible(&bounds));
    ctx.push_clip(ClipBounds::new(0, 0, 8, 8));
    assert!(ctx.is_visible(&bounds));
    ctx.pop_clip();
    let outside = PaintBounds::new(50, 50, 10, 10);
    ctx.push_clip(ClipBounds::new(0, 0, 8, 8));
    assert!(!ctx.is_visible(&outside));
    ctx.pop_clip();
}

// ============================================================================
// CULLING TESTS
// ============================================================================

fn make_sorted_children(start: u16, count: usize, step: u16, size: u16) -> Vec<PositionedChild> {
    let mut arena = NodeArena::new();
    (0..count)
        .map(|i| {
            let id = arena.insert(RenderNode::new(NodeKind::Box));
            PositionedChild {
                id,
                start: start + i as u16 * step,
                size,
            }
        })
        .collect()
}

#[test]
fn culling_empty_children() {
    let vp = Viewport::new(0, 0, 80, 24);
    let result = get_objects_in_viewport(&vp, &[], PrimaryAxis::Column);
    assert!(result.is_empty());
}

#[test]
fn culling_zero_size_viewport() {
    let children = make_sorted_children(0, 5, 5, 3);
    let vp = Viewport::new(0, 0, 0, 0);
    let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
    assert!(result.is_empty());
}

#[test]
fn culling_all_children_visible() {
    let children = make_sorted_children(0, 5, 2, 1);
    let vp = Viewport::new(0, 0, 10, 10);
    let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
    assert_eq!(result.len(), 5);
}

#[test]
fn culling_some_children_visible() {
    let children = make_sorted_children(0, 20, 2, 1);
    let vp = Viewport::new(0, 0, 10, 5);
    let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
    assert!(result.len() < 20);
    assert!(!result.is_empty());
    // With padding of 5, viewport expands to y=0..15 → first 8 children (y=0..14)
    assert_eq!(result.len(), 8);
}

#[test]
fn culling_no_children_visible() {
    let children = make_sorted_children(100, 5, 5, 3);
    let vp = Viewport::new(0, 0, 10, 10);
    let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
    assert!(result.is_empty());
}

#[test]
fn culling_spanning_object_caught() {
    let mut arena = NodeArena::new();
    let tall_id = arena.insert(RenderNode::new(NodeKind::Box));
    let children = vec![PositionedChild {
        id: tall_id,
        start: 0,
        size: 50,
    }];
    let vp = Viewport::new(0, 30, 10, 10);
    let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], tall_id);
}

#[test]
fn culling_row_axis() {
    let children = make_sorted_children(0, 10, 5, 3);
    let vp = Viewport::new(5, 0, 10, 10);
    let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Row);
    // Children at x=0..3 (idx 0), x=5..8 (idx 1), x=10..13 (idx 2), x=15..18 (idx 3)
    // Padded viewport x=0..30 → catches all 4 children in range
    assert_eq!(result.len(), 4);
}

#[test]
fn culling_small_array_bypasses_binary_search() {
    let children = make_sorted_children(0, 15, 2, 1);
    let vp = Viewport::new(0, 5, 10, 5);
    let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
    assert!(!result.is_empty());
    assert!(result.len() < 15);
}

#[test]
fn culling_gap_between_children() {
    let children = make_sorted_children(0, 3, 100, 5);
    let vp = Viewport::new(0, 50, 10, 10);
    let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
    assert!(result.is_empty());
}

#[test]
fn culling_partial_overlap_at_edge() {
    let children = make_sorted_children(0, 5, 5, 10);
    let vp = Viewport::new(0, 8, 10, 5);
    let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
    assert_eq!(
        result.len(),
        4,
        "with padding should catch 4 overlapping children"
    );
}

#[test]
fn culling_padding_includes_nearby_objects() {
    let children = make_sorted_children(10, 3, 10, 5);
    let vp = Viewport::new(0, 0, 80, 10);
    let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
    assert_eq!(
        result.len(),
        1,
        "padding should include child just below viewport"
    );
}

#[test]
fn culling_large_sparse_list() {
    let children = make_sorted_children(0, 1000, 10, 5);
    let vp = Viewport::new(0, 5000, 80, 24);
    let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
    assert!(
        result.len() <= 3,
        "should find only overlapping children from large sparse list"
    );
}

// ============================================================================
// ENGINE TESTS
// ============================================================================

fn make_ids(count: usize) -> Vec<NodeId> {
    let mut arena = NodeArena::new();
    let mut ids = Vec::new();
    for _ in 0..count {
        ids.push(arena.insert(RenderNode::new(NodeKind::Box)));
    }
    ids
}

#[test]
fn engine_new() {
    let engine = LayoutEngine::new();
    assert_eq!(engine.node_count(), 0);
}

#[test]
fn engine_register_node_and_remove() {
    let mut engine = LayoutEngine::new();
    let ids = make_ids(1);
    let id = ids[0];
    engine.register_node(id);
    assert!(engine.has_node(id));
    engine.remove_node(id);
    assert!(!engine.has_node(id));
}

#[test]
fn engine_register_container_and_compute() {
    let mut engine = LayoutEngine::new();
    let ids = make_ids(1);
    let id = ids[0];
    let props = LayoutProps {
        width: Some(Sizing::Points(80.0)),
        height: Some(Sizing::Points(24.0)),
        ..Default::default()
    };
    engine.register_container(id, &props);
    engine.compute_layout(id, 80.0, 24.0).unwrap();
    let results = engine.collect_results();
    assert!(results.contains_key(&id));
    let result = results.get(&id).unwrap();
    assert_eq!(result.width, 80);
    assert_eq!(result.height, 24);
}

#[test]
fn engine_add_child_and_compute() {
    let mut engine = LayoutEngine::new();
    let ids = make_ids(2);
    let parent = ids[0];
    let child = ids[1];
    let parent_props = LayoutProps {
        width: Some(Sizing::Points(80.0)),
        height: Some(Sizing::Points(24.0)),
        ..Default::default()
    };
    let child_props = LayoutProps {
        width: Some(Sizing::Points(20.0)),
        height: Some(Sizing::Points(10.0)),
        ..Default::default()
    };
    engine.register_container(parent, &parent_props);
    engine.register_container(child, &child_props);
    engine.add_child(parent, child);
    engine.compute_layout(parent, 80.0, 24.0).unwrap();
    let results = engine.collect_results();
    assert!(results.contains_key(&parent));
    assert!(results.contains_key(&child));
}

#[test]
fn engine_multiple_children() {
    let mut engine = LayoutEngine::new();
    let ids = make_ids(3);
    let parent = ids[0];
    let child1 = ids[1];
    let child2 = ids[2];
    let parent_props = LayoutProps {
        width: Some(Sizing::Points(80.0)),
        height: Some(Sizing::Points(24.0)),
        ..Default::default()
    };
    let child_props = LayoutProps {
        width: Some(Sizing::Points(40.0)),
        height: Some(Sizing::Points(5.0)),
        ..Default::default()
    };
    engine.register_container(parent, &parent_props);
    engine.register_container(child1, &child_props);
    engine.register_container(child2, &child_props);
    engine.add_child(parent, child1);
    engine.add_child(parent, child2);
    engine.compute_layout(parent, 80.0, 24.0).unwrap();
    let results = engine.collect_results();
    assert!(results.len() >= 3);
}

#[test]
fn engine_child_positioning_column() {
    let mut engine = LayoutEngine::new();
    let ids = make_ids(2);
    let parent = ids[0];
    let child = ids[1];
    let parent_props = LayoutProps {
        width: Some(Sizing::Points(80.0)),
        height: Some(Sizing::Points(24.0)),
        direction: FlexDirection::Column,
        ..Default::default()
    };
    let child_props = LayoutProps {
        width: Some(Sizing::Points(20.0)),
        height: Some(Sizing::Points(10.0)),
        ..Default::default()
    };
    engine.register_container(parent, &parent_props);
    engine.register_container(child, &child_props);
    engine.add_child(parent, child);
    engine.compute_layout(parent, 80.0, 24.0).unwrap();
    let results = engine.collect_results();
    let parent_r = results.get(&parent).unwrap();
    let child_r = results.get(&child).unwrap();
    assert_eq!(parent_r.width, 80);
    assert_eq!(child_r.width, 20);
    assert_eq!(child_r.height, 10);
}

#[test]
fn engine_child_positioning_row() {
    let mut engine = LayoutEngine::new();
    let ids = make_ids(2);
    let parent = ids[0];
    let child = ids[1];
    let parent_props = LayoutProps {
        width: Some(Sizing::Points(80.0)),
        height: Some(Sizing::Points(24.0)),
        direction: FlexDirection::Row,
        ..Default::default()
    };
    let child_props = LayoutProps {
        width: Some(Sizing::Points(20.0)),
        height: Some(Sizing::Points(10.0)),
        ..Default::default()
    };
    engine.register_container(parent, &parent_props);
    engine.register_container(child, &child_props);
    engine.add_child(parent, child);
    engine.compute_layout(parent, 80.0, 24.0).unwrap();
    let results = engine.collect_results();
    let parent_r = results.get(&parent).unwrap();
    let child_r = results.get(&child).unwrap();
    assert_eq!(parent_r.width, 80);
    assert_eq!(child_r.width, 20);
}

// ============================================================================
// LAYOUT TREE SYNC TESTS
// ============================================================================

#[test]
fn sync_full_basic() {
    let arena = NodeArena::new();
    let mut sync = LayoutTreeSync::new();
    sync.sync_full(&arena);
    assert_eq!(sync.node_count(), 1);
}

#[test]
fn sync_node_with_children() {
    let mut arena = NodeArena::new();
    let child = arena.insert(RenderNode::new(NodeKind::Box));
    arena.append_child(arena.root(), child).unwrap();

    let mut sync = LayoutTreeSync::new();
    sync.sync_node(&arena, arena.root());
    sync.sync_node(&arena, child);
    sync.sync_children(&arena, arena.root());
    assert_eq!(sync.node_count(), 2);
}

#[test]
fn sync_compute_layout() {
    let arena = NodeArena::new();
    let mut sync = LayoutTreeSync::new();
    sync.sync_full(&arena);
    sync.compute(arena.root(), 80, 24).unwrap();
    let results = sync.results();
    assert!(results.contains_key(&arena.root()));
}

#[test]
fn sync_remove_node_from_layout() {
    let mut arena = NodeArena::new();
    let child = arena.insert(RenderNode::new(NodeKind::Box));
    arena.append_child(arena.root(), child).unwrap();

    let mut sync = LayoutTreeSync::new();
    sync.sync_full(&arena);
    assert_eq!(sync.node_count(), 2);
    sync.remove_node(child);
    assert_eq!(sync.node_count(), 1);
}

#[test]
fn sync_adds_new_children() {
    let mut arena = NodeArena::new();
    let child = arena.insert(RenderNode::new(NodeKind::Box));
    arena.append_child(arena.root(), child).unwrap();

    let mut sync = LayoutTreeSync::new();
    sync.sync_full(&arena);
    assert_eq!(sync.node_count(), 2);
}

#[test]
fn sync_updates_existing() {
    let arena = NodeArena::new();
    let mut sync = LayoutTreeSync::new();
    sync.sync_full(&arena);
    assert_eq!(sync.node_count(), 1);
    sync.sync_full(&arena);
    assert_eq!(sync.node_count(), 1);
}

#[test]
fn sync_with_styled_node() {
    let mut arena = NodeArena::new();
    let mut node = RenderNode::new(NodeKind::Box);
    node.layout.width = Some(Sizing::Points(100.0));
    node.layout.height = Some(Sizing::Points(50.0));
    let id = arena.insert(node);
    arena.append_child(arena.root(), id).unwrap();

    let mut sync = LayoutTreeSync::new();
    sync.sync_full(&arena);
    assert_eq!(sync.node_count(), 2);
    sync.compute(arena.root(), 80, 24).unwrap();
    let results = sync.results();
    assert!(results.contains_key(&id));
}

#[test]
fn sync_children_multiple() {
    let mut arena = NodeArena::new();
    let c1 = arena.insert(RenderNode::new(NodeKind::Box));
    let c2 = arena.insert(RenderNode::new(NodeKind::Box));
    arena.append_child(arena.root(), c1).unwrap();
    arena.append_child(arena.root(), c2).unwrap();

    let mut sync = LayoutTreeSync::new();
    sync.sync_full(&arena);
    assert_eq!(sync.node_count(), 3);
}

#[test]
fn sync_remove_then_add() {
    let mut arena = NodeArena::new();
    let child = arena.insert(RenderNode::new(NodeKind::Box));
    arena.append_child(arena.root(), child).unwrap();

    let mut sync = LayoutTreeSync::new();
    sync.sync_full(&arena);
    assert_eq!(sync.node_count(), 2);
    sync.remove_node(child);
    assert_eq!(sync.node_count(), 1);
    sync.sync_node(&arena, child);
    assert_eq!(sync.node_count(), 2);
}

// ============================================================================
// LAYOUT RESULT TESTS
// ============================================================================

#[test]
fn result_default_layout_result() {
    let lr = LayoutResult::default();
    assert_eq!(lr.x, 0);
    assert_eq!(lr.y, 0);
    assert_eq!(lr.width, 0);
    assert_eq!(lr.height, 0);
}

#[test]
fn result_layout_result_new() {
    let lr = LayoutResult::new(5, 10, 20, 15);
    assert_eq!(lr.x, 5);
    assert_eq!(lr.y, 10);
    assert_eq!(lr.width, 20);
    assert_eq!(lr.height, 15);
    assert_eq!(lr.content_width, 20);
    assert_eq!(lr.content_height, 15);
}

#[test]
fn result_pixels_to_cells_rounding() {
    assert_eq!(LayoutResult::pixels_to_cells(0.0), 0);
    assert_eq!(LayoutResult::pixels_to_cells(1.0), 1);
    assert_eq!(LayoutResult::pixels_to_cells(1.4), 1);
    assert_eq!(LayoutResult::pixels_to_cells(1.5), 2);
    assert_eq!(LayoutResult::pixels_to_cells(1.6), 2);
    assert_eq!(LayoutResult::pixels_to_cells(-1.0), 0);
}

#[test]
fn result_layout_result_rect() {
    let lr = LayoutResult::new(5, 10, 20, 15);
    let rect = lr.rect();
    assert_eq!(rect.x, 5);
    assert_eq!(rect.y, 10);
    assert_eq!(rect.width, 20);
    assert_eq!(rect.height, 15);
}

#[test]
fn result_layout_result_edges() {
    let lr = LayoutResult::new(5, 10, 20, 15);
    assert_eq!(lr.right(), 25);
    assert_eq!(lr.bottom(), 25);
}

#[test]
fn result_layout_result_contains() {
    let lr = LayoutResult::new(5, 5, 10, 10);
    assert!(lr.contains(5, 5));
    assert!(lr.contains(14, 14));
    assert!(!lr.contains(4, 5));
    assert!(!lr.contains(5, 4));
    assert!(!lr.contains(15, 15));
}

#[test]
fn result_layout_result_content_rect() {
    let mut lr = LayoutResult::new(0, 0, 20, 10);
    lr.content_width = 18;
    lr.content_height = 8;
    let cr = lr.content_rect();
    assert_eq!(cr.width, 18);
    assert_eq!(cr.height, 8);
}

#[test]
fn result_layout_result_min_zero() {
    let lr = LayoutResult::new(0, 0, 0, 0);
    assert_eq!(lr.width, 0);
    assert_eq!(lr.height, 0);
}

#[test]
fn result_layout_result_padding_and_border() {
    let lr = LayoutResult::new(0, 0, 20, 10);
    assert_eq!(lr.padding_left, 0);
    assert_eq!(lr.padding_right, 0);
    assert_eq!(lr.border_left, 0);
    assert_eq!(lr.border_right, 0);
}

// ============================================================================
// BUILD TESTS
// ============================================================================

fn build_tree_with_layout() -> (RenderTree, NodeArena) {
    let mut arena = NodeArena::new();
    let child = arena.insert({
        let mut n = RenderNode::new(NodeKind::Text);
        n.text = Some("hello".into());
        n
    });
    arena.append_child(arena.root(), child).unwrap();

    let mut engine = LayoutEngine::new();
    let root = arena.root();
    let child_node = arena.get(root).unwrap();
    engine.register_container(root, &child_node.layout);
    let cn = arena.get(child).unwrap();
    engine.register_container(child, &cn.layout);
    engine.add_child(root, child);
    engine.compute_layout(root, 80.0, 24.0).unwrap();
    let results = engine.collect_results();

    let tree = build_render_tree(&arena, &results);
    (tree, arena)
}

#[test]
fn build_render_tree_basic() {
    let (tree, _) = build_tree_with_layout();
    assert!(!tree.is_empty());
    assert!(tree.root().is_some());
}

#[test]
fn build_render_tree_includes_text_node() {
    let (tree, arena) = build_tree_with_layout();
    let child = arena.children(arena.root())[0];
    let obj = tree.get(child);
    assert!(obj.is_some());
    assert_eq!(obj.unwrap().text.as_deref(), Some("hello"));
}

#[test]
fn build_render_tree_excludes_hidden() {
    let mut arena = NodeArena::new();
    let child = arena.insert({
        let mut n = RenderNode::new(NodeKind::Box);
        n.visibility.display = NodeDisplay::None;
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

    let tree = build_render_tree(&arena, &results);
    assert_eq!(tree.len(), 1);
    assert!(tree.get(child).is_none());
}

#[test]
fn build_render_tree_opacity_propagation() {
    let mut arena = NodeArena::new();
    let parent = arena.insert({
        let mut n = RenderNode::new(NodeKind::Box);
        n.visibility.opacity = 0.5;
        n
    });
    let child = arena.insert(RenderNode::new(NodeKind::Text));
    arena.append_child(arena.root(), parent).unwrap();
    arena.append_child(parent, child).unwrap();

    let mut engine = LayoutEngine::new();
    let root = arena.root();
    let rn = arena.get(root).unwrap();
    engine.register_container(root, &rn.layout);
    let pn = arena.get(parent).unwrap();
    engine.register_container(parent, &pn.layout);
    let cn = arena.get(child).unwrap();
    engine.register_container(child, &cn.layout);
    engine.add_child(root, parent);
    engine.add_child(parent, child);
    engine.compute_layout(root, 80.0, 24.0).unwrap();
    let results = engine.collect_results();

    let tree = build_render_tree(&arena, &results);
    let parent_obj = tree.get(parent).unwrap();
    let child_obj = tree.get(child).unwrap();
    assert_eq!(parent_obj.opacity, 0.5);
    assert_eq!(child_obj.opacity, 0.5);
}

fn build_tree_for_viewport_tests() -> (NodeArena, std::collections::HashMap<NodeId, LayoutResult>) {
    let mut arena = NodeArena::new();
    let child = arena.insert({
        let mut n = RenderNode::new(NodeKind::Box);
        n.layout.width = Some(Sizing::Points(10.0));
        n.layout.height = Some(Sizing::Points(5.0));
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
    (arena, results)
}

#[test]
fn build_viewport_culling_inside() {
    let (arena, results) = build_tree_for_viewport_tests();
    let vp = Viewport::new(0, 0, 80, 24);
    let tree = build_render_tree_with_viewport(&arena, &results, Some(&vp));
    let child = arena.children(arena.root())[0];
    assert!(tree.get(child).is_some());
}

#[test]
fn build_viewport_culling_outside() {
    let (arena, results) = build_tree_for_viewport_tests();
    let vp = Viewport::new(100, 100, 10, 10);
    let tree = build_render_tree_with_viewport(&arena, &results, Some(&vp));
    let child = arena.children(arena.root())[0];
    assert!(tree.get(child).is_none());
}

#[test]
fn build_viewport_culling_opacity_zero() {
    let mut arena = NodeArena::new();
    let parent = arena.insert({
        let mut n = RenderNode::new(NodeKind::Box);
        n.visibility.opacity = 0.0;
        n.layout.width = Some(Sizing::Points(10.0));
        n.layout.height = Some(Sizing::Points(5.0));
        n
    });
    let child = arena.insert(RenderNode::new(NodeKind::Box));
    arena.append_child(arena.root(), parent).unwrap();
    arena.append_child(parent, child).unwrap();

    let mut engine = LayoutEngine::new();
    let root = arena.root();
    let rn = arena.get(root).unwrap();
    engine.register_container(root, &rn.layout);
    let pn = arena.get(parent).unwrap();
    engine.register_container(parent, &pn.layout);
    let cn = arena.get(child).unwrap();
    engine.register_container(child, &cn.layout);
    engine.add_child(root, parent);
    engine.add_child(parent, child);
    engine.compute_layout(root, 80.0, 24.0).unwrap();
    let results = engine.collect_results();

    let vp = Viewport::new(0, 0, 80, 24);
    let tree = build_render_tree_with_viewport(&arena, &results, Some(&vp));
    assert!(tree.get(parent).is_none());
    assert!(tree.get(child).is_none());
}

#[test]
fn build_viewport_culling_clip_narrows() {
    let mut arena = NodeArena::new();
    let parent = arena.insert({
        let mut n = RenderNode::new(NodeKind::Box);
        n.overflow = Overflow::Hidden;
        n.layout.width = Some(Sizing::Points(10.0));
        n.layout.height = Some(Sizing::Points(5.0));
        n
    });
    let child = arena.insert({
        let mut n = RenderNode::new(NodeKind::Box);
        n.layout.width = Some(Sizing::Points(20.0));
        n.layout.height = Some(Sizing::Points(5.0));
        n
    });
    arena.append_child(arena.root(), parent).unwrap();
    arena.append_child(parent, child).unwrap();

    let mut engine = LayoutEngine::new();
    let root = arena.root();
    let rn = arena.get(root).unwrap();
    engine.register_container(root, &rn.layout);
    let pn = arena.get(parent).unwrap();
    engine.register_container(parent, &pn.layout);
    let cn = arena.get(child).unwrap();
    engine.register_container(child, &cn.layout);
    engine.add_child(root, parent);
    engine.add_child(parent, child);
    engine.compute_layout(root, 80.0, 24.0).unwrap();
    let results = engine.collect_results();

    let vp = Viewport::new(0, 0, 80, 24);
    let tree = build_render_tree_with_viewport(&arena, &results, Some(&vp));
    let _layout = results.get(&parent).unwrap();
    // Child at x=0 is within parent's 10-wide bounds, should be included
    assert!(tree.get(parent).is_some());
    assert!(tree.get(child).is_some());
}

#[test]
fn build_viewport_culling_scroll_offset() {
    let mut arena = NodeArena::new();
    let scroll = arena.insert({
        let mut n = RenderNode::new(NodeKind::Box);
        n.overflow = Overflow::Scroll;
        n.state = NodeState {
            scroll_y: 50,
            ..NodeState::default()
        };
        n.layout.width = Some(Sizing::Points(10.0));
        n.layout.height = Some(Sizing::Points(5.0));
        n
    });
    let child_outside = arena.insert({
        let mut n = RenderNode::new(NodeKind::Box);
        n.layout.width = Some(Sizing::Points(5.0));
        n.layout.height = Some(Sizing::Points(5.0));
        n
    });
    arena.append_child(arena.root(), scroll).unwrap();
    arena.append_child(scroll, child_outside).unwrap();

    let mut engine = LayoutEngine::new();
    let root = arena.root();
    let rn = arena.get(root).unwrap();
    engine.register_container(root, &rn.layout);
    let sn = arena.get(scroll).unwrap();
    engine.register_container(scroll, &sn.layout);
    let cn = arena.get(child_outside).unwrap();
    engine.register_container(child_outside, &cn.layout);
    engine.add_child(root, scroll);
    engine.add_child(scroll, child_outside);
    engine.compute_layout(root, 80.0, 24.0).unwrap();
    let results = engine.collect_results();

    // Scroll container at (0,0,10,5). With scroll_y=50, the child viewport
    // in natural coordinates is y=50..55. Child_outside at natural y=0 is
    // scrolled out of view and should be culled.
    let vp = Viewport::new(0, 0, 80, 24);
    let tree = build_render_tree_with_viewport(&arena, &results, Some(&vp));
    assert!(
        tree.get(scroll).is_some(),
        "scroll container itself is visible"
    );
    assert!(
        tree.get(child_outside).is_none(),
        "child at y=0 should be culled when scroll_y=50"
    );
}

#[test]
fn build_viewport_culling_partial_overlap() {
    let mut arena = NodeArena::new();
    let child = arena.insert({
        let mut n = RenderNode::new(NodeKind::Box);
        n.layout.width = Some(Sizing::Points(20.0));
        n.layout.height = Some(Sizing::Points(10.0));
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

    // Viewport partially overlaps child (right edge inside)
    let vp = Viewport::new(0, 0, 10, 10);
    let tree = build_render_tree_with_viewport(&arena, &results, Some(&vp));
    let child = arena.children(arena.root())[0];
    assert!(
        tree.get(child).is_some(),
        "partially overlapping child should be visible"
    );
}

#[test]
fn build_viewport_culling_deep_tree() {
    let mut arena = NodeArena::new();
    let mut prev = arena.root();
    for _ in 0..5 {
        let n = arena.insert({
            let mut n = RenderNode::new(NodeKind::Box);
            n.layout.width = Some(Sizing::Points(5.0));
            n.layout.height = Some(Sizing::Points(5.0));
            n
        });
        arena.append_child(prev, n).unwrap();
        prev = n;
    }

    let mut engine = LayoutEngine::new();
    let root = arena.root();
    let rn = arena.get(root).unwrap();
    engine.register_container(root, &rn.layout);
    for (id, _) in arena.iter() {
        let cn = arena.get(id).unwrap();
        engine.register_container(id, &cn.layout);
    }
    for (id, _) in arena.iter() {
        let children = arena.children(id);
        if !children.is_empty() {
            for &c in &children {
                engine.add_child(id, c);
            }
        }
    }
    engine.compute_layout(root, 80.0, 24.0).unwrap();
    let results = engine.collect_results();

    // Viewport that only covers first few nodes
    let vp = Viewport::new(0, 0, 80, 5);
    let tree = build_render_tree_with_viewport(&arena, &results, Some(&vp));

    // Root should be in tree
    assert!(tree.get(root).is_some());
    // At least one child in viewport
    let children = arena.children(root);
    assert!(children.iter().any(|c| tree.get(*c).is_some()));
}

#[test]
fn build_viewport_culling_nested_clip_narrows_viewport() {
    let mut arena = NodeArena::new();
    let outer = arena.insert({
        let mut n = RenderNode::new(NodeKind::Box);
        n.overflow = Overflow::Hidden;
        n.layout.width = Some(Sizing::Points(10.0));
        n.layout.height = Some(Sizing::Points(10.0));
        n
    });
    let inner = arena.insert({
        let mut n = RenderNode::new(NodeKind::Box);
        n.overflow = Overflow::Hidden;
        n.layout.width = Some(Sizing::Points(5.0));
        n.layout.height = Some(Sizing::Points(5.0));
        n
    });
    let deep_child = arena.insert({
        let mut n = RenderNode::new(NodeKind::Box);
        n.layout.width = Some(Sizing::Points(10.0));
        n.layout.height = Some(Sizing::Points(10.0));
        n
    });
    arena.append_child(arena.root(), outer).unwrap();
    arena.append_child(outer, inner).unwrap();
    arena.append_child(inner, deep_child).unwrap();

    let mut engine = LayoutEngine::new();
    let root = arena.root();
    let rn = arena.get(root).unwrap();
    engine.register_container(root, &rn.layout);
    for (id, _) in arena.iter() {
        let cn = arena.get(id).unwrap();
        engine.register_container(id, &cn.layout);
    }
    for (id, _) in arena.iter() {
        let children = arena.children(id);
        if !children.is_empty() {
            for &c in &children {
                engine.add_child(id, c);
            }
        }
    }
    engine.compute_layout(root, 80.0, 24.0).unwrap();
    let results = engine.collect_results();

    let vp = Viewport::new(0, 0, 80, 24);
    let tree = build_render_tree_with_viewport(&arena, &results, Some(&vp));

    // All nodes should be in tree as they fit within nested clips
    assert!(tree.get(outer).is_some());
    assert!(tree.get(inner).is_some());
    // deep_child at (0,0) in inner (0,0) in outer (0,0) = (0,0)
    // Size 10x10 but inner clip is 5x5, so deep child partially visible
    assert!(tree.get(deep_child).is_some());
}

#[test]
fn build_viewport_culling_outside_clip_skips_deep() {
    let mut arena = NodeArena::new();
    let outer = arena.insert({
        let mut n = RenderNode::new(NodeKind::Box);
        n.overflow = Overflow::Hidden;
        n.layout.width = Some(Sizing::Points(5.0));
        n.layout.height = Some(Sizing::Points(5.0));
        n
    });
    let deep_child = arena.insert({
        let mut n = RenderNode::new(NodeKind::Box);
        n.layout.width = Some(Sizing::Points(10.0));
        n.layout.height = Some(Sizing::Points(10.0));
        n
    });
    arena.append_child(arena.root(), outer).unwrap();
    arena.append_child(outer, deep_child).unwrap();

    let mut engine = LayoutEngine::new();
    let root = arena.root();
    let rn = arena.get(root).unwrap();
    engine.register_container(root, &rn.layout);
    for (id, _) in arena.iter() {
        let cn = arena.get(id).unwrap();
        engine.register_container(id, &cn.layout);
    }
    for (id, _) in arena.iter() {
        let children = arena.children(id);
        if !children.is_empty() {
            for &c in &children {
                engine.add_child(id, c);
            }
        }
    }
    engine.compute_layout(root, 80.0, 24.0).unwrap();
    let results = engine.collect_results();

    // Viewport far away from outer
    let vp = Viewport::new(50, 50, 10, 10);
    let tree = build_render_tree_with_viewport(&arena, &results, Some(&vp));
    assert!(
        tree.get(outer).is_none(),
        "outer outside viewport should be culled"
    );
    assert!(
        tree.get(deep_child).is_none(),
        "deep child should also be culled"
    );
}

#[test]
fn build_viewport_culling_multiple_children_some_visible() {
    let mut arena = NodeArena::new();
    let ids: Vec<NodeId> = (0..5)
        .map(|_| {
            let n = arena.insert({
                let mut n = RenderNode::new(NodeKind::Box);
                n.layout.width = Some(Sizing::Points(5.0));
                n.layout.height = Some(Sizing::Points(5.0));
                n
            });
            arena.append_child(arena.root(), n).unwrap();
            n
        })
        .collect();

    let mut engine = LayoutEngine::new();
    let root = arena.root();
    let rn = arena.get(root).unwrap();
    engine.register_container(root, &rn.layout);
    for (id, _) in arena.iter() {
        let cn = arena.get(id).unwrap();
        engine.register_container(id, &cn.layout);
    }
    for (id, _) in arena.iter() {
        let children = arena.children(id);
        if !children.is_empty() {
            for &c in &children {
                engine.add_child(id, c);
            }
        }
    }
    engine.compute_layout(root, 80.0, 24.0).unwrap();
    let results = engine.collect_results();

    // Narrow viewport that only covers first child
    let vp = Viewport::new(0, 0, 80, 3);
    let tree = build_render_tree_with_viewport(&arena, &results, Some(&vp));

    let visible = ids.iter().filter(|id| tree.get(**id).is_some()).count();
    assert!(visible > 0, "at least one child should be visible");
    assert!(
        visible < ids.len(),
        "not all children should be visible in narrow viewport"
    );
}

#[test]
fn build_viewport_culling_benchmark_large_tree() {
    let mut arena = NodeArena::new();
    let mut ids = Vec::new();
    for i in 0..200 {
        let n = arena.insert({
            let mut n = RenderNode::new(NodeKind::Box);
            n.layout.width = Some(Sizing::Points(5.0));
            n.layout.height = Some(Sizing::Points(1.0));
            if i % 2 == 0 {
                n.style.bg = Some(Color::Named(NamedColor::Blue));
            }
            n
        });
        arena.append_child(arena.root(), n).unwrap();
        ids.push(n);
    }

    let mut engine = LayoutEngine::new();
    let root = arena.root();
    let rn = arena.get(root).unwrap();
    engine.register_container(root, &rn.layout);
    for (id, _) in arena.iter() {
        let cn = arena.get(id).unwrap();
        engine.register_container(id, &cn.layout);
    }
    for (id, _) in arena.iter() {
        let children = arena.children(id);
        if !children.is_empty() {
            for &c in &children {
                engine.add_child(id, c);
            }
        }
    }
    engine.compute_layout(root, 80.0, 24.0).unwrap();
    let results = engine.collect_results();

    // Full tree build (no viewport culling)
    let start = Instant::now();
    for _ in 0..100 {
        let _tree = build_render_tree(&arena, &results);
    }
    let full_duration = start.elapsed();

    // Viewport-culled build (small viewport covering first 5 rows)
    let vp = Viewport::new(0, 0, 80, 5);
    let start = Instant::now();
    for _ in 0..100 {
        let _tree = build_render_tree_with_viewport(&arena, &results, Some(&vp));
    }
    let culled_duration = start.elapsed();

    // Verify culling: with viewport covering only 5 rows, fewer nodes should be in tree
    let full_tree = build_render_tree(&arena, &results);
    let culled_tree = build_render_tree_with_viewport(&arena, &results, Some(&vp));
    let visible_count = ids
        .iter()
        .filter(|id| culled_tree.get(**id).is_some())
        .count();
    let total_count = ids
        .iter()
        .filter(|id| full_tree.get(**id).is_some())
        .count();

    assert!(
        visible_count < total_count,
        "viewport culling should exclude some nodes (visible={}, total={})",
        visible_count,
        total_count
    );
    assert!(
        visible_count > 0,
        "viewport culling should keep some visible nodes"
    );
    assert!(
        culled_duration <= full_duration * 2,
        "culled build should not be drastically slower (culled={:?}, full={:?})",
        culled_duration,
        full_duration
    );
}

#[test]
fn build_viewport_culling_benchmark_mostly_offscreen() {
    let mut arena = NodeArena::new();
    for _ in 0..100 {
        let n = arena.insert({
            let mut n = RenderNode::new(NodeKind::Box);
            n.layout.width = Some(Sizing::Points(5.0));
            n.layout.height = Some(Sizing::Points(1.0));
            n
        });
        arena.append_child(arena.root(), n).unwrap();
    }

    let mut engine = LayoutEngine::new();
    let root = arena.root();
    let rn = arena.get(root).unwrap();
    engine.register_container(root, &rn.layout);
    for (id, _) in arena.iter() {
        let cn = arena.get(id).unwrap();
        engine.register_container(id, &cn.layout);
    }
    for (id, _) in arena.iter() {
        let children = arena.children(id);
        if !children.is_empty() {
            for &c in &children {
                engine.add_child(id, c);
            }
        }
    }
    engine.compute_layout(root, 80.0, 24.0).unwrap();
    let results = engine.collect_results();

    // Tiny viewport — should cull most of the 100 children
    let vp = Viewport::new(0, 0, 1, 1);
    let tree = build_render_tree_with_viewport(&arena, &results, Some(&vp));
    // Only root and at most 1 child should be visible in 1x1 viewport
    assert!(
        tree.len() <= 2,
        "tree should be tiny with 1x1 viewport, got {} nodes",
        tree.len()
    );
}

#[test]
fn build_render_tree_flags() {
    let mut arena = NodeArena::new();
    let child = arena.insert({
        let mut n = RenderNode::new(NodeKind::Text);
        n.text = Some("hi".into());
        n.style.bg = Some(Color::Named(NamedColor::Blue));
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

    let tree = build_render_tree(&arena, &results);
    let obj = tree.get(child).unwrap();
    assert!(obj.flags.contains(PaintFlags::BACKGROUND));
    assert!(obj.flags.contains(PaintFlags::TEXT));
}
