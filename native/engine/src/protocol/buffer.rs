use super::command::Command;

/// Pre-allocated buffer for batching commands before sending to the engine.
///
/// Commands are accumulated in the buffer during a React render cycle,
/// then flushed to the engine in a single FFI call.
pub struct CommandBuffer {
    commands: Vec<Command>,
    capacity: usize,
}

impl Default for CommandBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandBuffer {
    /// Create a new buffer with default capacity.
    pub fn new() -> Self {
        Self::with_capacity(64)
    }

    /// Create a new buffer with a specific capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            commands: Vec::with_capacity(capacity),
            capacity,
        }
    }

    /// Push a command into the buffer.
    pub fn push(&mut self, cmd: Command) {
        self.commands.push(cmd);
    }

    /// Take all commands from the buffer, leaving it empty.
    pub fn drain(&mut self) -> Vec<Command> {
        std::mem::take(&mut self.commands)
    }

    /// Peek at the commands without taking them.
    pub fn peek(&self) -> &[Command] {
        &self.commands
    }

    /// Clear the buffer.
    pub fn clear(&mut self) {
        self.commands.clear();
    }

    /// Number of commands in the buffer.
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Returns true if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// Estimated byte size of the buffer contents.
    pub fn estimated_size(&self) -> usize {
        // Rough estimate: each command is ~64-256 bytes
        self.commands.len() * 128
    }

    /// Pre-allocate for at least `additional` more commands.
    pub fn reserve(&mut self, additional: usize) {
        self.commands.reserve(additional);
    }

    /// Get the capacity of the buffer.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl From<Vec<Command>> for CommandBuffer {
    fn from(commands: Vec<Command>) -> Self {
        let capacity = commands.len();
        Self { commands, capacity }
    }
}

impl IntoIterator for CommandBuffer {
    type Item = Command;
    type IntoIter = std::vec::IntoIter<Command>;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::{NodeId, NodeKind};

    #[test]
    fn buffer_new() {
        let buf = CommandBuffer::new();
        assert!(buf.is_empty());
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn buffer_push() {
        let mut buf = CommandBuffer::new();
        buf.push(Command::CreateNode {
            id: NodeId::default(),
            kind: NodeKind::Box,
        });
        assert_eq!(buf.len(), 1);
        assert!(!buf.is_empty());
    }

    #[test]
    fn buffer_drain() {
        let mut buf = CommandBuffer::new();
        buf.push(Command::Shutdown);
        buf.push(Command::BeginFrame { frame_id: 1 });

        let cmds = buf.drain();
        assert_eq!(cmds.len(), 2);
        assert!(buf.is_empty());
    }

    #[test]
    fn buffer_clear() {
        let mut buf = CommandBuffer::new();
        buf.push(Command::Shutdown);
        buf.clear();
        assert!(buf.is_empty());
    }

    #[test]
    fn buffer_from_vec() {
        let cmds = vec![Command::Shutdown, Command::BeginFrame { frame_id: 1 }];
        let buf = CommandBuffer::from(cmds);
        assert_eq!(buf.len(), 2);
    }

    #[test]
    fn buffer_into_iter() {
        let mut buf = CommandBuffer::new();
        buf.push(Command::Shutdown);
        buf.push(Command::BeginFrame { frame_id: 1 });

        let cmds: Vec<_> = buf.into_iter().collect();
        assert_eq!(cmds.len(), 2);
    }

    #[test]
    fn buffer_estimated_size() {
        let mut buf = CommandBuffer::new();
        buf.push(Command::Shutdown);
        let size = buf.estimated_size();
        assert!(size > 0);
    }
}
