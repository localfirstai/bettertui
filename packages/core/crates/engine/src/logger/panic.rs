//! Panic hook integration for logging panics with full context.
//!
//! This module installs a custom panic hook that logs panics with full
//! stack traces and context information.

use std::panic;

/// Install a custom panic hook that logs panics.
pub fn install_panic_hook() {
    let default_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        let payload = panic_info.payload();

        let message = if let Some(s) = payload.downcast_ref::<&str>() {
            *s
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s.as_str()
        } else {
            "Unknown panic payload"
        };

        let location = if let Some(loc) = panic_info.location() {
            format!("{}:{}:{}", loc.file(), loc.line(), loc.column())
        } else {
            "unknown location".to_string()
        };

        tracing::error!(message = message, location = location, "PANIC occurred");

        // Call the default hook for standard panic output
        default_hook(panic_info);
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install_hook_does_not_panic() {
        install_panic_hook();
    }

    // Note: Testing actual panic behavior is tricky and requires
    // catching panics in isolated threads. The hook installation
    // itself is tested above.
}
