use crate::layout::types::{FlexDirection, FlexWrap, LayoutProps};
use crate::tree::{Color, NodeId, NodeKind, Style};

/// Commands that describe mutations to the UI tree.
///
/// Commands are the bridge between React (or any framework adapter) and the
/// Rust engine. They are batched, versioned, and applied atomically.
///
/// Each command targets a specific node by `NodeId` and describes a single
/// atomic mutation.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    // ─── Tree Commands ─────────────────────────────────────────────
    /// Create a new node with the given kind.
    CreateNode { id: NodeId, kind: NodeKind },

    /// Remove a node and all its descendants.
    RemoveNode { id: NodeId },

    /// Append a child to a parent node.
    AppendChild { parent: NodeId, child: NodeId },

    /// Insert a child before a reference node.
    InsertBefore { reference: NodeId, child: NodeId },

    /// Move a node to a new parent.
    MoveNode { node: NodeId, new_parent: NodeId },

    /// Replace one node with another.
    ReplaceNode { old: NodeId, new: NodeId },

    /// Detach a node from its parent (keep in arena).
    DetachNode { id: NodeId },

    // ─── Style Commands ───────────────────────────────────────────
    /// Set the full style of a node.
    SetStyle { id: NodeId, style: Style },

    /// Set the foreground color.
    SetForeground { id: NodeId, color: Color },

    /// Set the background color.
    SetBackground { id: NodeId, color: Color },

    /// Set bold attribute.
    SetBold { id: NodeId, value: bool },

    /// Set italic attribute.
    SetItalic { id: NodeId, value: bool },

    /// Set underline attribute.
    SetUnderline { id: NodeId, value: bool },

    /// Set strikethrough attribute.
    SetStrikethrough { id: NodeId, value: bool },

    /// Set dim attribute.
    SetDim { id: NodeId, value: bool },

    /// Set inverse attribute.
    SetInverse { id: NodeId, value: bool },

    /// Set hidden attribute.
    SetHidden { id: NodeId, value: bool },

    // ─── Layout Commands ──────────────────────────────────────────
    /// Set the full layout props of a node.
    SetLayout { id: NodeId, layout: LayoutProps },

    /// Set flex direction.
    SetFlexDirection {
        id: NodeId,
        direction: FlexDirection,
    },

    /// Set flex wrap.
    SetFlexWrap { id: NodeId, value: FlexWrap },

    /// Set justify content.
    SetJustifyContent {
        id: NodeId,
        value: crate::layout::types::JustifyContent,
    },

    /// Set align items.
    SetAlignItems {
        id: NodeId,
        value: crate::layout::types::AlignItems,
    },

    /// Set align self.
    SetAlignSelf {
        id: NodeId,
        value: crate::layout::types::AlignSelf,
    },

    /// Set width.
    SetWidth {
        id: NodeId,
        value: crate::layout::types::Sizing,
    },

    /// Set height.
    SetHeight {
        id: NodeId,
        value: crate::layout::types::Sizing,
    },

    /// Set min width.
    SetMinWidth {
        id: NodeId,
        value: crate::layout::types::Sizing,
    },

    /// Set min height.
    SetMinHeight {
        id: NodeId,
        value: crate::layout::types::Sizing,
    },

    /// Set max width.
    SetMaxWidth {
        id: NodeId,
        value: crate::layout::types::Sizing,
    },

    /// Set max height.
    SetMaxHeight {
        id: NodeId,
        value: crate::layout::types::Sizing,
    },

    /// Set padding.
    SetPadding {
        id: NodeId,
        value: crate::layout::types::RectValues,
    },

    /// Set margin.
    SetMargin {
        id: NodeId,
        value: crate::layout::types::RectValues,
    },

    /// Set gap.
    SetGap {
        id: NodeId,
        value: crate::layout::types::Gap,
    },

    /// Set flex grow.
    SetFlexGrow { id: NodeId, value: f32 },

    /// Set flex shrink.
    SetFlexShrink { id: NodeId, value: f32 },

    /// Set flex basis.
    SetFlexBasis {
        id: NodeId,
        value: crate::layout::types::Sizing,
    },

    /// Set position.
    SetPosition {
        id: NodeId,
        value: crate::layout::types::Position,
    },

    /// Set inset.
    SetInset {
        id: NodeId,
        value: crate::layout::types::RectValues,
    },

    // ─── Content Commands ─────────────────────────────────────────
    /// Set text content of a node.
    SetText { id: NodeId, text: String },

    /// Set an attribute on a node.
    SetAttribute {
        id: NodeId,
        key: String,
        value: String,
    },

    /// Remove an attribute from a node.
    RemoveAttribute { id: NodeId, key: String },

    // ─── Visibility Commands ──────────────────────────────────────
    /// Set display mode.
    SetDisplay {
        id: NodeId,
        value: crate::tree::VisibilityDisplay,
    },

    /// Set opacity.
    SetOpacity { id: NodeId, value: f32 },

    /// Set clip.
    SetClip { id: NodeId, value: bool },

    // ─── Transform Commands ───────────────────────────────────────
    /// Set translate X.
    SetTranslateX { id: NodeId, value: i32 },

    /// Set translate Y.
    SetTranslateY { id: NodeId, value: i32 },

    /// Set z-index.
    SetZIndex { id: NodeId, value: i32 },

    // ─── Overflow Commands ────────────────────────────────────────
    /// Set overflow.
    SetOverflow {
        id: NodeId,
        value: crate::tree::Overflow,
    },

    // ─── Focus Commands ───────────────────────────────────────────
    /// Focus a node.
    FocusNode { id: NodeId },

    /// Blur a node.
    BlurNode { id: NodeId },

    /// Set tab index.
    SetTabIndex { id: NodeId, value: i32 },

    // ─── Frame Commands ───────────────────────────────────────────
    /// Begin a new frame.
    BeginFrame { frame_id: u64 },

    /// Commit the current frame.
    CommitFrame { frame_id: u64 },

    /// Invalidate a node (mark as needing re-render).
    Invalidate { id: NodeId },

    // ─── Lifecycle Commands ───────────────────────────────────────
    /// Shut down the engine.
    Shutdown,
}

impl Command {
    /// Get the target node ID for this command, if applicable.
    pub fn target(&self) -> Option<NodeId> {
        match self {
            Self::CreateNode { id, .. } => Some(*id),
            Self::RemoveNode { id } => Some(*id),
            Self::AppendChild { parent, .. } => Some(*parent),
            Self::InsertBefore { reference, .. } => Some(*reference),
            Self::MoveNode { node, .. } => Some(*node),
            Self::ReplaceNode { old, .. } => Some(*old),
            Self::DetachNode { id } => Some(*id),
            Self::SetStyle { id, .. } => Some(*id),
            Self::SetForeground { id, .. } => Some(*id),
            Self::SetBackground { id, .. } => Some(*id),
            Self::SetBold { id, .. } => Some(*id),
            Self::SetItalic { id, .. } => Some(*id),
            Self::SetUnderline { id, .. } => Some(*id),
            Self::SetStrikethrough { id, .. } => Some(*id),
            Self::SetDim { id, .. } => Some(*id),
            Self::SetInverse { id, .. } => Some(*id),
            Self::SetHidden { id, .. } => Some(*id),
            Self::SetLayout { id, .. } => Some(*id),
            Self::SetFlexDirection { id, .. } => Some(*id),
            Self::SetFlexWrap { id, .. } => Some(*id),
            Self::SetJustifyContent { id, .. } => Some(*id),
            Self::SetAlignItems { id, .. } => Some(*id),
            Self::SetAlignSelf { id, .. } => Some(*id),
            Self::SetWidth { id, .. } => Some(*id),
            Self::SetHeight { id, .. } => Some(*id),
            Self::SetMinWidth { id, .. } => Some(*id),
            Self::SetMinHeight { id, .. } => Some(*id),
            Self::SetMaxWidth { id, .. } => Some(*id),
            Self::SetMaxHeight { id, .. } => Some(*id),
            Self::SetPadding { id, .. } => Some(*id),
            Self::SetMargin { id, .. } => Some(*id),
            Self::SetGap { id, .. } => Some(*id),
            Self::SetFlexGrow { id, .. } => Some(*id),
            Self::SetFlexShrink { id, .. } => Some(*id),
            Self::SetFlexBasis { id, .. } => Some(*id),
            Self::SetPosition { id, .. } => Some(*id),
            Self::SetInset { id, .. } => Some(*id),
            Self::SetText { id, .. } => Some(*id),
            Self::SetAttribute { id, .. } => Some(*id),
            Self::RemoveAttribute { id, .. } => Some(*id),
            Self::SetDisplay { id, .. } => Some(*id),
            Self::SetOpacity { id, .. } => Some(*id),
            Self::SetClip { id, .. } => Some(*id),
            Self::SetTranslateX { id, .. } => Some(*id),
            Self::SetTranslateY { id, .. } => Some(*id),
            Self::SetZIndex { id, .. } => Some(*id),
            Self::SetOverflow { id, .. } => Some(*id),
            Self::FocusNode { id } => Some(*id),
            Self::BlurNode { id } => Some(*id),
            Self::SetTabIndex { id, .. } => Some(*id),
            Self::Invalidate { id } => Some(*id),
            Self::BeginFrame { .. } => None,
            Self::CommitFrame { .. } => None,
            Self::Shutdown => None,
        }
    }

    /// Get a human-readable name for this command variant.
    pub fn name(&self) -> &'static str {
        match self {
            Self::CreateNode { .. } => "CreateNode",
            Self::RemoveNode { .. } => "RemoveNode",
            Self::AppendChild { .. } => "AppendChild",
            Self::InsertBefore { .. } => "InsertBefore",
            Self::MoveNode { .. } => "MoveNode",
            Self::ReplaceNode { .. } => "ReplaceNode",
            Self::DetachNode { .. } => "DetachNode",
            Self::SetStyle { .. } => "SetStyle",
            Self::SetForeground { .. } => "SetForeground",
            Self::SetBackground { .. } => "SetBackground",
            Self::SetBold { .. } => "SetBold",
            Self::SetItalic { .. } => "SetItalic",
            Self::SetUnderline { .. } => "SetUnderline",
            Self::SetStrikethrough { .. } => "SetStrikethrough",
            Self::SetDim { .. } => "SetDim",
            Self::SetInverse { .. } => "SetInverse",
            Self::SetHidden { .. } => "SetHidden",
            Self::SetLayout { .. } => "SetLayout",
            Self::SetFlexDirection { .. } => "SetFlexDirection",
            Self::SetFlexWrap { .. } => "SetFlexWrap",
            Self::SetJustifyContent { .. } => "SetJustifyContent",
            Self::SetAlignItems { .. } => "SetAlignItems",
            Self::SetAlignSelf { .. } => "SetAlignSelf",
            Self::SetWidth { .. } => "SetWidth",
            Self::SetHeight { .. } => "SetHeight",
            Self::SetMinWidth { .. } => "SetMinWidth",
            Self::SetMinHeight { .. } => "SetMinHeight",
            Self::SetMaxWidth { .. } => "SetMaxWidth",
            Self::SetMaxHeight { .. } => "SetMaxHeight",
            Self::SetPadding { .. } => "SetPadding",
            Self::SetMargin { .. } => "SetMargin",
            Self::SetGap { .. } => "SetGap",
            Self::SetFlexGrow { .. } => "SetFlexGrow",
            Self::SetFlexShrink { .. } => "SetFlexShrink",
            Self::SetFlexBasis { .. } => "SetFlexBasis",
            Self::SetPosition { .. } => "SetPosition",
            Self::SetInset { .. } => "SetInset",
            Self::SetText { .. } => "SetText",
            Self::SetAttribute { .. } => "SetAttribute",
            Self::RemoveAttribute { .. } => "RemoveAttribute",
            Self::SetDisplay { .. } => "SetDisplay",
            Self::SetOpacity { .. } => "SetOpacity",
            Self::SetClip { .. } => "SetClip",
            Self::SetTranslateX { .. } => "SetTranslateX",
            Self::SetTranslateY { .. } => "SetTranslateY",
            Self::SetZIndex { .. } => "SetZIndex",
            Self::SetOverflow { .. } => "SetOverflow",
            Self::FocusNode { .. } => "FocusNode",
            Self::BlurNode { .. } => "BlurNode",
            Self::SetTabIndex { .. } => "SetTabIndex",
            Self::BeginFrame { .. } => "BeginFrame",
            Self::CommitFrame { .. } => "CommitFrame",
            Self::Invalidate { .. } => "Invalidate",
            Self::Shutdown => "Shutdown",
        }
    }

    /// Returns true if this command creates a node.
    pub fn is_create(&self) -> bool {
        matches!(self, Self::CreateNode { .. })
    }

    /// Returns true if this command removes a node.
    pub fn is_remove(&self) -> bool {
        matches!(self, Self::RemoveNode { .. })
    }

    /// Returns true if this command modifies the tree structure.
    pub fn is_tree_mutation(&self) -> bool {
        matches!(
            self,
            Self::CreateNode { .. }
                | Self::RemoveNode { .. }
                | Self::AppendChild { .. }
                | Self::InsertBefore { .. }
                | Self::MoveNode { .. }
                | Self::ReplaceNode { .. }
                | Self::DetachNode { .. }
        )
    }

    /// Returns true if this command is a frame lifecycle command.
    pub fn is_frame_command(&self) -> bool {
        matches!(self, Self::BeginFrame { .. } | Self::CommitFrame { .. })
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreateNode { id, kind } => write!(f, "CreateNode({id:?}, {kind:?})"),
            Self::RemoveNode { id } => write!(f, "RemoveNode({id:?})"),
            Self::AppendChild { parent, child } => {
                write!(f, "AppendChild({parent:?}, {child:?})")
            }
            Self::InsertBefore { reference, child } => {
                write!(f, "InsertBefore({reference:?}, {child:?})")
            }
            Self::MoveNode { node, new_parent } => {
                write!(f, "MoveNode({node:?}, {new_parent:?})")
            }
            Self::ReplaceNode { old, new } => write!(f, "ReplaceNode({old:?}, {new:?})"),
            Self::DetachNode { id } => write!(f, "DetachNode({id:?})"),
            Self::SetStyle { id, style } => write!(f, "SetStyle({id:?}, {style:?})"),
            Self::SetText { id, text } => write!(f, "SetText({id:?}, \"{}\")", text),
            Self::SetForeground { id, color } => write!(f, "SetForeground({id:?}, {color:?})"),
            Self::SetBackground { id, color } => write!(f, "SetBackground({id:?}, {color:?})"),
            Self::SetBold { id, value } => write!(f, "SetBold({id:?}, {value})"),
            Self::SetItalic { id, value } => write!(f, "SetItalic({id:?}, {value})"),
            Self::SetUnderline { id, value } => write!(f, "SetUnderline({id:?}, {value})"),
            Self::SetStrikethrough { id, value } => {
                write!(f, "SetStrikethrough({id:?}, {value})")
            }
            Self::SetDim { id, value } => write!(f, "SetDim({id:?}, {value})"),
            Self::SetInverse { id, value } => write!(f, "SetInverse({id:?}, {value})"),
            Self::SetHidden { id, value } => write!(f, "SetHidden({id:?}, {value})"),
            Self::SetLayout { id, layout } => write!(f, "SetLayout({id:?}, {layout:?})"),
            Self::SetFlexDirection { id, direction } => {
                write!(f, "SetFlexDirection({id:?}, {direction:?})")
            }
            Self::SetFlexWrap { id, value } => {
                write!(f, "SetFlexWrap({id:?}, {value:?})")
            }
            Self::SetJustifyContent { id, value } => {
                write!(f, "SetJustifyContent({id:?}, {value:?})")
            }
            Self::SetAlignItems { id, value } => write!(f, "SetAlignItems({id:?}, {value:?})"),
            Self::SetAlignSelf { id, value } => write!(f, "SetAlignSelf({id:?}, {value:?})"),
            Self::SetWidth { id, value } => write!(f, "SetWidth({id:?}, {value:?})"),
            Self::SetHeight { id, value } => write!(f, "SetHeight({id:?}, {value:?})"),
            Self::SetMinWidth { id, value } => write!(f, "SetMinWidth({id:?}, {value:?})"),
            Self::SetMinHeight { id, value } => write!(f, "SetMinHeight({id:?}, {value:?})"),
            Self::SetMaxWidth { id, value } => write!(f, "SetMaxWidth({id:?}, {value:?})"),
            Self::SetMaxHeight { id, value } => write!(f, "SetMaxHeight({id:?}, {value:?})"),
            Self::SetPadding { id, value } => write!(f, "SetPadding({id:?}, {value:?})"),
            Self::SetMargin { id, value } => write!(f, "SetMargin({id:?}, {value:?})"),
            Self::SetGap { id, value } => write!(f, "SetGap({id:?}, {value:?})"),
            Self::SetFlexGrow { id, value } => write!(f, "SetFlexGrow({id:?}, {value})"),
            Self::SetFlexShrink { id, value } => write!(f, "SetFlexShrink({id:?}, {value})"),
            Self::SetFlexBasis { id, value } => write!(f, "SetFlexBasis({id:?}, {value:?})"),
            Self::SetPosition { id, value } => write!(f, "SetPosition({id:?}, {value:?})"),
            Self::SetInset { id, value } => write!(f, "SetInset({id:?}, {value:?})"),
            Self::SetAttribute { id, key, value } => {
                write!(f, "SetAttribute({id:?}, \"{key}\", \"{value}\")")
            }
            Self::RemoveAttribute { id, key } => {
                write!(f, "RemoveAttribute({id:?}, \"{key}\")")
            }
            Self::SetDisplay { id, value } => write!(f, "SetDisplay({id:?}, {value:?})"),
            Self::SetOpacity { id, value } => write!(f, "SetOpacity({id:?}, {value})"),
            Self::SetClip { id, value } => write!(f, "SetClip({id:?}, {value})"),
            Self::SetTranslateX { id, value } => write!(f, "SetTranslateX({id:?}, {value})"),
            Self::SetTranslateY { id, value } => write!(f, "SetTranslateY({id:?}, {value})"),
            Self::SetZIndex { id, value } => write!(f, "SetZIndex({id:?}, {value})"),
            Self::SetOverflow { id, value } => write!(f, "SetOverflow({id:?}, {value:?})"),
            Self::FocusNode { id } => write!(f, "FocusNode({id:?})"),
            Self::BlurNode { id } => write!(f, "BlurNode({id:?})"),
            Self::SetTabIndex { id, value } => write!(f, "SetTabIndex({id:?}, {value})"),
            Self::BeginFrame { frame_id } => write!(f, "BeginFrame({frame_id})"),
            Self::CommitFrame { frame_id } => write!(f, "CommitFrame({frame_id})"),
            Self::Invalidate { id } => write!(f, "Invalidate({id:?})"),
            Self::Shutdown => write!(f, "Shutdown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_target() {
        let id = NodeId::default();
        assert_eq!(
            Command::CreateNode {
                id,
                kind: NodeKind::Box
            }
            .target(),
            Some(id)
        );
        assert_eq!(Command::Shutdown.target(), None);
        assert_eq!(Command::BeginFrame { frame_id: 1 }.target(), None);
    }

    #[test]
    fn command_name() {
        assert_eq!(
            Command::CreateNode {
                id: NodeId::default(),
                kind: NodeKind::Box
            }
            .name(),
            "CreateNode"
        );
        assert_eq!(Command::Shutdown.name(), "Shutdown");
    }

    #[test]
    fn command_is_tree_mutation() {
        assert!(
            Command::CreateNode {
                id: NodeId::default(),
                kind: NodeKind::Box
            }
            .is_tree_mutation()
        );
        assert!(
            Command::AppendChild {
                parent: NodeId::default(),
                child: NodeId::default()
            }
            .is_tree_mutation()
        );
        assert!(
            !Command::SetBold {
                id: NodeId::default(),
                value: true
            }
            .is_tree_mutation()
        );
    }

    #[test]
    fn command_is_frame_command() {
        assert!(Command::BeginFrame { frame_id: 1 }.is_frame_command());
        assert!(Command::CommitFrame { frame_id: 1 }.is_frame_command());
        assert!(!Command::Shutdown.is_frame_command());
    }

    #[test]
    fn command_display() {
        let cmd = Command::SetText {
            id: NodeId::default(),
            text: "hello".into(),
        };
        let display = format!("{cmd}");
        assert!(display.contains("SetText"));
        assert!(display.contains("hello"));
    }
}
