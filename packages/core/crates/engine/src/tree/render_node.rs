use std::collections::HashMap;

use smallvec::SmallVec;

use super::{
    interaction::{EventHandlers, FocusProps, NodeState},
    layout::LayoutProps,
    metadata::{Accessibility, Metadata},
    node_id::NodeId,
    node_kind::NodeKind,
    style::Style,
    visual::{CursorProps, Overflow, Transform, Visibility},
};

/// The complete data for a single UI node. Stored in the arena, accessed by `NodeId`.
///
/// **Ownership:** Owned by the arena. The arena is the sole owner of all nodes.
/// References are by `NodeId`, not by pointer.
///
/// **Memory:** Approximately 256-320 bytes per node. The `SmallVec<[NodeId; 4]>`
/// stores up to 4 children inline (32 bytes) without heap allocation.
/// Most nodes have fewer than 4 children.
pub struct RenderNode {
    /// Unique identifier for this node.
    pub id: NodeId,
    /// Type of node (Text, Box, Flex, etc.).
    pub kind: NodeKind,
    /// Parent node. None for root.
    pub parent: Option<NodeId>,
    /// Child nodes. SmallVec stores up to 4 inline.
    pub children: SmallVec<[NodeId; 4]>,
    /// Visual styling.
    pub style: Style,
    /// Layout properties.
    pub layout: LayoutProps,
    /// Text content (for Text nodes).
    pub text: Option<Box<str>>,
    /// Visibility control.
    pub visibility: Visibility,
    /// Visual offset and layer ordering.
    pub transform: Transform,
    /// How content overflows.
    pub overflow: Overflow,
    /// Cursor appearance and position.
    pub cursor: Option<CursorProps>,
    /// Text alignment.
    pub text_align: crate::text::TextAlign,
    /// Whether text should wrap.
    pub text_wrap: bool,
    /// Focus properties.
    pub focus: FocusProps,
    /// Event handler placeholders.
    pub events: EventHandlers,
    /// Mutable node state.
    pub state: NodeState,
    /// Optional metadata.
    pub metadata: Option<Box<Metadata>>,
    /// Optional accessibility data.
    pub accessibility: Option<Box<Accessibility>>,
    /// Generic key-value attributes.
    pub attributes: HashMap<String, String>,
}

impl Default for RenderNode {
    fn default() -> Self {
        Self {
            id: NodeId::default(),
            kind: NodeKind::default(),
            parent: None,
            children: SmallVec::new(),
            style: Style::default(),
            layout: LayoutProps::default(),
            text: None,
            visibility: Visibility::default(),
            transform: Transform::default(),
            overflow: Overflow::default(),
            cursor: None,
            text_align: crate::text::TextAlign::Left,
            text_wrap: false,
            focus: FocusProps::default(),
            events: EventHandlers::default(),
            state: NodeState::default(),
            metadata: None,
            accessibility: None,
            attributes: HashMap::new(),
        }
    }
}

impl RenderNode {
    /// Create a new node with the given kind. ID is set by the arena.
    pub fn new(kind: NodeKind) -> Self {
        Self {
            kind,
            ..Default::default()
        }
    }

    /// Create a new text node.
    pub fn text(content: impl Into<Box<str>>) -> Self {
        Self {
            kind: NodeKind::Text,
            text: Some(content.into()),
            ..Default::default()
        }
    }

    /// Create a new box/container node.
    pub fn box_node() -> Self {
        Self::new(NodeKind::Box)
    }

    /// Create a new flex container node.
    pub fn flex() -> Self {
        Self::new(NodeKind::Flex)
    }

    /// Returns true if this node has children.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Returns the number of children.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Returns true if this node is the root (no parent).
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    /// Set text content on this node.
    pub fn set_text(&mut self, content: impl Into<Box<str>>) {
        self.text = Some(content.into());
        self.state.mark_render_dirty();
    }

    /// Set style on this node.
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
        self.state.mark_render_dirty();
    }

    /// Set layout properties on this node.
    pub fn set_layout(&mut self, layout: LayoutProps) {
        self.layout = layout;
        self.state.mark_layout_dirty();
    }

    /// Mark this node as focused.
    pub fn focus(&mut self) {
        self.focus.focused = true;
        self.state.mark_render_dirty();
    }

    /// Mark this node as unfocused.
    pub fn blur(&mut self) {
        self.focus.focused = false;
        self.state.mark_render_dirty();
    }

    /// Mark this node as focusable.
    pub fn set_focusable(&mut self, focusable: bool) {
        self.focus.focusable = focusable;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
