use std::process::{Command, Stdio};

use super::PtyError;

#[derive(Debug, Clone)]
pub struct PtyConfig {
    pub program: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub working_directory: Option<String>,
    pub size: PtySize,
}

impl Default for PtyConfig {
    fn default() -> Self {
        Self {
            program: "bash".to_string(),
            args: Vec::new(),
            env: Vec::new(),
            working_directory: None,
            size: PtySize::default(),
        }
    }
}

impl PtyConfig {
    pub fn new(program: &str) -> Self {
        Self {
            program: program.to_string(),
            ..Default::default()
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

    pub fn with_working_directory(mut self, dir: String) -> Self {
        self.working_directory = Some(dir);
        self
    }

    pub fn with_size(mut self, size: PtySize) -> Self {
        self.size = size;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PtySize {
    pub cols: u16,
    pub rows: u16,
    pub pixel_width: u32,
    pub pixel_height: u32,
}

impl Default for PtySize {
    fn default() -> Self {
        Self {
            cols: 80,
            rows: 24,
            pixel_width: 0,
            pixel_height: 0,
        }
    }
}

impl PtySize {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self {
            cols,
            rows,
            pixel_width: 0,
            pixel_height: 0,
        }
    }

    pub fn with_pixel_size(mut self, width: u32, height: u32) -> Self {
        self.pixel_width = width;
        self.pixel_height = height;
        self
    }
}

pub struct PtyProcess {
    child: Option<std::process::Child>,
    stdin: Option<std::process::ChildStdin>,
    stdout: Option<std::process::ChildStdout>,
    size: PtySize,
}

impl PtyProcess {
    pub fn spawn(config: PtyConfig) -> Result<Self, PtyError> {
        let mut cmd = Command::new(&config.program);
        cmd.args(&config.args);

        for (key, value) in &config.env {
            cmd.env(key, value);
        }

        if let Some(ref dir) = config.working_directory {
            cmd.current_dir(dir);
        }

        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd
            .spawn()
            .map_err(|e| PtyError::SpawnFailed(e.to_string()))?;

        let stdin = child.stdin.take();
        let stdout = child.stdout.take();

        Ok(Self {
            child: Some(child),
            stdin,
            stdout,
            size: config.size,
        })
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, PtyError> {
        use std::io::Read;
        if let Some(ref mut stdout) = self.stdout {
            stdout
                .read(buf)
                .map_err(|e| PtyError::ReadFailed(e.to_string()))
        } else {
            Err(PtyError::ReadFailed("No stdout".to_string()))
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, PtyError> {
        use std::io::Write;
        if let Some(ref mut stdin) = self.stdin {
            stdin
                .write(data)
                .map_err(|e| PtyError::WriteFailed(e.to_string()))
        } else {
            Err(PtyError::WriteFailed("No stdin".to_string()))
        }
    }

    pub fn resize(&mut self, size: PtySize) -> Result<(), PtyError> {
        self.size = size;
        Ok(())
    }

    pub fn is_running(&mut self) -> bool {
        if let Some(ref mut child) = self.child {
            child.try_wait().is_ok_and(|status| status.is_none())
        } else {
            false
        }
    }

    pub fn exit_status(&mut self) -> Option<i32> {
        if let Some(ref mut child) = self.child {
            child
                .try_wait()
                .ok()
                .flatten()
                .and_then(|status| status.code())
        } else {
            None
        }
    }

    pub fn kill(&mut self) -> Result<(), PtyError> {
        if let Some(ref mut child) = self.child {
            child
                .kill()
                .map_err(|e| PtyError::KillFailed(e.to_string()))?;
        }
        Ok(())
    }

    pub fn wait(&mut self) -> Result<Option<i32>, PtyError> {
        if let Some(ref mut child) = self.child {
            let status = child
                .wait()
                .map_err(|e| PtyError::KillFailed(e.to_string()))?;
            Ok(status.code())
        } else {
            Err(PtyError::NotRunning)
        }
    }

    pub fn size(&self) -> PtySize {
        self.size
    }
}

#[derive(Debug, Clone)]
pub struct PtyOutput {
    pub data: Vec<u8>,
    pub size: PtySize,
}

impl PtyOutput {
    pub fn new(data: Vec<u8>, size: PtySize) -> Self {
        Self { data, size }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pty_config_new() {
        let config = PtyConfig::new("bash");
        assert_eq!(config.program, "bash");
    }

    #[test]
    fn pty_config_default() {
        let config = PtyConfig::default();
        assert_eq!(config.program, "bash");
        assert!(config.args.is_empty());
    }

    #[test]
    fn pty_size_new() {
        let size = PtySize::new(80, 24);
        assert_eq!(size.cols, 80);
        assert_eq!(size.rows, 24);
    }

    #[test]
    fn pty_size_default() {
        let size = PtySize::default();
        assert_eq!(size.cols, 80);
        assert_eq!(size.rows, 24);
    }
}
