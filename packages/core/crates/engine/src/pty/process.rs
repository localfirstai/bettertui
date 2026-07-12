use std::io::{Read, Write};

use portable_pty::{CommandBuilder, MasterPty, PtySize as PortablePtySize};

use super::PtyError;

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
        // Close master end first to send SIGHUP to child process group
        self.master.take();
        self.reader.take();
        self.writer.take();

        // Also send SIGTERM via platform mechanism
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

    #[test]
    fn pty_size_to_portable() {
        let size = PtySize::new(80, 24);
        let p = size.to_portable();
        assert_eq!(p.cols, 80);
        assert_eq!(p.rows, 24);
    }

    #[test]
    fn pty_size_with_pixel() {
        let size = PtySize::new(80, 24).with_pixel_size(800, 600);
        assert_eq!(size.pixel_width, 800);
        assert_eq!(size.pixel_height, 600);
        let p = size.to_portable();
        assert_eq!(p.pixel_width as u32, 800);
        assert_eq!(p.pixel_height as u32, 600);
    }
}
