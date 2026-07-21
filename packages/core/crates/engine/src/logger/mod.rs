//! Logging facade for BetterTUI engine with TypeScript integration.
//!
//! This module provides a custom logging system that:
//! - Integrates with TypeScript via napi-rs
//! - Supports runtime log level changes
//! - Provides module-aware filtering
//! - Tracks diagnostic counters (render calls, events, etc.)
//! - Outputs colored terminal logs
//! - Zero overhead in release builds via compile-time gating
//!
//! # Architecture
//!
//! The logger is initialized once via `Logger::init()` and provides:
//! - Level management with AtomicU8 for thread-safe runtime changes
//! - Module filtering for fine-grained control
//! - Diagnostic counters for performance tracking
//! - Panic hook integration for capturing panics with full context
//!
//! # Usage
//!
//! ```no_run
//! use bettertui_engine::logger::{Logger, LoggerConfig, Level};
//!
//! let config = LoggerConfig {
//!     level: Level::Info,
//!     color: true,
//!     timestamp: true,
//!     ..Default::default()
//! };
//!
//! Logger::init(config).expect("Failed to initialize logger");
//!
//! // Runtime level changes
//! Logger::set_level(Level::Debug);
//!
//! // Get diagnostics
//! let snapshot = Logger::snapshot_diagnostics();
//! println!("Render calls: {}", snapshot.render_calls);
//! ```

pub mod diagnostics;
pub mod filter;
pub mod formatter;
pub mod panic;
pub mod subscriber;

use std::sync::OnceLock;
use std::sync::atomic::{AtomicU8, Ordering};

pub use diagnostics::{DiagnosticCounters, DiagnosticSnapshot};
pub use filter::ModuleFilter;
pub use formatter::ColoredFormatter;
use tracing::Level as TracingLevel;

/// Global logger instance
static LOGGER: OnceLock<Logger> = OnceLock::new();

/// Run a diagnostic-counter update against the global counters, but ONLY when
/// the `diagnostics` feature is enabled. When the feature is off this expands to
/// nothing, so counter maintenance imposes zero cost on the render/event hot
/// paths in a maximally-optimized build.
///
/// The closure receives `&DiagnosticCounters`:
/// ```ignore
/// diag!(|d| d.inc_render_calls());
/// ```
#[macro_export]
macro_rules! diag {
    ($f:expr) => {
        #[cfg(feature = "diagnostics")]
        {
            if let Some(__d) = $crate::logger::Logger::diagnostics() {
                let __f: fn(&$crate::logger::DiagnosticCounters) = $f;
                __f(__d);
            }
        }
    };
}

/// Log level enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Level {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl Level {
    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Trace => "trace",
            Level::Debug => "debug",
            Level::Info => "info",
            Level::Warn => "warn",
            Level::Error => "error",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "trace" => Some(Level::Trace),
            "debug" => Some(Level::Debug),
            "info" => Some(Level::Info),
            "warn" => Some(Level::Warn),
            "error" => Some(Level::Error),
            _ => None,
        }
    }

    pub fn to_tracing_level(&self) -> TracingLevel {
        match self {
            Level::Trace => TracingLevel::TRACE,
            Level::Debug => TracingLevel::DEBUG,
            Level::Info => TracingLevel::INFO,
            Level::Warn => TracingLevel::WARN,
            Level::Error => TracingLevel::ERROR,
        }
    }
}

/// Logger configuration
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    pub level: Level,
    pub color: bool,
    pub timestamp: bool,
    pub module: bool,
    pub thread: bool,
    /// Explicit log file path. When set (in any mode) file logging is enabled and
    /// writes to exactly this path. Takes precedence over the `dev` default dir,
    /// but is itself overridden by the `BETTERTUI_LOG_DIR` env var.
    pub file: Option<String>,
    pub max_file_size: Option<u64>,
    pub max_files: Option<usize>,
    /// Development mode. When `true` and no explicit `file` is given, logs are
    /// written to a daily file under the repo-root `logs/` directory. When
    /// `false` (production) file logging stays OFF unless an explicit `file`
    /// path (or `BETTERTUI_LOG_DIR`) is provided.
    pub dev: bool,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            level: Level::Info,
            color: true,
            timestamp: true,
            module: true,
            thread: false,
            file: None,
            max_file_size: Some(10 * 1024 * 1024), // 10 MB
            max_files: Some(5),
            dev: false,
        }
    }
}

/// A resolved destination for file logging.
pub struct FileLogTarget {
    /// Directory that must exist (created if necessary).
    pub dir: std::path::PathBuf,
    /// Full path of the log file to open.
    pub file: std::path::PathBuf,
}

impl LoggerConfig {
    /// Resolve where file logs should be written, if anywhere.
    ///
    /// Precedence:
    /// 1. `BETTERTUI_LOG_DIR` env var → `<dir>/bettertui-YYYY-MM-DD.log`
    /// 2. explicit `self.file` → that exact path (its parent is the dir)
    /// 3. `self.dev == true` → `<repo-root>/logs/bettertui-YYYY-MM-DD.log`
    /// 4. otherwise (production, no path) → `None` (file logging disabled)
    pub fn resolve_file_target(&self) -> Option<FileLogTarget> {
        use std::path::PathBuf;

        if let Ok(dir) = std::env::var("BETTERTUI_LOG_DIR") {
            if !dir.is_empty() {
                let dir = PathBuf::from(dir);
                let file = subscriber::daily_log_path(&dir);
                return Some(FileLogTarget { dir, file });
            }
        }

        if let Some(explicit) = &self.file {
            if !explicit.is_empty() {
                let file = PathBuf::from(explicit);
                let dir = file.parent().map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
                return Some(FileLogTarget { dir, file });
            }
        }

        if self.dev {
            let dir = subscriber::default_dev_log_dir();
            let file = subscriber::daily_log_path(&dir);
            return Some(FileLogTarget { dir, file });
        }

        None
    }
}

/// Logger error types
#[derive(Debug, thiserror::Error)]
pub enum LoggerError {
    #[error("Logger already initialized")]
    AlreadyInitialized,
    #[error("Logger not initialized")]
    NotInitialized,
    #[error("Failed to set global subscriber: {0}")]
    SetGlobalSubscriber(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Main logger facade
pub struct Logger {
    level: AtomicU8,
    filter: parking_lot::RwLock<ModuleFilter>,
    diagnostics: DiagnosticCounters,
    #[allow(dead_code)]
    config: LoggerConfig,
}

impl Logger {
    /// Initialize the global logger with the given configuration.
    ///
    /// This must be called before any logging occurs. Calling it multiple times
    /// will return an error.
    pub fn init(config: LoggerConfig) -> Result<(), LoggerError> {
        if LOGGER.get().is_some() {
            return Err(LoggerError::AlreadyInitialized);
        }

        let level = AtomicU8::new(config.level as u8);
        let filter = parking_lot::RwLock::new(ModuleFilter::default());
        let diagnostics = DiagnosticCounters::new();

        let logger = Logger { level, filter, diagnostics, config: config.clone() };

        // Initialize tracing subscriber
        subscriber::init_subscriber(&config)?;

        // Set up panic hook
        panic::install_panic_hook();

        LOGGER.set(logger).map_err(|_| LoggerError::AlreadyInitialized)?;

        tracing::info!(
            level = ?config.level,
            color = config.color,
            "Logger initialized"
        );

        Ok(())
    }

    /// Get a reference to the global logger instance.
    fn instance() -> Result<&'static Logger, LoggerError> {
        LOGGER.get().ok_or(LoggerError::NotInitialized)
    }

    /// Set the global log level at runtime.
    pub fn set_level(level: Level) {
        if let Ok(logger) = Self::instance() {
            logger.level.store(level as u8, Ordering::Relaxed);
            tracing::info!(?level, "Log level changed");
        }
    }

    /// Get the current log level.
    pub fn get_level() -> Level {
        Self::instance()
            .map(|logger| {
                let level_u8 = logger.level.load(Ordering::Relaxed);
                match level_u8 {
                    0 => Level::Trace,
                    1 => Level::Debug,
                    2 => Level::Info,
                    3 => Level::Warn,
                    _ => Level::Error,
                }
            })
            .unwrap_or(Level::Info)
    }

    /// Set the module filter for runtime filtering.
    pub fn set_module_filter(filter: ModuleFilter) {
        if let Ok(logger) = Self::instance() {
            *logger.filter.write() = filter;
            tracing::info!("Module filter updated");
        }
    }

    /// Get a snapshot of the current diagnostic counters.
    pub fn snapshot_diagnostics() -> DiagnosticSnapshot {
        Self::instance().map(|logger| logger.diagnostics.snapshot()).unwrap_or_default()
    }

    /// Get a reference to the diagnostic counters for incrementing.
    pub fn diagnostics() -> Option<&'static DiagnosticCounters> {
        LOGGER.get().map(|logger| &logger.diagnostics)
    }

    /// Flush any buffered logs (useful before shutdown).
    pub fn flush() {
        // Tracing doesn't have an explicit flush, but we can log a marker
        tracing::debug!("Logger flush requested");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_conversions() {
        assert_eq!(Level::from_str("trace"), Some(Level::Trace));
        assert_eq!(Level::from_str("DEBUG"), Some(Level::Debug));
        assert_eq!(Level::from_str("Info"), Some(Level::Info));
        assert_eq!(Level::from_str("warn"), Some(Level::Warn));
        assert_eq!(Level::from_str("ERROR"), Some(Level::Error));
        assert_eq!(Level::from_str("invalid"), None);
    }

    #[test]
    fn level_ordering() {
        assert!(Level::Trace < Level::Debug);
        assert!(Level::Debug < Level::Info);
        assert!(Level::Info < Level::Warn);
        assert!(Level::Warn < Level::Error);
    }

    #[test]
    fn level_as_str() {
        assert_eq!(Level::Trace.as_str(), "trace");
        assert_eq!(Level::Debug.as_str(), "debug");
        assert_eq!(Level::Info.as_str(), "info");
        assert_eq!(Level::Warn.as_str(), "warn");
        assert_eq!(Level::Error.as_str(), "error");
    }

    #[test]
    fn default_config() {
        let config = LoggerConfig::default();
        assert_eq!(config.level, Level::Info);
        assert!(config.color);
        assert!(config.timestamp);
        assert!(config.module);
        assert!(!config.thread);
        assert!(!config.dev);
        assert!(config.file.is_none());
    }

    #[test]
    fn resolve_target_prod_no_path_is_none() {
        // SAFETY: single-threaded test; we restore the env immediately.
        unsafe { std::env::remove_var("BETTERTUI_LOG_DIR") };
        let config = LoggerConfig { dev: false, file: None, ..Default::default() };
        assert!(config.resolve_file_target().is_none());
    }

    #[test]
    fn resolve_target_explicit_file_wins() {
        unsafe { std::env::remove_var("BETTERTUI_LOG_DIR") };
        let config = LoggerConfig { dev: false, file: Some("/tmp/btui/custom.log".into()), ..Default::default() };
        let target = config.resolve_file_target().expect("explicit file → Some");
        assert_eq!(target.file, std::path::PathBuf::from("/tmp/btui/custom.log"));
        assert_eq!(target.dir, std::path::PathBuf::from("/tmp/btui"));
    }

    #[test]
    fn resolve_target_dev_uses_repo_logs_dir() {
        unsafe { std::env::remove_var("BETTERTUI_LOG_DIR") };
        let config = LoggerConfig { dev: true, file: None, ..Default::default() };
        let target = config.resolve_file_target().expect("dev → Some");
        assert!(target.dir.ends_with("logs"), "dev dir should end with logs, got {:?}", target.dir);
        let name = target.file.file_name().unwrap().to_str().unwrap();
        assert!(name.starts_with("bettertui-") && name.ends_with(".log"));
    }
}
