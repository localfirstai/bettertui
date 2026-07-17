//! Custom tracing subscriber for the logger.
//!
//! This module sets up the tracing-subscriber with the appropriate configuration
//! for terminal output and optional file logging.

use crate::logger::{LoggerConfig, LoggerError};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Initialize the tracing subscriber with the given configuration.
pub fn init_subscriber(config: &LoggerConfig) -> Result<(), LoggerError> {
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(config.level.as_str()))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(config.color)
        .with_target(config.module)
        .with_thread_ids(config.thread)
        .with_thread_names(config.thread)
        .with_file(true)
        .with_line_number(true);

    let subscriber = tracing_subscriber::Registry::default().with(env_filter).with(fmt_layer);

    subscriber.init();

    Ok(())
}
