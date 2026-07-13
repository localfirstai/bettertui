//! Pane management: split panes with recursive layout, focus navigation, and resize.

use crate::tree::{Point, Rect, Size};
use slotmap::SlotMap;

slotmap::new_key_type! {
    pub struct PaneId;
}

/// Direction of a pane split.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

/// A split divides a pane into two children.
#[derive(Debug, Clone)]
pub struct PaneSplit {
    pub id: PaneId,
    pub direction: SplitDirection,
    /// Ratio of the first child's size (0.0 - 1.0).
    pub ratio: f32,
}

/// A pane in the layout tree.
#[derive(Debug, Clone)]
pub struct Pane {
    pub id: PaneId,
    pub parent: Option<PaneId>,
    pub children: Vec<PaneId>,
    pub split: Option<PaneSplit>,
    pub bounds: Rect,
    pub min_size: Size,
    pub focused: bool,
    /// Opaque widget root identifier. The pane manager does not interpret this.
    pub widget_root: Option<u64>,
}

impl Pane {
    fn new(id: PaneId, bounds: Rect) -> Self {
        Self {
            id,
            parent: None,
            children: Vec::new(),
            split: None,
            bounds,
            min_size: Size::new(4, 2),
            focused: false,
            widget_root: None,
        }
    }
}

/// Manages the pane layout tree.
///
/// The pane system sits above the widget tree. Each pane owns a region of the
/// screen and a widget root. The renderer does not know about panes — it
/// receives per-pane widget trees and renders them independently.
///
/// Integration with the existing engine:
/// - Pane bounds become layout constraints for the widget tree.
/// - Focus propagation uses `FocusManager` scopes (one scope per pane).
/// - The `CommandProcessor` handles widget mutations within a pane.
#[derive(Debug)]
pub struct PaneManager {
    panes: SlotMap<PaneId, Pane>,
    root: PaneId,
    focused: PaneId,
}

impl Default for PaneManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PaneManager {
    /// Creates a new pane manager with a single root pane.
    pub fn new() -> Self {
        let mut panes = SlotMap::with_key();
        let root = panes.insert(Pane::new(
            PaneId::default(),     // Will be replaced by actual key
            Rect::new(0, 0, 0, 0), // Set via resize()
        ));
        // Fix: the root's id field should match its actual key
        let mut root_pane = panes[root].clone();
        root_pane.id = root;
        panes[root] = root_pane;

        let mut mgr = Self {
            panes,
            root,
            focused: root,
        };
        mgr.panes[root].focused = true;
        mgr
    }

    /// Creates a new pane manager with specified terminal dimensions.
    pub fn with_size(width: u16, height: u16) -> Self {
        let mut mgr = Self::new();
        mgr.panes[mgr.root].bounds = Rect::new(0, 0, width, height);
        mgr
    }

    /// Returns the root pane id.
    pub fn root(&self) -> PaneId {
        self.root
    }

    /// Returns the currently focused pane id.
    pub fn focused(&self) -> PaneId {
        self.focused
    }

    /// Returns a reference to a pane.
    pub fn get(&self, id: PaneId) -> Option<&Pane> {
        self.panes.get(id)
    }

    /// Returns a mutable reference to a pane.
    pub fn get_mut(&mut self, id: PaneId) -> Option<&mut Pane> {
        self.panes.get_mut(id)
    }

    /// Returns the total number of panes.
    pub fn len(&self) -> usize {
        self.panes.len()
    }

    /// Returns true if the pane manager has no panes.
    pub fn is_empty(&self) -> bool {
        self.panes.is_empty()
    }

    /// Returns true if only the root pane exists.
    pub fn is_single(&self) -> bool {
        self.panes.len() == 1
    }

    /// Resizes the root pane and cascades to children.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.panes[self.root].bounds = Rect::new(0, 0, width, height);
        self.recompute_children(self.root);
    }

    /// Splits a pane into two children.
    ///
    /// Returns the id of the newly created second child. The original pane
    /// becomes the first child and retains its widget root.
    pub fn split(
        &mut self,
        pane_id: PaneId,
        direction: SplitDirection,
        ratio: f32,
    ) -> Result<PaneId, PaneError> {
        let ratio = ratio.clamp(0.1, 0.9);

        if !self.panes.contains_key(pane_id) {
            return Err(PaneError::PaneNotFound);
        }

        if self.panes[pane_id].children.len() >= 2 {
            return Err(PaneError::AlreadySplit);
        }

        let parent_bounds = self.panes[pane_id].bounds;
        let min_size = self.panes[pane_id].min_size;

        // Check minimum size
        match direction {
            SplitDirection::Horizontal => {
                if parent_bounds.width < min_size.width * 2 + 1 {
                    return Err(PaneError::TooSmall);
                }
            }
            SplitDirection::Vertical => {
                if parent_bounds.height < min_size.height * 2 + 1 {
                    return Err(PaneError::TooSmall);
                }
            }
        }

        // Create first child (takes original pane's widget root)
        let child1_bounds = match direction {
            SplitDirection::Horizontal => {
                let w = (parent_bounds.width as f32 * ratio) as u16;
                Rect::new(parent_bounds.x, parent_bounds.y, w, parent_bounds.height)
            }
            SplitDirection::Vertical => {
                let h = (parent_bounds.height as f32 * ratio) as u16;
                Rect::new(parent_bounds.x, parent_bounds.y, parent_bounds.width, h)
            }
        };

        let child1_id = self.panes.insert(Pane::new(
            PaneId::default(), // temp
            child1_bounds,
        ));
        {
            let child1 = &mut self.panes[child1_id];
            child1.id = child1_id;
            child1.parent = Some(pane_id);
            child1.min_size = min_size;
        }

        // Create second child (empty widget root)
        let child2_bounds = match direction {
            SplitDirection::Horizontal => {
                let w = parent_bounds.width - child1_bounds.width;
                Rect::new(
                    child1_bounds.right(),
                    parent_bounds.y,
                    w,
                    parent_bounds.height,
                )
            }
            SplitDirection::Vertical => {
                let h = parent_bounds.height - child1_bounds.height;
                Rect::new(
                    parent_bounds.x,
                    child1_bounds.bottom(),
                    parent_bounds.width,
                    h,
                )
            }
        };

        let child2_id = self.panes.insert(Pane::new(
            PaneId::default(), // temp
            child2_bounds,
        ));
        {
            let child2 = &mut self.panes[child2_id];
            child2.id = child2_id;
            child2.parent = Some(pane_id);
            child2.min_size = min_size;
        }

        // Move widget root from parent to first child
        let widget_root = self.panes[pane_id].widget_root.take();
        self.panes[child1_id].widget_root = widget_root;

        // Set up parent as split container
        {
            let parent = &mut self.panes[pane_id];
            parent.children = vec![child1_id, child2_id];
            parent.split = Some(PaneSplit {
                id: pane_id,
                direction,
                ratio,
            });
            parent.widget_root = None;
        }

        Ok(child2_id)
    }

    /// Removes a pane and merges its space into its sibling.
    ///
    /// The sibling inherits the removed pane's space. If the removed pane had
    /// a widget root, it is detached (caller should handle cleanup).
    pub fn remove(&mut self, pane_id: PaneId) -> Result<(), PaneError> {
        if pane_id == self.root {
            return Err(PaneError::CannotRemoveRoot);
        }

        let parent_id = self.panes[pane_id].parent.ok_or(PaneError::NoParent)?;

        let parent = &self.panes[parent_id];
        if parent.children.len() != 2 {
            return Err(PaneError::NotSplitChild);
        }

        let sibling_id = parent
            .children
            .iter()
            .find(|&&c| c != pane_id)
            .copied()
            .ok_or(PaneError::NoSibling)?;

        // If the removed pane was focused, focus the sibling
        if self.focused == pane_id {
            self.focused = sibling_id;
            self.panes[sibling_id].focused = true;
        }

        // Move sibling up to parent's position
        let parent_bounds = self.panes[parent_id].bounds;

        // Sibling takes parent's full bounds
        self.panes[sibling_id].bounds = Rect::new(
            parent_bounds.x,
            parent_bounds.y,
            parent_bounds.width,
            parent_bounds.height,
        );
        self.panes[sibling_id].parent = self.panes[parent_id].parent;

        // If parent was a split container, update grandparent
        if let Some(grandparent_id) = self.panes[parent_id].parent {
            let gp = &mut self.panes[grandparent_id];
            if let Some(idx) = gp.children.iter().position(|&c| c == parent_id) {
                gp.children[idx] = sibling_id;
            }
        } else {
            // Parent was root, sibling becomes root
            self.root = sibling_id;
        }

        // Remove the pane and its split
        self.panes.remove(pane_id);
        self.panes.remove(parent_id);

        Ok(())
    }

    /// Focus a specific pane.
    pub fn focus(&mut self, pane_id: PaneId) -> Result<(), PaneError> {
        if !self.panes.contains_key(pane_id) {
            return Err(PaneError::PaneNotFound);
        }

        // Blur current
        self.panes[self.focused].focused = false;

        // Focus new
        self.panes[pane_id].focused = true;
        self.focused = pane_id;

        Ok(())
    }

    /// Focus the next pane in direction.
    pub fn focus_direction(&mut self, direction: FocusDirection) -> Result<(), PaneError> {
        let current = self.focused;
        let current_bounds = self.panes[current].bounds;
        let current_center = Point::new(
            current_bounds.x + current_bounds.width / 2,
            current_bounds.y + current_bounds.height / 2,
        );

        let mut best: Option<PaneId> = None;
        let mut best_distance = u32::MAX;

        for (id, pane) in self.panes.iter() {
            if id == current {
                continue;
            }
            if !self.is_visible(id) {
                continue;
            }

            let b = pane.bounds;
            let center = Point::new(b.x + b.width / 2, b.y + b.height / 2);

            let valid = match direction {
                FocusDirection::Left => center.x < current_center.x,
                FocusDirection::Right => center.x > current_center.x,
                FocusDirection::Up => center.y < current_center.y,
                FocusDirection::Down => center.y > current_center.y,
            };

            if !valid {
                continue;
            }

            let dx = center.x.abs_diff(current_center.x) as u32;
            let dy = center.y.abs_diff(current_center.y) as u32;
            let distance = dx + dy;

            if distance < best_distance {
                best_distance = distance;
                best = Some(id);
            }
        }

        if let Some(target) = best {
            self.focus(target)?;
        }

        Ok(())
    }

    /// Set the widget root for a pane.
    pub fn set_widget_root(&mut self, pane_id: PaneId, widget_root: u64) -> Result<(), PaneError> {
        let pane = self.panes.get_mut(pane_id).ok_or(PaneError::PaneNotFound)?;
        pane.widget_root = Some(widget_root);
        Ok(())
    }

    /// Get the widget root for a pane.
    pub fn widget_root(&self, pane_id: PaneId) -> Option<u64> {
        self.panes.get(pane_id)?.widget_root
    }

    /// Get bounds for a pane.
    pub fn bounds(&self, pane_id: PaneId) -> Option<Rect> {
        self.panes.get(pane_id).map(|p| p.bounds)
    }

    /// Set minimum size for a pane.
    pub fn set_min_size(&mut self, pane_id: PaneId, min_size: Size) -> Result<(), PaneError> {
        let pane = self.panes.get_mut(pane_id).ok_or(PaneError::PaneNotFound)?;
        pane.min_size = min_size;
        Ok(())
    }

    /// Resize a specific pane.
    pub fn resize_pane(
        &mut self,
        pane_id: PaneId,
        width: u16,
        height: u16,
    ) -> Result<(), PaneError> {
        if !self.panes.contains_key(pane_id) {
            return Err(PaneError::PaneNotFound);
        }

        let min = self.panes[pane_id].min_size;
        let width = width.max(min.width);
        let height = height.max(min.height);

        self.panes[pane_id].bounds.width = width;
        self.panes[pane_id].bounds.height = height;

        // Recompute sibling
        if let Some(parent_id) = self.panes[pane_id].parent {
            self.recompute_sibling(parent_id, pane_id);
        }

        Ok(())
    }

    /// Collect all visible pane ids in depth-first order.
    pub fn visible_panes(&self) -> Vec<PaneId> {
        let mut result = Vec::new();
        self.collect_visible(self.root, &mut result);
        result
    }

    /// Collect all leaf panes (those with widget roots).
    pub fn leaf_panes(&self) -> Vec<PaneId> {
        self.panes
            .iter()
            .filter(|(_, p)| p.widget_root.is_some())
            .map(|(id, _)| id)
            .collect()
    }

    /// Check if a pane is a leaf (has no children).
    pub fn is_leaf(&self, pane_id: PaneId) -> bool {
        self.panes
            .get(pane_id)
            .map(|p| p.children.is_empty())
            .unwrap_or(false)
    }

    /// Get the depth of a pane in the tree.
    pub fn depth(&self, pane_id: PaneId) -> usize {
        let mut depth = 0;
        let mut current = pane_id;
        while let Some(parent_id) = self.panes.get(current).and_then(|p| p.parent) {
            depth += 1;
            current = parent_id;
        }
        depth
    }

    /// Swap the widget roots of two panes.
    pub fn swap_widgets(&mut self, a: PaneId, b: PaneId) -> Result<(), PaneError> {
        if !self.panes.contains_key(a) || !self.panes.contains_key(b) {
            return Err(PaneError::PaneNotFound);
        }

        let root_a = self.panes[a].widget_root;
        let root_b = self.panes[b].widget_root;
        self.panes[a].widget_root = root_b;
        self.panes[b].widget_root = root_a;

        Ok(())
    }

    // --- Private helpers ---

    fn is_visible(&self, pane_id: PaneId) -> bool {
        // A pane is visible if it has no parent (root) or if its parent has children
        // (meaning it was created by a split).
        self.panes
            .get(pane_id)
            .and_then(|p| p.parent)
            .map(|parent_id| !self.panes[parent_id].children.is_empty())
            .unwrap_or(true) // root is always visible
    }

    fn collect_visible(&self, pane_id: PaneId, result: &mut Vec<PaneId>) {
        result.push(pane_id);
        if let Some(pane) = self.panes.get(pane_id) {
            for &child in &pane.children {
                self.collect_visible(child, result);
            }
        }
    }

    fn recompute_children(&mut self, parent_id: PaneId) {
        let (bounds, direction, ratio) = {
            let parent = &self.panes[parent_id];
            match &parent.split {
                Some(split) => (parent.bounds, split.direction, split.ratio),
                None => return,
            }
        };

        let children = self.panes[parent_id].children.clone();
        if children.len() != 2 {
            return;
        }

        let child1_bounds = match direction {
            SplitDirection::Horizontal => {
                let w = (bounds.width as f32 * ratio) as u16;
                Rect::new(bounds.x, bounds.y, w, bounds.height)
            }
            SplitDirection::Vertical => {
                let h = (bounds.height as f32 * ratio) as u16;
                Rect::new(bounds.x, bounds.y, bounds.width, h)
            }
        };

        let child2_bounds = match direction {
            SplitDirection::Horizontal => {
                let w = bounds.width - child1_bounds.width;
                Rect::new(child1_bounds.right(), bounds.y, w, bounds.height)
            }
            SplitDirection::Vertical => {
                let h = bounds.height - child1_bounds.height;
                Rect::new(bounds.x, child1_bounds.bottom(), bounds.width, h)
            }
        };

        self.panes[children[0]].bounds = child1_bounds;
        self.panes[children[1]].bounds = child2_bounds;

        // Recurse
        self.recompute_children(children[0]);
        self.recompute_children(children[1]);
    }

    fn recompute_sibling(&mut self, parent_id: PaneId, exclude: PaneId) {
        let (bounds, direction, _ratio) = {
            let parent = &self.panes[parent_id];
            match &parent.split {
                Some(split) => (parent.bounds, split.direction, split.ratio),
                None => return,
            }
        };

        let children = self.panes[parent_id].children.clone();
        if children.len() != 2 {
            return;
        }

        let sibling_id = children.iter().find(|&&c| c != exclude).copied();
        let Some(sibling_id) = sibling_id else {
            return;
        };

        let primary_bounds = self.panes[exclude].bounds;

        let sibling_bounds = match direction {
            SplitDirection::Horizontal => {
                let w = bounds.width.saturating_sub(primary_bounds.width);
                Rect::new(primary_bounds.right(), bounds.y, w, bounds.height)
            }
            SplitDirection::Vertical => {
                let h = bounds.height.saturating_sub(primary_bounds.height);
                Rect::new(bounds.x, primary_bounds.bottom(), bounds.width, h)
            }
        };

        self.panes[sibling_id].bounds = sibling_bounds;
    }
}

/// Direction for focus navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusDirection {
    Left,
    Right,
    Up,
    Down,
}

/// Errors from pane operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaneError {
    PaneNotFound,
    CannotRemoveRoot,
    NoParent,
    NotSplitChild,
    NoSibling,
    AlreadySplit,
    TooSmall,
}

impl std::fmt::Display for PaneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PaneNotFound => write!(f, "pane not found"),
            Self::CannotRemoveRoot => write!(f, "cannot remove root pane"),
            Self::NoParent => write!(f, "pane has no parent"),
            Self::NotSplitChild => write!(f, "pane is not a split child"),
            Self::NoSibling => write!(f, "no sibling pane found"),
            Self::AlreadySplit => write!(f, "pane is already split"),
            Self::TooSmall => write!(f, "pane too small to split"),
        }
    }
}

impl std::error::Error for PaneError {}

#[cfg(test)]
mod tests {
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
        let _new_pane = mgr
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
}
