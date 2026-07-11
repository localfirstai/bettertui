use super::config::NeovimConfig;
use super::state::NeovimState;
use crate::pty::PtySize;
use crate::terminal_process::{TerminalError, TerminalRuntime};

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
