//! Tests for the engine module (Engine + Inspector).

use std::collections::HashMap;

use bettertui_engine::engine::Engine;
use bettertui_engine::engine::{Inspector, TreeSummary};
use bettertui_engine::protocol::Command;
use bettertui_engine::tree::{NodeArena, NodeId, NodeKind, RenderNode};

// ---------------------------------------------------------------------------
// Engine tests
// ---------------------------------------------------------------------------

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
    assert_eq!(engine.node_count(), 2);
}

#[test]
fn engine_append_child() {
    let mut engine = Engine::new();
    let root = engine.arena().root();
    let child = engine.create_node(NodeKind::Text);

    let result = engine.processor_mut().process_single(Command::AppendChild { parent: root, child });
    assert!(result.is_ok());
    assert_eq!(engine.node_count(), 2);
}

#[test]
fn engine_remove_node() {
    let mut engine = Engine::new();
    let root = engine.arena().root();
    let child = engine.create_node(NodeKind::Text);
    engine.processor_mut().process_single(Command::AppendChild { parent: root, child }).unwrap();

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
    use bettertui_engine::tree::Style;
    let mut engine = Engine::new();
    let root = engine.arena().root();

    let style = Style { bold: Some(true), ..Default::default() };
    engine.set_style(root, style);

    let node = engine.get_node(root).unwrap();
    assert_eq!(node.style.bold, Some(true));
}

#[test]
fn engine_set_layout() {
    use bettertui_engine::taffy::types::LayoutProps;
    let mut engine = Engine::new();
    let root = engine.arena().root();

    let layout = LayoutProps { flex_grow: 1.0, ..Default::default() };
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

    let commands =
        vec![Command::AppendChild { parent: root, child }, Command::SetText { id: child, text: "hello".into() }];

    let result = engine.process_commands(commands);
    assert!(result.total() > 0);
}

// ---------------------------------------------------------------------------
// Inspector tests
// ---------------------------------------------------------------------------

#[test]
fn inspector_new() {
    let inspector = Inspector::new();
    assert!(inspector.command_log().is_empty());
    assert!(inspector.mutation_log().is_empty());
}

#[test]
fn inspector_log_command() {
    let mut inspector = Inspector::new();
    let command = Command::CreateNode { id: NodeId::default(), kind: NodeKind::Box };
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
    inspector.log_command(Command::SetText { id, text: "hello".into() }, 1);
    inspector.log_command(Command::Shutdown, 2);

    let commands = inspector.commands_for_node(id);
    assert_eq!(commands.len(), 1);
}

#[test]
fn tree_summary_display() {
    let mut kind_counts = HashMap::new();
    kind_counts.insert("Box".to_string(), 5);
    kind_counts.insert("Text".to_string(), 3);

    let summary = TreeSummary { total_nodes: 8, max_depth: 3, kind_counts };

    let display = format!("{}", summary);
    assert!(display.contains("Total nodes: 8"));
    assert!(display.contains("Max depth: 3"));
}
