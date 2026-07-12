use std::collections::HashMap;

use crate::protocol::Command;
use crate::tree::{NodeArena, NodeId};

/// Developer inspector for debugging the UI tree.
///
/// Provides tools for inspecting the tree, logging commands,
/// and tracking mutations.
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
    /// Get the command.
    pub fn command(&self) -> &Command {
        &self.command
    }

    /// Get the timestamp.
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
    /// Get the node ID.
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Get the mutation type.
    pub fn mutation_type(&self) -> &MutationType {
        &self.mutation_type
    }

    /// Get the timestamp.
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

/// Type of mutation that occurred.
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
    /// Create a new inspector.
    pub fn new() -> Self {
        Self {
            command_log: Vec::new(),
            mutation_log: Vec::new(),
        }
    }

    /// Log a command.
    pub fn log_command(&mut self, command: Command, timestamp: u64) {
        self.command_log.push(CommandEntry { command, timestamp });
    }

    /// Log a mutation.
    pub fn log_mutation(&mut self, node_id: NodeId, mutation_type: MutationType, timestamp: u64) {
        self.mutation_log.push(MutationEntry {
            node_id,
            mutation_type,
            timestamp,
        });
    }

    /// Get the command log.
    pub fn command_log(&self) -> &[CommandEntry] {
        &self.command_log
    }

    /// Get the mutation log.
    pub fn mutation_log(&self) -> &[MutationEntry] {
        &self.mutation_log
    }

    /// Get a command entry.
    pub fn command(&self, index: usize) -> Option<&Command> {
        self.command_log.get(index).map(|e| &e.command)
    }

    /// Get a mutation entry's node ID.
    pub fn mutation_node_id(&self, index: usize) -> Option<NodeId> {
        self.mutation_log.get(index).map(|e| e.node_id)
    }

    /// Clear the logs.
    pub fn clear(&mut self) {
        self.command_log.clear();
        self.mutation_log.clear();
    }

    /// Print the tree with detailed node information.
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

    /// Get a summary of the tree.
    pub fn tree_summary(&self, arena: &NodeArena) -> TreeSummary {
        let mut kind_counts: HashMap<String, usize> = HashMap::new();
        let mut total_nodes = 0;
        let mut max_depth = 0;

        for (_, node) in arena.iter() {
            total_nodes += 1;
            *kind_counts.entry(node.kind.name().to_string()).or_insert(0) += 1;
        }

        // Calculate max depth
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

    /// Get the last N commands from the log.
    pub fn recent_commands(&self, n: usize) -> Vec<&Command> {
        self.command_log
            .iter()
            .rev()
            .take(n)
            .map(|entry| &entry.command)
            .collect()
    }

    /// Get commands targeting a specific node.
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
    pub total_nodes: usize,
    pub max_depth: u32,
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
    use crate::tree::{NodeKind, RenderNode};

    #[test]
    fn inspector_new() {
        let inspector = Inspector::new();
        assert!(inspector.command_log().is_empty());
        assert!(inspector.mutation_log().is_empty());
    }

    #[test]
    fn inspector_log_command() {
        let mut inspector = Inspector::new();
        let command = Command::CreateNode {
            id: NodeId::default(),
            kind: NodeKind::Box,
        };
        inspector.log_command(command, 1);
        assert_eq!(inspector.command_log().len(), 1);
    }

    #[test]
    fn inspector_clear() {
        let mut inspector = Inspector::new();
        let command = Command::Shutdown;
        inspector.log_command(command, 1);
        inspector.clear();
        assert!(inspector.command_log().is_empty());
    }

    #[test]
    fn inspector_print_tree_detail() {
        let inspector = Inspector::new();
        let mut arena = NodeArena::new();
        let root = arena.root();
        let child = arena.insert(RenderNode::new(NodeKind::Text));
        arena.append_child(root, child).unwrap();

        let output = inspector.print_tree_detail(&arena);
        assert!(output.contains("Box"));
        assert!(output.contains("Text"));
    }

    #[test]
    fn inspector_tree_summary() {
        let inspector = Inspector::new();
        let mut arena = NodeArena::new();
        let root = arena.root();
        let child = arena.insert(RenderNode::new(NodeKind::Text));
        arena.append_child(root, child).unwrap();

        let summary = inspector.tree_summary(&arena);
        assert_eq!(summary.total_nodes, 2);
        assert!(summary.kind_counts.contains_key("Box"));
        assert!(summary.kind_counts.contains_key("Text"));
    }

    #[test]
    fn inspector_recent_commands() {
        let mut inspector = Inspector::new();
        inspector.log_command(Command::Shutdown, 1);
        inspector.log_command(Command::BeginFrame { frame_id: 1 }, 2);

        let recent = inspector.recent_commands(1);
        assert_eq!(recent.len(), 1);
        assert!(matches!(recent[0], Command::BeginFrame { .. }));
    }

    #[test]
    fn inspector_commands_for_node() {
        let mut inspector = Inspector::new();
        let id = NodeId::default();
        inspector.log_command(
            Command::SetText {
                id,
                text: "hello".into(),
            },
            1,
        );
        inspector.log_command(Command::Shutdown, 2);

        let commands = inspector.commands_for_node(id);
        assert_eq!(commands.len(), 1);
    }

    #[test]
    fn tree_summary_display() {
        let mut kind_counts = HashMap::new();
        kind_counts.insert("Box".to_string(), 5);
        kind_counts.insert("Text".to_string(), 3);

        let summary = TreeSummary {
            total_nodes: 8,
            max_depth: 3,
            kind_counts,
        };

        let display = format!("{}", summary);
        assert!(display.contains("Total nodes: 8"));
        assert!(display.contains("Max depth: 3"));
    }
}
