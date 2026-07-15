//! High-level engine API integrating the renderer, event system, and runtime.
//!
//! The [`Engine`] is the main entry point for framework users (React via FFI).
//! It wraps a [`CommandProcessor`] and provides tree management, frame tracking,
//! and debug utilities.
//!
//! The [`Inspector`] provides developer tools for debugging the UI tree:
//! command logging, mutation tracking, and tree visualization.

use std::collections::HashMap;

use tracing::{debug, info, warn};

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
        info!("Engine::new() - creating new engine instance");
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
        debug!(
            command_count = commands.len(),
            "Engine::process_commands() - processing batch"
        );
        self.processor.process_batch(commands)
    }

    /// Processes a single command.
    pub fn process_command(
        &mut self,
        command: Command,
    ) -> Result<(), crate::protocol::CommandError> {
        debug!(
            ?command,
            "Engine::process_command() - processing single command"
        );
        let result = self.processor.process_single(command);
        if let Err(ref e) = result {
            warn!(error = ?e, "Engine::process_command() - command failed");
        }
        result
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
        debug!(
            frame_id = self.frame_count,
            "Engine::begin_frame() - starting frame"
        );
        let _ = self.processor.process_single(Command::BeginFrame {
            frame_id: self.frame_count,
        });
    }

    /// Commits the current frame.
    pub fn commit_frame(&mut self) {
        debug!(
            frame_id = self.frame_count,
            "Engine::commit_frame() - committing frame"
        );
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
        let id = self.arena_mut().insert(node);
        debug!(node_id = ?id, kind = ?kind, "Engine::create_node() - created node");
        id
    }

    /// Appends a child to a parent node.
    pub fn append_child(
        &mut self,
        parent: NodeId,
        child: NodeId,
    ) -> Result<(), crate::protocol::CommandError> {
        debug!(?parent, ?child, "Engine::append_child() - appending child");
        self.processor
            .process_single(Command::AppendChild { parent, child })
    }

    /// Removes a node and its descendants.
    pub fn remove_node(&mut self, id: NodeId) {
        debug!(?id, "Engine::remove_node() - removing node");
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
    pub fn set_layout(&mut self, id: NodeId, layout: crate::taffy::LayoutProps) {
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
#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::{NodeKind, Style};

    #[test]
    fn engine_new_creates_empty_tree() {
        let engine = Engine::new();
        assert_eq!(engine.node_count(), 1);
        assert_eq!(engine.frame_count(), 0);
    }

    #[test]
    fn engine_default_equals_new() {
        let e1 = Engine::new();
        let e2 = Engine::default();
        assert_eq!(e1.node_count(), e2.node_count());
        assert_eq!(e1.frame_count(), e2.frame_count());
    }

    #[test]
    fn engine_create_node() {
        let mut engine = Engine::new();
        let id = engine.create_node(NodeKind::Text);
        assert_eq!(engine.node_count(), 2);
        assert!(engine.get_node(id).is_some());
    }

    #[test]
    fn engine_append_and_remove_child() {
        let mut engine = Engine::new();
        let root = engine.arena().root();
        let child = engine.create_node(NodeKind::Box);
        engine.append_child(root, child).unwrap();
        engine.remove_node(child);
        assert!(engine.get_node(child).is_none());
    }

    #[test]
    fn engine_set_text_and_style() {
        let mut engine = Engine::new();
        let id = engine.create_node(NodeKind::Text);
        engine.set_text(id, "hello");
        engine.set_style(id, Style::default());
        let node = engine.get_node(id).unwrap();
        assert_eq!(node.text.as_deref(), Some("hello"));
    }

    #[test]
    fn engine_begin_and_commit_frame() {
        let mut engine = Engine::new();
        assert_eq!(engine.frame_count(), 0);
        engine.begin_frame();
        assert_eq!(engine.frame_count(), 1);
        engine.commit_frame();
        assert_eq!(engine.frame_count(), 1);
    }

    #[test]
    fn engine_process_command() {
        let mut engine = Engine::new();
        let id = engine.create_node(NodeKind::Text);
        let result = engine.process_command(crate::protocol::Command::SetText {
            id,
            text: "from command".into(),
        });
        assert!(result.is_ok());
        let node = engine.get_node(id).unwrap();
        assert_eq!(node.text.as_deref(), Some("from command"));
    }

    #[test]
    fn engine_process_commands_batch() {
        use crate::protocol::Command;
        let mut engine = Engine::new();
        let id = engine.create_node(NodeKind::Box);
        let cmd1 = Command::SetText {
            id,
            text: "batch1".into(),
        };
        let cmd2 = Command::SetText {
            id,
            text: "batch2".into(),
        };
        let result = engine.process_commands(vec![cmd1, cmd2]);
        assert!(result.is_success());
        let node = engine.get_node(id).unwrap();
        assert_eq!(node.text.as_deref(), Some("batch2"));
    }

    #[test]
    fn engine_validate_ok() {
        let engine = Engine::new();
        assert!(engine.validate().is_ok());
    }

    #[test]
    fn engine_tree_summary() {
        let mut engine = Engine::new();
        let child = engine.create_node(NodeKind::Text);
        engine.append_child(engine.arena().root(), child).unwrap();
        let summary = engine.tree_summary();
        assert!(summary.contains("Nodes: 2"));
        assert!(summary.contains("Frames: 0"));
    }

    #[test]
    fn engine_print_tree() {
        let mut engine = Engine::new();
        let child = engine.create_node(NodeKind::Text);
        engine.set_text(child, "test");
        engine.append_child(engine.arena().root(), child).unwrap();
        let tree = engine.print_tree();
        assert!(!tree.is_empty());
    }

    #[test]
    fn engine_arena_access() {
        let mut engine = Engine::new();
        let arena = engine.arena();
        assert!(arena.get(arena.root()).is_some());
        let id = engine.create_node(NodeKind::Box);
        let arena_mut = engine.arena_mut();
        assert!(arena_mut.get(id).is_some());
    }

    #[test]
    fn engine_debug_fmt() {
        let engine = Engine::new();
        let s = format!("{:?}", engine);
        assert!(s.contains("node_count"));
        assert!(s.contains("frame_count"));
    }

    // ── Inspector tests ──

    #[test]
    fn inspector_new_has_empty_logs() {
        let insp = Inspector::new();
        assert!(insp.command_log().is_empty());
        assert!(insp.mutation_log().is_empty());
    }

    #[test]
    fn inspector_log_and_retrieve_command() {
        let mut insp = Inspector::new();
        let cmd = crate::protocol::Command::BeginFrame { frame_id: 1 };
        insp.log_command(cmd, 42);
        assert_eq!(insp.command_log().len(), 1);
        assert!(insp.command(0).is_some());
    }

    #[test]
    fn inspector_log_mutation() {
        let mut insp = Inspector::new();
        let id = crate::tree::NodeArena::new().root();
        insp.log_mutation(id, MutationType::Created, 1);
        assert_eq!(insp.mutation_log().len(), 1);
        let node_id = insp.mutation_node_id(0);
        assert!(node_id.is_some());
    }

    #[test]
    fn inspector_clear() {
        let mut insp = Inspector::new();
        insp.log_command(crate::protocol::Command::BeginFrame { frame_id: 1 }, 0);
        insp.log_mutation(
            crate::tree::NodeArena::new().root(),
            MutationType::Created,
            0,
        );
        insp.clear();
        assert!(insp.command_log().is_empty());
        assert!(insp.mutation_log().is_empty());
    }

    #[test]
    fn inspector_recent_commands() {
        let mut insp = Inspector::new();
        insp.log_command(crate::protocol::Command::BeginFrame { frame_id: 1 }, 1);
        insp.log_command(crate::protocol::Command::BeginFrame { frame_id: 2 }, 2);
        let recent = insp.recent_commands(1);
        assert_eq!(recent.len(), 1);
    }

    #[test]
    fn inspector_commands_for_node() {
        use crate::protocol::Command;
        let mut insp = Inspector::new();
        let id = crate::tree::NodeArena::new().root();
        insp.log_command(
            Command::SetText {
                id,
                text: "a".into(),
            },
            1,
        );
        insp.log_command(Command::BeginFrame { frame_id: 1 }, 2);
        let cmds = insp.commands_for_node(id);
        assert_eq!(cmds.len(), 1);
    }

    #[test]
    fn inspector_tree_summary() {
        let arena = crate::tree::NodeArena::new();
        let insp = Inspector::new();
        let summary = insp.tree_summary(&arena);
        assert_eq!(summary.total_nodes, 1);
        assert_eq!(summary.max_depth, 0);
    }

    #[test]
    fn inspector_print_tree_detail() {
        let arena = crate::tree::NodeArena::new();
        let insp = Inspector::new();
        let output = insp.print_tree_detail(&arena);
        assert!(output.contains("Box"));
    }

    #[test]
    fn inspector_debug_fmt() {
        let insp = Inspector::new();
        let s = format!("{:?}", insp);
        assert!(s.contains("command_log_len"));
        assert!(s.contains("mutation_log_len"));
    }

    #[test]
    fn tree_summary_display() {
        use std::collections::HashMap;
        let summary = TreeSummary {
            total_nodes: 5,
            max_depth: 3,
            kind_counts: HashMap::from([("Text".into(), 3), ("Box".into(), 2)]),
        };
        let s = summary.to_string();
        assert!(s.contains("Total nodes: 5"));
        assert!(s.contains("Max depth: 3"));
        assert!(s.contains("Text: 3"));
        assert!(s.contains("Box: 2"));
    }

    #[test]
    fn command_entry_accessors() {
        let cmd = crate::protocol::Command::BeginFrame { frame_id: 7 };
        let entry = CommandEntry {
            command: cmd,
            timestamp: 99,
        };
        assert_eq!(entry.timestamp(), 99);
    }

    #[test]
    fn mutation_entry_accessors() {
        let id = crate::tree::NodeArena::new().root();
        let entry = MutationEntry {
            node_id: id,
            mutation_type: MutationType::Modified,
            timestamp: 5,
        };
        assert_eq!(entry.mutation_type(), &MutationType::Modified);
        assert_eq!(entry.timestamp(), 5);
    }
}
