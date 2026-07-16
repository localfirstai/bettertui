//! Frame scheduler with priority queue, frame budgeting, and idle/animation callbacks.

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

use crate::clock::Clock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameStatus {
    Idle,
    Pending,
    Due,
    Overdue,
}

#[derive(Debug, Clone)]
pub struct FrameRequest {
    pub priority: Priority,
    pub requested_at: Instant,
    pub deadline: Option<Instant>,
}

impl FrameRequest {
    pub fn new(priority: Priority) -> Self {
        Self { priority, requested_at: Instant::now(), deadline: None }
    }

    pub fn with_deadline(mut self, deadline: Instant) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub fn is_overdue_at(&self, now: Instant) -> bool {
        self.deadline.is_some_and(|d| now > d)
    }

    pub fn is_overdue(&self) -> bool {
        self.is_overdue_at(Instant::now())
    }
}

impl PartialEq for FrameRequest {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for FrameRequest {}

impl PartialOrd for FrameRequest {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FrameRequest {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

pub type IdleCallback = Box<dyn FnOnce() + Send>;
type AnimationCallback = Box<dyn FnMut(u64) + Send>;

pub struct FrameBudget {
    pub target_frame_time: Duration,
    pub min_frame_time: Duration,
    pub max_frame_time: Duration,
    pub current_frame_start: Option<Instant>,
    pub last_frame_duration: Duration,
    pub budget_exceeded_count: u64,
}

impl FrameBudget {
    pub fn new(target_fps: u32) -> Self {
        Self::with_fps_range(target_fps, target_fps)
    }

    /// Create frame budget with separate target and max FPS.
    ///
    /// - `target_fps`: Desired frame rate for continuous mode
    /// - `max_fps`: Maximum frame rate cap for immediate re-renders
    pub fn with_fps_range(target_fps: u32, max_fps: u32) -> Self {
        let target =
            if target_fps > 0 { Duration::from_millis(1000 / target_fps as u64) } else { Duration::from_millis(33) };

        let min = if max_fps > 0 { Duration::from_millis(1000 / max_fps as u64) } else { Duration::from_millis(16) };

        Self {
            target_frame_time: target,
            min_frame_time: min,
            max_frame_time: target * 2,
            current_frame_start: None,
            last_frame_duration: Duration::ZERO,
            budget_exceeded_count: 0,
        }
    }

    /// Start a frame, recording the given `now` as the frame start time.
    pub fn start_frame_at(&mut self, now: Instant) {
        self.current_frame_start = Some(now);
    }

    /// Start a frame using `Instant::now()` (convenience for non-clock contexts).
    pub fn start_frame(&mut self) {
        self.start_frame_at(Instant::now());
    }

    /// End a frame, computing duration from the given `now`.
    pub fn end_frame_at(&mut self, now: Instant) {
        if let Some(start) = self.current_frame_start.take() {
            self.last_frame_duration = now.saturating_duration_since(start);
            if self.last_frame_duration > self.target_frame_time {
                self.budget_exceeded_count += 1;
            }
        }
    }

    /// End a frame using real elapsed time (convenience for non-clock contexts).
    pub fn end_frame(&mut self) {
        if let Some(start) = self.current_frame_start.take() {
            self.last_frame_duration = start.elapsed();
            if self.last_frame_duration > self.target_frame_time {
                self.budget_exceeded_count += 1;
            }
        }
    }

    /// Remaining budget computed against the given `now`.
    pub fn remaining_budget_at(&self, now: Instant) -> Duration {
        match self.current_frame_start {
            Some(start) => {
                let elapsed = now.saturating_duration_since(start);
                if elapsed >= self.target_frame_time { Duration::ZERO } else { self.target_frame_time - elapsed }
            }
            None => self.target_frame_time,
        }
    }

    /// Remaining budget using real elapsed time (convenience for non-clock contexts).
    pub fn remaining_budget(&self) -> Duration {
        match self.current_frame_start {
            Some(start) => {
                let elapsed = start.elapsed();
                if elapsed >= self.target_frame_time { Duration::ZERO } else { self.target_frame_time - elapsed }
            }
            None => self.target_frame_time,
        }
    }

    pub fn has_budget(&self, additional: Duration) -> bool {
        self.remaining_budget() >= additional
    }

    pub fn utilization(&self) -> f64 {
        if self.target_frame_time.is_zero() {
            0.0
        } else {
            self.last_frame_duration.as_secs_f64() / self.target_frame_time.as_secs_f64()
        }
    }

    /// Get minimum frame time (for max FPS).
    pub fn min_frame_time(&self) -> Duration {
        self.min_frame_time
    }
}

pub struct Scheduler {
    #[doc(hidden)]
    pub frame_interval: Duration,
    min_frame_interval: Duration,
    last_frame: Instant,
    pending: bool,
    immediate_mode: bool,
    frame_count: u64,
    dropped_frames: u64,
    #[doc(hidden)]
    pub priority_queue: BinaryHeap<FrameRequest>,
    idle_callbacks: Vec<IdleCallback>,
    animation_frames: Vec<(u64, AnimationCallback)>,
    animation_frame_id: u64,
    #[doc(hidden)]
    pub frame_budget: FrameBudget,
    stats: SchedulerStats,
    clock: Option<Box<dyn Clock + Send>>,
    clock_base: Option<(Instant, Duration)>,
    /// Coalescing: when true, `request_render_coalesced()` is a no-op.
    has_scheduled_frame: bool,
    /// Set during a frame; pending requests are deferred to the next frame.
    rendering: bool,
    /// One-shot: request an immediate re-render after the current frame completes.
    immediate_rerender_requested: bool,
}

#[derive(Debug, Clone, Default)]
pub struct SchedulerStats {
    pub total_frames: u64,
    pub dropped_frames: u64,
    pub avg_frame_time: Duration,
    pub max_frame_time: Duration,
    pub budget_exceeded: u64,
    pub idle_callbacks_executed: u64,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    /// Create scheduler with default 60 FPS.
    pub fn new() -> Self {
        Self::with_fps(60)
    }

    /// Create scheduler with specified target FPS.
    pub fn with_fps(fps: u32) -> Self {
        Self::with_fps_range(fps, fps)
    }

    /// Create scheduler with target and max FPS.
    ///
    /// - `target_fps`: Desired frame rate for continuous rendering (e.g., 30)
    /// - `max_fps`: Maximum frame rate for immediate re-renders (e.g., 120)
    pub fn with_fps_range(target_fps: u32, max_fps: u32) -> Self {
        let interval =
            if target_fps > 0 { Duration::from_millis(1000 / target_fps as u64) } else { Duration::from_millis(33) };

        let min_interval =
            if max_fps > 0 { Duration::from_millis(1000 / max_fps as u64) } else { Duration::from_millis(8) };

        let now = Instant::now();
        Self {
            frame_interval: interval,
            min_frame_interval: min_interval,
            last_frame: now,
            pending: false,
            immediate_mode: false,
            frame_count: 0,
            dropped_frames: 0,
            priority_queue: BinaryHeap::new(),
            idle_callbacks: Vec::new(),
            animation_frames: Vec::new(),
            animation_frame_id: 0,
            frame_budget: FrameBudget::with_fps_range(target_fps, max_fps),
            stats: SchedulerStats::default(),
            clock: None,
            clock_base: None,
            has_scheduled_frame: false,
            rendering: false,
            immediate_rerender_requested: false,
        }
    }

    /// Create scheduler with an external clock source (for testing).
    pub fn with_clock(target_fps: u32, clock: Box<dyn Clock + Send>) -> Self {
        let mut s = Self::with_fps(target_fps);
        let now_dur = clock.now();
        s.clock = Some(clock);
        s.clock_base = Some((Instant::now(), now_dur));
        s.last_frame = s.now();
        s
    }

    /// Get the current `Instant`, using the injected clock if available.
    fn now(&self) -> Instant {
        match (&self.clock, &self.clock_base) {
            (Some(clock), Some((base_instant, base_dur))) => {
                let elapsed = clock.now().saturating_sub(*base_dur);
                *base_instant + elapsed
            }
            _ => Instant::now(),
        }
    }

    /// Enable immediate mode for animation-driven rendering.
    /// Uses min_frame_interval (max FPS) for tighter frame timing.
    pub fn set_immediate_mode(&mut self, enabled: bool) {
        self.immediate_mode = enabled;
    }

    /// Check if immediate mode is enabled.
    pub fn is_immediate_mode(&self) -> bool {
        self.immediate_mode
    }

    pub fn request_frame(&mut self) {
        self.request_frame_with_priority(Priority::Normal);
    }

    pub fn request_frame_with_priority(&mut self, priority: Priority) {
        let request = FrameRequest::new(priority);
        self.priority_queue.push(request);
        self.pending = true;
    }

    pub fn request_high_priority_frame(&mut self) {
        self.request_frame_with_priority(Priority::High);
    }

    pub fn request_low_priority_frame(&mut self) {
        self.request_frame_with_priority(Priority::Low);
    }

    pub fn request_idle_frame(&mut self) {
        self.request_frame_with_priority(Priority::Idle);
    }

    /// Request a frame with render coalescing.
    ///
    /// Multiple calls between frames are collapsed into a single pending frame.
    /// Uses the highest priority seen. If a frame is already being rendered,
    /// the request is deferred via `immediate_rerender_requested`.
    pub fn request_render_coalesced(&mut self) {
        self.request_render_coalesced_with_priority(Priority::Normal);
    }

    /// Coalesced frame request with explicit priority.
    pub fn request_render_coalesced_with_priority(&mut self, priority: Priority) {
        if self.rendering {
            self.immediate_rerender_requested = true;
            return;
        }

        if self.has_scheduled_frame {
            // Already have a pending frame — just promote priority if higher.
            if let Some(top) = self.priority_queue.peek() {
                if priority > top.priority {
                    self.priority_queue.pop();
                    self.priority_queue.push(FrameRequest::new(priority));
                }
            }
            return;
        }

        self.has_scheduled_frame = true;
        self.request_frame_with_priority(priority);
    }

    /// Request an immediate re-render, bypassing coalescing.
    ///
    /// Use this for critical updates that must render in the next available frame
    /// even if a frame is already scheduled.
    pub fn request_render_immediate(&mut self) {
        self.request_render_immediate_with_priority(Priority::Critical);
    }

    /// Immediate frame request with explicit priority.
    pub fn request_render_immediate_with_priority(&mut self, priority: Priority) {
        if self.rendering {
            self.immediate_rerender_requested = true;
            return;
        }

        self.has_scheduled_frame = true;
        self.request_frame_with_priority(priority);
    }

    /// Returns `true` if a frame is already scheduled (coalescing active).
    pub fn has_scheduled_frame(&self) -> bool {
        self.has_scheduled_frame
    }

    /// Returns `true` if a deferred immediate render was requested during a frame.
    pub fn immediate_rerender_requested(&self) -> bool {
        self.immediate_rerender_requested
    }

    /// Returns `true` if a frame is currently being rendered.
    pub fn is_rendering(&self) -> bool {
        self.rendering
    }

    /// Mark the start of a render pass. Call before rendering begins.
    pub fn begin_render(&mut self) {
        self.rendering = true;
    }

    /// Mark the end of a render pass. Call after rendering completes.
    /// Returns `true` if an immediate re-render was requested during the frame.
    pub fn end_render(&mut self) -> bool {
        self.rendering = false;
        let should_rerender = self.immediate_rerender_requested;
        self.immediate_rerender_requested = false;
        should_rerender
    }

    /// Clear the scheduled frame flag. Called when a frame is consumed.
    pub fn clear_scheduled_frame(&mut self) {
        self.has_scheduled_frame = false;
    }

    pub fn schedule_animation(&mut self, callback: impl FnMut(u64) + Send + 'static) -> u64 {
        let id = self.animation_frame_id;
        self.animation_frames.push((id, Box::new(callback)));
        self.animation_frame_id += 1;
        id
    }

    pub fn cancel_animation(&mut self, id: u64) {
        if let Some(pos) = self.animation_frames.iter().position(|(fid, _)| *fid == id) {
            drop(self.animation_frames.swap_remove(pos));
        }
    }

    pub fn on_idle(&mut self, callback: impl FnOnce() + Send + 'static) {
        self.idle_callbacks.push(Box::new(callback));
    }

    pub fn status(&self) -> FrameStatus {
        if !self.pending && self.priority_queue.is_empty() {
            return FrameStatus::Idle;
        }

        let effective_interval = if self.immediate_mode { self.min_frame_interval } else { self.frame_interval };

        let now = self.now();
        let elapsed = now.saturating_duration_since(self.last_frame);
        if elapsed >= effective_interval {
            if elapsed >= effective_interval * 2 { FrameStatus::Overdue } else { FrameStatus::Due }
        } else {
            FrameStatus::Pending
        }
    }

    pub fn begin_frame(&mut self) -> bool {
        if self.status() != FrameStatus::Due && self.status() != FrameStatus::Overdue {
            return false;
        }

        self.pending = false;
        self.has_scheduled_frame = false;
        self.priority_queue.clear();
        self.last_frame = self.now();
        self.frame_count += 1;
        self.frame_budget.start_frame_at(self.now());
        self.stats.total_frames = self.frame_count;
        true
    }

    pub fn end_frame(&mut self) {
        let now = self.now();
        self.frame_budget.end_frame_at(now);

        if self.frame_budget.last_frame_duration > self.stats.max_frame_time {
            self.stats.max_frame_time = self.frame_budget.last_frame_duration;
        }

        if self.frame_budget.last_frame_duration > self.frame_interval {
            self.stats.dropped_frames += 1;
            self.dropped_frames += 1;
        }

        self.stats.budget_exceeded = self.frame_budget.budget_exceeded_count;

        let prev_avg = self.stats.avg_frame_time.as_nanos() as f64;
        let new_dur = self.frame_budget.last_frame_duration.as_nanos() as f64;
        let count = self.frame_count.max(1) as f64;
        let smoothed = prev_avg + (new_dur - prev_avg) / count;
        self.stats.avg_frame_time = Duration::from_nanos(smoothed as u64);
    }

    pub fn execute_idle_callbacks(&mut self) {
        let callbacks: Vec<_> = self.idle_callbacks.drain(..).collect();
        for callback in callbacks {
            callback();
            self.stats.idle_callbacks_executed += 1;
        }
    }

    pub fn has_idle_callbacks(&self) -> bool {
        !self.idle_callbacks.is_empty()
    }

    pub fn has_pending_frames(&self) -> bool {
        self.pending || !self.priority_queue.is_empty()
    }

    pub fn highest_priority(&self) -> Option<Priority> {
        self.priority_queue.peek().map(|r| r.priority)
    }

    pub fn skip_frame(&mut self) {
        if self.pending || !self.priority_queue.is_empty() {
            self.dropped_frames += 1;
            self.stats.dropped_frames = self.dropped_frames;
            self.pending = false;
            self.has_scheduled_frame = false;
            self.priority_queue.clear();
        }
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn dropped_frames(&self) -> u64 {
        self.dropped_frames
    }

    pub fn frame_budget(&self) -> &FrameBudget {
        &self.frame_budget
    }

    pub fn stats(&self) -> &SchedulerStats {
        &self.stats
    }

    pub fn set_fps(&mut self, fps: u32) {
        self.set_fps_range(fps, fps);
    }

    /// Set both target and max FPS.
    pub fn set_fps_range(&mut self, target_fps: u32, max_fps: u32) {
        self.frame_interval =
            if target_fps > 0 { Duration::from_millis(1000 / target_fps as u64) } else { Duration::from_millis(33) };

        self.min_frame_interval =
            if max_fps > 0 { Duration::from_millis(1000 / max_fps as u64) } else { Duration::from_millis(8) };

        self.frame_budget = FrameBudget::with_fps_range(target_fps, max_fps);
    }

    /// Get current target FPS.
    pub fn target_fps(&self) -> u32 {
        if self.frame_interval.is_zero() { 60 } else { (1000 / self.frame_interval.as_millis()) as u32 }
    }

    /// Get current max FPS.
    pub fn max_fps(&self) -> u32 {
        if self.min_frame_interval.is_zero() { 120 } else { (1000 / self.min_frame_interval.as_millis()) as u32 }
    }

    pub fn time_until_next_frame(&self) -> Duration {
        let now = self.now();
        let elapsed = now.saturating_duration_since(self.last_frame);
        if elapsed >= self.frame_interval { Duration::ZERO } else { self.frame_interval - elapsed }
    }

    pub fn reset(&mut self) {
        self.last_frame = self.now();
        self.pending = false;
        self.immediate_mode = false;
        self.frame_count = 0;
        self.dropped_frames = 0;
        self.priority_queue.clear();
        self.idle_callbacks.clear();
        self.animation_frames.clear();
        self.animation_frame_id = 0;
        self.frame_budget = FrameBudget::with_fps_range(60, 60);
        self.stats = SchedulerStats::default();
        self.has_scheduled_frame = false;
        self.rendering = false;
        self.immediate_rerender_requested = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduler_default_fps() {
        let scheduler = Scheduler::new();
        // 1000/16 ≈ 62 due to 60fps = 16.67ms
        assert!(scheduler.target_fps() >= 60);
    }

    #[test]
    fn scheduler_custom_fps() {
        let scheduler = Scheduler::with_fps(30);
        // 1000/33 ≈ 30
        assert_eq!(scheduler.target_fps(), 30);
    }

    #[test]
    fn scheduler_fps_range() {
        let scheduler = Scheduler::with_fps_range(30, 120);
        assert_eq!(scheduler.target_fps(), 30);
        // 1000/8 = 125 (120fps = 8.33ms)
        assert!(scheduler.max_fps() >= 120);
    }

    #[test]
    fn scheduler_immediate_mode() {
        let mut scheduler = Scheduler::with_fps_range(30, 120);
        assert!(!scheduler.is_immediate_mode());

        scheduler.set_immediate_mode(true);
        assert!(scheduler.is_immediate_mode());
    }

    #[test]
    fn frame_budget_with_range() {
        let budget = FrameBudget::with_fps_range(30, 120);
        assert_eq!(budget.target_frame_time, Duration::from_millis(33));
        assert_eq!(budget.min_frame_time(), Duration::from_millis(8));
    }

    #[test]
    fn set_fps_range() {
        let mut scheduler = Scheduler::new();
        scheduler.set_fps_range(30, 120);
        assert_eq!(scheduler.target_fps(), 30);
        assert!(scheduler.max_fps() >= 120);
    }

    #[test]
    fn scheduler_with_clock_tracks_time() {
        let mut clock = crate::clock::ManualClock::new();
        let mut scheduler = Scheduler::with_clock(60, Box::new(clock.clone()));
        let t0 = scheduler.now();
        clock.advance(100);
        let t1 = scheduler.now();
        assert!(t1 >= t0);
    }

    #[test]
    fn scheduler_with_clock_advances_frame_time() {
        let mut clock = crate::clock::ManualClock::new();
        let mut scheduler = Scheduler::with_clock(60, Box::new(clock.clone()));
        scheduler.request_frame();
        // Advance clock past frame interval so frame is due
        clock.advance(200);
        assert!(scheduler.begin_frame());
        scheduler.end_frame();
        assert_eq!(scheduler.frame_count(), 1);
    }

    #[test]
    fn scheduler_clock_controls_status() {
        let mut clock = crate::clock::ManualClock::new();
        let mut scheduler = Scheduler::with_clock(60, Box::new(clock.clone()));
        assert_eq!(scheduler.status(), FrameStatus::Idle);
        scheduler.request_frame();
        // No time elapsed — frame should be pending
        assert_eq!(scheduler.status(), FrameStatus::Pending);
        clock.advance(20);
        // Enough time for 60fps (16ms) — should be due
        assert_eq!(scheduler.status(), FrameStatus::Due);
        clock.advance(100);
        // More than 2x frame interval — should be overdue
        assert_eq!(scheduler.status(), FrameStatus::Overdue);
    }

    // ─── Render Coalescing Tests ──────────────────────────────────────────────

    #[test]
    fn coalesced_request_single_pending() {
        let mut clock = crate::clock::ManualClock::new();
        let mut s = Scheduler::with_clock(60, Box::new(clock.clone()));

        s.request_render_coalesced();
        assert!(s.has_scheduled_frame());
        assert_eq!(s.priority_queue.len(), 1);

        // Second call should be a no-op (coalesced)
        s.request_render_coalesced();
        assert_eq!(s.priority_queue.len(), 1);
    }

    #[test]
    fn coalesced_request_promotes_priority() {
        let mut clock = crate::clock::ManualClock::new();
        let mut s = Scheduler::with_clock(60, Box::new(clock.clone()));

        s.request_render_coalesced_with_priority(Priority::Low);
        assert_eq!(s.highest_priority(), Some(Priority::Low));

        // Higher priority should promote
        s.request_render_coalesced_with_priority(Priority::High);
        assert_eq!(s.highest_priority(), Some(Priority::High));
        assert_eq!(s.priority_queue.len(), 1);

        // Lower priority should not demote
        s.request_render_coalesced_with_priority(Priority::Idle);
        assert_eq!(s.highest_priority(), Some(Priority::High));
        assert_eq!(s.priority_queue.len(), 1);
    }

    #[test]
    fn coalesced_request_defers_during_render() {
        let mut clock = crate::clock::ManualClock::new();
        let mut s = Scheduler::with_clock(60, Box::new(clock.clone()));

        s.begin_render();
        assert!(s.is_rendering());

        s.request_render_coalesced();
        assert!(s.immediate_rerender_requested());
        assert!(!s.has_scheduled_frame());

        let should_rerender = s.end_render();
        assert!(should_rerender);
        assert!(!s.is_rendering());
        assert!(!s.immediate_rerender_requested());
    }

    #[test]
    fn coalesced_frame_cleared_on_begin_frame() {
        let mut clock = crate::clock::ManualClock::new();
        let mut s = Scheduler::with_clock(60, Box::new(clock.clone()));

        s.request_render_coalesced();
        assert!(s.has_scheduled_frame());

        clock.advance(20);
        assert!(s.begin_frame());
        assert!(!s.has_scheduled_frame());
    }

    #[test]
    fn immediate_request_bypasses_coalescing() {
        let mut clock = crate::clock::ManualClock::new();
        let mut s = Scheduler::with_clock(60, Box::new(clock.clone()));

        s.request_render_coalesced_with_priority(Priority::Normal);
        assert_eq!(s.priority_queue.len(), 1);

        // Immediate request adds a new entry (bypasses coalescing)
        s.request_render_immediate();
        assert_eq!(s.priority_queue.len(), 2);
        assert_eq!(s.highest_priority(), Some(Priority::Critical));
    }

    #[test]
    fn immediate_request_defers_during_render() {
        let mut clock = crate::clock::ManualClock::new();
        let mut s = Scheduler::with_clock(60, Box::new(clock.clone()));

        s.begin_render();
        s.request_render_immediate();
        assert!(s.immediate_rerender_requested());
        assert!(!s.has_scheduled_frame());
        s.end_render();
    }

    #[test]
    fn non_coalesced_request_still_works() {
        let mut clock = crate::clock::ManualClock::new();
        let mut s = Scheduler::with_clock(60, Box::new(clock.clone()));

        // Legacy request_frame() always pushes (no coalescing)
        s.request_frame();
        s.request_frame();
        s.request_frame();
        assert_eq!(s.priority_queue.len(), 3);
    }

    #[test]
    fn critical_priority_is_highest() {
        assert!(Priority::Critical > Priority::High);
        assert!(Priority::Critical > Priority::Normal);
        assert!(Priority::Critical > Priority::Low);
        assert!(Priority::Critical > Priority::Idle);
    }

    // ─── Clock-Based Frame Budget Tests ───────────────────────────────────────

    #[test]
    fn frame_budget_uses_clock_for_start_frame() {
        let mut clock = crate::clock::ManualClock::new();
        let mut scheduler = Scheduler::with_clock(60, Box::new(clock.clone()));

        scheduler.request_frame();
        clock.advance(20);
        assert!(scheduler.begin_frame());

        // Advance clock by 5ms — remaining budget should reflect that
        clock.advance(5);
        let remaining = scheduler.frame_budget().remaining_budget_at(scheduler.now());
        // 60fps = 16.67ms frame time. After 5ms, ~11ms remaining.
        assert!(remaining < Duration::from_millis(16));
        assert!(remaining > Duration::from_millis(5));

        scheduler.end_frame();
    }

    #[test]
    fn frame_budget_uses_clock_for_end_frame() {
        let mut clock = crate::clock::ManualClock::new();
        let mut scheduler = Scheduler::with_clock(60, Box::new(clock.clone()));

        scheduler.request_frame();
        clock.advance(20);
        assert!(scheduler.begin_frame());

        // Advance clock by 8ms to simulate render work
        clock.advance(8);
        scheduler.end_frame();

        let dur = scheduler.frame_budget().last_frame_duration;
        assert!(dur >= Duration::from_millis(7));
        assert!(dur <= Duration::from_millis(10));
    }

    #[test]
    fn frame_budget_detects_exceeded_with_clock() {
        let mut clock = crate::clock::ManualClock::new();
        let mut scheduler = Scheduler::with_clock(30, Box::new(clock.clone())); // 33ms frame time

        scheduler.request_frame();
        clock.advance(40);
        assert!(scheduler.begin_frame());

        // Advance past frame budget
        clock.advance(40);
        scheduler.end_frame();

        assert!(scheduler.frame_budget().last_frame_duration > Duration::from_millis(33));
        assert!(scheduler.frame_budget().budget_exceeded_count >= 1);
    }

    #[test]
    fn time_until_next_frame_uses_clock() {
        let mut clock = crate::clock::ManualClock::new();
        let mut scheduler = Scheduler::with_clock(60, Box::new(clock.clone()));

        scheduler.request_frame();
        clock.advance(20);
        assert!(scheduler.begin_frame());
        scheduler.end_frame();

        // Right after frame, time_until_next_frame should be near the frame interval
        let remaining = scheduler.time_until_next_frame();
        assert!(remaining <= Duration::from_millis(16));

        // Advance halfway
        clock.advance(8);
        let remaining = scheduler.time_until_next_frame();
        assert!(remaining < Duration::from_millis(10));
    }

    // ─── Full Coalescing + Clock Integration ──────────────────────────────────

    #[test]
    fn coalescing_with_clock_full_cycle() {
        let mut clock = crate::clock::ManualClock::new();
        let mut s = Scheduler::with_clock(60, Box::new(clock.clone()));

        // Request several coalesced frames
        s.request_render_coalesced();
        s.request_render_coalesced();
        s.request_render_coalesced_with_priority(Priority::High);
        assert_eq!(s.priority_queue.len(), 1);
        assert_eq!(s.highest_priority(), Some(Priority::High));

        // Advance past frame interval
        clock.advance(20);
        assert!(s.begin_frame());
        assert_eq!(s.priority_queue.len(), 0);

        // Simulate render work
        s.begin_render();
        s.request_render_coalesced(); // deferred during render
        assert!(s.immediate_rerender_requested());
        let should_rerender = s.end_render();
        assert!(should_rerender);

        s.end_frame();
        assert_eq!(s.frame_count(), 1);
    }

    #[test]
    fn coalescing_preserves_highest_priority() {
        let mut clock = crate::clock::ManualClock::new();
        let mut s = Scheduler::with_clock(60, Box::new(clock.clone()));

        s.request_render_coalesced_with_priority(Priority::Idle);
        s.request_render_coalesced_with_priority(Priority::Low);
        s.request_render_coalesced_with_priority(Priority::Normal);
        s.request_render_coalesced_with_priority(Priority::High);

        // Should have only 1 entry with highest priority
        assert_eq!(s.priority_queue.len(), 1);
        assert_eq!(s.highest_priority(), Some(Priority::High));
    }
}
