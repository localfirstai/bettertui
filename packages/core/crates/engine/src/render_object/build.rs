use std::collections::HashMap;

use super::object::RenderObject;
use super::paint::{ClipBounds, PaintBounds, PaintFlags};
use super::tree::RenderTree;
use crate::layout::LayoutResult;
use crate::tree::NodeId;
use crate::tree::arena::NodeArena;
use crate::tree::visual::{Display, Overflow};

pub fn build_render_tree(
    arena: &NodeArena,
    layout_results: &HashMap<NodeId, LayoutResult>,
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

    let resolved_style = node.style.resolve(
        parent_styles
            .get(&id)
            .unwrap_or(&crate::tree::style::Style::default()),
    );

    let layout = layout_results.get(&id).cloned().unwrap_or_default();

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

    for &child_id in &node.children {
        build_node(
            arena,
            layout_results,
            child_id,
            parent_styles,
            child_clip_x,
            child_clip_y,
            opacity,
            tree,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::LayoutEngine;
    use crate::tree::arena::NodeArena;
    use crate::tree::node_kind::NodeKind;
    use crate::tree::render_node::RenderNode;

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
