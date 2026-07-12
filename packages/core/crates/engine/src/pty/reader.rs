use super::PtyError;
use std::io::Read;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pty_reader_new() {
        let reader = PtyReader::new();
        assert!(reader.is_empty());
        assert_eq!(reader.available(), 0);
    }

    #[test]
    fn pty_reader_default() {
        let reader = PtyReader::default();
        assert!(reader.is_empty());
    }

    #[test]
    fn pty_reader_read_line() {
        let mut reader = PtyReader::new();
        reader.buffer = b"hello\nworld\n".to_vec();
        assert_eq!(reader.read_line(), Some("hello\n".to_string()));
        assert_eq!(reader.read_line(), Some("world\n".to_string()));
        assert_eq!(reader.read_line(), None);
    }

    #[test]
    fn pty_reader_read_bytes() {
        let mut reader = PtyReader::new();
        reader.buffer = b"hello world".to_vec();
        let bytes = reader.read_bytes(5);
        assert_eq!(bytes, b"hello");
        assert_eq!(reader.available(), 6);
    }

    #[test]
    fn pty_reader_compact() {
        let mut reader = PtyReader::new();
        reader.buffer = b"hello world".to_vec();
        reader.position = 5;
        reader.compact();
        assert_eq!(reader.buffer, b" world");
        assert_eq!(reader.position, 0);
    }
}
