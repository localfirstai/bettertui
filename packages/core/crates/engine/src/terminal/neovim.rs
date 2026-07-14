//! Neovim integration: process management, configuration, and state tracking.

use std::path::{Path, PathBuf};

use crate::pty::PtySize;
use crate::terminal::{ProcessConfig, TerminalError, TerminalRuntime, TerminalState};

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug)]
pub enum NeovimError {
    SpawnFailed(String),
    TerminalError(TerminalError),
    NotRunning,
}

impl std::fmt::Display for NeovimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpawnFailed(msg) => write!(f, "Failed to spawn Neovim: {}", msg),
            Self::TerminalError(err) => write!(f, "Terminal error: {}", err),
            Self::NotRunning => write!(f, "Neovim is not running"),
        }
    }
}

impl std::error::Error for NeovimError {}

impl From<TerminalError> for NeovimError {
    fn from(err: TerminalError) -> Self {
        Self::TerminalError(err)
    }
}

// ============================================================================
// NeovimMode
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NeovimMode {
    #[default]
    Normal,
    Insert,
    Visual,
    Command,
    Replace,
    Terminal,
}

impl NeovimMode {
    pub fn mode_name(&self) -> &'static str {
        match self {
            NeovimMode::Normal => "NORMAL",
            NeovimMode::Insert => "INSERT",
            NeovimMode::Visual => "VISUAL",
            NeovimMode::Command => "COMMAND",
            NeovimMode::Replace => "REPLACE",
            NeovimMode::Terminal => "TERMINAL",
        }
    }
}

// ============================================================================
// NeovimState
// ============================================================================

#[derive(Debug, Clone)]
pub struct NeovimState {
    base: TerminalState,
    mode: NeovimMode,
    modified: bool,
    filename: Option<String>,
    line_count: usize,
    cursor_position: (usize, usize),
}

impl Default for NeovimState {
    fn default() -> Self {
        Self::new()
    }
}

impl NeovimState {
    pub fn new() -> Self {
        Self {
            base: TerminalState::new(),
            mode: NeovimMode::Normal,
            modified: false,
            filename: None,
            line_count: 0,
            cursor_position: (1, 1),
        }
    }

    pub fn base_state(&self) -> &TerminalState {
        &self.base
    }

    pub fn base_state_mut(&mut self) -> &mut TerminalState {
        &mut self.base
    }

    pub fn is_running(&self) -> bool {
        self.base.is_running()
    }

    pub fn set_running(&mut self, running: bool) {
        if running {
            self.base.mark_started(0);
        } else {
            self.base.mark_exited(0);
        }
    }

    pub fn mode(&self) -> NeovimMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: NeovimMode) {
        self.mode = mode;
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn set_modified(&mut self, modified: bool) {
        self.modified = modified;
    }

    pub fn filename(&self) -> Option<&str> {
        self.filename.as_deref()
    }

    pub fn set_filename(&mut self, filename: Option<String>) {
        self.filename = filename;
    }

    pub fn line_count(&self) -> usize {
        self.line_count
    }

    pub fn set_line_count(&mut self, count: usize) {
        self.line_count = count;
    }

    pub fn cursor_position(&self) -> (usize, usize) {
        self.cursor_position
    }

    pub fn set_cursor_position(&mut self, row: usize, col: usize) {
        self.cursor_position = (row, col);
    }

    pub fn mode_name(&self) -> &'static str {
        self.mode.mode_name()
    }

    pub fn status_line(&self) -> String {
        let mode = self.mode_name();
        let file = self.filename().unwrap_or("[No Name]");
        let modified = if self.modified { " [+]" } else { "" };
        let position = format!("{}:{}", self.cursor_position.0, self.cursor_position.1);

        format!(" {} | {}{} | {}", mode, file, modified, position)
    }
}

// ============================================================================
// NeovimConfig
// ============================================================================

#[derive(Debug, Clone)]
pub struct NeovimConfig {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub session_file: Option<PathBuf>,
    pub init_lua: Option<PathBuf>,
    pub preserve_user_config: bool,
}

impl Default for NeovimConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl NeovimConfig {
    pub fn new() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let home_path = PathBuf::from(home);
        let config_dir = home_path.join(".config").join("nvim");
        let data_dir = home_path.join(".local").join("share").join("nvim");
        let cache_dir = home_path.join(".cache").join("nvim");

        Self {
            config_dir,
            data_dir,
            cache_dir,
            session_file: None,
            init_lua: None,
            preserve_user_config: true,
        }
    }

    /// Converts this NeovimConfig into a generic ProcessConfig.
    /// This is the thin adapters layer — all neovim-specific logic is isolated here.
    pub fn to_process_config(&self) -> ProcessConfig {
        let program = std::env::var("NVIM_PATH").unwrap_or_else(|_| "nvim".to_string());
        let args = self.build_nvim_args();

        ProcessConfig::new(&program).with_args(args)
    }

    pub fn with_config_dir(mut self, dir: PathBuf) -> Self {
        self.config_dir = dir;
        self
    }

    pub fn with_data_dir(mut self, dir: PathBuf) -> Self {
        self.data_dir = dir;
        self
    }

    pub fn with_cache_dir(mut self, dir: PathBuf) -> Self {
        self.cache_dir = dir;
        self
    }

    pub fn with_session_file(mut self, file: PathBuf) -> Self {
        self.session_file = Some(file);
        self
    }

    pub fn with_init_lua(mut self, file: PathBuf) -> Self {
        self.init_lua = Some(file);
        self
    }

    pub fn with_preserve_user_config(mut self, preserve: bool) -> Self {
        self.preserve_user_config = preserve;
        self
    }

    pub fn user_config_exists(&self) -> bool {
        self.config_dir.exists() && self.config_dir.is_dir()
    }

    pub fn user_init_lua_exists(&self) -> bool {
        let init_lua = self.config_dir.join("init.lua");
        init_lua.exists()
    }

    pub fn user_init_vim_exists(&self) -> bool {
        let init_vim = self.config_dir.join("init.vim");
        init_vim.exists()
    }

    pub fn session_file_path(&self) -> Option<&Path> {
        self.session_file.as_deref()
    }

    fn build_nvim_args(&self) -> Vec<String> {
        let mut args = Vec::new();

        if self.preserve_user_config && self.user_config_exists() {
            args.push("--cmd".to_string());
            args.push(format!("set rtp^={}", self.config_dir.display()));
        }

        if let Some(ref init_lua) = self.init_lua {
            args.push("-u".to_string());
            args.push(init_lua.display().to_string());
        }

        if let Some(ref session) = self.session_file {
            args.push("-S".to_string());
            args.push(session.display().to_string());
        }

        args.push("--clean".to_string());
        args
    }

    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.config_dir)?;
        std::fs::create_dir_all(&self.data_dir)?;
        std::fs::create_dir_all(&self.cache_dir)?;
        Ok(())
    }
}

// ============================================================================
// NeovimProcess
// ============================================================================

pub struct NeovimProcess {
    runtime: TerminalRuntime,
    state: NeovimState,
    config: NeovimConfig,
}

impl Default for NeovimProcess {
    fn default() -> Self {
        Self::new()
    }
}

impl NeovimProcess {
    pub fn new() -> Self {
        Self {
            runtime: TerminalRuntime::new(),
            state: NeovimState::new(),
            config: NeovimConfig::new(),
        }
    }

    pub fn with_config(config: NeovimConfig) -> Self {
        Self {
            runtime: TerminalRuntime::with_config(config.to_process_config()),
            state: NeovimState::new(),
            config,
        }
    }

    pub fn spawn(&mut self, size: PtySize) -> Result<(), NeovimError> {
        if self.runtime.is_running() {
            return Err(NeovimError::SpawnFailed("Already running".to_string()));
        }

        self.config.ensure_dirs().map_err(|e| {
            NeovimError::SpawnFailed(format!("Failed to create directories: {}", e))
        })?;

        self.runtime.config_mut().size = size;
        self.runtime.config_mut().args = self.config.to_process_config().args;

        self.runtime.spawn()?;
        self.state.set_running(true);

        Ok(())
    }

    pub fn write_input(&mut self, data: &[u8]) -> Result<(), NeovimError> {
        if !self.runtime.is_running() {
            return Err(NeovimError::NotRunning);
        }
        self.runtime.write(data)?;
        Ok(())
    }

    pub fn read_output(&mut self) -> Result<Vec<u8>, NeovimError> {
        if !self.runtime.is_running() {
            return Err(NeovimError::NotRunning);
        }
        let mut buf = vec![0u8; 4096];
        let n = self.runtime.read(&mut buf)?;
        buf.truncate(n);
        Ok(buf)
    }

    pub fn resize(&mut self, size: PtySize) -> Result<(), NeovimError> {
        if !self.runtime.is_running() {
            return Err(NeovimError::NotRunning);
        }
        self.runtime.resize(size)?;
        Ok(())
    }

    pub fn kill(&mut self) -> Result<(), NeovimError> {
        if !self.runtime.is_running() {
            return Err(NeovimError::NotRunning);
        }
        self.runtime.kill()?;
        self.state.set_running(false);
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.runtime.is_running() && self.state.is_running()
    }

    pub fn state(&self) -> &NeovimState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut NeovimState {
        &mut self.state
    }

    pub fn config(&self) -> &NeovimConfig {
        &self.config
    }

    pub fn runtime(&self) -> &TerminalRuntime {
        &self.runtime
    }

    pub fn runtime_mut(&mut self) -> &mut TerminalRuntime {
        &mut self.runtime
    }
}

impl Drop for NeovimProcess {
    fn drop(&mut self) {
        let _ = self.kill();
    }
}
