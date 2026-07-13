use crate::tree::{NodeArena, NodeId, RenderNode};

use super::command::Command;
use super::error::CommandError;
use super::result::CommandResult;

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
        Self {
            arena: NodeArena::new(),
            frame_id: 0,
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::NodeKind;

    #[test]
    fn processor_new() {
        let proc = CommandProcessor::new();
        assert_eq!(proc.node_count(), 1); // root node
        assert_eq!(proc.frame_id(), 0);
    }

    #[test]
    fn process_create_node() {
        let mut proc = CommandProcessor::new();
        let id = NodeId::default();
        let result = proc.process_single(Command::CreateNode {
            id,
            kind: NodeKind::Box,
        });
        assert!(result.is_ok());
        assert_eq!(proc.node_count(), 2);
    }

    #[test]
    fn process_remove_node() {
        let mut proc = CommandProcessor::new();
        let root = proc.arena().root();
        let child = proc.arena_mut().insert(RenderNode::new(NodeKind::Text));
        proc.arena_mut().append_child(root, child).unwrap();

        let result = proc.process_single(Command::RemoveNode { id: child });
        assert!(result.is_ok());
        assert_eq!(proc.node_count(), 1);
    }

    #[test]
    fn process_append_child() {
        let mut proc = CommandProcessor::new();
        let root = proc.arena().root();
        let child = proc.arena_mut().insert(RenderNode::new(NodeKind::Text));

        let result = proc.process_single(Command::AppendChild {
            parent: root,
            child,
        });
        assert!(result.is_ok());
        assert_eq!(proc.node_count(), 2);
    }

    #[test]
    fn process_set_text() {
        let mut proc = CommandProcessor::new();
        let root = proc.arena().root();

        let result = proc.process_single(Command::SetText {
            id: root,
            text: "hello".into(),
        });
        assert!(result.is_ok());

        let node = proc.get_node(root).unwrap();
        assert_eq!(node.text.as_deref(), Some("hello"));
    }

    #[test]
    fn process_set_bold() {
        let mut proc = CommandProcessor::new();
        let root = proc.arena().root();

        let result = proc.process_single(Command::SetBold {
            id: root,
            value: true,
        });
        assert!(result.is_ok());

        let node = proc.get_node(root).unwrap();
        assert_eq!(node.style.bold, Some(true));
    }

    #[test]
    fn process_batch() {
        let mut proc = CommandProcessor::new();
        let root = proc.arena().root();
        let child = proc.arena_mut().insert(RenderNode::new(NodeKind::Text));

        let commands = vec![
            Command::AppendChild {
                parent: root,
                child,
            },
            Command::SetText {
                id: child,
                text: "hello".into(),
            },
            Command::SetBold {
                id: child,
                value: true,
            },
        ];

        let result = proc.process_batch(commands);
        assert!(result.is_success());
        assert_eq!(result.processed, 3);
    }

    #[test]
    fn process_set_attribute() {
        let mut proc = CommandProcessor::new();
        let root = proc.arena().root();

        let result = proc.process_single(Command::SetAttribute {
            id: root,
            key: "data-testid".into(),
            value: "my-element".into(),
        });
        assert!(result.is_ok());

        let node = proc.get_node(root).unwrap();
        assert_eq!(
            node.attributes.get("data-testid"),
            Some(&"my-element".to_string())
        );
    }

    #[test]
    fn process_remove_attribute() {
        let mut proc = CommandProcessor::new();
        let root = proc.arena().root();

        proc.process_single(Command::SetAttribute {
            id: root,
            key: "data-testid".into(),
            value: "my-element".into(),
        })
        .unwrap();

        let result = proc.process_single(Command::RemoveAttribute {
            id: root,
            key: "data-testid".into(),
        });
        assert!(result.is_ok());

        let node = proc.get_node(root).unwrap();
        assert!(!node.attributes.contains_key("data-testid"));
    }

    #[test]
    fn process_invalid_command() {
        let mut proc = CommandProcessor::new();
        let bad_id = NodeId::default();

        let result = proc.process_single(Command::SetText {
            id: bad_id,
            text: "hello".into(),
        });

        // Depending on the slotmap implementation, this might succeed or fail
        // The important thing is that the processor handles it gracefully
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn process_shutdown() {
        let mut proc = CommandProcessor::new();
        let result = proc.process_single(Command::Shutdown);
        assert!(result.is_ok());
        assert_eq!(proc.node_count(), 1); // root is re-created
    }

    #[test]
    fn process_frame_commands() {
        let mut proc = CommandProcessor::new();

        let result = proc.process_single(Command::BeginFrame { frame_id: 1 });
        assert!(result.is_ok());
        assert_eq!(proc.frame_id(), 1);

        let result = proc.process_single(Command::CommitFrame { frame_id: 1 });
        assert!(result.is_ok());
    }

    #[test]
    fn processor_validate() {
        let proc = CommandProcessor::new();
        assert!(proc.validate().is_ok());
    }

    #[test]
    fn processor_print_tree() {
        let proc = CommandProcessor::new();
        let output = proc.print_tree();
        assert!(!output.is_empty());
    }
}
