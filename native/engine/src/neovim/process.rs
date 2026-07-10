use super::config::NeovimConfig;
use super::state::NeovimState;
use crate::pty::{PtyConfig, PtyProcess, PtySize};

#[derive(Debug)]
pub enum NeovimError {
    SpawnFailed(String),
    PtyError(String),
    IoError(std::io::Error),
    NotRunning,
}

impl std::fmt::Display for NeovimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpawnFailed(msg) => write!(f, "Failed to spawn Neovim: {}", msg),
            Self::PtyError(msg) => write!(f, "PTY error: {}", msg),
            Self::IoError(err) => write!(f, "IO error: {}", err),
            Self::NotRunning => write!(f, "Neovim is not running"),
        }
    }
}

impl std::error::Error for NeovimError {}

impl From<std::io::Error> for NeovimError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<crate::pty::PtyError> for NeovimError {
    fn from(err: crate::pty::PtyError) -> Self {
        Self::PtyError(err.to_string())
    }
}

pub struct NeovimProcess {
    process: Option<PtyProcess>,
    config: NeovimConfig,
    state: NeovimState,
}

impl Default for NeovimProcess {
    fn default() -> Self {
        Self::new()
    }
}

impl NeovimProcess {
    pub fn new() -> Self {
        Self {
            process: None,
            config: NeovimConfig::new(),
            state: NeovimState::new(),
        }
    }

    pub fn with_config(config: NeovimConfig) -> Self {
        Self {
            process: None,
            config,
            state: NeovimState::new(),
        }
    }

    pub fn spawn(&mut self, size: PtySize) -> Result<(), NeovimError> {
        if self.process.is_some() {
            return Err(NeovimError::SpawnFailed("Already running".to_string()));
        }

        self.config.ensure_dirs()?;

        let nvim_path = std::env::var("NVIM_PATH").unwrap_or_else(|_| "nvim".to_string());

        let args = self.config.build_nvim_args();

        let pty_config = PtyConfig {
            program: nvim_path,
            args,
            env: Vec::new(),
            working_directory: None,
            size,
        };

        let process = PtyProcess::spawn(pty_config)?;
        self.process = Some(process);
        self.state.set_running(true);

        Ok(())
    }

    pub fn write_input(&mut self, data: &[u8]) -> Result<(), NeovimError> {
        if let Some(ref mut process) = self.process {
            process.write(data)?;
            Ok(())
        } else {
            Err(NeovimError::NotRunning)
        }
    }

    pub fn read_output(&mut self) -> Result<Vec<u8>, NeovimError> {
        if let Some(ref mut process) = self.process {
            let mut buf = vec![0u8; 4096];
            let n = process.read(&mut buf)?;
            buf.truncate(n);
            Ok(buf)
        } else {
            Err(NeovimError::NotRunning)
        }
    }

    pub fn resize(&mut self, size: PtySize) -> Result<(), NeovimError> {
        if let Some(ref mut process) = self.process {
            process.resize(size)?;
            Ok(())
        } else {
            Err(NeovimError::NotRunning)
        }
    }

    pub fn kill(&mut self) -> Result<(), NeovimError> {
        if let Some(ref mut process) = self.process {
            process.kill()?;
            self.state.set_running(false);
            Ok(())
        } else {
            Err(NeovimError::NotRunning)
        }
    }

    pub fn is_running(&self) -> bool {
        self.process.is_some() && self.state.is_running()
    }

    pub fn state(&self) -> &NeovimState {
        &self.state
    }

    pub fn config(&self) -> &NeovimConfig {
        &self.config
    }

    pub fn process(&self) -> Option<&PtyProcess> {
        self.process.as_ref()
    }

    pub fn process_mut(&mut self) -> Option<&mut PtyProcess> {
        self.process.as_mut()
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
}
