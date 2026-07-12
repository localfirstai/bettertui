//! Clipboard manager for copy/paste operations.
//!
//! Manages clipboard content with support for system clipboard, selection clipboard,
//! and internal clipboard buffers. Provides OSC 52 integration for terminal clipboard access.

/// The type of clipboard buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ClipboardType {
    /// System clipboard (Ctrl+C/Ctrl+V).
    #[default]
    System,
    /// Selection clipboard (primary, middle-click paste on X11).
    Selection,
    /// Internal clipboard (within the application).
    Internal,
}

/// Stores and manages clipboard content.
#[derive(Debug, Clone)]
pub struct ClipboardManager {
    /// Content for each clipboard type.
    buffers: [String; 3],
    /// Maximum buffer size in bytes (0 = unlimited).
    max_size: usize,
    /// Whether OSC 52 clipboard integration is enabled.
    osc52_enabled: bool,
    /// Last operation for undo tracking.
    last_operation: Option<ClipboardOperation>,
}

/// Records a clipboard operation for potential undo.
#[derive(Debug, Clone)]
struct ClipboardOperation {
    clipboard_type: ClipboardType,
    previous_content: String,
}

impl Default for ClipboardManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipboardManager {
    /// Creates a new ClipboardManager.
    pub fn new() -> Self {
        Self {
            buffers: [String::new(), String::new(), String::new()],
            max_size: 0,
            osc52_enabled: false,
            last_operation: None,
        }
    }

    /// Sets the maximum buffer size in bytes.
    pub fn with_max_size(mut self, max: usize) -> Self {
        self.max_size = max;
        self
    }

    /// Enables or disables OSC 52 clipboard integration.
    pub fn with_osc52(mut self, enabled: bool) -> Self {
        self.osc52_enabled = enabled;
        self
    }

    /// Copies text to the specified clipboard.
    pub fn copy(&mut self, clipboard_type: ClipboardType, text: &str) {
        let idx = clipboard_type_index(clipboard_type);
        self.last_operation = Some(ClipboardOperation {
            clipboard_type,
            previous_content: self.buffers[idx].clone(),
        });
        self.buffers[idx] = truncate_to_size(text, self.max_size);
    }

    /// Pastes text from the specified clipboard.
    pub fn paste(&self, clipboard_type: ClipboardType) -> &str {
        &self.buffers[clipboard_type_index(clipboard_type)]
    }

    /// Clears the specified clipboard.
    pub fn clear(&mut self, clipboard_type: ClipboardType) {
        let idx = clipboard_type_index(clipboard_type);
        self.last_operation = Some(ClipboardOperation {
            clipboard_type,
            previous_content: self.buffers[idx].clone(),
        });
        self.buffers[idx].clear();
    }

    /// Clears all clipboards.
    pub fn clear_all(&mut self) {
        for buf in &mut self.buffers {
            buf.clear();
        }
        self.last_operation = None;
    }

    /// Undoes the last clipboard operation.
    pub fn undo(&mut self) -> bool {
        if let Some(op) = self.last_operation.take() {
            let idx = clipboard_type_index(op.clipboard_type);
            self.buffers[idx] = op.previous_content;
            true
        } else {
            false
        }
    }

    /// Returns whether the specified clipboard is empty.
    pub fn is_empty(&self, clipboard_type: ClipboardType) -> bool {
        self.buffers[clipboard_type_index(clipboard_type)].is_empty()
    }

    /// Returns the content length of the specified clipboard.
    pub fn len(&self, clipboard_type: ClipboardType) -> usize {
        self.buffers[clipboard_type_index(clipboard_type)].len()
    }

    /// Returns whether OSC 52 is enabled.
    pub fn osc52_enabled(&self) -> bool {
        self.osc52_enabled
    }

    /// Generates an OSC 52 escape sequence for setting clipboard content.
    pub fn osc52_set(&self, clipboard_type: ClipboardType, text: &str) -> Option<String> {
        if !self.osc52_enabled {
            return None;
        }
        let b64 = base64_encode(text);
        let selection = match clipboard_type {
            ClipboardType::System => "c",
            ClipboardType::Selection => "p",
            ClipboardType::Internal => "q",
        };
        Some(format!("\x1b]52;{selection};{b64}\x07"))
    }

    /// Parses an OSC 52 escape sequence to extract clipboard content.
    pub fn osc52_parse(&self, osc_data: &str) -> Option<(ClipboardType, String)> {
        let data = osc_data.strip_prefix("52;")?;
        let (selection, rest) = data.split_once(';')?;
        let clipboard_type = match selection {
            "c" | "s" => ClipboardType::System,
            "p" => ClipboardType::Selection,
            _ => ClipboardType::Internal,
        };
        let decoded = base64_decode(rest)?;
        Some((clipboard_type, decoded))
    }
}

fn clipboard_type_index(ct: ClipboardType) -> usize {
    match ct {
        ClipboardType::System => 0,
        ClipboardType::Selection => 1,
        ClipboardType::Internal => 2,
    }
}

fn truncate_to_size(text: &str, max: usize) -> String {
    if max == 0 || text.len() <= max {
        text.to_string()
    } else {
        text[..max].to_string()
    }
}

/// Simple base64 encoding for OSC 52.
fn base64_encode(input: &str) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes();
    let mut result = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

/// Simple base64 decoding for OSC 52.
fn base64_decode(input: &str) -> Option<String> {
    let input = input.trim_end_matches('=');
    if input.is_empty() {
        return Some(String::new());
    }

    let mut result = Vec::new();
    let chars: Vec<u8> = input
        .bytes()
        .map(|b| match b {
            b'A'..=b'Z' => Some(b - b'A'),
            b'a'..=b'z' => Some(b - b'a' + 26),
            b'0'..=b'9' => Some(b - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        })
        .collect::<Option<Vec<_>>>()?;

    for chunk in chars.chunks(4) {
        if chunk.len() < 2 {
            break;
        }
        let b0 = chunk[0] as u32;
        let b1 = chunk[1] as u32;
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let b3 = if chunk.len() > 3 { chunk[3] as u32 } else { 0 };

        let triple = (b0 << 18) | (b1 << 12) | (b2 << 6) | b3;
        result.push(((triple >> 16) & 0xFF) as u8);
        if chunk.len() > 2 {
            result.push(((triple >> 8) & 0xFF) as u8);
        }
        if chunk.len() > 3 {
            result.push((triple & 0xFF) as u8);
        }
    }

    String::from_utf8(result).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_and_paste() {
        let mut mgr = ClipboardManager::new();
        mgr.copy(ClipboardType::System, "hello");
        assert_eq!(mgr.paste(ClipboardType::System), "hello");
    }

    #[test]
    fn separate_buffers() {
        let mut mgr = ClipboardManager::new();
        mgr.copy(ClipboardType::System, "system");
        mgr.copy(ClipboardType::Selection, "selection");
        assert_eq!(mgr.paste(ClipboardType::System), "system");
        assert_eq!(mgr.paste(ClipboardType::Selection), "selection");
    }

    #[test]
    fn clear() {
        let mut mgr = ClipboardManager::new();
        mgr.copy(ClipboardType::System, "test");
        mgr.clear(ClipboardType::System);
        assert!(mgr.is_empty(ClipboardType::System));
    }

    #[test]
    fn clear_all() {
        let mut mgr = ClipboardManager::new();
        mgr.copy(ClipboardType::System, "a");
        mgr.copy(ClipboardType::Selection, "b");
        mgr.clear_all();
        assert!(mgr.is_empty(ClipboardType::System));
        assert!(mgr.is_empty(ClipboardType::Selection));
    }

    #[test]
    fn undo() {
        let mut mgr = ClipboardManager::new();
        mgr.copy(ClipboardType::System, "first");
        mgr.copy(ClipboardType::System, "second");
        assert_eq!(mgr.paste(ClipboardType::System), "second");
        assert!(mgr.undo());
        assert_eq!(mgr.paste(ClipboardType::System), "first");
    }

    #[test]
    fn undo_empty() {
        let mut mgr = ClipboardManager::new();
        assert!(!mgr.undo());
    }

    #[test]
    fn max_size() {
        let mut mgr = ClipboardManager::new().with_max_size(5);
        mgr.copy(ClipboardType::System, "hello world");
        assert_eq!(mgr.paste(ClipboardType::System), "hello");
    }

    #[test]
    fn base64_roundtrip() {
        let original = "Hello, World!";
        let encoded = base64_encode(original);
        let decoded = base64_decode(&encoded);
        assert_eq!(decoded.as_deref(), Some(original));
    }

    #[test]
    fn osc52_generate() {
        let mgr = ClipboardManager::new().with_osc52(true);
        let seq = mgr.osc52_set(ClipboardType::System, "test");
        assert!(seq.is_some());
        let seq = seq.unwrap();
        assert!(seq.starts_with("\x1b]52;c;"));
        assert!(seq.ends_with("\x07"));
    }

    #[test]
    fn osc52_disabled() {
        let mgr = ClipboardManager::new().with_osc52(false);
        assert!(mgr.osc52_set(ClipboardType::System, "test").is_none());
    }

    #[test]
    fn len_tracking() {
        let mut mgr = ClipboardManager::new();
        assert_eq!(mgr.len(ClipboardType::System), 0);
        mgr.copy(ClipboardType::System, "hello");
        assert_eq!(mgr.len(ClipboardType::System), 5);
    }
}
