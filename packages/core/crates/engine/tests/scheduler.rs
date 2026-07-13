//! Tests for the scheduler module.

use std::time::Duration;

use bettertui_engine::scheduler::{FrameBudget, FrameStatus, Priority, Scheduler};

#[test]
fn scheduler_new() {
    let s = Scheduler::new();
    assert_eq!(s.status(), FrameStatus::Idle);
    assert_eq!(s.frame_count(), 0);
}

#[test]
fn scheduler_request_frame() {
    let mut s = Scheduler::new();
    s.request_frame();
    assert_ne!(s.status(), FrameStatus::Idle);
}

#[test]
fn scheduler_request_with_priority() {
    let mut s = Scheduler::new();
    s.request_frame_with_priority(Priority::High);
    assert_eq!(s.highest_priority(), Some(Priority::High));
}

#[test]
fn scheduler_priority_ordering() {
    let mut s = Scheduler::new();
    s.request_frame_with_priority(Priority::Low);
    s.request_frame_with_priority(Priority::High);
    s.request_frame_with_priority(Priority::Normal);

    assert_eq!(s.highest_priority(), Some(Priority::High));
    let _ = s.priority_queue.pop();
    assert_eq!(s.highest_priority(), Some(Priority::Normal));
}

#[test]
fn scheduler_begin_frame() {
    let mut s = Scheduler::new();
    s.request_frame();
    std::thread::sleep(Duration::from_millis(20));
    assert!(s.begin_frame());
    assert_eq!(s.frame_count(), 1);
    assert_eq!(s.status(), FrameStatus::Idle);
}

#[test]
fn scheduler_end_frame() {
    let mut s = Scheduler::new();
    s.request_frame();
    std::thread::sleep(Duration::from_millis(20));
    s.begin_frame();
    std::thread::sleep(Duration::from_millis(1));
    s.end_frame();
    assert!(s.frame_budget().last_frame_duration > Duration::ZERO);
}

#[test]
fn scheduler_skip_frame() {
    let mut s = Scheduler::new();
    s.request_frame();
    assert!(s.has_pending_frames());
    s.skip_frame();
    assert!(!s.has_pending_frames());
    assert_eq!(s.status(), FrameStatus::Idle);
    assert_eq!(s.dropped_frames(), 1);
}

#[test]
fn scheduler_with_fps() {
    let s = Scheduler::with_fps(30);
    assert_eq!(s.frame_interval, Duration::from_millis(33));
}

#[test]
fn scheduler_set_fps() {
    let mut s = Scheduler::new();
    s.set_fps(60);
    assert_eq!(s.frame_interval, Duration::from_millis(16));
}

#[test]
fn scheduler_time_until_next_frame() {
    let s = Scheduler::new();
    let t = s.time_until_next_frame();
    assert!(t <= Duration::from_millis(16));
}

#[test]
fn scheduler_reset() {
    let mut s = Scheduler::new();
    s.request_frame();
    s.reset();
    assert_eq!(s.status(), FrameStatus::Idle);
    assert_eq!(s.frame_count(), 0);
}

#[test]
fn scheduler_multiple_frames() {
    let mut s = Scheduler::new();
    for _ in 0..5 {
        s.request_frame();
        std::thread::sleep(Duration::from_millis(20));
        s.begin_frame();
        s.end_frame();
    }
    assert_eq!(s.frame_count(), 5);
}

#[test]
fn frame_budget_new() {
    let b = FrameBudget::new(60);
    assert_eq!(b.target_frame_time, Duration::from_millis(16));
}

#[test]
fn frame_budget_remaining() {
    let mut b = FrameBudget::new(60);
    b.start_frame();
    let remaining = b.remaining_budget();
    assert!(remaining <= Duration::from_millis(16));
}

#[test]
fn frame_budget_utilization() {
    let mut b = FrameBudget::new(60);
    b.start_frame();
    std::thread::sleep(Duration::from_millis(5));
    b.end_frame();
    assert!(b.utilization() > 0.0);
}

#[test]
fn scheduler_idle_callbacks() {
    let mut s = Scheduler::new();
    let called = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let called_clone = called.clone();

    s.on_idle(move || {
        called_clone.store(true, std::sync::atomic::Ordering::SeqCst);
    });

    assert!(s.has_idle_callbacks());
    s.execute_idle_callbacks();
    assert!(!s.has_idle_callbacks());
    assert!(called.load(std::sync::atomic::Ordering::SeqCst));
}

#[test]
fn scheduler_animation_frames() {
    let mut s = Scheduler::new();
    let id = s.schedule_animation(|_frame| {});
    assert_eq!(id, 0);

    s.cancel_animation(id);
}

#[test]
fn scheduler_stats() {
    let mut s = Scheduler::new();
    s.request_frame();
    std::thread::sleep(Duration::from_millis(20));
    s.begin_frame();
    s.end_frame();

    let stats = s.stats();
    assert_eq!(stats.total_frames, 1);
}
