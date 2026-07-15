//! Command protocol layer for communicating with the rendering engine.
//! Defines commands, buffers, processing, and error types.

use std::collections::VecDeque;

use crate::taffy::{FlexDirection, FlexWrap, LayoutProps};
use crate::tree::{Color, NodeArena, NodeId, NodeKind, RenderNode, Style};

// ══════════════════════════════════════════════════════════════════════════════
// COMMAND
// ══════════════════════════════════════════════════════════════════════════════

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
    SetFlexDirection { id: NodeId, direction: FlexDirection },

    /// Set flex wrap.
    SetFlexWrap { id: NodeId, value: FlexWrap },

    /// Set justify content.
    SetJustifyContent { id: NodeId, value: crate::taffy::JustifyContent },

    /// Set align items.
    SetAlignItems { id: NodeId, value: crate::taffy::AlignItems },

    /// Set align self.
    SetAlignSelf { id: NodeId, value: crate::taffy::AlignSelf },

    /// Set width.
    SetWidth { id: NodeId, value: crate::taffy::Sizing },

    /// Set height.
    SetHeight { id: NodeId, value: crate::taffy::Sizing },

    /// Set min width.
    SetMinWidth { id: NodeId, value: crate::taffy::Sizing },

    /// Set min height.
    SetMinHeight { id: NodeId, value: crate::taffy::Sizing },

    /// Set max width.
    SetMaxWidth { id: NodeId, value: crate::taffy::Sizing },

    /// Set max height.
    SetMaxHeight { id: NodeId, value: crate::taffy::Sizing },

    /// Set padding.
    SetPadding { id: NodeId, value: crate::taffy::RectValues },

    /// Set margin.
    SetMargin { id: NodeId, value: crate::taffy::RectValues },

    /// Set gap.
    SetGap { id: NodeId, value: crate::taffy::Gap },

    /// Set flex grow.
    SetFlexGrow { id: NodeId, value: f32 },

    /// Set flex shrink.
    SetFlexShrink { id: NodeId, value: f32 },

    /// Set flex basis.
    SetFlexBasis { id: NodeId, value: crate::taffy::Sizing },

    /// Set position.
    SetPosition { id: NodeId, value: crate::taffy::Position },

    /// Set inset.
    SetInset { id: NodeId, value: crate::taffy::RectValues },

    // ─── Content Commands ─────────────────────────────────────────
    /// Set text content of a node.
    SetText { id: NodeId, text: String },

    /// Set an attribute on a node.
    SetAttribute { id: NodeId, key: String, value: String },

    /// Remove an attribute from a node.
    RemoveAttribute { id: NodeId, key: String },

    // ─── Visibility Commands ──────────────────────────────────────
    /// Set display mode.
    SetDisplay { id: NodeId, value: crate::tree::VisibilityDisplay },

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
    SetOverflow { id: NodeId, value: crate::tree::Overflow },

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

// ══════════════════════════════════════════════════════════════════════════════
// COMMAND EXTENSION (Registry)
// ══════════════════════════════════════════════════════════════════════════════

/// A command that can be executed and potentially undone.
#[derive(Debug, Clone)]
pub struct CommandEntry {
    /// The command name/identifier.
    pub name: String,
    /// Platform-specific command data.
    pub data: String,
    /// Whether this command supports undo.
    pub undoable: bool,
    /// Timestamp (milliseconds since epoch).
    pub timestamp: u64,
}

impl CommandEntry {
    /// Creates a new command entry.
    pub fn new(name: impl Into<String>, data: impl Into<String>, undoable: bool) -> Self {
        Self { name: name.into(), data: data.into(), undoable, timestamp: 0 }
    }

    /// Sets the timestamp.
    pub fn with_timestamp(mut self, ts: u64) -> Self {
        self.timestamp = ts;
        self
    }
}

/// Result of executing a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryResult {
    /// Command executed successfully.
    Success,
    /// Command failed with a message.
    Failure(String),
    /// Command was not found.
    NotFound,
}

/// Manages command execution, undo/redo, and history.
#[derive(Debug)]
pub struct CommandRegistry {
    /// Command history (most recent last).
    history: VecDeque<CommandEntry>,
    /// Undo stack (commands that can be undone).
    undo_stack: Vec<CommandEntry>,
    /// Redo stack (commands that were undone).
    redo_stack: Vec<CommandEntry>,
    /// Maximum history size.
    max_history: usize,
    /// Maximum undo stack size.
    max_undo: usize,
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandRegistry {
    /// Creates a new CommandRegistry.
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history: 1000,
            max_undo: 100,
        }
    }

    /// Sets the maximum history size.
    pub fn with_max_history(mut self, max: usize) -> Self {
        self.max_history = max;
        self
    }

    /// Sets the maximum undo stack size.
    pub fn with_max_undo(mut self, max: usize) -> Self {
        self.max_undo = max;
        self
    }

    /// Executes a command and records it in history.
    pub fn execute(&mut self, entry: CommandEntry) -> RegistryResult {
        let undoable = entry.undoable;
        self.history.push_back(entry.clone());
        while self.history.len() > self.max_history {
            self.history.pop_front();
        }
        if undoable {
            self.undo_stack.push(entry);
            if self.undo_stack.len() > self.max_undo {
                self.undo_stack.remove(0);
            }
            // Clear redo stack on new command
            self.redo_stack.clear();
        }
        RegistryResult::Success
    }

    /// Undoes the last undoable command.
    pub fn undo(&mut self) -> Option<CommandEntry> {
        let entry = self.undo_stack.pop()?;
        self.redo_stack.push(entry.clone());
        Some(entry)
    }

    /// Redoes the last undone command.
    pub fn redo(&mut self) -> Option<CommandEntry> {
        let entry = self.redo_stack.pop()?;
        self.undo_stack.push(entry.clone());
        Some(entry)
    }

    /// Returns whether undo is possible.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Returns whether redo is possible.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Returns the command history.
    pub fn history(&self) -> impl Iterator<Item = &CommandEntry> {
        self.history.iter()
    }

    /// Returns the number of commands in history.
    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    /// Returns the undo stack depth.
    pub fn undo_depth(&self) -> usize {
        self.undo_stack.len()
    }

    /// Returns the redo stack depth.
    pub fn redo_depth(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clears all history and stacks.
    pub fn clear(&mut self) {
        self.history.clear();
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Finds commands by name.
    pub fn find(&self, name: &str) -> Vec<&CommandEntry> {
        self.history.iter().filter(|e| e.name == name).collect()
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// COMMAND BUFFER
// ══════════════════════════════════════════════════════════════════════════════

/// Pre-allocated buffer for batching commands before sending to the engine.
///
/// Commands are accumulated in the buffer during a React render cycle,
/// then flushed to the engine in a single FFI call.
pub struct CommandBuffer {
    commands: Vec<Command>,
    capacity: usize,
}

impl Default for CommandBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandBuffer {
    /// Create a new buffer with default capacity.
    pub fn new() -> Self {
        Self::with_capacity(64)
    }

    /// Create a new buffer with a specific capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self { commands: Vec::with_capacity(capacity), capacity }
    }

    /// Push a command into the buffer.
    pub fn push(&mut self, cmd: Command) {
        self.commands.push(cmd);
    }

    /// Take all commands from the buffer, leaving it empty.
    pub fn drain(&mut self) -> Vec<Command> {
        std::mem::take(&mut self.commands)
    }

    /// Peek at the commands without taking them.
    pub fn peek(&self) -> &[Command] {
        &self.commands
    }

    /// Clear the buffer.
    pub fn clear(&mut self) {
        self.commands.clear();
    }

    /// Number of commands in the buffer.
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Returns true if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// Estimated byte size of the buffer contents.
    pub fn estimated_size(&self) -> usize {
        // Rough estimate: each command is ~64-256 bytes
        self.commands.len() * 128
    }

    /// Pre-allocate for at least `additional` more commands.
    pub fn reserve(&mut self, additional: usize) {
        self.commands.reserve(additional);
    }

    /// Get the capacity of the buffer.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl From<Vec<Command>> for CommandBuffer {
    fn from(commands: Vec<Command>) -> Self {
        let capacity = commands.len();
        Self { commands, capacity }
    }
}

impl IntoIterator for CommandBuffer {
    type Item = Command;
    type IntoIter = std::vec::IntoIter<Command>;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.into_iter()
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// COMMAND ERROR
// ══════════════════════════════════════════════════════════════════════════════

/// Errors that can occur during command processing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandError {
    /// Node with the given ID was not found.
    NodeNotFound(NodeId),
    /// A cycle would be created by this operation.
    CycleDetected { node: NodeId, ancestor: NodeId },
    /// The operation is invalid for some other reason.
    InvalidOperation(String),
    /// The command is not valid in the current state.
    InvalidState(String),
    /// The command references a node that was already removed.
    StaleReference(NodeId),
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NodeNotFound(id) => write!(f, "Node not found: {id:?}"),
            Self::CycleDetected { node, ancestor } => {
                write!(f, "Cycle detected: node {node:?} is ancestor of {ancestor:?}")
            }
            Self::InvalidOperation(msg) => write!(f, "Invalid operation: {msg}"),
            Self::InvalidState(msg) => write!(f, "Invalid state: {msg}"),
            Self::StaleReference(id) => write!(f, "Stale reference: {id:?}"),
        }
    }
}

impl std::error::Error for CommandError {}

impl From<crate::tree::TreeError> for CommandError {
    fn from(err: crate::tree::TreeError) -> Self {
        match err {
            crate::tree::TreeError::NodeNotFound(id) => Self::NodeNotFound(id),
            crate::tree::TreeError::CycleDetected { node, ancestor } => Self::CycleDetected { node, ancestor },
            crate::tree::TreeError::InvalidOperation(msg) => Self::InvalidOperation(msg),
        }
    }
}

/// Non-fatal warnings that can occur during command processing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandWarning {
    /// A command targeted a node that doesn't exist (skipped).
    NodeSkipped(NodeId),
    /// A style property was set but has no effect in the current context.
    NoEffect(String),
    /// A command was redundant (e.g., setting the same value twice).
    Redundant(String),
    /// A deprecated command was used.
    Deprecated(String),
}

impl std::fmt::Display for CommandWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NodeSkipped(id) => write!(f, "Node skipped: {id:?}"),
            Self::NoEffect(msg) => write!(f, "No effect: {msg}"),
            Self::Redundant(msg) => write!(f, "Redundant: {msg}"),
            Self::Deprecated(msg) => write!(f, "Deprecated: {msg}"),
        }
    }
}

impl std::error::Error for CommandWarning {}

// ══════════════════════════════════════════════════════════════════════════════
// COMMAND RESULT
// ══════════════════════════════════════════════════════════════════════════════

/// Result of processing a batch of commands.
#[derive(Debug, Clone, Default)]
pub struct CommandResult {
    /// Number of commands successfully processed.
    pub processed: usize,
    /// Number of commands that failed.
    pub failed: usize,
    /// Errors from failed commands.
    pub errors: Vec<CommandError>,
    /// Non-fatal warnings.
    pub warnings: Vec<CommandWarning>,
}

impl CommandResult {
    /// Create an empty result.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a result for a single successful command.
    pub fn success() -> Self {
        Self { processed: 1, ..Default::default() }
    }

    /// Create a result for a single failed command.
    pub fn error(err: CommandError) -> Self {
        Self { failed: 1, errors: vec![err], ..Default::default() }
    }

    /// Add a success to the result.
    pub fn push_success(&mut self) {
        self.processed += 1;
    }

    /// Add a failure to the result.
    pub fn push_error(&mut self, err: CommandError) {
        self.failed += 1;
        self.errors.push(err);
    }

    /// Add a warning to the result.
    pub fn push_warning(&mut self, warn: CommandWarning) {
        self.warnings.push(warn);
    }

    /// Merge another result into this one.
    pub fn merge(&mut self, other: CommandResult) {
        self.processed += other.processed;
        self.failed += other.failed;
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
    }

    /// Returns true if all commands succeeded.
    pub fn is_success(&self) -> bool {
        self.failed == 0
    }

    /// Returns true if any commands failed.
    pub fn has_errors(&self) -> bool {
        self.failed > 0
    }

    /// Total commands processed (success + failed).
    pub fn total(&self) -> usize {
        self.processed + self.failed
    }
}

impl std::fmt::Display for CommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CommandResult(processed={}, failed={}, warnings={})",
            self.processed,
            self.failed,
            self.warnings.len()
        )
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// COMMAND PROCESSOR
// ══════════════════════════════════════════════════════════════════════════════

/// Processes commands and applies them to the tree.
///
/// The processor is the bridge between the command buffer and the arena.
/// It validates commands, applies them atomically, and propagates dirty flags.
pub struct CommandProcessor {
    arena: NodeArena,
    frame_id: u64,
}

impl Default for CommandProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandProcessor {
    /// Create a new processor with a fresh arena.
    pub fn new() -> Self {
        Self { arena: NodeArena::new(), frame_id: 0 }
    }

    /// Create a processor with an existing arena.
    pub fn with_arena(arena: NodeArena) -> Self {
        Self { arena, frame_id: 0 }
    }

    /// Get a reference to the arena.
    pub fn arena(&self) -> &NodeArena {
        &self.arena
    }

    /// Get a mutable reference to the arena.
    pub fn arena_mut(&mut self) -> &mut NodeArena {
        &mut self.arena
    }

    /// Get the current frame ID.
    pub fn frame_id(&self) -> u64 {
        self.frame_id
    }

    /// Process a batch of commands atomically.
    ///
    /// All commands are processed in order. If any command fails, it is recorded
    /// as an error but processing continues with the remaining commands.
    pub fn process_batch(&mut self, commands: Vec<Command>) -> CommandResult {
        let mut result = CommandResult::new();
        for cmd in commands {
            match self.process_single(cmd) {
                Ok(()) => result.push_success(),
                Err(err) => result.push_error(err),
            }
        }
        result
    }

    /// Process a single command.
    pub fn process_single(&mut self, cmd: Command) -> Result<(), CommandError> {
        match cmd {
            // ─── Tree Commands ─────────────────────────────────────
            Command::CreateNode { id: _, kind } => {
                let node = RenderNode::new(kind);
                let _created_id = self.arena.insert(node);
                Ok(())
            }
            Command::RemoveNode { id } => {
                self.arena.remove_subtree(id);
                Ok(())
            }
            Command::AppendChild { parent, child } => {
                self.arena.append_child(parent, child)?;
                Ok(())
            }
            Command::InsertBefore { reference, child } => {
                self.arena.insert_before(reference, child)?;
                Ok(())
            }
            Command::MoveNode { node, new_parent } => {
                self.arena.move_node(node, new_parent)?;
                Ok(())
            }
            Command::ReplaceNode { old, new } => {
                self.arena.replace_node(old, new)?;
                Ok(())
            }
            Command::DetachNode { id } => {
                self.arena.detach(id);
                Ok(())
            }

            // ─── Style Commands ───────────────────────────────────
            Command::SetStyle { id, style } => {
                let node = self.get_node_mut(id)?;
                node.style = style;
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetForeground { id, color } => {
                let node = self.get_node_mut(id)?;
                node.style.fg = Some(color);
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetBackground { id, color } => {
                let node = self.get_node_mut(id)?;
                node.style.bg = Some(color);
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetBold { id, value } => {
                let node = self.get_node_mut(id)?;
                node.style.bold = Some(value);
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetItalic { id, value } => {
                let node = self.get_node_mut(id)?;
                node.style.italic = Some(value);
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetUnderline { id, value } => {
                let node = self.get_node_mut(id)?;
                node.style.underline = Some(value);
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetStrikethrough { id, value } => {
                let node = self.get_node_mut(id)?;
                node.style.strikethrough = Some(value);
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetDim { id, value } => {
                let node = self.get_node_mut(id)?;
                node.style.dim = Some(value);
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetInverse { id, value } => {
                let node = self.get_node_mut(id)?;
                node.style.inverse = Some(value);
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetHidden { id, value } => {
                let node = self.get_node_mut(id)?;
                node.style.hidden = Some(value);
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }

            // ─── Layout Commands ──────────────────────────────────
            Command::SetLayout { id, layout } => {
                let node = self.get_node_mut(id)?;
                node.layout = layout;
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetFlexDirection { id, direction } => {
                let node = self.get_node_mut(id)?;
                node.layout.direction = direction;
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetFlexWrap { id, value } => {
                let node = self.get_node_mut(id)?;
                node.layout.flex_wrap = value;
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetJustifyContent { id, value } => {
                let node = self.get_node_mut(id)?;
                node.layout.justify = value;
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetAlignItems { id, value } => {
                let node = self.get_node_mut(id)?;
                node.layout.align = value;
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetAlignSelf { id, value } => {
                let node = self.get_node_mut(id)?;
                node.layout.align_self = Some(value);
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetWidth { id, value } => {
                let node = self.get_node_mut(id)?;
                node.layout.width = Some(value);
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetHeight { id, value } => {
                let node = self.get_node_mut(id)?;
                node.layout.height = Some(value);
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetMinWidth { id, value } => {
                let node = self.get_node_mut(id)?;
                node.layout.min_width = Some(value);
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetMinHeight { id, value } => {
                let node = self.get_node_mut(id)?;
                node.layout.min_height = Some(value);
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetMaxWidth { id, value } => {
                let node = self.get_node_mut(id)?;
                node.layout.max_width = Some(value);
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetMaxHeight { id, value } => {
                let node = self.get_node_mut(id)?;
                node.layout.max_height = Some(value);
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetPadding { id, value } => {
                let node = self.get_node_mut(id)?;
                node.layout.padding = Some(value);
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetMargin { id, value } => {
                let node = self.get_node_mut(id)?;
                node.layout.margin = Some(value);
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetGap { id, value } => {
                let node = self.get_node_mut(id)?;
                node.layout.gap = Some(value);
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetFlexGrow { id, value } => {
                let node = self.get_node_mut(id)?;
                node.layout.flex_grow = value;
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetFlexShrink { id, value } => {
                let node = self.get_node_mut(id)?;
                node.layout.flex_shrink = value;
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetFlexBasis { id, value } => {
                let node = self.get_node_mut(id)?;
                node.layout.flex_basis = Some(value);
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetPosition { id, value } => {
                let node = self.get_node_mut(id)?;
                node.layout.position = value;
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetInset { id, value } => {
                let node = self.get_node_mut(id)?;
                node.layout.inset = Some(value);
                node.state.mark_layout_dirty();
                self.arena.mark_changed();
                Ok(())
            }

            // ─── Content Commands ─────────────────────────────────
            Command::SetText { id, text } => {
                let node = self.get_node_mut(id)?;
                node.set_text(text);
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetAttribute { id, key, value } => {
                let node = self.get_node_mut(id)?;
                node.attributes.insert(key, value);
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::RemoveAttribute { id, key } => {
                let node = self.get_node_mut(id)?;
                node.attributes.remove(&key);
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }

            // ─── Visibility Commands ──────────────────────────────
            Command::SetDisplay { id, value } => {
                let node = self.get_node_mut(id)?;
                node.visibility.display = value;
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetOpacity { id, value } => {
                let node = self.get_node_mut(id)?;
                node.visibility.opacity = value;
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetClip { id, value } => {
                let node = self.get_node_mut(id)?;
                node.visibility.clip = value;
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }

            // ─── Transform Commands ───────────────────────────────
            Command::SetTranslateX { id, value } => {
                let node = self.get_node_mut(id)?;
                node.transform.translate_x = value;
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetTranslateY { id, value } => {
                let node = self.get_node_mut(id)?;
                node.transform.translate_y = value;
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetZIndex { id, value } => {
                let node = self.get_node_mut(id)?;
                node.transform.z_index = value;
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }

            // ─── Overflow Commands ────────────────────────────────
            Command::SetOverflow { id, value } => {
                let node = self.get_node_mut(id)?;
                node.overflow = value;
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }

            // ─── Focus Commands ───────────────────────────────────
            Command::FocusNode { id } => {
                let node = self.get_node_mut(id)?;
                node.focus();
                self.arena.mark_changed();
                Ok(())
            }
            Command::BlurNode { id } => {
                let node = self.get_node_mut(id)?;
                node.blur();
                self.arena.mark_changed();
                Ok(())
            }
            Command::SetTabIndex { id, value } => {
                let node = self.get_node_mut(id)?;
                node.focus.tab_index = Some(value);
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }

            // ─── Frame Commands ───────────────────────────────────
            Command::BeginFrame { frame_id } => {
                self.frame_id = frame_id;
                Ok(())
            }
            Command::CommitFrame { frame_id: _ } => Ok(()),
            Command::Invalidate { id } => {
                let node = self.get_node_mut(id)?;
                node.state.mark_dirty();
                self.arena.mark_changed();
                Ok(())
            }

            // ─── Lifecycle Commands ───────────────────────────────
            Command::Shutdown => {
                self.arena.clear();
                Ok(())
            }
        }
    }

    /// Get a mutable reference to a node, returning an error if not found.
    fn get_node_mut(&mut self, id: NodeId) -> Result<&mut RenderNode, CommandError> {
        self.arena.get_mut(id).ok_or(CommandError::NodeNotFound(id))
    }

    /// Get a reference to a node, returning an error if not found.
    pub fn get_node(&self, id: NodeId) -> Option<&RenderNode> {
        self.arena.get(id)
    }

    /// Validate the tree invariants.
    pub fn validate(&self) -> Result<(), CommandError> {
        self.arena.validate().map_err(CommandError::from)
    }

    /// Print the tree for debugging.
    pub fn print_tree(&self) -> String {
        self.arena.print_tree()
    }

    /// Get the number of nodes in the arena.
    pub fn node_count(&self) -> usize {
        self.arena.len()
    }
}

impl std::fmt::Debug for CommandProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandProcessor")
            .field("node_count", &self.node_count())
            .field("frame_id", &self.frame_id)
            .finish()
    }
}
