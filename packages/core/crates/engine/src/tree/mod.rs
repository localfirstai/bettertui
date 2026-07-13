//! Tree data structures: arena-backed node tree with layout, style, and visual properties.
//! The core data model for BetterTUI's render tree.

pub mod arena;
pub mod color;
pub mod interaction;
pub mod metadata;
pub mod node_id;
pub mod node_kind;
pub mod render_node;
pub mod style;
pub mod tree_error;
pub mod visual;

pub use arena::NodeArena;
pub use color::{Color, NamedColor};
pub use interaction::{EventHandlers, FocusProps, NodeState, UpdateFlags};
pub use metadata::{Accessibility, AriaLabel, AriaLive, AriaRole, Metadata};
pub use node_id::NodeId;
pub use node_kind::NodeKind;
pub use render_node::RenderNode;
pub use style::{ResolvedStyle, Style};
pub use tree_error::TreeError;
pub use visual::{
    CursorProps, CursorStyle, Display as VisibilityDisplay, Overflow, Point, Rect, Size, Transform,
    Visibility,
};
