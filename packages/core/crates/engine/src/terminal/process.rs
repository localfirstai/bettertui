//! Terminal process management: spawning, runtime, state tracking, and viewport control.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use crate::pty::{PtyConfig, PtyError, PtyProcess, PtySize};

// === config.rs ===

#[derive(Debug, Clone)]
pub struct ProcessConfig {
    pub program: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub working_directory: Option<PathBuf>,
    pub size: PtySize,
    pub auto_restart: bool,
    pub restart_delay: Duration,
}

impl Default for ProcessConfig {
    fn default() -> Self {
        Self {
            program: String::new(),
            args: Vec::new(),
            env: Vec::new(),
            working_directory: None,
            size: PtySize::default(),
            auto_restart: false,
            restart_delay: Duration::from_millis(500),
        }
    }
}

impl ProcessConfig {
    pub fn new(program: &str) -> Self {
        Self { program: program.to_string(), ..Default::default() }
    }

    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    pub fn with_env(mut self, env: Vec<(String, String)>) -> Self {
        self.env = env;
        self
    }

    pub fn with_working_directory(mut self, dir: PathBuf) -> Self {
        self.working_directory = Some(dir);
        self
    }

    pub fn with_size(mut self, size: PtySize) -> Self {
        self.size = size;
        self
    }

    pub fn with_auto_restart(mut self, enabled: bool) -> Self {
        self.auto_restart = enabled;
        self
    }

    pub fn with_restart_delay(mut self, delay: Duration) -> Self {
        self.restart_delay = delay;
        self
    }

    pub fn is_valid(&self) -> bool {
        !self.program.is_empty()
    }
}

// === state.rs ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessStatus {
    Stopped,
    Running,
    Exited(i32),
    Signaled(i32),
    Error,
}

impl ProcessStatus {
    pub fn is_running(&self) -> bool {
        matches!(self, ProcessStatus::Running)
    }

    pub fn is_exited(&self) -> bool {
        matches!(self, ProcessStatus::Exited(_) | ProcessStatus::Signaled(_))
    }

    pub fn exit_code(&self) -> Option<i32> {
        match self {
            ProcessStatus::Exited(code) => Some(*code),
            ProcessStatus::Signaled(code) => Some(*code),
            _ => None,
        }
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self, ProcessStatus::Stopped | ProcessStatus::Error)
    }
}

#[derive(Debug, Clone)]
pub struct TerminalState {
    status: ProcessStatus,
    pid: Option<u32>,
    started_at: Option<Instant>,
    exit_code: Option<i32>,
    restart_count: u32,
    uptime_seconds: u64,
}

impl Default for TerminalState {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalState {
    pub fn new() -> Self {
        Self {
            status: ProcessStatus::Stopped,
            pid: None,
            started_at: None,
            exit_code: None,
            restart_count: 0,
            uptime_seconds: 0,
        }
    }

    pub fn status(&self) -> ProcessStatus {
        self.status
    }

    pub fn is_running(&self) -> bool {
        self.status.is_running()
    }

    pub fn pid(&self) -> Option<u32> {
        self.pid
    }

    pub fn is_exited(&self) -> bool {
        self.status.is_exited()
    }

    pub fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    pub fn started_at(&self) -> Option<Instant> {
        self.started_at
    }

    pub fn restart_count(&self) -> u32 {
        self.restart_count
    }

    pub fn uptime_seconds(&self) -> u64 {
        self.uptime_seconds
    }

    pub fn mark_started(&mut self, pid: u32) {
        self.status = ProcessStatus::Running;
        self.pid = Some(pid);
        self.started_at = Some(Instant::now());
        self.exit_code = None;
    }

    pub fn mark_exited(&mut self, code: i32) {
        self.status = ProcessStatus::Exited(code);
        self.exit_code = Some(code);
        self.pid = None;
        if let Some(start) = self.started_at {
            self.uptime_seconds = start.elapsed().as_secs();
        }
    }

    pub fn mark_signaled(&mut self, signal: i32) {
        self.status = ProcessStatus::Signaled(signal);
        self.exit_code = Some(signal);
        self.pid = None;
        if let Some(start) = self.started_at {
            self.uptime_seconds = start.elapsed().as_secs();
        }
    }

    pub fn mark_error(&mut self) {
        self.status = ProcessStatus::Error;
        self.pid = None;
    }

    pub fn mark_restart(&mut self) {
        self.restart_count += 1;
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

// === spawner.rs ===

pub struct SpawnResult {
    pub process_config: ProcessConfig,
    pub pid: u32,
    pub time: Instant,
}

impl SpawnResult {
    pub fn new(process_config: ProcessConfig, pid: u32) -> Self {
        Self { process_config, pid, time: Instant::now() }
    }
}

pub struct ProcessSpawner {
    default_shell: String,
    default_size: PtySize,
    default_env: Vec<(String, String)>,
    search_paths: Vec<String>,
    spawn_history: Vec<SpawnResult>,
    max_history: usize,
}

impl Default for ProcessSpawner {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessSpawner {
    pub fn new() -> Self {
        let default_shell =
            std::env::var("SHELL").or_else(|_| std::env::var("COMSPEC")).unwrap_or_else(|_| "/bin/bash".to_string());

        Self {
            default_shell,
            default_size: PtySize::default(),
            default_env: Vec::new(),
            search_paths: Self::default_search_paths(),
            spawn_history: Vec::new(),
            max_history: 100,
        }
    }

    pub fn with_default_shell(mut self, shell: &str) -> Self {
        self.default_shell = shell.to_string();
        self
    }

    pub fn with_default_size(mut self, size: PtySize) -> Self {
        self.default_size = size;
        self
    }

    pub fn with_default_env(mut self, env: Vec<(String, String)>) -> Self {
        self.default_env = env;
        self
    }

    pub fn with_max_history(mut self, max: usize) -> Self {
        self.max_history = max;
        self
    }

    pub fn resolve_command(&self, command: &str) -> String {
        if command.contains('/') || cfg!(target_os = "windows") {
            return command.to_string();
        }
        for path in &self.search_paths {
            let full = format!("{}/{}", path, command);
            if std::path::Path::new(&full).exists() {
                return full;
            }
        }
        command.to_string()
    }

    pub fn build_config(&self, program: &str) -> ProcessConfig {
        ProcessConfig {
            program: self.resolve_command(program),
            args: Vec::new(),
            env: self.default_env.clone(),
            working_directory: None,
            size: self.default_size,
            auto_restart: false,
            restart_delay: Duration::from_millis(500),
        }
    }

    pub fn build_config_with_args(&self, program: &str, args: Vec<String>) -> ProcessConfig {
        ProcessConfig {
            program: self.resolve_command(program),
            args,
            env: self.default_env.clone(),
            working_directory: None,
            size: self.default_size,
            auto_restart: false,
            restart_delay: Duration::from_millis(500),
        }
    }

    pub fn build_config_from_parts(&self, parts: ProcessConfigBuilder) -> ProcessConfig {
        let program =
            if parts.program.is_empty() { self.default_shell.clone() } else { self.resolve_command(&parts.program) };

        let mut env = self.default_env.clone();
        env.extend(parts.env);

        ProcessConfig {
            program,
            args: parts.args,
            env,
            working_directory: parts.working_directory,
            size: parts.size.unwrap_or(self.default_size),
            auto_restart: parts.auto_restart,
            restart_delay: parts.restart_delay.unwrap_or(Duration::from_millis(500)),
        }
    }

    pub fn record_spawn(&mut self, result: SpawnResult) {
        self.spawn_history.push(result);
        if self.spawn_history.len() > self.max_history {
            self.spawn_history.remove(0);
        }
    }

    pub fn spawn_history(&self) -> &[SpawnResult] {
        &self.spawn_history
    }

    pub fn default_shell(&self) -> &str {
        &self.default_shell
    }

    pub fn default_size(&self) -> PtySize {
        self.default_size
    }

    fn default_search_paths() -> Vec<String> {
        let mut paths = Vec::new();
        if let Ok(path_var) = std::env::var("PATH") {
            paths.extend(std::env::split_paths(&path_var).map(|p| p.to_string_lossy().to_string()));
        }
        paths
    }
}

pub struct ProcessConfigBuilder {
    pub program: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub working_directory: Option<PathBuf>,
    pub size: Option<PtySize>,
    pub auto_restart: bool,
    pub restart_delay: Option<Duration>,
}

impl ProcessConfigBuilder {
    pub fn new(program: &str) -> Self {
        Self {
            program: program.to_string(),
            args: Vec::new(),
            env: Vec::new(),
            working_directory: None,
            size: None,
            auto_restart: false,
            restart_delay: None,
        }
    }

    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    pub fn with_env(mut self, env: Vec<(String, String)>) -> Self {
        self.env = env;
        self
    }

    pub fn with_env_map(mut self, env: HashMap<String, String>) -> Self {
        self.env = env.into_iter().collect();
        self
    }

    pub fn with_working_directory(mut self, dir: PathBuf) -> Self {
        self.working_directory = Some(dir);
        self
    }

    pub fn with_size(mut self, size: PtySize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_auto_restart(mut self, enabled: bool) -> Self {
        self.auto_restart = enabled;
        self
    }

    pub fn with_restart_delay(mut self, delay: Duration) -> Self {
        self.restart_delay = Some(delay);
        self
    }
}

// === runtime.rs ===

#[derive(Debug, Clone)]
pub enum TerminalError {
    SpawnFailed(String),
    ReadFailed(String),
    WriteFailed(String),
    ResizeFailed(String),
    KillFailed(String),
    NotRunning,
    ProcessExited(i32),
    InvalidConfig(String),
    IoError(String),
}

impl std::fmt::Display for TerminalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpawnFailed(msg) => write!(f, "Failed to spawn process: {}", msg),
            Self::ReadFailed(msg) => write!(f, "Failed to read from process: {}", msg),
            Self::WriteFailed(msg) => write!(f, "Failed to write to process: {}", msg),
            Self::ResizeFailed(msg) => write!(f, "Failed to resize terminal: {}", msg),
            Self::KillFailed(msg) => write!(f, "Failed to kill process: {}", msg),
            Self::NotRunning => write!(f, "Process is not running"),
            Self::ProcessExited(code) => write!(f, "Process exited with code: {}", code),
            Self::InvalidConfig(msg) => write!(f, "Invalid process configuration: {}", msg),
            Self::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for TerminalError {}

impl From<PtyError> for TerminalError {
    fn from(err: PtyError) -> Self {
        match err {
            PtyError::SpawnFailed(msg) => Self::SpawnFailed(msg),
            PtyError::ResizeFailed(msg) => Self::ResizeFailed(msg),
            PtyError::ReadFailed(msg) => Self::ReadFailed(msg),
            PtyError::WriteFailed(msg) => Self::WriteFailed(msg),
            PtyError::KillFailed(msg) => Self::KillFailed(msg),
            PtyError::NotRunning => Self::NotRunning,
            PtyError::ProcessExited(code) => Self::ProcessExited(code),
        }
    }
}

pub struct TerminalRuntime {
    process: Option<PtyProcess>,
    config: ProcessConfig,
    state: TerminalState,
    auto_restart: bool,
    shutdown_requested: Arc<AtomicBool>,
    last_read: Instant,
    last_write: Instant,
}

impl Default for TerminalRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalRuntime {
    pub fn new() -> Self {
        Self {
            process: None,
            config: ProcessConfig::default(),
            state: TerminalState::new(),
            auto_restart: false,
            shutdown_requested: Arc::new(AtomicBool::new(false)),
            last_read: Instant::now(),
            last_write: Instant::now(),
        }
    }

    pub fn with_config(config: ProcessConfig) -> Self {
        let auto_restart = config.auto_restart;
        Self {
            process: None,
            config,
            state: TerminalState::new(),
            auto_restart,
            shutdown_requested: Arc::new(AtomicBool::new(false)),
            last_read: Instant::now(),
            last_write: Instant::now(),
        }
    }

    pub fn spawn(&mut self) -> Result<(), TerminalError> {
        if !self.config.is_valid() {
            return Err(TerminalError::InvalidConfig("No program specified in ProcessConfig".to_string()));
        }

        if self.state.is_running() {
            let _ = self.kill();
        }

        let pty_config = PtyConfig {
            program: self.config.program.clone(),
            args: self.config.args.clone(),
            env: self.config.env.clone(),
            working_directory: self.config.working_directory.as_ref().map(|p| p.to_string_lossy().to_string()),
            size: self.config.size,
        };

        let process = PtyProcess::spawn(pty_config)?;
        let pid = std::process::id();
        self.state.mark_started(pid);
        self.process = Some(process);
        self.last_read = Instant::now();
        self.last_write = Instant::now();

        Ok(())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, TerminalError> {
        if let Some(ref mut process) = self.process {
            let n = process.read(buf)?;
            if n > 0 {
                self.last_read = Instant::now();
            }
            self.check_process();
            Ok(n)
        } else {
            Err(TerminalError::NotRunning)
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, TerminalError> {
        if let Some(ref mut process) = self.process {
            let n = process.write(data)?;
            if n > 0 {
                self.last_write = Instant::now();
            }
            self.check_process();
            Ok(n)
        } else {
            Err(TerminalError::NotRunning)
        }
    }

    pub fn resize(&mut self, size: PtySize) -> Result<(), TerminalError> {
        self.config.size = size;
        if let Some(ref mut process) = self.process {
            process.resize(size)?;
        }
        Ok(())
    }

    pub fn kill(&mut self) -> Result<(), TerminalError> {
        if let Some(ref mut process) = self.process {
            let result = process.kill();
            self.state.mark_exited(1);
            self.process = None;
            result?;
        }
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<(), TerminalError> {
        self.shutdown_requested.store(true, Ordering::SeqCst);
        let result = self.kill();
        self.state.reset();
        result
    }

    pub fn shutdown_requested(&self) -> bool {
        self.shutdown_requested.load(Ordering::SeqCst)
    }

    pub fn wait(&mut self) -> Result<Option<i32>, TerminalError> {
        if let Some(ref mut process) = self.process {
            let code = process.wait()?;
            if let Some(c) = code {
                self.state.mark_exited(c);
            }
            self.process = None;
            Ok(code)
        } else {
            Err(TerminalError::NotRunning)
        }
    }

    pub fn is_running(&self) -> bool {
        self.state.is_running()
    }

    pub fn state(&self) -> &TerminalState {
        &self.state
    }

    pub fn config(&self) -> &ProcessConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut ProcessConfig {
        &mut self.config
    }

    pub fn size(&self) -> PtySize {
        self.config.size
    }

    pub fn idle_since_read(&self) -> Duration {
        self.last_read.elapsed()
    }

    pub fn idle_since_write(&self) -> Duration {
        self.last_write.elapsed()
    }

    pub fn try_restart(&mut self) -> Result<(), TerminalError> {
        if !self.auto_restart {
            return Err(TerminalError::NotRunning);
        }
        self.state.mark_restart();
        self.spawn()
    }

    pub fn process(&self) -> Option<&PtyProcess> {
        self.process.as_ref()
    }

    pub fn process_mut(&mut self) -> Option<&mut PtyProcess> {
        self.process.as_mut()
    }

    fn check_process(&mut self) {
        if let Some(ref mut process) = self.process
            && !process.is_running()
            && let Some(code) = process.exit_status()
        {
            self.state.mark_exited(code);
        }
    }
}

impl Drop for TerminalRuntime {
    fn drop(&mut self) {
        self.shutdown_requested.store(true, Ordering::SeqCst);
        let _ = self.kill();
    }
}

// === viewport.rs ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollMode {
    Fixed,
    Scrollable,
    Infinite,
}

#[derive(Debug, Clone)]
pub struct TerminalViewport {
    cols: u16,
    rows: u16,
    scroll_offset: u32,
    scrollback_lines: u32,
    scroll_mode: ScrollMode,
    pixel_width: u32,
    pixel_height: u32,
    cell_width: u16,
    cell_height: u16,
}

impl Default for TerminalViewport {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalViewport {
    pub fn new() -> Self {
        Self {
            cols: 80,
            rows: 24,
            scroll_offset: 0,
            scrollback_lines: 10000,
            scroll_mode: ScrollMode::Scrollable,
            pixel_width: 0,
            pixel_height: 0,
            cell_width: 1,
            cell_height: 1,
        }
    }

    pub fn with_size(cols: u16, rows: u16) -> Self {
        Self {
            cols,
            rows,
            scroll_offset: 0,
            scrollback_lines: 10000,
            scroll_mode: ScrollMode::Scrollable,
            pixel_width: 0,
            pixel_height: 0,
            cell_width: 1,
            cell_height: 1,
        }
    }

    pub fn cols(&self) -> u16 {
        self.cols
    }

    pub fn rows(&self) -> u16 {
        self.rows
    }

    pub fn scroll_offset(&self) -> u32 {
        self.scroll_offset
    }

    pub fn scrollback_lines(&self) -> u32 {
        self.scrollback_lines
    }

    pub fn scroll_mode(&self) -> ScrollMode {
        self.scroll_mode
    }

    pub fn pixel_width(&self) -> u32 {
        self.pixel_width
    }

    pub fn pixel_height(&self) -> u32 {
        self.pixel_height
    }

    pub fn cell_width(&self) -> u16 {
        self.cell_width
    }

    pub fn cell_height(&self) -> u16 {
        self.cell_height
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
    }

    pub fn resize_with_pixels(&mut self, cols: u16, rows: u16, pixel_w: u32, pixel_h: u32) {
        self.cols = cols;
        self.rows = rows;
        self.pixel_width = pixel_w;
        self.pixel_height = pixel_h;
    }

    pub fn set_cell_size(&mut self, width: u16, height: u16) {
        self.cell_width = width;
        self.cell_height = height;
    }

    pub fn set_scrollback_lines(&mut self, lines: u32) {
        self.scrollback_lines = lines;
    }

    pub fn set_scroll_mode(&mut self, mode: ScrollMode) {
        self.scroll_mode = mode;
    }

    pub fn scroll_up(&mut self, lines: u32) {
        if self.scroll_mode == ScrollMode::Fixed {
            return;
        }
        self.scroll_offset = self.scroll_offset.saturating_add(lines).min(self.scrollback_lines);
    }

    pub fn scroll_down(&mut self, lines: u32) {
        if self.scroll_mode == ScrollMode::Fixed {
            return;
        }
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }

    pub fn scroll_reset(&mut self) {
        self.scroll_offset = 0;
    }

    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = self.scrollback_lines;
    }

    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
    }

    pub fn is_scrolled(&self) -> bool {
        self.scroll_offset > 0
    }

    pub fn visible_line_count(&self) -> u32 {
        self.rows as u32
    }

    pub fn total_line_count(&self) -> u32 {
        self.scrollback_lines + self.rows as u32
    }

    pub fn to_pty_size(&self) -> PtySize {
        PtySize { cols: self.cols, rows: self.rows, pixel_width: self.pixel_width, pixel_height: self.pixel_height }
    }

    pub fn total_cells(&self) -> u32 {
        self.cols as u32 * self.rows as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_config_new() {
        let c = ProcessConfig::new("bash");
        assert_eq!(c.program, "bash");
        assert!(c.is_valid());
    }

    #[test]
    fn process_config_default_invalid() {
        let c = ProcessConfig::default();
        assert!(!c.is_valid());
        assert!(c.args.is_empty());
    }

    #[test]
    fn process_config_builder() {
        let c = ProcessConfig::new("nvim")
            .with_args(vec!["--clean".to_string()])
            .with_env(vec![("TERM".to_string(), "xterm-256color".to_string())])
            .with_size(PtySize { cols: 120, rows: 40, pixel_width: 0, pixel_height: 0 })
            .with_auto_restart(true)
            .with_restart_delay(Duration::from_millis(200));
        assert_eq!(c.program, "nvim");
        assert_eq!(c.args.len(), 1);
        assert!(c.auto_restart);
        assert_eq!(c.restart_delay, Duration::from_millis(200));
    }

    #[test]
    fn process_status_running() {
        assert!(ProcessStatus::Running.is_running());
        assert!(!ProcessStatus::Stopped.is_running());
        assert!(!ProcessStatus::Exited(0).is_running());
    }

    #[test]
    fn process_status_exit_code() {
        assert_eq!(ProcessStatus::Exited(0).exit_code(), Some(0));
        assert_eq!(ProcessStatus::Signaled(9).exit_code(), Some(9));
        assert_eq!(ProcessStatus::Running.exit_code(), None);
        assert_eq!(ProcessStatus::Stopped.exit_code(), None);
    }

    #[test]
    fn process_status_is_exited() {
        assert!(ProcessStatus::Exited(1).is_exited());
        assert!(ProcessStatus::Signaled(15).is_exited());
        assert!(!ProcessStatus::Running.is_exited());
    }

    #[test]
    fn process_status_is_stopped() {
        assert!(ProcessStatus::Stopped.is_stopped());
        assert!(ProcessStatus::Error.is_stopped());
        assert!(!ProcessStatus::Running.is_stopped());
    }

    #[test]
    fn terminal_state_new() {
        let s = TerminalState::new();
        assert_eq!(s.status(), ProcessStatus::Stopped);
        assert!(s.pid().is_none());
        assert_eq!(s.restart_count(), 0);
    }

    #[test]
    fn terminal_state_mark_started() {
        let mut s = TerminalState::new();
        s.mark_started(42);
        assert!(s.is_running());
        assert_eq!(s.pid(), Some(42));
        assert!(s.started_at().is_some());
    }

    #[test]
    fn terminal_state_mark_exited() {
        let mut s = TerminalState::new();
        s.mark_started(1);
        s.mark_exited(0);
        assert_eq!(s.status(), ProcessStatus::Exited(0));
        assert_eq!(s.exit_code(), Some(0));
        assert!(s.pid().is_none());
    }

    #[test]
    fn terminal_state_mark_signaled() {
        let mut s = TerminalState::new();
        s.mark_started(1);
        s.mark_signaled(9);
        assert_eq!(s.status(), ProcessStatus::Signaled(9));
        assert_eq!(s.exit_code(), Some(9));
    }

    #[test]
    fn terminal_state_mark_error() {
        let mut s = TerminalState::new();
        s.mark_started(1);
        s.mark_error();
        assert_eq!(s.status(), ProcessStatus::Error);
        assert!(s.pid().is_none());
    }

    #[test]
    fn terminal_state_restart() {
        let mut s = TerminalState::new();
        assert_eq!(s.restart_count(), 0);
        s.mark_restart();
        assert_eq!(s.restart_count(), 1);
        s.mark_restart();
        assert_eq!(s.restart_count(), 2);
    }

    #[test]
    fn terminal_state_reset() {
        let mut s = TerminalState::new();
        s.mark_started(42);
        s.mark_exited(0);
        s.mark_restart();
        s.reset();
        assert_eq!(s.status(), ProcessStatus::Stopped);
        assert_eq!(s.restart_count(), 0);
    }

    #[test]
    fn spawn_result_new() {
        let config = ProcessConfig::new("echo");
        let r = SpawnResult::new(config, 42);
        assert_eq!(r.pid, 42);
        assert_eq!(r.process_config.program, "echo");
    }

    #[test]
    fn terminal_viewport_default() {
        let vp = TerminalViewport::default();
        assert_eq!(vp.cols, 80);
        assert_eq!(vp.rows, 24);
        assert_eq!(vp.total_cells(), 1920);
    }

    #[test]
    fn terminal_viewport_scroll() {
        let mut vp = TerminalViewport::default();
        assert!(!vp.is_scrolled());
        vp.scroll_offset = 10;
        assert!(vp.is_scrolled());
    }

    #[test]
    fn terminal_viewport_line_counts() {
        let vp = TerminalViewport::default();
        assert_eq!(vp.visible_line_count(), 24);
        assert_eq!(vp.total_line_count(), 24 + vp.scrollback_lines);
    }

    #[test]
    fn terminal_viewport_to_pty_size() {
        let vp = TerminalViewport::default();
        let pty = vp.to_pty_size();
        assert_eq!(pty.cols, 80);
        assert_eq!(pty.rows, 24);
    }
}
