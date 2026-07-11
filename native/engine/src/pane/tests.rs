use super::*;

fn small_pane() -> PaneManager {
    PaneManager::with_size(80, 24)
}

#[test]
fn root_exists() {
    let mgr = small_pane();
    assert_eq!(mgr.len(), 1);
    assert!(mgr.get(mgr.root()).is_some());
}

#[test]
fn root_is_focused() {
    let mgr = small_pane();
    assert_eq!(mgr.focused(), mgr.root());
    assert!(mgr.get(mgr.root()).unwrap().focused);
}

#[test]
fn root_bounds() {
    let mgr = small_pane();
    let root = mgr.get(mgr.root()).unwrap();
    assert_eq!(root.bounds, Rect::new(0, 0, 80, 24));
}

#[test]
fn split_horizontal() {
    let mut mgr = small_pane();
    let new_pane = mgr
        .split(mgr.root(), SplitDirection::Horizontal, 0.5)
        .unwrap();

    assert_eq!(mgr.len(), 3);
    assert!(mgr.get(new_pane).is_some());

    let root = mgr.get(mgr.root()).unwrap();
    assert_eq!(root.children.len(), 2);
    assert!(root.split.is_some());
    assert!(root.widget_root.is_none()); // Moved to first child
}

#[test]
fn split_vertical() {
    let mut mgr = small_pane();
    let new_pane = mgr
        .split(mgr.root(), SplitDirection::Vertical, 0.5)
        .unwrap();

    assert_eq!(mgr.len(), 3);

    let root = mgr.get(mgr.root()).unwrap();
    let split = root.split.as_ref().unwrap();
    assert_eq!(split.direction, SplitDirection::Vertical);
}

#[test]
fn split_preserves_widget_root() {
    let mut mgr = small_pane();
    mgr.set_widget_root(mgr.root(), 42).unwrap();

    let _new = mgr
        .split(mgr.root(), SplitDirection::Horizontal, 0.5)
        .unwrap();

    // First child should have the widget root
    let root = mgr.get(mgr.root()).unwrap();
    let first_child = root.children[0];
    assert_eq!(mgr.widget_root(first_child), Some(42));
}

#[test]
fn split_horizontal_bounds() {
    let mut mgr = small_pane();
    let new_pane = mgr
        .split(mgr.root(), SplitDirection::Horizontal, 0.5)
        .unwrap();

    let root_bounds = mgr.bounds(mgr.root()).unwrap();
    let first_bounds = mgr.bounds(root_bounds_in(mgr.root(), 0, &mgr)).unwrap();
    let second_bounds = mgr.bounds(new_pane).unwrap();

    // First pane takes left half
    assert_eq!(first_bounds.x, 0);
    assert_eq!(first_bounds.width, 40); // 80 * 0.5

    // Second pane takes right half
    assert_eq!(second_bounds.x, 40);
    assert_eq!(second_bounds.width, 40);
}

#[test]
fn split_vertical_bounds() {
    let mut mgr = small_pane();
    let new_pane = mgr
        .split(mgr.root(), SplitDirection::Vertical, 0.5)
        .unwrap();

    let first_bounds = mgr.bounds(root_bounds_in(mgr.root(), 0, &mgr)).unwrap();
    let second_bounds = mgr.bounds(new_pane).unwrap();

    // First pane takes top half
    assert_eq!(first_bounds.y, 0);
    assert_eq!(first_bounds.height, 12); // 24 * 0.5

    // Second pane takes bottom half
    assert_eq!(second_bounds.y, 12);
    assert_eq!(second_bounds.height, 12);
}

#[test]
fn split_already_split_fails() {
    let mut mgr = small_pane();
    mgr.split(mgr.root(), SplitDirection::Horizontal, 0.5)
        .unwrap();
    let result = mgr.split(mgr.root(), SplitDirection::Vertical, 0.5);
    assert_eq!(result, Err(PaneError::AlreadySplit));
}

#[test]
fn split_too_small_fails() {
    let mut mgr = PaneManager::with_size(5, 3);
    let result = mgr.split(mgr.root(), SplitDirection::Horizontal, 0.5);
    assert_eq!(result, Err(PaneError::TooSmall));
}

#[test]
fn remove_pane_merges_sibling() {
    let mut mgr = small_pane();
    let new_pane = mgr
        .split(mgr.root(), SplitDirection::Horizontal, 0.5)
        .unwrap();
    mgr.remove(new_pane).unwrap();

    assert_eq!(mgr.len(), 1);
    let root = mgr.get(mgr.root()).unwrap();
    assert!(root.children.is_empty());
    assert!(root.split.is_none());
}

#[test]
fn remove_root_fails() {
    let mut mgr = small_pane();
    let result = mgr.remove(mgr.root());
    assert_eq!(result, Err(PaneError::CannotRemoveRoot));
}

#[test]
fn focus_switches() {
    let mut mgr = small_pane();
    let new_pane = mgr
        .split(mgr.root(), SplitDirection::Horizontal, 0.5)
        .unwrap();

    mgr.focus(new_pane).unwrap();
    assert_eq!(mgr.focused(), new_pane);
    assert!(mgr.get(new_pane).unwrap().focused);
    assert!(!mgr.get(mgr.root()).unwrap().focused);
}

#[test]
fn focus_direction_right() {
    let mut mgr = small_pane();
    let right = mgr
        .split(mgr.root(), SplitDirection::Horizontal, 0.5)
        .unwrap();
    mgr.focus(mgr.root()).unwrap();
    mgr.focus_direction(FocusDirection::Right).unwrap();
    assert_eq!(mgr.focused(), right);
}

#[test]
fn focus_direction_left() {
    let mut mgr = small_pane();
    let _right = mgr
        .split(mgr.root(), SplitDirection::Horizontal, 0.5)
        .unwrap();
    mgr.focus_direction(FocusDirection::Right).unwrap(); // focus right
    mgr.focus_direction(FocusDirection::Left).unwrap(); // back to root
    assert_eq!(mgr.focused(), mgr.root());
}

#[test]
fn focus_direction_down() {
    let mut mgr = small_pane();
    let bottom = mgr
        .split(mgr.root(), SplitDirection::Vertical, 0.5)
        .unwrap();
    mgr.focus(mgr.root()).unwrap();
    mgr.focus_direction(FocusDirection::Down).unwrap();
    assert_eq!(mgr.focused(), bottom);
}

#[test]
fn resize_root() {
    let mut mgr = small_pane();
    mgr.resize(120, 40);
    let root = mgr.get(mgr.root()).unwrap();
    assert_eq!(root.bounds, Rect::new(0, 0, 120, 40));
}

#[test]
fn resize_cascades() {
    let mut mgr = small_pane();
    mgr.split(mgr.root(), SplitDirection::Horizontal, 0.5)
        .unwrap();
    mgr.resize(160, 48);

    let root_bounds = mgr.bounds(mgr.root()).unwrap();
    let first_bounds = mgr.bounds(root_bounds_in(mgr.root(), 0, &mgr)).unwrap();
    let second_bounds = mgr.bounds(root_bounds_in(mgr.root(), 1, &mgr)).unwrap();

    assert_eq!(root_bounds.width, 160);
    assert_eq!(first_bounds.width, 80);
    assert_eq!(second_bounds.width, 80);
}

#[test]
fn swap_widgets() {
    let mut mgr = small_pane();
    let b = mgr
        .split(mgr.root(), SplitDirection::Horizontal, 0.5)
        .unwrap();
    mgr.set_widget_root(mgr.root(), 1).unwrap();
    mgr.set_widget_root(b, 2).unwrap();

    mgr.swap_widgets(mgr.root(), b).unwrap();

    assert_eq!(mgr.widget_root(mgr.root()), Some(2));
    assert_eq!(mgr.widget_root(b), Some(1));
}

#[test]
fn visible_panes() {
    let mut mgr = small_pane();
    let b = mgr
        .split(mgr.root(), SplitDirection::Horizontal, 0.5)
        .unwrap();
    let _c = mgr.split(b, SplitDirection::Vertical, 0.5).unwrap();

    let visible = mgr.visible_panes();
    // DFS: root, first_child, b, b_child1, c
    assert_eq!(visible.len(), 5);
}

#[test]
fn leaf_panes() {
    let mut mgr = small_pane();
    mgr.set_widget_root(mgr.root(), 1).unwrap();
    let b = mgr
        .split(mgr.root(), SplitDirection::Horizontal, 0.5)
        .unwrap();
    mgr.set_widget_root(b, 2).unwrap();
    let c = mgr.split(b, SplitDirection::Vertical, 0.5).unwrap();
    mgr.set_widget_root(c, 3).unwrap();

    let leaves = mgr.leaf_panes();
    assert_eq!(leaves.len(), 3);
}

#[test]
fn depth() {
    let mut mgr = small_pane();
    assert_eq!(mgr.depth(mgr.root()), 0);

    let b = mgr
        .split(mgr.root(), SplitDirection::Horizontal, 0.5)
        .unwrap();
    assert_eq!(mgr.depth(b), 1);

    let c = mgr.split(b, SplitDirection::Vertical, 0.5).unwrap();
    assert_eq!(mgr.depth(c), 2);
}

#[test]
fn is_leaf() {
    let mut mgr = small_pane();
    assert!(mgr.is_leaf(mgr.root()));

    let b = mgr
        .split(mgr.root(), SplitDirection::Horizontal, 0.5)
        .unwrap();
    assert!(!mgr.is_leaf(mgr.root()));
    assert!(mgr.is_leaf(b));
}

#[test]
fn nested_splits() {
    let mut mgr = small_pane();
    let b = mgr
        .split(mgr.root(), SplitDirection::Horizontal, 0.5)
        .unwrap();
    let c = mgr.split(b, SplitDirection::Vertical, 0.5).unwrap();
    let _d = mgr.split(c, SplitDirection::Horizontal, 0.5).unwrap();

    // root (split) -> [first_child, b (split)] -> [b_child1, c (split)] -> [c_child1, d]
    // Total: 1 + 2 + 2 + 2 = 7
    assert_eq!(mgr.len(), 7);
}

#[test]
fn focus_invalid_pane_fails() {
    let mut mgr = small_pane();
    // Create a pane, split, remove it, then try to focus
    let b = mgr
        .split(mgr.root(), SplitDirection::Horizontal, 0.5)
        .unwrap();
    mgr.remove(b).unwrap();
    let result = mgr.focus(b);
    assert_eq!(result, Err(PaneError::PaneNotFound));
}

#[test]
fn min_size_respected() {
    let mut mgr = small_pane();
    mgr.set_min_size(mgr.root(), Size::new(20, 10)).unwrap();

    let mut mgr2 = PaneManager::with_size(30, 15);
    mgr2.set_min_size(mgr2.root(), Size::new(20, 10)).unwrap();
    let result = mgr2.split(mgr2.root(), SplitDirection::Horizontal, 0.5);
    assert_eq!(result, Err(PaneError::TooSmall));
}

#[test]
fn split_ratio_clamped() {
    let mut mgr = small_pane();
    let _new = mgr
        .split(mgr.root(), SplitDirection::Horizontal, 0.0)
        .unwrap();
    let root = mgr.get(mgr.root()).unwrap();
    let split = root.split.as_ref().unwrap();
    assert_eq!(split.ratio, 0.1); // Clamped to minimum
}

// Helper to get a child pane id by index
fn root_bounds_in(pane_id: PaneId, index: usize, mgr: &PaneManager) -> PaneId {
    mgr.get(pane_id).unwrap().children[index]
}
