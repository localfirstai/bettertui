use super::PtyError;
use std::io::Write;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pty_writer_new() {
        let writer = PtyWriter::new();
        assert!(writer.is_empty());
        assert_eq!(writer.pending(), 0);
    }

    #[test]
    fn pty_writer_default() {
        let writer = PtyWriter::default();
        assert!(writer.is_empty());
    }

    #[test]
    fn pty_writer_write() {
        let mut writer = PtyWriter::new();
        writer.write(b"hello");
        assert_eq!(writer.pending(), 5);
        assert!(!writer.is_empty());
    }

    #[test]
    fn pty_writer_write_str() {
        let mut writer = PtyWriter::new();
        writer.write_str("hello");
        assert_eq!(writer.pending(), 5);
    }

    #[test]
    fn pty_writer_write_byte() {
        let mut writer = PtyWriter::new();
        writer.write_byte(b'x');
        assert_eq!(writer.pending(), 1);
    }

    #[test]
    fn pty_writer_clear() {
        let mut writer = PtyWriter::new();
        writer.write(b"hello");
        writer.clear();
        assert!(writer.is_empty());
    }
}
