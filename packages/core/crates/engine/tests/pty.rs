use std::io::Cursor;

use bettertui_engine::pty::{PtyConfig, PtyError, PtyReader, PtyRuntime, PtySize, PtyWriter};

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
fn pty_size_with_pixel() {
    let size = PtySize::new(80, 24).with_pixel_size(800, 600);
    assert_eq!(size.pixel_width, 800);
    assert_eq!(size.pixel_height, 600);
}

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
fn pty_reader_read_from_and_read_line() {
    let mut reader = PtyReader::new();
    let mut cursor = Cursor::new(b"hello\nworld\n");
    reader.read_from(&mut cursor).unwrap();
    assert_eq!(reader.read_line(), Some("hello\n".to_string()));
    assert_eq!(reader.read_line(), Some("world\n".to_string()));
    assert_eq!(reader.read_line(), None);
}

#[test]
fn pty_reader_read_bytes() {
    let mut reader = PtyReader::new();
    let mut cursor = Cursor::new(b"hello world");
    reader.read_from(&mut cursor).unwrap();
    let bytes = reader.read_bytes(5);
    assert_eq!(bytes, b"hello");
    assert_eq!(reader.available(), 6);
}

#[test]
fn pty_reader_clear() {
    let mut reader = PtyReader::new();
    let mut cursor = Cursor::new(b"hello");
    reader.read_from(&mut cursor).unwrap();
    assert!(!reader.is_empty());
    reader.clear();
    assert!(reader.is_empty());
}

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
