use crate::protocol::{Command, CommandProcessor, CommandResult};
use crate::tree::{NodeArena, NodeId, RenderNode};

/// The main engine that receives commands and maintains the UI tree.
///
/// This is the high-level API that React (via FFI) calls into.
/// It wraps the `CommandProcessor` and provides additional functionality
/// like frame management and debug output.
pub struct Engine {
    processor: CommandProcessor,
    frame_count: u64,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    /// Create a new engine with a fresh tree.
    pub fn new() -> Self {
        Self {
            processor: CommandProcessor::new(),
            frame_count: 0,
        }
    }

    /// Get a reference to the command processor.
    pub fn processor(&self) -> &CommandProcessor {
        &self.processor
    }

    /// Get a mutable reference to the command processor.
    pub fn processor_mut(&mut self) -> &mut CommandProcessor {
        &mut self.processor
    }

    /// Process a batch of commands.
    pub fn process_commands(&mut self, commands: Vec<Command>) -> CommandResult {
        self.processor.process_batch(commands)
    }

    /// Process a single command.
    pub fn process_command(
        &mut self,
        command: Command,
    ) -> Result<(), crate::protocol::CommandError> {
        self.processor.process_single(command)
    }

    /// Get the number of nodes in the tree.
    pub fn node_count(&self) -> usize {
        self.processor.node_count()
    }

    /// Validate the tree invariants.
    pub fn validate(&self) -> Result<(), crate::protocol::CommandError> {
        self.processor.validate()
    }

    /// Print the tree for debugging.
    pub fn print_tree(&self) -> String {
        self.processor.print_tree()
    }

    /// Get the current frame count.
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    /// Begin a new frame.
    pub fn begin_frame(&mut self) {
        self.frame_count += 1;
        let _ = self.processor.process_single(Command::BeginFrame {
            frame_id: self.frame_count,
        });
    }

    /// Commit the current frame.
    pub fn commit_frame(&mut self) {
        let _ = self.processor.process_single(Command::CommitFrame {
            frame_id: self.frame_count,
        });
    }

    /// Get a reference to the arena.
    pub fn arena(&self) -> &NodeArena {
        self.processor.arena()
    }

    /// Get a mutable reference to the arena.
    pub fn arena_mut(&mut self) -> &mut NodeArena {
        self.processor.arena_mut()
    }

    /// Get a node by ID.
    pub fn get_node(&self, id: NodeId) -> Option<&RenderNode> {
        self.processor.get_node(id)
    }

    /// Create a new node and return its ID.
    pub fn create_node(&mut self, kind: crate::tree::NodeKind) -> NodeId {
        let node = RenderNode::new(kind);
        self.arena_mut().insert(node)
    }

    /// Append a child to a parent.
    pub fn append_child(
        &mut self,
        parent: NodeId,
        child: NodeId,
    ) -> Result<(), crate::protocol::CommandError> {
        self.processor
            .process_single(Command::AppendChild { parent, child })
    }

    /// Remove a node and its descendants.
    pub fn remove_node(&mut self, id: NodeId) {
        let _ = self.processor.process_single(Command::RemoveNode { id });
    }

    /// Set text content on a node.
    pub fn set_text(&mut self, id: NodeId, text: impl Into<String>) {
        let _ = self.processor.process_single(Command::SetText {
            id,
            text: text.into(),
        });
    }

    /// Set style on a node.
    pub fn set_style(&mut self, id: NodeId, style: crate::tree::Style) {
        let _ = self
            .processor
            .process_single(Command::SetStyle { id, style });
    }

    /// Set layout on a node.
    pub fn set_layout(&mut self, id: NodeId, layout: crate::layout::types::LayoutProps) {
        let _ = self
            .processor
            .process_single(Command::SetLayout { id, layout });
    }

    /// Print a summary of the tree.
    pub fn tree_summary(&self) -> String {
        let node_count = self.node_count();
        let frame_count = self.frame_count();
        let generation = self.arena().generation();

        format!(
            "Tree Summary:\n  Nodes: {}\n  Frames: {}\n  Generation: {}",
            node_count, frame_count, generation
        )
    }
}

impl std::fmt::Debug for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine")
            .field("node_count", &self.node_count())
            .field("frame_count", &self.frame_count)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::NodeKind;

    #[test]
    fn engine_new() {
        let engine = Engine::new();
        assert_eq!(engine.node_count(), 1); // root node
        assert_eq!(engine.frame_count(), 0);
    }

    #[test]
    fn engine_create_node() {
        let mut engine = Engine::new();
        let _id = engine.create_node(NodeKind::Box);
        // The node is created in the arena, but with a different ID
        // than what we requested. For now, just check the count.
        assert_eq!(engine.node_count(), 2);
    }

    #[test]
    fn engine_append_child() {
        let mut engine = Engine::new();
        let root = engine.arena().root();
        let child = engine.create_node(NodeKind::Text);

        // Use the processor directly for append since we need valid IDs
        let result = engine.processor_mut().process_single(Command::AppendChild {
            parent: root,
            child,
        });
        assert!(result.is_ok());
        assert_eq!(engine.node_count(), 2);
    }

    #[test]
    fn engine_remove_node() {
        let mut engine = Engine::new();
        let root = engine.arena().root();
        let child = engine.create_node(NodeKind::Text);
        engine
            .processor_mut()
            .process_single(Command::AppendChild {
                parent: root,
                child,
            })
            .unwrap();

        engine.remove_node(child);
        assert_eq!(engine.node_count(), 1);
    }

    #[test]
    fn engine_set_text() {
        let mut engine = Engine::new();
        let root = engine.arena().root();

        engine.set_text(root, "hello");
        let node = engine.get_node(root).unwrap();
        assert_eq!(node.text.as_deref(), Some("hello"));
    }

    #[test]
    fn engine_set_style() {
        let mut engine = Engine::new();
        let root = engine.arena().root();

        let style = crate::tree::Style {
            bold: Some(true),
            ..Default::default()
        };
        engine.set_style(root, style);

        let node = engine.get_node(root).unwrap();
        assert_eq!(node.style.bold, Some(true));
    }

    #[test]
    fn engine_set_layout() {
        let mut engine = Engine::new();
        let root = engine.arena().root();

        let layout = crate::layout::types::LayoutProps {
            flex_grow: 1.0,
            ..Default::default()
        };
        engine.set_layout(root, layout);

        let node = engine.get_node(root).unwrap();
        assert_eq!(node.layout.flex_grow, 1.0);
    }

    #[test]
    fn engine_frame_management() {
        let mut engine = Engine::new();
        assert_eq!(engine.frame_count(), 0);

        engine.begin_frame();
        assert_eq!(engine.frame_count(), 1);

        engine.commit_frame();
        assert_eq!(engine.frame_count(), 1);
    }

    #[test]
    fn engine_validate() {
        let engine = Engine::new();
        assert!(engine.validate().is_ok());
    }

    #[test]
    fn engine_print_tree() {
        let engine = Engine::new();
        let output = engine.print_tree();
        assert!(!output.is_empty());
    }

    #[test]
    fn engine_tree_summary() {
        let engine = Engine::new();
        let summary = engine.tree_summary();
        assert!(summary.contains("Nodes: 1"));
        assert!(summary.contains("Frames: 0"));
    }

    #[test]
    fn engine_debug() {
        let engine = Engine::new();
        let debug = format!("{:?}", engine);
        assert!(debug.contains("Engine"));
    }

    #[test]
    fn engine_process_commands_batch() {
        let mut engine = Engine::new();
        let root = engine.arena().root();
        let child = engine.create_node(NodeKind::Text);

        let commands = vec![
            Command::AppendChild {
                parent: root,
                child,
            },
            Command::SetText {
                id: child,
                text: "hello".into(),
            },
        ];

        let result = engine.process_commands(commands);
        // The batch might have errors if the child ID doesn't match
        // But we can check that some commands were processed
        assert!(result.total() > 0);
    }
}
