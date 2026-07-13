use std::collections::HashMap;

use super::culling::{PositionedChild, PrimaryAxis, get_objects_in_viewport};
use super::object::RenderObject;
use super::paint::{ClipBounds, PaintBounds, PaintFlags, Viewport};
use super::tree::RenderTree;
use crate::layout::LayoutResult;
use crate::tree::NodeId;
use crate::tree::arena::NodeArena;
use crate::tree::visual::{Display, Overflow};

/// Minimum children to trigger binary search culling for scroll containers.
const BINARY_SEARCH_MIN_CHILDREN: usize = 32;

pub fn build_render_tree(
    arena: &NodeArena,
    layout_results: &HashMap<NodeId, LayoutResult>,
) -> RenderTree {
    build_render_tree_with_viewport(arena, layout_results, None)
}

pub fn build_render_tree_with_viewport(
    arena: &NodeArena,
    layout_results: &HashMap<NodeId, LayoutResult>,
    viewport: Option<&Viewport>,
) -> RenderTree {
    let mut tree = RenderTree::new();
    let root = arena.root();
    let parent_styles: HashMap<NodeId, crate::tree::style::Style> = HashMap::new();
    build_node(
        arena,
        layout_results,
        root,
        &parent_styles,
        0,
        0,
        1.0,
        viewport,
        &mut tree,
    );
    tree
}

#[allow(clippy::too_many_arguments)]
fn build_node(
    arena: &NodeArena,
    layout_results: &HashMap<NodeId, LayoutResult>,
    id: NodeId,
    parent_styles: &HashMap<NodeId, crate::tree::style::Style>,
    clip_x: u16,
    clip_y: u16,
    parent_opacity: f32,
    viewport: Option<&Viewport>,
    tree: &mut RenderTree,
) {
    let node = match arena.get(id) {
        Some(n) => n,
        None => return,
    };

    if node.visibility.display == Display::None {
        return;
    }

    let opacity = parent_opacity * node.visibility.opacity;

    if opacity == 0.0 {
        return;
    }

    let layout = layout_results.get(&id).cloned().unwrap_or_default();

    let (_current_viewport, child_viewport) = match viewport {
        None => (None, None),
        Some(vp) => {
            let narrowed = if flags_need_clip(node) {
                vp.intersect(&Viewport::new(
                    layout.x,
                    layout.y,
                    layout.width,
                    layout.height,
                ))
            } else {
                Some(*vp)
            };

            match narrowed {
                None => return, // outside clip → cull entire subtree
                Some(nv) => {
                    if !nv.contains_rect(layout.x, layout.y, layout.width, layout.height) {
                        return; // outside viewport → cull subtree
                    }
                    let cv = if node.overflow == Overflow::Scroll {
                        nv.offset(node.state.scroll_x, node.state.scroll_y)
                    } else {
                        nv
                    };
                    (Some(nv), Some(cv))
                }
            }
        }
    };

    let resolved_style = node.style.resolve(
        parent_styles
            .get(&id)
            .unwrap_or(&crate::tree::style::Style::default()),
    );

    let mut flags = PaintFlags::empty();
    if resolved_style.bg.is_some() {
        flags |= PaintFlags::BACKGROUND;
    }
    if node.text.is_some() {
        flags |= PaintFlags::TEXT;
    }
    if node.overflow == Overflow::Hidden || node.overflow == Overflow::Scroll {
        flags |= PaintFlags::NEEDS_CLIP;
    }
    if !node.visibility.clip && node.overflow == Overflow::Visible {
        // no clip needed
    } else if node.visibility.clip {
        flags |= PaintFlags::NEEDS_CLIP;
    }

    let mut bounds = PaintBounds::new(layout.x, layout.y, layout.width, layout.height);

    let padding_left = node
        .layout
        .padding
        .map(|p| p.left.unwrap_or(0.0) as u16)
        .unwrap_or(0);
    let padding_right = node
        .layout
        .padding
        .map(|p| p.right.unwrap_or(0.0) as u16)
        .unwrap_or(0);
    let padding_top = node
        .layout
        .padding
        .map(|p| p.top.unwrap_or(0.0) as u16)
        .unwrap_or(0);
    let padding_bottom = node
        .layout
        .padding
        .map(|p| p.bottom.unwrap_or(0.0) as u16)
        .unwrap_or(0);
    bounds = bounds.with_padding(padding_left, padding_right, padding_top, padding_bottom);

    let clip = if flags.contains(PaintFlags::NEEDS_CLIP) {
        Some(ClipBounds::new(
            layout.x,
            layout.y,
            layout.width,
            layout.height,
        ))
    } else {
        None
    };

    let mut obj = RenderObject::new(id);
    obj.bounds = bounds;
    obj.clip = clip;
    obj.style = resolved_style;
    obj.opacity = opacity;
    obj.z_index = node.transform.z_index;
    obj.text = node.text.clone();
    obj.overflow = node.overflow;
    obj.flags = flags;

    tree.push(obj);

    let child_clip_x = if flags.contains(PaintFlags::NEEDS_CLIP) {
        layout.x
    } else {
        clip_x
    };
    let child_clip_y = if flags.contains(PaintFlags::NEEDS_CLIP) {
        layout.y
    } else {
        clip_y
    };

    let child_ids: Vec<NodeId> = match child_viewport {
        Some(ref vp)
            if node.overflow == Overflow::Scroll
                && node.children.len() >= BINARY_SEARCH_MIN_CHILDREN =>
        {
            let primary = determine_primary_axis(&node.layout);
            let mut positioned: Vec<PositionedChild> = node
                .children
                .iter()
                .filter_map(|&cid| {
                    let layout = layout_results.get(&cid)?;
                    let (start, size) = match primary {
                        PrimaryAxis::Column => (layout.y, layout.height),
                        PrimaryAxis::Row => (layout.x, layout.width),
                    };
                    Some(PositionedChild {
                        id: cid,
                        start,
                        size,
                    })
                })
                .collect();
            positioned.sort_by_key(|c| c.start);
            get_objects_in_viewport(vp, &positioned, primary)
        }
        _ => node.children.to_vec(),
    };

    for &child_id in &child_ids {
        build_node(
            arena,
            layout_results,
            child_id,
            parent_styles,
            child_clip_x,
            child_clip_y,
            opacity,
            child_viewport.as_ref(),
            tree,
        );
    }
}

fn flags_need_clip(node: &crate::tree::render_node::RenderNode) -> bool {
    node.overflow == Overflow::Hidden || node.overflow == Overflow::Scroll || node.visibility.clip
}

fn determine_primary_axis(layout: &crate::tree::layout::LayoutProps) -> PrimaryAxis {
    match layout.direction {
        crate::tree::layout::FlexDirection::Row
        | crate::tree::layout::FlexDirection::RowReverse => PrimaryAxis::Row,
        crate::tree::layout::FlexDirection::Column
        | crate::tree::layout::FlexDirection::ColumnReverse => PrimaryAxis::Column,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::LayoutEngine;
    use crate::tree::arena::NodeArena;
    use crate::tree::interaction::NodeState;
    use crate::tree::node_kind::NodeKind;
    use crate::tree::render_node::RenderNode;
    use crate::tree::visual::Overflow;
    use std::time::Instant;

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

    fn build_tree_for_viewport_tests() -> (NodeArena, HashMap<NodeId, LayoutResult>) {
        let mut arena = NodeArena::new();
        let child = arena.insert({
            let mut n = RenderNode::new(NodeKind::Box);
            n.layout.width = Some(crate::tree::Sizing::Points(10.0));
            n.layout.height = Some(crate::tree::Sizing::Points(5.0));
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
    fn viewport_culling_inside() {
        let (arena, results) = build_tree_for_viewport_tests();
        let vp = Viewport::new(0, 0, 80, 24);
        let tree = build_render_tree_with_viewport(&arena, &results, Some(&vp));
        let child = arena.children(arena.root())[0];
        assert!(tree.get(child).is_some());
    }

    #[test]
    fn viewport_culling_outside() {
        let (arena, results) = build_tree_for_viewport_tests();
        let vp = Viewport::new(100, 100, 10, 10);
        let tree = build_render_tree_with_viewport(&arena, &results, Some(&vp));
        let child = arena.children(arena.root())[0];
        assert!(tree.get(child).is_none());
    }

    #[test]
    fn viewport_culling_opacity_zero() {
        let mut arena = NodeArena::new();
        let parent = arena.insert({
            let mut n = RenderNode::new(NodeKind::Box);
            n.visibility.opacity = 0.0;
            n.layout.width = Some(crate::tree::Sizing::Points(10.0));
            n.layout.height = Some(crate::tree::Sizing::Points(5.0));
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
    fn viewport_culling_clip_narrows() {
        let mut arena = NodeArena::new();
        let parent = arena.insert({
            let mut n = RenderNode::new(NodeKind::Box);
            n.overflow = Overflow::Hidden;
            n.layout.width = Some(crate::tree::Sizing::Points(10.0));
            n.layout.height = Some(crate::tree::Sizing::Points(5.0));
            n
        });
        let child = arena.insert({
            let mut n = RenderNode::new(NodeKind::Box);
            n.layout.width = Some(crate::tree::Sizing::Points(20.0));
            n.layout.height = Some(crate::tree::Sizing::Points(5.0));
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
    fn viewport_culling_scroll_offset() {
        let mut arena = NodeArena::new();
        let scroll = arena.insert({
            let mut n = RenderNode::new(NodeKind::Box);
            n.overflow = Overflow::Scroll;
            n.state = NodeState {
                scroll_y: 50,
                ..NodeState::default()
            };
            n.layout.width = Some(crate::tree::Sizing::Points(10.0));
            n.layout.height = Some(crate::tree::Sizing::Points(5.0));
            n
        });
        let child_outside = arena.insert({
            let mut n = RenderNode::new(NodeKind::Box);
            n.layout.width = Some(crate::tree::Sizing::Points(5.0));
            n.layout.height = Some(crate::tree::Sizing::Points(5.0));
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
    fn viewport_culling_partial_overlap() {
        let mut arena = NodeArena::new();
        let child = arena.insert({
            let mut n = RenderNode::new(NodeKind::Box);
            n.layout.width = Some(crate::tree::Sizing::Points(20.0));
            n.layout.height = Some(crate::tree::Sizing::Points(10.0));
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
    fn viewport_culling_deep_tree() {
        let mut arena = NodeArena::new();
        let mut prev = arena.root();
        for _ in 0..5 {
            let n = arena.insert({
                let mut n = RenderNode::new(NodeKind::Box);
                n.layout.width = Some(crate::tree::Sizing::Points(5.0));
                n.layout.height = Some(crate::tree::Sizing::Points(5.0));
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
    fn viewport_culling_nested_clip_narrows_viewport() {
        let mut arena = NodeArena::new();
        let outer = arena.insert({
            let mut n = RenderNode::new(NodeKind::Box);
            n.overflow = Overflow::Hidden;
            n.layout.width = Some(crate::tree::Sizing::Points(10.0));
            n.layout.height = Some(crate::tree::Sizing::Points(10.0));
            n
        });
        let inner = arena.insert({
            let mut n = RenderNode::new(NodeKind::Box);
            n.overflow = Overflow::Hidden;
            n.layout.width = Some(crate::tree::Sizing::Points(5.0));
            n.layout.height = Some(crate::tree::Sizing::Points(5.0));
            n
        });
        let deep_child = arena.insert({
            let mut n = RenderNode::new(NodeKind::Box);
            n.layout.width = Some(crate::tree::Sizing::Points(10.0));
            n.layout.height = Some(crate::tree::Sizing::Points(10.0));
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
    fn viewport_culling_outside_clip_skips_deep() {
        let mut arena = NodeArena::new();
        let outer = arena.insert({
            let mut n = RenderNode::new(NodeKind::Box);
            n.overflow = Overflow::Hidden;
            n.layout.width = Some(crate::tree::Sizing::Points(5.0));
            n.layout.height = Some(crate::tree::Sizing::Points(5.0));
            n
        });
        let deep_child = arena.insert({
            let mut n = RenderNode::new(NodeKind::Box);
            n.layout.width = Some(crate::tree::Sizing::Points(10.0));
            n.layout.height = Some(crate::tree::Sizing::Points(10.0));
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
    fn viewport_culling_multiple_children_some_visible() {
        let mut arena = NodeArena::new();
        let ids: Vec<NodeId> = (0..5)
            .map(|_| {
                let n = arena.insert({
                    let mut n = RenderNode::new(NodeKind::Box);
                    n.layout.width = Some(crate::tree::Sizing::Points(5.0));
                    n.layout.height = Some(crate::tree::Sizing::Points(5.0));
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
    fn viewport_culling_benchmark_large_tree() {
        let mut arena = NodeArena::new();
        let mut ids = Vec::new();
        for i in 0..200 {
            let n = arena.insert({
                let mut n = RenderNode::new(NodeKind::Box);
                n.layout.width = Some(crate::tree::Sizing::Points(5.0));
                n.layout.height = Some(crate::tree::Sizing::Points(1.0));
                if i % 2 == 0 {
                    n.style.bg = Some(crate::tree::color::Color::Named(
                        crate::tree::color::NamedColor::Blue,
                    ));
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
    fn viewport_culling_benchmark_mostly_offscreen() {
        let mut arena = NodeArena::new();
        for _ in 0..100 {
            let n = arena.insert({
                let mut n = RenderNode::new(NodeKind::Box);
                n.layout.width = Some(crate::tree::Sizing::Points(5.0));
                n.layout.height = Some(crate::tree::Sizing::Points(1.0));
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
            n.style.bg = Some(crate::tree::color::Color::Named(
                crate::tree::color::NamedColor::Blue,
            ));
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
}
