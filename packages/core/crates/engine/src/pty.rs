//! PTY abstraction: portable pseudo-terminal with process spawning, reading, and writing.

use std::io::{Read, Write};

use portable_pty::{CommandBuilder, MasterPty, PtySize as PortablePtySize};

// ============================================================================
// Platform-specific signal handling
// ============================================================================

#[cfg(unix)]
mod platform {
    const SIGTERM: i32 = 15;
    const SIGKILL: i32 = 9;

    unsafe extern "C" {
        fn kill(pid: i32, sig: i32) -> i32;
    }

    pub fn send_signal(pid: u32, sig: i32) -> Result<(), String> {
        let ret = unsafe { kill(pid as i32, sig) };
        if ret != 0 {
            Err(std::io::Error::last_os_error().to_string())
        } else {
            Ok(())
        }
    }

    pub fn terminate(pid: u32) -> Result<(), String> {
        send_signal(pid, SIGTERM).or_else(|_| send_signal(pid, SIGKILL))
    }
}

#[cfg(windows)]
mod platform {
    pub fn terminate(_pid: u32) -> Result<(), String> {
        Err("Force-kill not supported on Windows yet".to_string())
    }
}

// ============================================================================
// PtyConfig
// ============================================================================

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

// ============================================================================
// PtySize
// ============================================================================

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

    #[allow(clippy::wrong_self_convention)]
    fn to_portable(self) -> PortablePtySize {
        PortablePtySize {
            rows: self.rows,
            cols: self.cols,
            pixel_width: self.pixel_width as u16,
            pixel_height: self.pixel_height as u16,
        }
    }
}

// ============================================================================
// PtyProcess
// ============================================================================

pub struct PtyProcess {
    child: Option<Box<dyn portable_pty::Child + Send + Sync>>,
    reader: Option<Box<dyn Read + Send>>,
    writer: Option<Box<dyn Write + Send>>,
    master: Option<Box<dyn MasterPty + Send>>,
    size: PtySize,
    pid: Option<u32>,
}

impl PtyProcess {
    pub fn spawn(config: PtyConfig) -> Result<Self, PtyError> {
        let system = portable_pty::native_pty_system();
        let pair = system
            .openpty(config.size.to_portable())
            .map_err(|e| PtyError::SpawnFailed(e.to_string()))?;

        let mut cmd = CommandBuilder::new(&config.program);
        for arg in &config.args {
            cmd.arg(arg);
        }
        for (key, value) in &config.env {
            cmd.env(key, value);
        }
        if let Some(ref dir) = config.working_directory {
            cmd.cwd(dir);
        }

        let child = pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| PtyError::SpawnFailed(e.to_string()))?;
        let pid = child.process_id();

        let reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| PtyError::SpawnFailed(e.to_string()))?;
        let writer = pair
            .master
            .take_writer()
            .map_err(|e| PtyError::SpawnFailed(e.to_string()))?;

        Ok(Self {
            child: Some(child),
            reader: Some(reader),
            writer: Some(writer),
            master: Some(pair.master),
            size: config.size,
            pid,
        })
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, PtyError> {
        if let Some(ref mut reader) = self.reader {
            reader
                .read(buf)
                .map_err(|e| PtyError::ReadFailed(e.to_string()))
        } else {
            Err(PtyError::ReadFailed("No reader".to_string()))
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, PtyError> {
        if let Some(ref mut writer) = self.writer {
            writer
                .write(data)
                .map_err(|e| PtyError::WriteFailed(e.to_string()))
        } else {
            Err(PtyError::WriteFailed("No writer".to_string()))
        }
    }

    pub fn resize(&mut self, size: PtySize) -> Result<(), PtyError> {
        self.size = size;
        if let Some(ref master) = self.master {
            master
                .resize(size.to_portable())
                .map_err(|e| PtyError::ResizeFailed(e.to_string()))?;
        }
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
                .map(|status| status.exit_code() as i32)
        } else {
            None
        }
    }

    pub fn kill(&mut self) -> Result<(), PtyError> {
        self.master.take();
        self.reader.take();
        self.writer.take();

        if let Some(pid) = self.pid {
            platform::terminate(pid).map_err(PtyError::KillFailed)?;
        }
        Ok(())
    }

    pub fn wait(&mut self) -> Result<Option<i32>, PtyError> {
        if let Some(ref mut child) = self.child {
            let status = child
                .wait()
                .map_err(|e| PtyError::KillFailed(e.to_string()))?;
            Ok(Some(status.exit_code() as i32))
        } else {
            Err(PtyError::NotRunning)
        }
    }

    pub fn size(&self) -> PtySize {
        self.size
    }

    pub fn pid(&self) -> Option<u32> {
        self.pid
    }
}

// ============================================================================
// PtyOutput
// ============================================================================

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

// ============================================================================
// PtyReader
// ============================================================================

pub struct PtyReader {
    buffer: Vec<u8>,
    position: usize,
}

impl PtyReader {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(4096),
            position: 0,
        }
    }

    pub fn read_from(&mut self, reader: &mut dyn Read) -> Result<usize, PtyError> {
        let mut temp = [0u8; 4096];
        let n = reader
            .read(&mut temp)
            .map_err(|e| PtyError::ReadFailed(e.to_string()))?;
        if n > 0 {
            self.buffer.extend_from_slice(&temp[..n]);
        }
        Ok(n)
    }

    pub fn read_line(&mut self) -> Option<String> {
        if let Some(pos) = self.buffer[self.position..]
            .iter()
            .position(|&b| b == b'\n')
        {
            let end = self.position + pos + 1;
            let line = String::from_utf8_lossy(&self.buffer[self.position..end]).to_string();
            self.position = end;
            Some(line)
        } else {
            None
        }
    }

    pub fn read_bytes(&mut self, count: usize) -> Vec<u8> {
        let end = std::cmp::min(self.position + count, self.buffer.len());
        let data = self.buffer[self.position..end].to_vec();
        self.position = end;
        data
    }

    pub fn available(&self) -> usize {
        self.buffer.len() - self.position
    }

    pub fn is_empty(&self) -> bool {
        self.available() == 0
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.position = 0;
    }

    pub fn compact(&mut self) {
        if self.position > 0 {
            self.buffer.drain(0..self.position);
            self.position = 0;
        }
    }
}

impl Default for PtyReader {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// PtyWriter
// ============================================================================

pub struct PtyWriter {
    buffer: Vec<u8>,
    flushed: bool,
}

impl PtyWriter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(4096),
            flushed: true,
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
        self.flushed = false;
    }

    pub fn write_str(&mut self, s: &str) {
        self.write(s.as_bytes());
    }

    pub fn write_byte(&mut self, b: u8) {
        self.buffer.push(b);
        self.flushed = false;
    }

    pub fn flush(&mut self, writer: &mut dyn Write) -> Result<(), PtyError> {
        if !self.flushed && !self.buffer.is_empty() {
            writer
                .write_all(&self.buffer)
                .map_err(|e| PtyError::WriteFailed(e.to_string()))?;
            writer
                .flush()
                .map_err(|e| PtyError::WriteFailed(e.to_string()))?;
            self.buffer.clear();
            self.flushed = true;
        }
        Ok(())
    }

    pub fn pending(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.flushed = true;
    }
}

impl Default for PtyWriter {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// PtyRuntime
// ============================================================================

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

// ============================================================================
// PtyError
// ============================================================================

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
