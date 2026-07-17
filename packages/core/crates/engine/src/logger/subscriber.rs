//! Custom tracing subscriber for the logger.
//!
//! This module sets up the tracing-subscriber with the appropriate configuration
//! for terminal output and optional file logging.
//!
//! # File logging
//!
//! In development (`config.dev == true`) or when an explicit `file` path /
//! `BETTERTUI_LOG_DIR` is given, log records are also written to a daily file
//! (`bettertui-YYYY-MM-DD.log`). The daily-file + path-validation logic is ported
//! from the original standalone `bettertui-logger` crate. In production with no
//! path configured, file logging stays disabled.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;

use crate::logger::{LoggerConfig, LoggerError};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Repo-root `logs/` directory used as the default in dev mode.
///
/// Anchored at the engine crate's manifest dir at build time:
/// `packages/core/crates/engine` → up four → repo root → `logs`.
pub fn default_dev_log_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .map(|root| root.join("logs"))
        .unwrap_or_else(|| PathBuf::from("logs"))
}

/// Compute today's date as `YYYY-MM-DD` from the system clock without pulling in
/// a date/time crate (ported from the original logger crate).
fn today_date() -> String {
    let dur = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
    let secs = dur.as_secs();
    let days = secs / 86400;
    let mut y = 1970u64;
    let mut d = days;
    loop {
        let year_days = if is_leap(y) { 366 } else { 365 };
        if d < year_days {
            break;
        }
        d -= year_days;
        y += 1;
    }
    let month_days = if is_leap(y) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut m = 0u64;
    for (i, &md) in month_days.iter().enumerate() {
        if d < md {
            m = i as u64;
            break;
        }
        d -= md;
    }
    format!("{:04}-{:02}-{:02}", y, m + 1, d + 1)
}

fn is_leap(year: u64) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

/// Validate a log path for the current platform (ported from the original crate).
fn platform_validate_path(path: &Path) -> Result<(), String> {
    let path_str = path.to_str().ok_or("Log path contains invalid characters")?;

    if path_str.is_empty() {
        return Err("Log path must not be empty".into());
    }

    #[cfg(target_os = "windows")]
    {
        let invalid_chars = ['<', '>', '"', '|', '?', '*'];
        for c in invalid_chars {
            if path_str.contains(c) {
                return Err(format!("Invalid character '{}' in log path", c));
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        if path_str.contains('\0') {
            return Err("Log path contains null character".into());
        }
    }

    Ok(())
}

/// `<dir>/bettertui-YYYY-MM-DD.log`
pub fn daily_log_path(log_dir: &Path) -> PathBuf {
    log_dir.join(format!("bettertui-{}.log", today_date()))
}

/// Open the log file, appending if it was last modified today, otherwise starting
/// fresh for the new day. Creates parent directories as needed. Ported from the
/// original crate.
fn open_or_truncate(path: &Path) -> Result<fs::File, String> {
    if path.exists() {
        let metadata = fs::metadata(path).map_err(|e| format!("Failed to get file metadata: {}", e))?;
        let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let modified_secs = modified.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_secs();
        let now_secs = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_secs();

        if modified_secs / 86400 == now_secs / 86400 {
            return fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .map_err(|e| format!("Failed to open log file: {}", e));
        }

        let _ = fs::remove_file(path);
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create log directory: {}", e))?;
    }

    fs::File::create(path).map_err(|e| format!("Failed to create log file: {}", e))
}

/// Try to open the resolved file-log target, returning an open handle or `None`
/// (after emitting a warning) if it can't be created. Never panics.
fn try_open_log_file(config: &LoggerConfig) -> Option<fs::File> {
    let target = config.resolve_file_target()?;

    if let Err(e) = platform_validate_path(&target.file) {
        eprintln!("[logger] Invalid log path {:?}, file logging disabled: {}", target.file, e);
        return None;
    }

    if let Err(e) = fs::create_dir_all(&target.dir) {
        eprintln!("[logger] Failed to create log dir {:?}, file logging disabled: {}", target.dir, e);
        return None;
    }

    match open_or_truncate(&target.file) {
        Ok(file) => {
            eprintln!("[logger] Writing logs to: {}", target.file.display());
            Some(file)
        }
        Err(e) => {
            eprintln!("[logger] Failed to open log file, file logging disabled: {}", e);
            None
        }
    }
}

/// Initialize the tracing subscriber with the given configuration.
///
/// Always installs a terminal (stderr) layer. Additionally installs a file layer
/// when [`LoggerConfig::resolve_file_target`] yields a writable destination.
pub fn init_subscriber(config: &LoggerConfig) -> Result<(), LoggerError> {
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(config.level.as_str()))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let terminal_layer = tracing_subscriber::fmt::layer()
        .with_ansi(config.color)
        .with_target(config.module)
        .with_thread_ids(config.thread)
        .with_thread_names(config.thread)
        .with_file(true)
        .with_line_number(true);

    // Optional file layer (never colored; always shows target + location).
    let file_layer = try_open_log_file(config).map(|file| {
        tracing_subscriber::fmt::layer()
            .with_writer(Mutex::new(file))
            .with_ansi(false)
            .with_target(true)
            .with_thread_ids(config.thread)
            .with_thread_names(config.thread)
            .with_file(true)
            .with_line_number(true)
    });

    tracing_subscriber::Registry::default().with(env_filter).with(terminal_layer).with(file_layer).init();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_years() {
        assert!(is_leap(2020));
        assert!(is_leap(2000));
        assert!(is_leap(2400));
        assert!(!is_leap(2021));
        assert!(!is_leap(1900));
        assert!(!is_leap(2100));
    }

    #[test]
    fn today_date_format() {
        let date = today_date();
        assert_eq!(date.len(), 10, "date should be YYYY-MM-DD");
        let parts: Vec<&str> = date.split('-').collect();
        assert_eq!(parts.len(), 3);
        let year: u64 = parts[0].parse().unwrap();
        let month: u64 = parts[1].parse().unwrap();
        let day: u64 = parts[2].parse().unwrap();
        assert!(year >= 2020);
        assert!((1..=12).contains(&month));
        assert!((1..=31).contains(&day));
    }

    #[test]
    fn validate_path_rejects_empty() {
        assert!(platform_validate_path(Path::new("")).is_err());
    }

    #[test]
    fn validate_path_accepts_absolute() {
        assert!(platform_validate_path(Path::new("/tmp/bettertui/app.log")).is_ok());
    }

    #[test]
    fn daily_log_path_shape() {
        let dir = std::env::temp_dir().join("bettertui_sub_test");
        let path = daily_log_path(&dir);
        assert!(path.starts_with(&dir));
        let name = path.file_name().unwrap().to_str().unwrap();
        assert!(name.starts_with("bettertui-"));
        assert!(name.ends_with(".log"));
    }

    #[test]
    fn default_dev_log_dir_ends_with_logs() {
        let dir = default_dev_log_dir();
        assert!(dir.ends_with("logs"), "got {:?}", dir);
    }

    #[test]
    fn open_or_truncate_creates_and_appends() {
        let dir = std::env::temp_dir().join(format!("btui_sub_{}", std::process::id()));
        let path = dir.join("t.log");
        let _ = fs::remove_dir_all(&dir);

        {
            use std::io::Write;
            let mut f = open_or_truncate(&path).expect("create");
            writeln!(f, "line one").unwrap();
        }
        {
            use std::io::Write;
            let mut f = open_or_truncate(&path).expect("append same day");
            writeln!(f, "line two").unwrap();
        }
        let contents = fs::read_to_string(&path).unwrap();
        assert!(contents.contains("line one"));
        assert!(contents.contains("line two"));
        let _ = fs::remove_dir_all(&dir);
    }
}
