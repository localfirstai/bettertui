use bitflags::bitflags;

/// Focus properties for a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FocusProps {
    /// Tab order. `None` = follow tree order. `Some(n)` = explicit order (lower first).
    pub tab_index: Option<i32>,
    /// Whether this node can receive focus.
    pub focusable: bool,
    /// Whether this node currently has focus.
    pub focused: bool,
}

/// Mutable state of a node, updated by the engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeState {
    /// Horizontal scroll offset.
    pub scroll_x: i32,
    /// Vertical scroll offset.
    pub scroll_y: i32,
    /// Measured content width in cells.
    pub content_width: u32,
    /// Measured content height in cells.
    pub content_height: u32,
    /// Generic dirty flag for any change.
    pub dirty: bool,
    /// Layout needs recalculation.
    pub layout_dirty: bool,
    /// Render needs redraw.
    pub render_dirty: bool,
}

impl Default for NodeState {
    fn default() -> Self {
        Self {
            scroll_x: 0,
            scroll_y: 0,
            content_width: 0,
            content_height: 0,
            dirty: true,
            layout_dirty: true,
            render_dirty: true,
        }
    }
}

impl NodeState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Mark node and propagate dirty flags upward.
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
        self.layout_dirty = true;
        self.render_dirty = true;
    }

    /// Mark only layout as dirty.
    pub fn mark_layout_dirty(&mut self) {
        self.dirty = true;
        self.layout_dirty = true;
        self.render_dirty = true;
    }

    /// Mark only render as dirty.
    pub fn mark_render_dirty(&mut self) {
        self.dirty = true;
        self.render_dirty = true;
    }

    /// Clear all dirty flags.
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
        self.layout_dirty = false;
        self.render_dirty = false;
    }
}

bitflags! {
    /// Flags indicating what changed on a node. Used for efficient dirty propagation.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct UpdateFlags: u32 {
        const STYLE = 0b0000_0001;
        const LAYOUT = 0b0000_0010;
        const TEXT = 0b0000_0100;
        const CHILDREN = 0b0000_1000;
        const VISIBILITY = 0b0001_0000;
        const TRANSFORM = 0b0010_0000;
        const FOCUS = 0b0100_0000;
        const METADATA = 0b1000_0000;
        const ALL = 0b1111_1111;
    }
}

impl Default for UpdateFlags {
    fn default() -> Self {
        Self::empty()
    }
}

/// Event handler placeholders. Actual handler implementation comes in later phases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EventHandlers {
    /// Whether this node has any event handlers registered.
    pub has_handlers: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
