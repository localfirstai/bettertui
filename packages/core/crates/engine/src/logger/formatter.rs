//! Colored terminal output formatter for logs.
//!
//! This module provides ANSI color formatting for log output, making logs
//! more readable in terminal environments.

use crate::logger::Level;
use std::fmt;

/// ANSI color codes
pub mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";

    pub const BLACK: &str = "\x1b[30m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";

    pub const BRIGHT_BLACK: &str = "\x1b[90m";
    pub const BRIGHT_RED: &str = "\x1b[91m";
    pub const BRIGHT_GREEN: &str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &str = "\x1b[93m";
    pub const BRIGHT_BLUE: &str = "\x1b[94m";
    pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
    pub const BRIGHT_CYAN: &str = "\x1b[96m";
    pub const BRIGHT_WHITE: &str = "\x1b[97m";
}

/// Colored formatter for log output.
#[derive(Debug, Clone)]
pub struct ColoredFormatter {
    pub use_color: bool,
    pub show_timestamp: bool,
    pub show_module: bool,
    pub show_thread: bool,
}

impl ColoredFormatter {
    pub fn new() -> Self {
        Self { use_color: true, show_timestamp: true, show_module: true, show_thread: false }
    }

    pub fn with_color(mut self, use_color: bool) -> Self {
        self.use_color = use_color;
        self
    }

    pub fn with_timestamp(mut self, show_timestamp: bool) -> Self {
        self.show_timestamp = show_timestamp;
        self
    }

    pub fn with_module(mut self, show_module: bool) -> Self {
        self.show_module = show_module;
        self
    }

    pub fn with_thread(mut self, show_thread: bool) -> Self {
        self.show_thread = show_thread;
        self
    }

    /// Get the color code for a log level.
    pub fn level_color(&self, level: Level) -> &'static str {
        if !self.use_color {
            return "";
        }

        match level {
            Level::Trace => colors::BRIGHT_BLACK,
            Level::Debug => colors::BLUE,
            Level::Info => colors::GREEN,
            Level::Warn => colors::YELLOW,
            Level::Error => colors::RED,
        }
    }

    /// Get the color for module names.
    pub fn module_color(&self) -> &'static str {
        if !self.use_color {
            return "";
        }
        colors::CYAN
    }

    /// Get the color for timestamps.
    pub fn timestamp_color(&self) -> &'static str {
        if !self.use_color {
            return "";
        }
        colors::DIM
    }

    /// Get the color for thread IDs.
    pub fn thread_color(&self) -> &'static str {
        if !self.use_color {
            return "";
        }
        colors::BRIGHT_BLACK
    }

    /// Get the reset code.
    pub fn reset(&self) -> &'static str {
        if !self.use_color {
            return "";
        }
        colors::RESET
    }

    /// Format a level badge (e.g., "[INFO]", "[ERROR]").
    pub fn format_level_badge(&self, level: Level) -> String {
        let color = self.level_color(level);
        let reset = self.reset();
        let level_str = match level {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info => "INFO ",
            Level::Warn => "WARN ",
            Level::Error => "ERROR",
        };

        format!("{}{}{}", color, level_str, reset)
    }

    /// Format a module name.
    pub fn format_module(&self, module: &str) -> String {
        if !self.show_module {
            return String::new();
        }

        let color = self.module_color();
        let reset = self.reset();
        format!("{}[{}]{}", color, module, reset)
    }

    /// Format a timestamp.
    pub fn format_timestamp(&self, timestamp: &str) -> String {
        if !self.show_timestamp {
            return String::new();
        }

        let color = self.timestamp_color();
        let reset = self.reset();
        format!("{}{}{}", color, timestamp, reset)
    }

    /// Format a thread ID.
    pub fn format_thread(&self, thread_id: &str) -> String {
        if !self.show_thread {
            return String::new();
        }

        let color = self.thread_color();
        let reset = self.reset();
        format!("{}[{}]{}", color, thread_id, reset)
    }
}

impl Default for ColoredFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ColoredFormatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ColoredFormatter {{ color: {}, timestamp: {}, module: {}, thread: {} }}",
            self.use_color, self.show_timestamp, self.show_module, self.show_thread
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_formatter() {
        let formatter = ColoredFormatter::default();
        assert!(formatter.use_color);
        assert!(formatter.show_timestamp);
        assert!(formatter.show_module);
        assert!(!formatter.show_thread);
    }

    #[test]
    fn level_colors_with_color() {
        let formatter = ColoredFormatter::new();

        assert_eq!(formatter.level_color(Level::Trace), colors::BRIGHT_BLACK);
        assert_eq!(formatter.level_color(Level::Debug), colors::BLUE);
        assert_eq!(formatter.level_color(Level::Info), colors::GREEN);
        assert_eq!(formatter.level_color(Level::Warn), colors::YELLOW);
        assert_eq!(formatter.level_color(Level::Error), colors::RED);
    }

    #[test]
    fn level_colors_without_color() {
        let formatter = ColoredFormatter::new().with_color(false);

        assert_eq!(formatter.level_color(Level::Info), "");
        assert_eq!(formatter.level_color(Level::Error), "");
    }

    #[test]
    fn format_level_badge_with_color() {
        let formatter = ColoredFormatter::new();
        let badge = formatter.format_level_badge(Level::Info);

        assert!(badge.contains("INFO"));
        assert!(badge.contains(colors::GREEN));
        assert!(badge.contains(colors::RESET));
    }

    #[test]
    fn format_level_badge_without_color() {
        let formatter = ColoredFormatter::new().with_color(false);
        let badge = formatter.format_level_badge(Level::Info);

        assert_eq!(badge, "INFO ");
    }

    #[test]
    fn format_module_with_show() {
        let formatter = ColoredFormatter::new().with_module(true);
        let module = formatter.format_module("bettertui_engine::render");

        assert!(module.contains("bettertui_engine::render"));
        assert!(module.contains(colors::CYAN));
    }

    #[test]
    fn format_module_without_show() {
        let formatter = ColoredFormatter::new().with_module(false);
        let module = formatter.format_module("bettertui_engine::render");

        assert_eq!(module, "");
    }

    #[test]
    fn format_timestamp_with_show() {
        let formatter = ColoredFormatter::new().with_timestamp(true);
        let timestamp = formatter.format_timestamp("2024-01-01 12:00:00");

        assert!(timestamp.contains("2024-01-01 12:00:00"));
    }

    #[test]
    fn format_timestamp_without_show() {
        let formatter = ColoredFormatter::new().with_timestamp(false);
        let timestamp = formatter.format_timestamp("2024-01-01 12:00:00");

        assert_eq!(timestamp, "");
    }

    #[test]
    fn builder_pattern() {
        let formatter =
            ColoredFormatter::new().with_color(false).with_timestamp(false).with_module(false).with_thread(true);

        assert!(!formatter.use_color);
        assert!(!formatter.show_timestamp);
        assert!(!formatter.show_module);
        assert!(formatter.show_thread);
    }
}
