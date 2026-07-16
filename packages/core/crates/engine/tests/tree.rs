//! Integration tests for the tree module.
//!
//! Tests all tree types: NodeId, NodeKind, NodeArena, Color, Style, Visual,
//! Metadata, Interaction, RenderNode, and TreeError.

use bettertui_engine::tree::*;

// === color.rs ===

// Uses tree types via prelude

#[test]
fn default_color() {
    assert_eq!(Color::default(), Color::Default);
}

#[test]
fn color_equality() {
    assert_eq!(Color::Named(NamedColor::Red), Color::Named(NamedColor::Red));
    assert_ne!(Color::Named(NamedColor::Red), Color::Named(NamedColor::Blue));
    assert_eq!(Color::Rgb { r: 255, g: 0, b: 0 }, Color::Rgb { r: 255, g: 0, b: 0 });
}

#[test]
fn named_color_indices() {
    assert_eq!(NamedColor::Black.ansi_index(), 0);
    assert_eq!(NamedColor::Red.ansi_index(), 1);
    assert_eq!(NamedColor::BrightWhite.ansi_index(), 15);
}

#[test]
fn default_named_color() {
    assert_eq!(NamedColor::default(), NamedColor::White);
}

#[test]
fn color_intent() {
    assert_eq!(Color::Named(NamedColor::Red).intent(), ColorIntent::Rgb);
    assert_eq!(Color::Indexed(196).intent(), ColorIntent::Indexed);
    assert_eq!(Color::Rgb { r: 0, g: 0, b: 0 }.intent(), ColorIntent::Rgb);
    assert_eq!(Color::Default.intent(), ColorIntent::Default);
}

#[test]
fn color_parse_hex() {
    let c = Color::parse("#FF0000").unwrap();
    assert_eq!(c, Color::Rgb { r: 255, g: 0, b: 0 });

    let c = Color::parse("#00FF00").unwrap();
    assert_eq!(c, Color::Rgb { r: 0, g: 255, b: 0 });
}

#[test]
fn color_parse_named() {
    let c = Color::parse("red").unwrap();
    assert_eq!(c, Color::Named(NamedColor::Red));

    let c = Color::parse("blue").unwrap();
    assert_eq!(c, Color::Named(NamedColor::Blue));

    let c = Color::parse("purple").unwrap();
    assert_eq!(c, Color::Named(NamedColor::Magenta));
}

#[test]
fn color_lerp() {
    let c1 = Color::Rgb { r: 0, g: 0, b: 0 };
    let c2 = Color::Rgb { r: 255, g: 255, b: 255 };
    let blended = c1.lerp(&c2, 0.5);
    match blended {
        Color::Rgb { r, g, b } => {
            assert_eq!(r, 127);
            assert_eq!(g, 127);
            assert_eq!(b, 127);
        }
        _ => panic!("Expected RGB color"),
    }
}

// ─── Rgba Tests ────────────────────────────────────────────────────────

#[test]
fn rgba_new() {
    let c = Rgba::new(255, 128, 0, 200);
    assert_eq!(c.r, 255);
    assert_eq!(c.g, 128);
    assert_eq!(c.b, 0);
    assert_eq!(c.a, 200);
}

#[test]
fn rgba_rgb() {
    let c = Rgba::rgb(255, 128, 0);
    assert_eq!(c.a, 255);
}

#[test]
fn rgba_from_hex() {
    let c = Rgba::from_hex("#FF0000").unwrap();
    assert_eq!(c.r, 255);
    assert_eq!(c.g, 0);
    assert_eq!(c.b, 0);
    assert_eq!(c.a, 255);

    let c = Rgba::from_hex("#FF000080").unwrap();
    assert_eq!(c.a, 128);
}

#[test]
fn rgba_to_hex() {
    let c = Rgba::rgb(255, 128, 0);
    assert_eq!(c.to_hex(), "#FF8000");

    let c = Rgba::new(255, 128, 0, 128);
    assert_eq!(c.to_hex(), "#FF800080");
}

#[test]
fn rgba_lerp() {
    let c1 = Rgba::rgb(0, 0, 0);
    let c2 = Rgba::rgb(255, 255, 255);
    let blended = c1.lerp(&c2, 0.5);
    assert_eq!(blended.r, 127);
    assert_eq!(blended.g, 127);
    assert_eq!(blended.b, 127);
}

#[test]
fn rgba_blend_over() {
    let fg = Rgba::new(255, 0, 0, 128);
    let bg = Rgba::rgb(0, 0, 255);
    let result = fg.blend_over(&bg);
    assert!(result.r > 0);
    assert!(result.b > 0);
    assert_eq!(result.a, 255);
}

#[test]
fn named_color_to_rgb() {
    let (r, g, b) = NamedColor::Red.to_rgb();
    assert_eq!((r, g, b), (170, 0, 0));

    let (r, g, b) = NamedColor::BrightWhite.to_rgb();
    assert_eq!((r, g, b), (255, 255, 255));
}

#[test]
fn named_color_from_index() {
    assert_eq!(NamedColor::from_ansi_index(0), Some(NamedColor::Black));
    assert_eq!(NamedColor::from_ansi_index(1), Some(NamedColor::Red));
    assert_eq!(NamedColor::from_ansi_index(15), Some(NamedColor::BrightWhite));
    assert_eq!(NamedColor::from_ansi_index(16), None);
}

#[test]
fn indexed_to_rgb_conversion() {
    // Black (index 0)
    let (r, g, b) = indexed_to_rgb(0);
    assert_eq!((r, g, b), (0, 0, 0));

    // White (index 15)
    let (r, g, b) = indexed_to_rgb(15);
    assert_eq!((r, g, b), (255, 255, 255));

    // Color cube (index 16 = black)
    let (r, g, b) = indexed_to_rgb(16);
    assert_eq!((r, g, b), (0, 0, 0));

    // Grayscale (index 232)
    let (r, g, b) = indexed_to_rgb(232);
    assert_eq!((r, g, b), (8, 8, 8));
}

#[test]
fn color_to_rgba() {
    let c = Color::Named(NamedColor::Red);
    let rgba = c.to_rgba(200);
    assert_eq!(rgba.r, 170);
    assert_eq!(rgba.a, 200);
}
// end test module
// === style.rs ===

// Uses tree types via prelude
use bettertui_engine::tree::NamedColor;

#[test]
fn default_style_is_empty() {
    assert!(Style::default().is_empty());
}

#[test]
fn style_resolve_inherits_parent() {
    let parent = Style { bold: Some(true), fg: Some(Color::Named(NamedColor::Red)), ..Default::default() };
    let child = Style { italic: Some(true), ..Default::default() };

    let resolved = child.resolve(&parent);
    assert!(resolved.bold);
    assert!(resolved.italic);
    assert_eq!(resolved.fg, Some(Color::Named(NamedColor::Red)));
}

#[test]
fn style_resolve_child_overrides_parent() {
    let parent = Style { bold: Some(true), fg: Some(Color::Named(NamedColor::Red)), ..Default::default() };
    let child = Style { bold: Some(false), fg: Some(Color::Named(NamedColor::Blue)), ..Default::default() };

    let resolved = child.resolve(&parent);
    assert!(!resolved.bold);
    assert_eq!(resolved.fg, Some(Color::Named(NamedColor::Blue)));
}

#[test]
fn resolved_style_defaults() {
    let resolved = ResolvedStyle::default();
    assert!(!resolved.bold);
    assert!(!resolved.italic);
    assert!(!resolved.underline);
    assert!(!resolved.dim);
    assert!(!resolved.strikethrough);
    assert!(!resolved.inverse);
    assert!(!resolved.hidden);
    assert_eq!(resolved.border_style, BorderStyle::None);
    assert_eq!(resolved.border_width, 0);
    assert!(!resolved.rounded_corners);
    assert_eq!(resolved.overflow, Overflow::Visible);
    assert_eq!(resolved.opacity, 255);
}

#[test]
fn style_builder_fg() {
    let style = Style::new().fg(Color::Named(NamedColor::Red));
    assert_eq!(style.fg, Some(Color::Named(NamedColor::Red)));
}

#[test]
fn style_builder_bg() {
    let style = Style::new().bg(Color::Named(NamedColor::Blue));
    assert_eq!(style.bg, Some(Color::Named(NamedColor::Blue)));
}

#[test]
fn style_builder_bold() {
    let style = Style::new().bold(true);
    assert_eq!(style.bold, Some(true));
}

#[test]
fn style_builder_border() {
    let style = Style::new().border(BorderStyle::Solid, Color::Named(NamedColor::White));
    assert_eq!(style.border_style, Some(BorderStyle::Solid));
    assert_eq!(style.border_color, Some(Color::Named(NamedColor::White)));
    assert_eq!(style.border_width, Some(1));
}

#[test]
fn style_builder_rounded() {
    let style = Style::new().rounded(true);
    assert_eq!(style.rounded_corners, Some(true));
}

#[test]
fn style_builder_opacity() {
    let style = Style::new().opacity(128);
    assert_eq!(style.opacity, Some(128));
}

#[test]
fn style_resolve_border() {
    let parent = Style {
        border_style: Some(BorderStyle::Solid),
        border_color: Some(Color::Named(NamedColor::White)),
        border_width: Some(2),
        ..Default::default()
    };
    let child = Style::default();
    let resolved = child.resolve(&parent);
    assert_eq!(resolved.border_style, BorderStyle::Solid);
    assert_eq!(resolved.border_color, Some(Color::Named(NamedColor::White)));
    assert_eq!(resolved.border_width, 2);
}

#[test]
fn style_resolve_opacity() {
    let parent = Style { opacity: Some(128), ..Default::default() };
    let child = Style::default();
    let resolved = child.resolve(&parent);
    assert_eq!(resolved.opacity, 128);
}

#[test]
fn border_style_variants() {
    assert_eq!(BorderStyle::None, BorderStyle::None);
    assert_eq!(BorderStyle::Solid, BorderStyle::Solid);
    assert_eq!(BorderStyle::Dashed, BorderStyle::Dashed);
    assert_eq!(BorderStyle::Dotted, BorderStyle::Dotted);
    assert_eq!(BorderStyle::Double, BorderStyle::Double);
}

#[test]
fn overflow_variants() {
    assert_eq!(Overflow::Visible, Overflow::Visible);
    assert_eq!(Overflow::Hidden, Overflow::Hidden);
    assert_eq!(Overflow::Scroll, Overflow::Scroll);
}
// end test module
// === visual.rs ===

// Uses tree types via prelude

#[test]
fn default_visibility() {
    let vis = Visibility::default();
    assert_eq!(vis.display, Display::Flex);
    assert_eq!(vis.opacity, 1.0);
    assert!(!vis.clip);
}

#[test]
fn default_transform() {
    let t = Transform::default();
    assert_eq!(t.translate_x, 0);
    assert_eq!(t.translate_y, 0);
    assert_eq!(t.z_index, 0);
}

#[test]
fn point_creation() {
    let p = Point::new(10, 20);
    assert_eq!(p.x, 10);
    assert_eq!(p.y, 20);
}

#[test]
fn rect_contains() {
    let rect = Rect::new(5, 5, 10, 10);
    assert!(rect.contains(Point::new(7, 7)));
    assert!(rect.contains(Point::new(5, 5)));
    assert!(!rect.contains(Point::new(4, 5)));
    assert!(!rect.contains(Point::new(5, 4)));
    assert!(!rect.contains(Point::new(15, 15)));
}

#[test]
fn rect_intersects() {
    let a = Rect::new(0, 0, 10, 10);
    let b = Rect::new(5, 5, 10, 10);
    let c = Rect::new(20, 20, 10, 10);
    assert!(a.intersects(&b));
    assert!(b.intersects(&a));
    assert!(!a.intersects(&c));
    assert!(!c.intersects(&a));
}
// end test module
// === metadata.rs ===

// Uses tree types via prelude

#[test]
fn default_metadata() {
    let meta = Metadata::default();
    assert!(meta.key.is_none());
    assert!(meta.test_id.is_none());
    assert!(meta.aria_label.is_none());
    assert!(meta.tooltip.is_none());
}

#[test]
fn default_accessibility() {
    let acc = Accessibility::default();
    assert_eq!(acc.role, AriaRole::Text);
    assert!(acc.label.is_none());
    assert!(acc.description.is_none());
    assert_eq!(acc.live, AriaLive::Off);
    assert!(!acc.hidden);
}

#[test]
fn aria_role_custom() {
    let role = AriaRole::Custom(42);
    assert_eq!(role, AriaRole::Custom(42));
    assert_ne!(role, AriaRole::Custom(43));
}

#[test]
fn aria_properties_default() {
    let props = AriaProperties::default();
    assert!(props.expanded.is_none());
    assert!(props.selected.is_none());
    assert!(props.checked.is_none());
    assert!(props.disabled.is_none());
}

#[test]
fn aria_checked_variants() {
    assert_eq!(AriaChecked::False, AriaChecked::False);
    assert_eq!(AriaChecked::True, AriaChecked::True);
    assert_eq!(AriaChecked::Mixed, AriaChecked::Mixed);
}

#[test]
fn aria_pressed_variants() {
    assert_eq!(AriaPressed::False, AriaPressed::False);
    assert_eq!(AriaPressed::True, AriaPressed::True);
    assert_eq!(AriaPressed::Mixed, AriaPressed::Mixed);
}

#[test]
fn aria_current_variants() {
    assert_eq!(AriaCurrent::False, AriaCurrent::False);
    assert_eq!(AriaCurrent::Page, AriaCurrent::Page);
    assert_eq!(AriaCurrent::Step, AriaCurrent::Step);
}

#[test]
fn aria_relevant_default() {
    let rel = AriaRelevant::default();
    assert!(rel.additions);
    assert!(!rel.removals);
    assert!(rel.text);
    assert!(!rel.all);
}

#[test]
fn focus_info_default() {
    let info = FocusInfo::default();
    assert!(!info.focusable);
    assert!(info.tabindex.is_none());
    assert!(!info.focused);
}

#[test]
fn keybinding_new() {
    let kb = Keybinding::new("Enter", "Activate");
    assert_eq!(kb.key.as_ref(), "Enter");
    assert_eq!(kb.description.as_ref(), "Activate");
}

#[test]
fn keyboard_info_default() {
    let info = KeyboardInfo::default();
    assert!(info.keybindings.is_empty());
    assert!(info.roledescription.is_none());
}

#[test]
fn accessibility_with_properties() {
    let acc = Accessibility {
        role: AriaRole::Button,
        properties: AriaProperties { expanded: Some(true), pressed: Some(AriaPressed::True), ..Default::default() },
        ..Default::default()
    };
    assert_eq!(acc.role, AriaRole::Button);
    assert_eq!(acc.properties.expanded, Some(true));
    assert_eq!(acc.properties.pressed, Some(AriaPressed::True));
}
// end test module
// === interaction.rs ===

// Uses tree types via prelude

#[test]
fn default_focus_props() {
    let focus = FocusProps::default();
    assert_eq!(focus.tab_index, None);
    assert!(!focus.focusable);
    assert!(!focus.focused);
}

#[test]
fn node_state_dirty_flags() {
    let mut state = NodeState::default();
    assert!(state.dirty);
    assert!(state.layout_dirty);
    assert!(state.render_dirty);

    state.clear_dirty();
    assert!(!state.dirty);
    assert!(!state.layout_dirty);
    assert!(!state.render_dirty);

    state.mark_dirty();
    assert!(state.dirty);
    assert!(state.layout_dirty);
    assert!(state.render_dirty);
}

#[test]
fn update_flags_bitflags() {
    let flags = UpdateFlags::STYLE | UpdateFlags::LAYOUT;
    assert!(flags.contains(UpdateFlags::STYLE));
    assert!(flags.contains(UpdateFlags::LAYOUT));
    assert!(!flags.contains(UpdateFlags::TEXT));
}

#[test]
fn event_handlers_default() {
    let handlers = EventHandlers::default();
    assert!(!handlers.has_handlers);
}
// end test module
// === render_node.rs ===

// Uses tree types via prelude

#[test]
fn default_render_node() {
    let node = RenderNode::default();
    assert_eq!(node.kind, NodeKind::Box);
    assert!(node.parent.is_none());
    assert!(node.children.is_empty());
    assert!(node.text.is_none());
    assert!(!node.has_children());
    assert!(node.is_root());
}

#[test]
fn new_node_with_kind() {
    let node = RenderNode::new(NodeKind::Text);
    assert_eq!(node.kind, NodeKind::Text);
    assert!(node.is_root());
}

#[test]
fn text_node_creation() {
    let node = RenderNode::text("Hello");
    assert_eq!(node.kind, NodeKind::Text);
    assert_eq!(node.text.as_deref(), Some("Hello"));
}

#[test]
fn box_node_creation() {
    let node = RenderNode::box_node();
    assert_eq!(node.kind, NodeKind::Box);
}

#[test]
fn flex_node_creation() {
    let node = RenderNode::flex();
    assert_eq!(node.kind, NodeKind::Flex);
}

#[test]
fn set_text_marks_dirty() {
    let mut node = RenderNode::new(NodeKind::Text);
    node.state.clear_dirty();
    node.set_text("New text");
    assert!(node.state.render_dirty);
}

#[test]
fn focus_blur() {
    let mut node = RenderNode::new(NodeKind::Input);
    node.set_focusable(true);
    assert!(!node.focus.focused);

    node.focus();
    assert!(node.focus.focused);

    node.blur();
    assert!(!node.focus.focused);
}

#[test]
fn child_count() {
    let mut node = RenderNode::box_node();
    assert_eq!(node.child_count(), 0);

    // We can't easily add children without an arena,
    // but we can test the SmallVec directly
    node.children.push(NodeId::default());
    assert_eq!(node.child_count(), 1);
}
// end test module
// === arena.rs ===

// Uses tree types via prelude

fn create_test_arena() -> NodeArena {
    NodeArena::new()
}

#[test]
fn arena_new_has_root() {
    let arena = create_test_arena();
    assert_eq!(arena.len(), 1);
    assert!(!arena.is_empty());
    assert!(arena.contains(arena.root()));
}

#[test]
fn insert_node() {
    let mut arena = create_test_arena();
    let node = arena.insert(RenderNode::new(NodeKind::Text));
    assert!(arena.contains(node));
    assert_eq!(arena.len(), 2);
}

#[test]
fn get_node() {
    let mut arena = create_test_arena();
    let node = arena.insert(RenderNode::new(NodeKind::Text));
    let retrieved = arena.get(node);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().kind, NodeKind::Text);
}

#[test]
fn remove_node() {
    let mut arena = create_test_arena();
    let node = arena.insert(RenderNode::new(NodeKind::Text));
    let removed = arena.remove(node);
    assert!(removed.is_some());
    assert!(!arena.contains(node));
    assert_eq!(arena.len(), 1);
}

#[test]
fn cannot_remove_root() {
    let mut arena = create_test_arena();
    let removed = arena.remove(arena.root());
    assert!(removed.is_none());
    assert_eq!(arena.len(), 1);
}

#[test]
fn append_child() {
    let mut arena = create_test_arena();
    let child = arena.insert(RenderNode::new(NodeKind::Text));
    arena.append_child(arena.root(), child).unwrap();

    let root = arena.get(arena.root()).unwrap();
    assert_eq!(root.children.len(), 1);
    assert_eq!(root.children[0], child);

    let child_node = arena.get(child).unwrap();
    assert_eq!(child_node.parent, Some(arena.root()));
}

#[test]
fn append_multiple_children() {
    let mut arena = create_test_arena();
    let c1 = arena.insert(RenderNode::new(NodeKind::Text));
    let c2 = arena.insert(RenderNode::new(NodeKind::Box));
    let c3 = arena.insert(RenderNode::new(NodeKind::Flex));

    arena.append_child(arena.root(), c1).unwrap();
    arena.append_child(arena.root(), c2).unwrap();
    arena.append_child(arena.root(), c3).unwrap();

    let children = arena.children(arena.root());
    assert_eq!(children.len(), 3);
    assert_eq!(children[0], c1);
    assert_eq!(children[1], c2);
    assert_eq!(children[2], c3);
}

#[test]
fn insert_before() {
    let mut arena = create_test_arena();
    let c1 = arena.insert(RenderNode::new(NodeKind::Text));
    let c2 = arena.insert(RenderNode::new(NodeKind::Box));
    let c3 = arena.insert(RenderNode::new(NodeKind::Flex));

    arena.append_child(arena.root(), c1).unwrap();
    arena.append_child(arena.root(), c2).unwrap();
    arena.insert_before(c2, c3).unwrap();

    let children = arena.children(arena.root());
    assert_eq!(children.len(), 3);
    assert_eq!(children[0], c1);
    assert_eq!(children[1], c3);
    assert_eq!(children[2], c2);
}

#[test]
fn move_node() {
    let mut arena = create_test_arena();
    let parent1 = arena.insert(RenderNode::new(NodeKind::Box));
    let parent2 = arena.insert(RenderNode::new(NodeKind::Box));
    let child = arena.insert(RenderNode::new(NodeKind::Text));

    arena.append_child(arena.root(), parent1).unwrap();
    arena.append_child(arena.root(), parent2).unwrap();
    arena.append_child(parent1, child).unwrap();

    arena.move_node(child, parent2).unwrap();

    assert!(arena.children(parent1).is_empty());
    assert_eq!(arena.children(parent2).len(), 1);
    assert_eq!(arena.children(parent2)[0], child);
}

#[test]
fn detach_node() {
    let mut arena = create_test_arena();
    let child = arena.insert(RenderNode::new(NodeKind::Text));
    arena.append_child(arena.root(), child).unwrap();

    arena.detach(child);

    let root = arena.get(arena.root()).unwrap();
    assert!(root.children.is_empty());

    let child_node = arena.get(child).unwrap();
    assert!(child_node.parent.is_none());
}

#[test]
fn remove_subtree() {
    let mut arena = create_test_arena();
    let parent = arena.insert(RenderNode::new(NodeKind::Box));
    let child1 = arena.insert(RenderNode::new(NodeKind::Text));
    let child2 = arena.insert(RenderNode::new(NodeKind::Text));
    let grandchild = arena.insert(RenderNode::new(NodeKind::Text));

    arena.append_child(arena.root(), parent).unwrap();
    arena.append_child(parent, child1).unwrap();
    arena.append_child(parent, child2).unwrap();
    arena.append_child(child1, grandchild).unwrap();

    arena.remove_subtree(parent);

    assert!(!arena.contains(parent));
    assert!(!arena.contains(child1));
    assert!(!arena.contains(child2));
    assert!(!arena.contains(grandchild));
}

#[test]
fn replace_node() {
    let mut arena = create_test_arena();
    let old = arena.insert(RenderNode::new(NodeKind::Text));
    let new = arena.insert(RenderNode::new(NodeKind::Box));
    let child = arena.insert(RenderNode::new(NodeKind::Text));

    arena.append_child(arena.root(), old).unwrap();
    arena.append_child(old, child).unwrap();

    arena.replace_node(old, new).unwrap();

    assert!(!arena.contains(old));
    assert!(arena.contains(new));

    let root = arena.get(arena.root()).unwrap();
    assert!(root.children.contains(&new));

    let new_node = arena.get(new).unwrap();
    assert_eq!(new_node.parent, Some(arena.root()));
    assert!(new_node.children.contains(&child));
}

#[test]
fn descendants() {
    let mut arena = create_test_arena();
    let a = arena.insert(RenderNode::new(NodeKind::Box));
    let b = arena.insert(RenderNode::new(NodeKind::Box));
    let c = arena.insert(RenderNode::new(NodeKind::Text));
    let d = arena.insert(RenderNode::new(NodeKind::Text));

    arena.append_child(arena.root(), a).unwrap();
    arena.append_child(a, b).unwrap();
    arena.append_child(a, c).unwrap();
    arena.append_child(b, d).unwrap();

    let descendants = arena.descendants(arena.root());
    assert_eq!(descendants.len(), 4);
    assert!(descendants.contains(&a));
    assert!(descendants.contains(&b));
    assert!(descendants.contains(&c));
    assert!(descendants.contains(&d));
}

#[test]
fn ancestors() {
    let mut arena = create_test_arena();
    let a = arena.insert(RenderNode::new(NodeKind::Box));
    let b = arena.insert(RenderNode::new(NodeKind::Box));
    let c = arena.insert(RenderNode::new(NodeKind::Text));

    arena.append_child(arena.root(), a).unwrap();
    arena.append_child(a, b).unwrap();
    arena.append_child(b, c).unwrap();

    let ancestors = arena.ancestors(c);
    assert_eq!(ancestors.len(), 3);
    assert_eq!(ancestors[0], b);
    assert_eq!(ancestors[1], a);
    assert_eq!(ancestors[2], arena.root());
}

#[test]
fn depth() {
    let mut arena = create_test_arena();
    assert_eq!(arena.depth(arena.root()), 0);

    let a = arena.insert(RenderNode::new(NodeKind::Box));
    let b = arena.insert(RenderNode::new(NodeKind::Box));

    arena.append_child(arena.root(), a).unwrap();
    arena.append_child(a, b).unwrap();

    assert_eq!(arena.depth(a), 1);
    assert_eq!(arena.depth(b), 2);
}

#[test]
fn is_ancestor() {
    let mut arena = create_test_arena();
    let a = arena.insert(RenderNode::new(NodeKind::Box));
    let b = arena.insert(RenderNode::new(NodeKind::Box));
    let c = arena.insert(RenderNode::new(NodeKind::Text));

    arena.append_child(arena.root(), a).unwrap();
    arena.append_child(a, b).unwrap();
    arena.append_child(b, c).unwrap();

    assert!(arena.is_ancestor(arena.root(), a));
    assert!(arena.is_ancestor(a, c));
    assert!(arena.is_ancestor(arena.root(), c));
    assert!(!arena.is_ancestor(c, a));
    assert!(!arena.is_ancestor(a, arena.root()));
}

#[test]
fn cycle_detection() {
    let mut arena = create_test_arena();
    let a = arena.insert(RenderNode::new(NodeKind::Box));
    let b = arena.insert(RenderNode::new(NodeKind::Box));

    arena.append_child(arena.root(), a).unwrap();
    arena.append_child(a, b).unwrap();

    // Try to make a a child of b (would create cycle)
    let result = arena.append_child(b, a);
    assert!(result.is_err());
}

#[test]
fn validate_tree() {
    let mut arena = create_test_arena();
    let a = arena.insert(RenderNode::new(NodeKind::Box));
    let b = arena.insert(RenderNode::new(NodeKind::Text));

    arena.append_child(arena.root(), a).unwrap();
    arena.append_child(a, b).unwrap();

    assert!(arena.validate().is_ok());
}

#[test]
fn print_tree_output() {
    let mut arena = create_test_arena();
    let a = arena.insert(RenderNode::new(NodeKind::Box));
    let b = arena.insert(RenderNode::text("Hello"));

    arena.append_child(arena.root(), a).unwrap();
    arena.append_child(a, b).unwrap();

    let output = arena.print_tree();
    assert!(output.contains("Box"));
    assert!(output.contains("Text"));
    assert!(output.contains("\"Hello\""));
}

#[test]
fn descendant_count() {
    let mut arena = create_test_arena();
    let a = arena.insert(RenderNode::new(NodeKind::Box));
    let b = arena.insert(RenderNode::new(NodeKind::Box));
    let c = arena.insert(RenderNode::new(NodeKind::Text));

    arena.append_child(arena.root(), a).unwrap();
    arena.append_child(a, b).unwrap();
    arena.append_child(b, c).unwrap();

    assert_eq!(arena.descendant_count(arena.root()), 3);
    assert_eq!(arena.descendant_count(a), 2);
    assert_eq!(arena.descendant_count(b), 1);
    assert_eq!(arena.descendant_count(c), 0);
}

#[test]
fn generation_increments() {
    let mut arena = create_test_arena();
    let gen0 = arena.generation();

    let node = arena.insert(RenderNode::new(NodeKind::Text));
    assert!(arena.generation() > gen0);

    let gen1 = arena.generation();
    arena.append_child(arena.root(), node).unwrap();
    assert!(arena.generation() > gen1);
}

#[test]
fn clear_arena() {
    let mut arena = create_test_arena();
    let a = arena.insert(RenderNode::new(NodeKind::Box));
    let b = arena.insert(RenderNode::new(NodeKind::Text));

    arena.append_child(arena.root(), a).unwrap();
    arena.append_child(a, b).unwrap();

    arena.clear();

    assert_eq!(arena.len(), 1);
    assert!(arena.validate().is_ok());
}
// end test module
// === node_id.rs ===
// === node_kind.rs ===

// Uses tree types via prelude

#[test]
fn default_kind_is_box() {
    assert_eq!(NodeKind::default(), NodeKind::Box);
}

#[test]
fn kind_names() {
    assert_eq!(NodeKind::Text.name(), "Text");
    assert_eq!(NodeKind::Box.name(), "Box");
    assert_eq!(NodeKind::Custom(42).name(), "Custom");
}

#[test]
fn container_detection() {
    assert!(NodeKind::Box.is_container());
    assert!(NodeKind::Flex.is_container());
    assert!(NodeKind::Scroll.is_container());
    assert!(!NodeKind::Text.is_container());
    assert!(!NodeKind::Spacer.is_container());
    assert!(!NodeKind::Separator.is_container());
}

#[test]
fn custom_kind_equality() {
    assert_eq!(NodeKind::Custom(1), NodeKind::Custom(1));
    assert_ne!(NodeKind::Custom(1), NodeKind::Custom(2));
}
// end test module
// === tree_error.rs ===

// Uses tree types via prelude

#[test]
fn error_display() {
    let err = TreeError::NodeNotFound(NodeId::default());
    assert!(format!("{err}").contains("Node not found"));

    let err = TreeError::InvalidOperation("test".into());
    assert!(format!("{err}").contains("Invalid operation"));
}
// end test module
