//! Diagnostic counters for tracking engine performance metrics.
//!
//! This module provides atomic counters for tracking various engine operations:
//! - Render calls and bytes
//! - Event dispatches
//! - Layout computations
//! - Cache hits/misses
//! - Memory allocations
//! - Frame timing
//!
//! All counters are thread-safe and can be incremented from any thread.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use parking_lot::Mutex;

/// Diagnostic counters for engine performance tracking.
#[derive(Debug)]
pub struct DiagnosticCounters {
    pub(crate) render_calls: AtomicU64,
    pub(crate) render_bytes: AtomicU64,
    pub(crate) event_dispatches: AtomicU64,
    pub(crate) layout_computations: AtomicU64,
    pub(crate) cache_hits: AtomicU64,
    pub(crate) cache_misses: AtomicU64,
    pub(crate) allocations: AtomicU64,
    pub(crate) frame_times: Mutex<VecDeque<Duration>>,
}

impl DiagnosticCounters {
    const MAX_FRAME_SAMPLES: usize = 60; // Keep last 60 frames

    pub fn new() -> Self {
        Self {
            render_calls: AtomicU64::new(0),
            render_bytes: AtomicU64::new(0),
            event_dispatches: AtomicU64::new(0),
            layout_computations: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            allocations: AtomicU64::new(0),
            frame_times: Mutex::new(VecDeque::with_capacity(Self::MAX_FRAME_SAMPLES)),
        }
    }

    /// Increment render call counter
    pub fn inc_render_calls(&self) {
        self.render_calls.fetch_add(1, Ordering::Relaxed);
    }

    /// Add bytes to render counter
    pub fn add_render_bytes(&self, bytes: u64) {
        self.render_bytes.fetch_add(bytes, Ordering::Relaxed);
    }

    /// Increment event dispatch counter
    pub fn inc_event_dispatches(&self) {
        self.event_dispatches.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment layout computation counter
    pub fn inc_layout_computations(&self) {
        self.layout_computations.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment cache hit counter
    pub fn inc_cache_hits(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment cache miss counter
    pub fn inc_cache_misses(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment allocation counter
    pub fn inc_allocations(&self) {
        self.allocations.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a frame time
    pub fn record_frame_time(&self, duration: Duration) {
        let mut frame_times = self.frame_times.lock();
        if frame_times.len() >= Self::MAX_FRAME_SAMPLES {
            frame_times.pop_front();
        }
        frame_times.push_back(duration);
    }

    /// Create a snapshot of current diagnostics
    pub fn snapshot(&self) -> DiagnosticSnapshot {
        let frame_times = self.frame_times.lock();
        let (average_frame_time, fps) = if frame_times.is_empty() {
            (0.0, 0.0)
        } else {
            let total: Duration = frame_times.iter().sum();
            let avg = total.as_secs_f64() / frame_times.len() as f64;
            let fps = if avg > 0.0 { 1.0 / avg } else { 0.0 };
            (avg * 1000.0, fps) // Convert to milliseconds
        };

        DiagnosticSnapshot {
            render_calls: self.render_calls.load(Ordering::Relaxed),
            render_bytes: self.render_bytes.load(Ordering::Relaxed),
            event_dispatches: self.event_dispatches.load(Ordering::Relaxed),
            layout_computations: self.layout_computations.load(Ordering::Relaxed),
            cache_hits: self.cache_hits.load(Ordering::Relaxed),
            cache_misses: self.cache_misses.load(Ordering::Relaxed),
            allocations: self.allocations.load(Ordering::Relaxed),
            average_frame_time,
            fps,
        }
    }

    /// Reset all counters
    pub fn reset(&self) {
        self.render_calls.store(0, Ordering::Relaxed);
        self.render_bytes.store(0, Ordering::Relaxed);
        self.event_dispatches.store(0, Ordering::Relaxed);
        self.layout_computations.store(0, Ordering::Relaxed);
        self.cache_hits.store(0, Ordering::Relaxed);
        self.cache_misses.store(0, Ordering::Relaxed);
        self.allocations.store(0, Ordering::Relaxed);
        self.frame_times.lock().clear();
    }
}

impl Default for DiagnosticCounters {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot of diagnostic counters at a point in time.
#[derive(Debug, Clone, Copy, Default)]
pub struct DiagnosticSnapshot {
    pub render_calls: u64,
    pub render_bytes: u64,
    pub event_dispatches: u64,
    pub layout_computations: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub allocations: u64,
    pub average_frame_time: f64, // milliseconds
    pub fps: f64,
}

impl DiagnosticSnapshot {
    /// Calculate cache hit ratio (0.0 to 1.0)
    pub fn cache_hit_ratio(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 { 0.0 } else { self.cache_hits as f64 / total as f64 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn new_counters_are_zero() {
        let counters = DiagnosticCounters::new();
        let snapshot = counters.snapshot();

        assert_eq!(snapshot.render_calls, 0);
        assert_eq!(snapshot.render_bytes, 0);
        assert_eq!(snapshot.event_dispatches, 0);
        assert_eq!(snapshot.layout_computations, 0);
        assert_eq!(snapshot.cache_hits, 0);
        assert_eq!(snapshot.cache_misses, 0);
        assert_eq!(snapshot.allocations, 0);
        assert_eq!(snapshot.average_frame_time, 0.0);
        assert_eq!(snapshot.fps, 0.0);
    }

    #[test]
    fn increment_counters() {
        let counters = DiagnosticCounters::new();

        counters.inc_render_calls();
        counters.inc_render_calls();
        counters.add_render_bytes(1024);

        let snapshot = counters.snapshot();
        assert_eq!(snapshot.render_calls, 2);
        assert_eq!(snapshot.render_bytes, 1024);
    }

    #[test]
    fn frame_time_tracking() {
        let counters = DiagnosticCounters::new();

        // Record 16ms frames (60 FPS)
        for _ in 0..10 {
            counters.record_frame_time(Duration::from_millis(16));
        }

        let snapshot = counters.snapshot();
        assert!((snapshot.average_frame_time - 16.0).abs() < 0.1);
        // FPS calculation: 1000ms / 16ms = 62.5 FPS, not exactly 60
        assert!((snapshot.fps - 62.5).abs() < 1.0);
    }

    #[test]
    fn frame_time_rolling_window() {
        let counters = DiagnosticCounters::new();

        // Fill beyond max capacity
        for _ in 0..(DiagnosticCounters::MAX_FRAME_SAMPLES + 10) {
            counters.record_frame_time(Duration::from_millis(16));
        }

        let frame_times = counters.frame_times.lock();
        assert_eq!(frame_times.len(), DiagnosticCounters::MAX_FRAME_SAMPLES);
    }

    #[test]
    fn cache_hit_ratio() {
        let counters = DiagnosticCounters::new();

        // 3 hits, 1 miss = 75% hit ratio
        counters.inc_cache_hits();
        counters.inc_cache_hits();
        counters.inc_cache_hits();
        counters.inc_cache_misses();

        let snapshot = counters.snapshot();
        assert!((snapshot.cache_hit_ratio() - 0.75).abs() < 0.01);
    }

    #[test]
    fn cache_hit_ratio_with_no_accesses() {
        let counters = DiagnosticCounters::new();
        let snapshot = counters.snapshot();
        assert_eq!(snapshot.cache_hit_ratio(), 0.0);
    }

    #[test]
    fn reset_clears_all_counters() {
        let counters = DiagnosticCounters::new();

        counters.inc_render_calls();
        counters.add_render_bytes(1024);
        counters.inc_event_dispatches();
        counters.record_frame_time(Duration::from_millis(16));

        counters.reset();

        let snapshot = counters.snapshot();
        assert_eq!(snapshot.render_calls, 0);
        assert_eq!(snapshot.render_bytes, 0);
        assert_eq!(snapshot.event_dispatches, 0);
        assert_eq!(snapshot.average_frame_time, 0.0);
    }
}
