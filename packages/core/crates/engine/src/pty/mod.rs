//! PTY abstraction: portable pseudo-terminal with process spawning, reading, and writing.

mod process;
mod reader;
mod writer;

pub use process::{PtyConfig, PtyOutput, PtyProcess, PtySize};
pub use reader::PtyReader;
pub use writer::PtyWriter;

pub struct PtyRuntime {
    process: Option<PtyProcess>,
}

impl Default for PtyRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl PtyRuntime {
    pub fn new() -> Self {
        Self { process: None }
    }

    pub fn spawn(&mut self, config: PtyConfig) -> Result<(), PtyError> {
        let process = PtyProcess::spawn(config)?;
        self.process = Some(process);
        Ok(())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, PtyError> {
        if let Some(ref mut process) = self.process {
            process.read(buf)
        } else {
            Err(PtyError::NotRunning)
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, PtyError> {
        if let Some(ref mut process) = self.process {
            process.write(data)
        } else {
            Err(PtyError::NotRunning)
        }
    }

    pub fn resize(&mut self, size: PtySize) -> Result<(), PtyError> {
        if let Some(ref mut process) = self.process {
            process.resize(size)
        } else {
            Err(PtyError::NotRunning)
        }
    }

    pub fn is_running(&mut self) -> bool {
        self.process.as_mut().is_some_and(|p| p.is_running())
    }

    pub fn exit_status(&mut self) -> Option<i32> {
        self.process.as_mut().and_then(|p| p.exit_status())
    }

    pub fn kill(&mut self) -> Result<(), PtyError> {
        if let Some(ref mut process) = self.process {
            process.kill()
        } else {
            Err(PtyError::NotRunning)
        }
    }

    pub fn wait(&mut self) -> Result<Option<i32>, PtyError> {
        if let Some(ref mut process) = self.process {
            process.wait()
        } else {
            Err(PtyError::NotRunning)
        }
    }
}

#[derive(Debug, Clone)]
pub enum PtyError {
    SpawnFailed(String),
    ResizeFailed(String),
    ReadFailed(String),
    WriteFailed(String),
    KillFailed(String),
    NotRunning,
    ProcessExited(i32),
}

impl std::fmt::Display for PtyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpawnFailed(msg) => write!(f, "Failed to spawn PTY: {}", msg),
            Self::ResizeFailed(msg) => write!(f, "Failed to resize PTY: {}", msg),
            Self::ReadFailed(msg) => write!(f, "Failed to read from PTY: {}", msg),
            Self::WriteFailed(msg) => write!(f, "Failed to write to PTY: {}", msg),
            Self::KillFailed(msg) => write!(f, "Failed to kill PTY process: {}", msg),
            Self::NotRunning => write!(f, "PTY is not running"),
            Self::ProcessExited(code) => write!(f, "PTY process exited with code: {}", code),
        }
    }
}

impl std::error::Error for PtyError {}

impl From<std::io::Error> for PtyError {
    fn from(err: std::io::Error) -> Self {
        Self::SpawnFailed(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pty_runtime_new() {
        let mut runtime = PtyRuntime::new();
        assert!(!runtime.is_running());
        assert!(runtime.exit_status().is_none());
    }

    #[test]
    fn pty_runtime_default() {
        let mut runtime = PtyRuntime::default();
        assert!(!runtime.is_running());
    }

    #[test]
    fn pty_error_display() {
        let err = PtyError::NotRunning;
        assert!(err.to_string().contains("not running"));
    }
}
