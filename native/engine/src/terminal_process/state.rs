use std::time::Instant;

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

#[cfg(test)]
mod tests {
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
