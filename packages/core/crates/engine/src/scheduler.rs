//! Frame scheduler with priority queue, frame budgeting, and idle/animation callbacks.

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
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
        Self {
            priority,
            requested_at: Instant::now(),
            deadline: None,
        }
    }

    pub fn with_deadline(mut self, deadline: Instant) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub fn is_overdue(&self) -> bool {
        self.deadline.is_some_and(|d| Instant::now() > d)
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
    pub max_frame_time: Duration,
    pub current_frame_start: Option<Instant>,
    pub last_frame_duration: Duration,
    pub budget_exceeded_count: u64,
}

impl FrameBudget {
    pub fn new(target_fps: u32) -> Self {
        let target = if target_fps > 0 {
            Duration::from_millis(1000 / target_fps as u64)
        } else {
            Duration::from_millis(16)
        };

        Self {
            target_frame_time: target,
            max_frame_time: target * 2,
            current_frame_start: None,
            last_frame_duration: Duration::ZERO,
            budget_exceeded_count: 0,
        }
    }

    pub fn start_frame(&mut self) {
        self.current_frame_start = Some(Instant::now());
    }

    pub fn end_frame(&mut self) {
        if let Some(start) = self.current_frame_start.take() {
            self.last_frame_duration = start.elapsed();
            if self.last_frame_duration > self.target_frame_time {
                self.budget_exceeded_count += 1;
            }
        }
    }

    pub fn remaining_budget(&self) -> Duration {
        match self.current_frame_start {
            Some(start) => {
                let elapsed = start.elapsed();
                if elapsed >= self.target_frame_time {
                    Duration::ZERO
                } else {
                    self.target_frame_time - elapsed
                }
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
}

pub struct Scheduler {
    #[doc(hidden)]
    pub frame_interval: Duration,
    last_frame: Instant,
    pending: bool,
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
    pub fn new() -> Self {
        Self::with_fps(60)
    }

    pub fn with_fps(fps: u32) -> Self {
        let interval = if fps > 0 {
            Duration::from_millis(1000 / fps as u64)
        } else {
            Duration::from_millis(16)
        };

        Self {
            frame_interval: interval,
            last_frame: Instant::now(),
            pending: false,
            frame_count: 0,
            dropped_frames: 0,
            priority_queue: BinaryHeap::new(),
            idle_callbacks: Vec::new(),
            animation_frames: Vec::new(),
            animation_frame_id: 0,
            frame_budget: FrameBudget::new(fps),
            stats: SchedulerStats::default(),
        }
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

        let elapsed = self.last_frame.elapsed();
        if elapsed >= self.frame_interval {
            if elapsed >= self.frame_interval * 2 {
                FrameStatus::Overdue
            } else {
                FrameStatus::Due
            }
        } else {
            FrameStatus::Pending
        }
    }

    pub fn begin_frame(&mut self) -> bool {
        if self.status() != FrameStatus::Due && self.status() != FrameStatus::Overdue {
            return false;
        }

        self.pending = false;
        self.priority_queue.clear();
        self.last_frame = Instant::now();
        self.frame_count += 1;
        self.frame_budget.start_frame();
        self.stats.total_frames = self.frame_count;
        true
    }

    pub fn end_frame(&mut self) {
        self.frame_budget.end_frame();

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
        self.frame_interval = if fps > 0 {
            Duration::from_millis(1000 / fps as u64)
        } else {
            Duration::from_millis(16)
        };
        self.frame_budget = FrameBudget::new(fps);
    }

    pub fn time_until_next_frame(&self) -> Duration {
        let elapsed = self.last_frame.elapsed();
        if elapsed >= self.frame_interval {
            Duration::ZERO
        } else {
            self.frame_interval - elapsed
        }
    }

    pub fn reset(&mut self) {
        self.last_frame = Instant::now();
        self.pending = false;
        self.frame_count = 0;
        self.dropped_frames = 0;
        self.priority_queue.clear();
        self.idle_callbacks.clear();
        self.animation_frames.clear();
        self.animation_frame_id = 0;
        self.frame_budget = FrameBudget::new(60);
        self.stats = SchedulerStats::default();
    }
}
