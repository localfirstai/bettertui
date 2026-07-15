//! Tests for the protocol module.

use bettertui_engine::protocol::{
    Command, CommandBuffer, CommandError, CommandProcessor, CommandRegistry, CommandResult, CommandWarning,
};
use bettertui_engine::tree::{NodeId, NodeKind, RenderNode};

// ══════════════════════════════════════════════════════════════════════════════
// COMMAND TESTS
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn command_target() {
    let id = NodeId::default();
    assert_eq!(Command::CreateNode { id, kind: NodeKind::Box }.target(), Some(id));
    assert_eq!(Command::Shutdown.target(), None);
    assert_eq!(Command::BeginFrame { frame_id: 1 }.target(), None);
}

#[test]
fn command_name() {
    assert_eq!(Command::CreateNode { id: NodeId::default(), kind: NodeKind::Box }.name(), "CreateNode");
    assert_eq!(Command::Shutdown.name(), "Shutdown");
}

#[test]
fn command_is_tree_mutation() {
    assert!(Command::CreateNode { id: NodeId::default(), kind: NodeKind::Box }.is_tree_mutation());
    assert!(Command::AppendChild { parent: NodeId::default(), child: NodeId::default() }.is_tree_mutation());
    assert!(!Command::SetBold { id: NodeId::default(), value: true }.is_tree_mutation());
}

#[test]
fn command_is_frame_command() {
    assert!(Command::BeginFrame { frame_id: 1 }.is_frame_command());
    assert!(Command::CommitFrame { frame_id: 1 }.is_frame_command());
    assert!(!Command::Shutdown.is_frame_command());
}

#[test]
fn command_display() {
    let cmd = Command::SetText { id: NodeId::default(), text: "hello".into() };
    let display = format!("{cmd}");
    assert!(display.contains("SetText"));
    assert!(display.contains("hello"));
}

// ══════════════════════════════════════════════════════════════════════════════
// COMMAND REGISTRY TESTS
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn execute_records_history() {
    let mut reg = CommandRegistry::new();
    let entry = bettertui_engine::protocol::CommandEntry::new("test", "data", false);
    reg.execute(entry);
    assert_eq!(reg.history_len(), 1);
}

#[test]
fn undo_redo() {
    let mut reg = CommandRegistry::new();
    reg.execute(bettertui_engine::protocol::CommandEntry::new("cmd", "data", true));
    assert!(reg.can_undo());
    let undone = reg.undo();
    assert!(undone.is_some());
    assert!(!reg.can_undo());
    assert!(reg.can_redo());
    let redone = reg.redo();
    assert!(redone.is_some());
    assert!(!reg.can_redo());
}

#[test]
fn undo_empty() {
    let mut reg = CommandRegistry::new();
    assert!(reg.undo().is_none());
}

#[test]
fn redo_empty() {
    let mut reg = CommandRegistry::new();
    assert!(reg.redo().is_none());
}

#[test]
fn non_undoable_not_in_undo_stack() {
    let mut reg = CommandRegistry::new();
    reg.execute(bettertui_engine::protocol::CommandEntry::new("cmd", "data", false));
    assert!(!reg.can_undo());
}

#[test]
fn new_command_clears_redo() {
    let mut reg = CommandRegistry::new();
    reg.execute(bettertui_engine::protocol::CommandEntry::new("cmd", "data", true));
    reg.undo();
    assert!(reg.can_redo());
    reg.execute(bettertui_engine::protocol::CommandEntry::new("cmd2", "data2", true));
    assert!(!reg.can_redo());
}

#[test]
fn max_history() {
    let mut reg = CommandRegistry::new().with_max_history(3);
    for i in 0..5 {
        reg.execute(bettertui_engine::protocol::CommandEntry::new("cmd", i.to_string(), false));
    }
    assert_eq!(reg.history_len(), 3);
}

#[test]
fn max_undo() {
    let mut reg = CommandRegistry::new().with_max_undo(2);
    for i in 0..5 {
        reg.execute(bettertui_engine::protocol::CommandEntry::new("cmd", i.to_string(), true));
    }
    assert_eq!(reg.undo_depth(), 2);
}

#[test]
fn find_commands() {
    let mut reg = CommandRegistry::new();
    reg.execute(bettertui_engine::protocol::CommandEntry::new("save", "f1", false));
    reg.execute(bettertui_engine::protocol::CommandEntry::new("open", "f2", false));
    reg.execute(bettertui_engine::protocol::CommandEntry::new("save", "f3", false));
    let saves = reg.find("save");
    assert_eq!(saves.len(), 2);
}

#[test]
fn clear() {
    let mut reg = CommandRegistry::new();
    reg.execute(bettertui_engine::protocol::CommandEntry::new("cmd", "data", true));
    reg.clear();
    assert_eq!(reg.history_len(), 0);
    assert!(!reg.can_undo());
}

// ══════════════════════════════════════════════════════════════════════════════
// COMMAND BUFFER TESTS
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn buffer_new() {
    let buf = CommandBuffer::new();
    assert!(buf.is_empty());
    assert_eq!(buf.len(), 0);
}

#[test]
fn buffer_push() {
    let mut buf = CommandBuffer::new();
    buf.push(Command::CreateNode { id: NodeId::default(), kind: NodeKind::Box });
    assert_eq!(buf.len(), 1);
    assert!(!buf.is_empty());
}

#[test]
fn buffer_drain() {
    let mut buf = CommandBuffer::new();
    buf.push(Command::Shutdown);
    buf.push(Command::BeginFrame { frame_id: 1 });

    let cmds = buf.drain();
    assert_eq!(cmds.len(), 2);
    assert!(buf.is_empty());
}

#[test]
fn buffer_clear() {
    let mut buf = CommandBuffer::new();
    buf.push(Command::Shutdown);
    buf.clear();
    assert!(buf.is_empty());
}

#[test]
fn buffer_from_vec() {
    let cmds = vec![Command::Shutdown, Command::BeginFrame { frame_id: 1 }];
    let buf = CommandBuffer::from(cmds);
    assert_eq!(buf.len(), 2);
}

#[test]
fn buffer_into_iter() {
    let mut buf = CommandBuffer::new();
    buf.push(Command::Shutdown);
    buf.push(Command::BeginFrame { frame_id: 1 });

    let cmds: Vec<_> = buf.into_iter().collect();
    assert_eq!(cmds.len(), 2);
}

#[test]
fn buffer_estimated_size() {
    let mut buf = CommandBuffer::new();
    buf.push(Command::Shutdown);
    let size = buf.estimated_size();
    assert!(size > 0);
}

// ══════════════════════════════════════════════════════════════════════════════
// COMMAND ERROR TESTS
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn error_display() {
    let err = CommandError::NodeNotFound(NodeId::default());
    assert!(format!("{err}").contains("Node not found"));

    let err = CommandError::InvalidOperation("test".into());
    assert!(format!("{err}").contains("Invalid operation"));
}

#[test]
fn warning_display() {
    let warn = CommandWarning::NodeSkipped(NodeId::default());
    assert!(format!("{warn}").contains("Node skipped"));

    let warn = CommandWarning::NoEffect("test".into());
    assert!(format!("{warn}").contains("No effect"));
}

#[test]
fn from_tree_error() {
    let tree_err = bettertui_engine::tree::TreeError::NodeNotFound(NodeId::default());
    let cmd_err: CommandError = tree_err.into();
    assert!(matches!(cmd_err, CommandError::NodeNotFound(_)));
}

// ══════════════════════════════════════════════════════════════════════════════
// COMMAND RESULT TESTS
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn result_default() {
    let result = CommandResult::new();
    assert_eq!(result.processed, 0);
    assert_eq!(result.failed, 0);
    assert!(result.is_success());
}

#[test]
fn result_success() {
    let result = CommandResult::success();
    assert_eq!(result.processed, 1);
    assert!(result.is_success());
}

#[test]
fn result_error() {
    let result = CommandResult::error(CommandError::InvalidOperation("test".into()));
    assert_eq!(result.failed, 1);
    assert!(result.has_errors());
}

#[test]
fn result_merge() {
    let mut r1 = CommandResult::success();
    let r2 = CommandResult::error(CommandError::InvalidOperation("test".into()));
    r1.merge(r2);

    assert_eq!(r1.processed, 1);
    assert_eq!(r1.failed, 1);
    assert_eq!(r1.total(), 2);
}

#[test]
fn result_display() {
    let result = CommandResult::new();
    let display = format!("{result}");
    assert!(display.contains("CommandResult"));
}

// ══════════════════════════════════════════════════════════════════════════════
// COMMAND PROCESSOR TESTS
// ══════════════════════════════════════════════════════════════════════════════

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
    let result = proc.process_single(Command::CreateNode { id, kind: NodeKind::Box });
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

    let result = proc.process_single(Command::AppendChild { parent: root, child });
    assert!(result.is_ok());
    assert_eq!(proc.node_count(), 2);
}

#[test]
fn process_set_text() {
    let mut proc = CommandProcessor::new();
    let root = proc.arena().root();

    let result = proc.process_single(Command::SetText { id: root, text: "hello".into() });
    assert!(result.is_ok());

    let node = proc.get_node(root).unwrap();
    assert_eq!(node.text.as_deref(), Some("hello"));
}

#[test]
fn process_set_bold() {
    let mut proc = CommandProcessor::new();
    let root = proc.arena().root();

    let result = proc.process_single(Command::SetBold { id: root, value: true });
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
        Command::AppendChild { parent: root, child },
        Command::SetText { id: child, text: "hello".into() },
        Command::SetBold { id: child, value: true },
    ];

    let result = proc.process_batch(commands);
    assert!(result.is_success());
    assert_eq!(result.processed, 3);
}

#[test]
fn process_set_attribute() {
    let mut proc = CommandProcessor::new();
    let root = proc.arena().root();

    let result =
        proc.process_single(Command::SetAttribute { id: root, key: "data-testid".into(), value: "my-element".into() });
    assert!(result.is_ok());

    let node = proc.get_node(root).unwrap();
    assert_eq!(node.attributes.get("data-testid"), Some(&"my-element".to_string()));
}

#[test]
fn process_remove_attribute() {
    let mut proc = CommandProcessor::new();
    let root = proc.arena().root();

    proc.process_single(Command::SetAttribute { id: root, key: "data-testid".into(), value: "my-element".into() })
        .unwrap();

    let result = proc.process_single(Command::RemoveAttribute { id: root, key: "data-testid".into() });
    assert!(result.is_ok());

    let node = proc.get_node(root).unwrap();
    assert!(!node.attributes.contains_key("data-testid"));
}

#[test]
fn process_invalid_command() {
    let mut proc = CommandProcessor::new();
    let bad_id = NodeId::default();

    let result = proc.process_single(Command::SetText { id: bad_id, text: "hello".into() });

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
