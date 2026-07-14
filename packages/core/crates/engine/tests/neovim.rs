//! Tests for the neovim module (NeovimConfig, NeovimState, NeovimProcess).

use std::path::PathBuf;

use bettertui_engine::terminal::neovim::{NeovimConfig, NeovimMode, NeovimProcess, NeovimState};

// ---------------------------------------------------------------------------
// NeovimConfig tests
// ---------------------------------------------------------------------------

#[test]
fn config_new() {
    let config = NeovimConfig::new();
    assert!(config.preserve_user_config);
    assert!(config.config_dir.ends_with("nvim"));
}

#[test]
fn config_to_process_config() {
    let config = NeovimConfig::new().with_preserve_user_config(false);

    let process_config = config.to_process_config();
    assert!(process_config.program.contains("nvim"));
    assert!(process_config.args.contains(&"--clean".to_string()));
}

#[test]
fn config_builder() {
    let config = NeovimConfig::new()
        .with_config_dir(PathBuf::from("/tmp/test-config"))
        .with_data_dir(PathBuf::from("/tmp/test-data"))
        .with_cache_dir(PathBuf::from("/tmp/test-cache"))
        .with_preserve_user_config(false);

    assert_eq!(config.config_dir, PathBuf::from("/tmp/test-config"));
    assert!(!config.preserve_user_config);
}

// ---------------------------------------------------------------------------
// NeovimState tests
// ---------------------------------------------------------------------------

#[test]
fn state_new() {
    let state = NeovimState::new();
    assert!(!state.is_running());
    assert_eq!(state.mode(), NeovimMode::Normal);
}

#[test]
fn state_base_delegation() {
    let mut state = NeovimState::new();
    state.base_state_mut().mark_started(42);
    assert!(state.is_running());
}

#[test]
fn state_mode() {
    let mut state = NeovimState::new();
    state.set_mode(NeovimMode::Insert);
    assert_eq!(state.mode(), NeovimMode::Insert);
    assert_eq!(state.mode_name(), "INSERT");
}

#[test]
fn state_filename() {
    let mut state = NeovimState::new();
    state.set_filename(Some("test.rs".to_string()));
    assert_eq!(state.filename(), Some("test.rs"));
}

#[test]
fn state_modified() {
    let mut state = NeovimState::new();
    state.set_modified(true);
    assert!(state.is_modified());
}

#[test]
fn state_cursor() {
    let mut state = NeovimState::new();
    state.set_cursor_position(10, 5);
    assert_eq!(state.cursor_position(), (10, 5));
}

#[test]
fn state_status_line() {
    let mut state = NeovimState::new();
    state.set_filename(Some("test.rs".to_string()));
    state.set_modified(true);
    state.set_cursor_position(10, 5);

    let status = state.status_line();
    assert!(status.contains("NORMAL"));
    assert!(status.contains("test.rs"));
    assert!(status.contains("[+]"));
    assert!(status.contains("10:5"));
}

#[test]
fn state_mode_names() {
    assert_eq!(NeovimMode::Normal.mode_name(), "NORMAL");
    assert_eq!(NeovimMode::Insert.mode_name(), "INSERT");
    assert_eq!(NeovimMode::Visual.mode_name(), "VISUAL");
    assert_eq!(NeovimMode::Command.mode_name(), "COMMAND");
    assert_eq!(NeovimMode::Replace.mode_name(), "REPLACE");
    assert_eq!(NeovimMode::Terminal.mode_name(), "TERMINAL");
}

#[test]
fn state_running_delegation() {
    let mut state = NeovimState::new();
    state.set_running(true);
    assert!(state.is_running());
    state.set_running(false);
    assert!(!state.is_running());
}

// ---------------------------------------------------------------------------
// NeovimProcess tests
// ---------------------------------------------------------------------------

#[test]
fn process_new() {
    let process = NeovimProcess::new();
    assert!(!process.is_running());
}

#[test]
fn process_with_config() {
    let config = NeovimConfig::new().with_preserve_user_config(false);
    let process = NeovimProcess::with_config(config);
    assert!(!process.is_running());
    assert!(!process.config().preserve_user_config);
}

#[test]
fn process_not_running_errors() {
    let mut process = NeovimProcess::new();
    assert!(process.write_input(b"test").is_err());
    assert!(process.read_output().is_err());
    assert!(process.kill().is_err());
}

#[test]
fn process_delegates_to_runtime() {
    let process = NeovimProcess::new();
    assert!(!process.runtime().is_running());
    assert_eq!(process.runtime().config().program, "");
}
