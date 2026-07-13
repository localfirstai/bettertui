//! High-level engine API integrating the renderer, event system, and runtime.
//!
//! The [`Engine`] is the main entry point for framework users (React via FFI).
//! It wraps a [`CommandProcessor`] and provides tree management, frame tracking,
//! and debug utilities.
//!
//! The [`Inspector`] provides developer tools for debugging the UI tree:
//! command logging, mutation tracking, and tree visualization.

use std::collections::HashMap;

use crate::protocol::{Command, CommandProcessor, CommandResult};
use crate::tree::{NodeArena, NodeId, RenderNode};

// ---------------------------------------------------------------------------
// Engine
// ---------------------------------------------------------------------------

/// The main engine that receives commands and maintains the UI tree.
///
/// This is the high-level API that framework adapters (React via FFI) call into.
/// It wraps the [`CommandProcessor`] and provides additional functionality
/// like frame management and debug output.
///
/// # Example
///
/// ```no_run
/// use bettertui_engine::engine::Engine;
/// use bettertui_engine::tree::NodeKind;
///
/// let mut engine = Engine::new();
/// let child = engine.create_node(NodeKind::Text);
/// engine.set_text(child, "Hello");
/// ```
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
    /// Creates a new engine with a fresh tree containing only a root node.
    pub fn new() -> Self {
        Self {
            processor: CommandProcessor::new(),
            frame_count: 0,
        }
    }

    /// Returns a reference to the command processor.
    pub fn processor(&self) -> &CommandProcessor {
        &self.processor
    }

    /// Returns a mutable reference to the command processor.
    pub fn processor_mut(&mut self) -> &mut CommandProcessor {
        &mut self.processor
    }

    /// Processes a batch of commands.
    pub fn process_commands(&mut self, commands: Vec<Command>) -> CommandResult {
        self.processor.process_batch(commands)
    }

    /// Processes a single command.
    pub fn process_command(
        &mut self,
        command: Command,
    ) -> Result<(), crate::protocol::CommandError> {
        self.processor.process_single(command)
    }

    /// Returns the number of nodes in the tree.
    pub fn node_count(&self) -> usize {
        self.processor.node_count()
    }

    /// Validates the tree invariants.
    pub fn validate(&self) -> Result<(), crate::protocol::CommandError> {
        self.processor.validate()
    }

    /// Prints the tree for debugging.
    pub fn print_tree(&self) -> String {
        self.processor.print_tree()
    }

    /// Returns the current frame count.
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    /// Begins a new frame, incrementing the frame counter.
    pub fn begin_frame(&mut self) {
        self.frame_count += 1;
        let _ = self.processor.process_single(Command::BeginFrame {
            frame_id: self.frame_count,
        });
    }

    /// Commits the current frame.
    pub fn commit_frame(&mut self) {
        let _ = self.processor.process_single(Command::CommitFrame {
            frame_id: self.frame_count,
        });
    }

    /// Returns a reference to the node arena.
    pub fn arena(&self) -> &NodeArena {
        self.processor.arena()
    }

    /// Returns a mutable reference to the node arena.
    pub fn arena_mut(&mut self) -> &mut NodeArena {
        self.processor.arena_mut()
    }

    /// Returns a node by ID.
    pub fn get_node(&self, id: NodeId) -> Option<&RenderNode> {
        self.processor.get_node(id)
    }

    /// Creates a new node and returns its ID.
    pub fn create_node(&mut self, kind: crate::tree::NodeKind) -> NodeId {
        let node = RenderNode::new(kind);
        self.arena_mut().insert(node)
    }

    /// Appends a child to a parent node.
    pub fn append_child(
        &mut self,
        parent: NodeId,
        child: NodeId,
    ) -> Result<(), crate::protocol::CommandError> {
        self.processor
            .process_single(Command::AppendChild { parent, child })
    }

    /// Removes a node and its descendants.
    pub fn remove_node(&mut self, id: NodeId) {
        let _ = self.processor.process_single(Command::RemoveNode { id });
    }

    /// Sets text content on a node.
    pub fn set_text(&mut self, id: NodeId, text: impl Into<String>) {
        let _ = self.processor.process_single(Command::SetText {
            id,
            text: text.into(),
        });
    }

    /// Sets the style on a node.
    pub fn set_style(&mut self, id: NodeId, style: crate::tree::Style) {
        let _ = self
            .processor
            .process_single(Command::SetStyle { id, style });
    }

    /// Sets the layout on a node.
    pub fn set_layout(&mut self, id: NodeId, layout: crate::layout::LayoutProps) {
        let _ = self
            .processor
            .process_single(Command::SetLayout { id, layout });
    }

    /// Returns a formatted summary of the tree.
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

// ---------------------------------------------------------------------------
// Inspector
// ---------------------------------------------------------------------------

/// Developer inspector for debugging the UI tree.
///
/// Provides tools for inspecting the tree, logging commands,
/// and tracking mutations over time.
pub struct Inspector {
    command_log: Vec<CommandEntry>,
    mutation_log: Vec<MutationEntry>,
}

/// A logged command with timestamp.
pub struct CommandEntry {
    command: Command,
    timestamp: u64,
}

impl CommandEntry {
    /// Returns the command.
    pub fn command(&self) -> &Command {
        &self.command
    }

    /// Returns the timestamp.
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

/// A logged mutation with timestamp.
pub struct MutationEntry {
    node_id: NodeId,
    mutation_type: MutationType,
    timestamp: u64,
}

impl MutationEntry {
    /// Returns the node ID.
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Returns the mutation type.
    pub fn mutation_type(&self) -> &MutationType {
        &self.mutation_type
    }

    /// Returns the timestamp.
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

/// Type of mutation that occurred on a node.
pub enum MutationType {
    Created,
    Removed,
    Modified,
}

impl Default for Inspector {
    fn default() -> Self {
        Self::new()
    }
}

impl Inspector {
    /// Creates a new inspector with empty logs.
    pub fn new() -> Self {
        Self {
            command_log: Vec::new(),
            mutation_log: Vec::new(),
        }
    }

    /// Logs a command with a timestamp.
    pub fn log_command(&mut self, command: Command, timestamp: u64) {
        self.command_log.push(CommandEntry { command, timestamp });
    }

    /// Logs a mutation with a timestamp.
    pub fn log_mutation(&mut self, node_id: NodeId, mutation_type: MutationType, timestamp: u64) {
        self.mutation_log.push(MutationEntry {
            node_id,
            mutation_type,
            timestamp,
        });
    }

    /// Returns the command log.
    pub fn command_log(&self) -> &[CommandEntry] {
        &self.command_log
    }

    /// Returns the mutation log.
    pub fn mutation_log(&self) -> &[MutationEntry] {
        &self.mutation_log
    }

    /// Returns a command from the log by index.
    pub fn command(&self, index: usize) -> Option<&Command> {
        self.command_log.get(index).map(|e| &e.command)
    }

    /// Returns a mutation's node ID by index.
    pub fn mutation_node_id(&self, index: usize) -> Option<NodeId> {
        self.mutation_log.get(index).map(|e| e.node_id)
    }

    /// Clears all logs.
    pub fn clear(&mut self) {
        self.command_log.clear();
        self.mutation_log.clear();
    }

    /// Prints the tree with detailed node information.
    pub fn print_tree_detail(&self, arena: &NodeArena) -> String {
        let mut output = String::new();
        self.print_node_detail(arena, arena.root(), &mut output, "", true);
        output
    }

    fn print_node_detail(
        &self,
        arena: &NodeArena,
        id: NodeId,
        output: &mut String,
        prefix: &str,
        is_last: bool,
    ) {
        if let Some(node) = arena.get(id) {
            let connector = if is_last { "└── " } else { "├── " };
            let kind_name = node.kind.name();
            let text_preview = node
                .text
                .as_ref()
                .map(|t| format!(" \"{}\"", t))
                .unwrap_or_default();

            output.push_str(&format!(
                "{}{}{}{}\n",
                prefix, connector, kind_name, text_preview
            ));

            let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            let child_count = node.children.len();
            for (i, &child) in node.children.iter().enumerate() {
                self.print_node_detail(arena, child, output, &child_prefix, i == child_count - 1);
            }
        }
    }

    /// Returns a summary of the tree structure.
    pub fn tree_summary(&self, arena: &NodeArena) -> TreeSummary {
        let mut kind_counts: HashMap<String, usize> = HashMap::new();
        let mut total_nodes = 0;
        let mut max_depth = 0;

        for (_, node) in arena.iter() {
            total_nodes += 1;
            *kind_counts.entry(node.kind.name().to_string()).or_insert(0) += 1;
        }

        for (id, _) in arena.iter() {
            let depth = arena.depth(id);
            if depth > max_depth {
                max_depth = depth;
            }
        }

        TreeSummary {
            total_nodes,
            max_depth,
            kind_counts,
        }
    }

    /// Returns the last N commands from the log.
    pub fn recent_commands(&self, n: usize) -> Vec<&Command> {
        self.command_log
            .iter()
            .rev()
            .take(n)
            .map(|entry| &entry.command)
            .collect()
    }

    /// Returns all commands targeting a specific node.
    pub fn commands_for_node(&self, node_id: NodeId) -> Vec<&Command> {
        self.command_log
            .iter()
            .filter(|entry| entry.command.target() == Some(node_id))
            .map(|entry| &entry.command)
            .collect()
    }
}

/// Summary of the tree structure.
#[derive(Debug, Clone)]
pub struct TreeSummary {
    /// Total number of nodes.
    pub total_nodes: usize,
    /// Maximum depth of the tree.
    pub max_depth: u32,
    /// Count of nodes by kind name.
    pub kind_counts: HashMap<String, usize>,
}

impl std::fmt::Display for TreeSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tree Summary:")?;
        writeln!(f, "  Total nodes: {}", self.total_nodes)?;
        writeln!(f, "  Max depth: {}", self.max_depth)?;
        writeln!(f, "  Node kinds:")?;
        for (kind, count) in &self.kind_counts {
            writeln!(f, "    {}: {}", kind, count)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for Inspector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Inspector")
            .field("command_log_len", &self.command_log.len())
            .field("mutation_log_len", &self.mutation_log.len())
            .finish()
    }
}
