use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use crate::pty::{PtyConfig, PtyError, PtyProcess, PtySize};
use crate::terminal::process::config::ProcessConfig;
use crate::terminal::process::state::TerminalState;

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
            return Err(TerminalError::InvalidConfig(
                "No program specified in ProcessConfig".to_string(),
            ));
        }

        if self.state.is_running() {
            let _ = self.kill();
        }

        let pty_config = PtyConfig {
            program: self.config.program.clone(),
            args: self.config.args.clone(),
            env: self.config.env.clone(),
            working_directory: self
                .config
                .working_directory
                .as_ref()
                .map(|p| p.to_string_lossy().to_string()),
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

#[cfg(test)]
mod tests {
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
        // Second shutdown should be safe
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
