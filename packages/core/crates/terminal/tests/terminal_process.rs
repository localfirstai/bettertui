use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

use bettertui_engine::pty::PtySize;
use bettertui_terminal::process::{
    ProcessConfig, ProcessConfigBuilder, ProcessSpawner, ProcessStatus, ScrollMode, SpawnResult,
    TerminalError, TerminalRuntime, TerminalState, TerminalViewport,
};

mod config {
    use super::*;

    #[test]
    fn config_new() {
        let config = ProcessConfig::new("bash");
        assert_eq!(config.program, "bash");
        assert!(config.is_valid());
    }

    #[test]
    fn config_default() {
        let config = ProcessConfig::default();
        assert!(!config.is_valid());
        assert_eq!(config.args.len(), 0);
    }

    #[test]
    fn config_builder() {
        let config = ProcessConfig::new("nvim")
            .with_args(vec!["--clean".to_string()])
            .with_env(vec![("TERM".to_string(), "xterm-256color".to_string())])
            .with_working_directory(PathBuf::from("/tmp"))
            .with_auto_restart(true)
            .with_restart_delay(Duration::from_secs(1));

        assert_eq!(config.program, "nvim");
        assert_eq!(config.args.len(), 1);
        assert!(!config.env.is_empty());
        assert!(config.auto_restart);
    }

    #[test]
    fn config_empty_program_invalid() {
        let config = ProcessConfig::default();
        assert!(!config.is_valid());
    }
}

mod state {
    use super::*;

    #[test]
    fn state_new() {
        let state = TerminalState::new();
        assert!(!state.is_running());
        assert_eq!(state.status(), ProcessStatus::Stopped);
        assert!(state.pid().is_none());
    }

    #[test]
    fn state_mark_started() {
        let mut state = TerminalState::new();
        state.mark_started(42);
        assert!(state.is_running());
        assert_eq!(state.pid(), Some(42));
        assert!(state.started_at().is_some());
    }

    #[test]
    fn state_mark_exited() {
        let mut state = TerminalState::new();
        state.mark_started(42);
        state.mark_exited(0);
        assert!(!state.is_running());
        assert_eq!(state.exit_code(), Some(0));
        assert!(state.status().is_exited());
    }

    #[test]
    fn state_restart_count() {
        let mut state = TerminalState::new();
        assert_eq!(state.restart_count(), 0);
        state.mark_restart();
        assert_eq!(state.restart_count(), 1);
    }

    #[test]
    fn process_status_running() {
        assert!(ProcessStatus::Running.is_running());
        assert!(!ProcessStatus::Stopped.is_running());
    }

    #[test]
    fn process_status_exit_code() {
        assert_eq!(ProcessStatus::Exited(0).exit_code(), Some(0));
        assert_eq!(ProcessStatus::Signaled(9).exit_code(), Some(9));
        assert_eq!(ProcessStatus::Running.exit_code(), None);
    }
}

mod spawner {
    use super::*;

    #[test]
    fn spawner_new() {
        let spawner = ProcessSpawner::new();
        assert!(!spawner.default_shell().is_empty());
        assert!(spawner.spawn_history().is_empty());
    }

    #[test]
    fn spawner_default() {
        let spawner = ProcessSpawner::default();
        assert!(!spawner.default_shell().is_empty());
    }

    #[test]
    fn spawner_build_config() {
        let spawner = ProcessSpawner::new();
        let config = spawner.build_config("bash");
        assert!(config.program.ends_with("bash"));
    }

    #[test]
    fn spawner_build_config_with_args() {
        let spawner = ProcessSpawner::new();
        let config = spawner.build_config_with_args("bash", vec!["--norc".to_string()]);
        assert!(config.program.ends_with("bash"));
        assert_eq!(config.args.len(), 1);
    }

    #[test]
    fn spawner_record_history() {
        let mut spawner = ProcessSpawner::new();
        let config = spawner.build_config("echo");
        let result = SpawnResult::new(config, 42);
        spawner.record_spawn(result);
        assert_eq!(spawner.spawn_history().len(), 1);
    }

    #[test]
    fn spawner_builder() {
        let builder = ProcessConfigBuilder::new("nvim")
            .with_args(vec!["file.txt".to_string()])
            .with_size(PtySize::new(120, 40));

        let spawner = ProcessSpawner::new();
        let config = spawner.build_config_from_parts(builder);
        assert!(config.program.ends_with("nvim"));
        assert_eq!(config.args.len(), 1);
        assert_eq!(config.size.cols, 120);
    }

    #[test]
    fn builder_env_map() {
        let mut map = HashMap::new();
        map.insert("TERM".to_string(), "xterm-256color".to_string());
        map.insert("COLORTERM".to_string(), "truecolor".to_string());

        let builder = ProcessConfigBuilder::new("test").with_env_map(map);
        assert_eq!(builder.env.len(), 2);
    }
}

mod runtime {
    use super::*;

    #[test]
    fn runtime_new() {
        let runtime = TerminalRuntime::new();
        assert!(!runtime.is_running());
    }

    #[test]
    fn runtime_default() {
        let runtime = TerminalRuntime::default();
        assert!(!runtime.is_running());
    }

    #[test]
    fn runtime_with_config() {
        let config = ProcessConfig::new("bash");
        let runtime = TerminalRuntime::with_config(config);
        assert!(!runtime.is_running());
        assert_eq!(runtime.config().program, "bash");
    }

    #[test]
    fn runtime_spawn_fails_for_invalid_config() {
        let mut runtime = TerminalRuntime::new();
        let result = runtime.spawn();
        assert!(result.is_err());
        match result {
            Err(TerminalError::InvalidConfig(_)) => {}
            _ => panic!("Expected InvalidConfig error"),
        }
    }

    #[test]
    fn runtime_shutdown_idempotent() {
        let mut runtime = TerminalRuntime::new();
        assert!(!runtime.shutdown_requested());
        let _ = runtime.shutdown();
        assert!(runtime.shutdown_requested());
        let _ = runtime.shutdown();
    }

    #[test]
    fn terminal_error_display() {
        let err = TerminalError::NotRunning;
        assert!(err.to_string().contains("not running"));

        let err = TerminalError::InvalidConfig("test".to_string());
        assert!(err.to_string().contains("Invalid"));
    }

    #[test]
    fn runtime_try_restart_without_auto() {
        let mut runtime = TerminalRuntime::new();
        let result = runtime.try_restart();
        assert!(result.is_err());
    }

    #[test]
    fn runtime_size() {
        let runtime = TerminalRuntime::new();
        let size = runtime.size();
        assert_eq!(size.cols, 80);
        assert_eq!(size.rows, 24);
    }

    #[test]
    fn runtime_resize() {
        let mut runtime = TerminalRuntime::new();
        let size = PtySize::new(120, 40);
        let result = runtime.resize(size);
        assert!(result.is_ok());
        assert_eq!(runtime.size().cols, 120);
    }
}

mod viewport {
    use super::*;

    #[test]
    fn viewport_new() {
        let vp = TerminalViewport::new();
        assert_eq!(vp.cols(), 80);
        assert_eq!(vp.rows(), 24);
        assert!(!vp.is_scrolled());
    }

    #[test]
    fn viewport_default() {
        let vp = TerminalViewport::default();
        assert_eq!(vp.cols(), 80);
    }

    #[test]
    fn viewport_with_size() {
        let vp = TerminalViewport::with_size(120, 40);
        assert_eq!(vp.cols(), 120);
        assert_eq!(vp.rows(), 40);
    }

    #[test]
    fn viewport_resize() {
        let mut vp = TerminalViewport::new();
        vp.resize(100, 30);
        assert_eq!(vp.cols(), 100);
        assert_eq!(vp.rows(), 30);
    }

    #[test]
    fn viewport_scroll_up_down() {
        let mut vp = TerminalViewport::new();
        vp.scroll_up(5);
        assert_eq!(vp.scroll_offset(), 5);
        assert!(vp.is_scrolled());

        vp.scroll_down(2);
        assert_eq!(vp.scroll_offset(), 3);
    }

    #[test]
    fn viewport_scroll_reset() {
        let mut vp = TerminalViewport::new();
        vp.scroll_up(10);
        vp.scroll_reset();
        assert_eq!(vp.scroll_offset(), 0);
        assert!(!vp.is_scrolled());
    }

    #[test]
    fn viewport_scroll_to_top_bottom() {
        let mut vp = TerminalViewport::new();
        vp.scroll_to_top();
        assert_eq!(vp.scroll_offset(), vp.scrollback_lines());

        vp.scroll_to_bottom();
        assert_eq!(vp.scroll_offset(), 0);
    }

    #[test]
    fn viewport_fixed_mode() {
        let mut vp = TerminalViewport::new();
        vp.set_scroll_mode(ScrollMode::Fixed);
        vp.scroll_up(5);
        assert_eq!(vp.scroll_offset(), 0);
    }

    #[test]
    fn viewport_to_pty_size() {
        let mut vp = TerminalViewport::new();
        vp.resize_with_pixels(120, 40, 960, 640);
        let size = vp.to_pty_size();
        assert_eq!(size.cols, 120);
        assert_eq!(size.rows, 40);
        assert_eq!(size.pixel_width, 960);
        assert_eq!(size.pixel_height, 640);
    }

    #[test]
    fn viewport_total_cells() {
        let vp = TerminalViewport::with_size(80, 25);
        assert_eq!(vp.total_cells(), 2000);
    }

    #[test]
    fn viewport_scroll_mode() {
        let vp = TerminalViewport::new();
        assert_eq!(vp.scroll_mode(), ScrollMode::Scrollable);
    }

    #[test]
    fn viewport_visible_line_count() {
        let vp = TerminalViewport::with_size(80, 30);
        assert_eq!(vp.visible_line_count(), 30);
    }
}
