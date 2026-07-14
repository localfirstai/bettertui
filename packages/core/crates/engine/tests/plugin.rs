//! Tests for the plugin module.

use std::collections::HashMap;

use bettertui_engine::plugin::{Capability, PluginHost, PluginInfo, PluginState};

fn test_info(name: &str) -> PluginInfo {
    PluginInfo {
        name: name.to_string(),
        version: "1.0.0".to_string(),
        author: "test".to_string(),
        capabilities: vec![Capability::Commands],
        metadata: HashMap::new(),
    }
}

#[test]
fn register_plugin() {
    let mut host = PluginHost::new();
    assert!(host.register(test_info("p1")).is_ok());
    assert_eq!(host.len(), 1);
}

#[test]
fn register_duplicate() {
    let mut host = PluginHost::new();
    host.register(test_info("p1")).unwrap();
    assert!(host.register(test_info("p1")).is_err());
}

#[test]
fn unregister_plugin() {
    let mut host = PluginHost::new();
    host.register(test_info("p1")).unwrap();
    assert!(host.unregister("p1").is_ok());
    assert!(host.is_empty());
}

#[test]
fn unregister_nonexistent() {
    let mut host = PluginHost::new();
    assert!(host.unregister("p1").is_err());
}

#[test]
fn state_transitions() {
    let mut host = PluginHost::new();
    host.register(test_info("p1")).unwrap();
    assert_eq!(host.state("p1"), Some(PluginState::Registered));
    host.set_state("p1", PluginState::Initialized).unwrap();
    assert_eq!(host.state("p1"), Some(PluginState::Initialized));
    host.set_state("p1", PluginState::Running).unwrap();
    assert_eq!(host.state("p1"), Some(PluginState::Running));
}

#[test]
fn capability_filter() {
    let mut host = PluginHost::new();
    let mut info = test_info("p1");
    info.capabilities.push(Capability::Widgets);
    host.register(info).unwrap();
    host.register(test_info("p2")).unwrap();
    let with_cmds = host.with_capability(&Capability::Commands);
    assert_eq!(with_cmds.len(), 2);
    let with_widgets = host.with_capability(&Capability::Widgets);
    assert_eq!(with_widgets.len(), 1);
    assert_eq!(with_widgets[0].info.name, "p1");
}

#[test]
fn plugin_commands() {
    let mut host = PluginHost::new();
    host.register(test_info("p1")).unwrap();
    host.add_command("p1", "cmd1".into()).unwrap();
    host.add_command("p1", "cmd2".into()).unwrap();
    assert_eq!(host.all_commands().len(), 2);
}

#[test]
fn running_plugins() {
    let mut host = PluginHost::new();
    host.register(test_info("p1")).unwrap();
    host.register(test_info("p2")).unwrap();
    host.set_state("p1", PluginState::Running).unwrap();
    assert_eq!(host.running().len(), 1);
}

#[test]
fn names() {
    let mut host = PluginHost::new();
    host.register(test_info("a")).unwrap();
    host.register(test_info("b")).unwrap();
    let mut names = host.names();
    names.sort();
    assert_eq!(names, vec!["a", "b"]);
}
