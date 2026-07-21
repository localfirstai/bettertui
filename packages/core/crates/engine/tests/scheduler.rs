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

// ─── Render Coalescing Tests ──────────────────────────────────────────────

#[test]
fn coalesced_request_coalesces_multiple() {
    let mut s = Scheduler::new();
    s.request_render_coalesced();
    s.request_render_coalesced();
    s.request_render_coalesced();
    assert!(s.has_scheduled_frame());
    assert_eq!(s.priority_queue.len(), 1);
}

#[test]
fn coalesced_request_promotes_priority() {
    let mut s = Scheduler::new();
    s.request_render_coalesced_with_priority(Priority::Low);
    assert_eq!(s.highest_priority(), Some(Priority::Low));

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
    let mut s = Scheduler::new();
    s.begin_render();
    assert!(s.is_rendering());

    s.request_render_coalesced();
    assert!(s.immediate_rerender_requested());
    assert!(!s.has_scheduled_frame());

    let should_rerender = s.end_render();
    assert!(should_rerender);
    assert!(!s.is_rendering());
}

#[test]
fn immediate_request_bypasses_coalescing() {
    let mut s = Scheduler::new();
    s.request_render_coalesced_with_priority(Priority::Normal);
    assert_eq!(s.priority_queue.len(), 1);

    s.request_render_immediate();
    assert_eq!(s.priority_queue.len(), 2);
    assert_eq!(s.highest_priority(), Some(Priority::Critical));
}

#[test]
fn critical_priority_is_highest() {
    assert!(Priority::Critical > Priority::High);
    assert!(Priority::Critical > Priority::Normal);
    assert!(Priority::Critical > Priority::Low);
    assert!(Priority::Critical > Priority::Idle);
}

#[test]
fn coalesced_frame_cleared_on_begin_frame() {
    let mut s = Scheduler::new();
    s.request_render_coalesced();
    assert!(s.has_scheduled_frame());
    std::thread::sleep(Duration::from_millis(20));
    assert!(s.begin_frame());
    assert!(!s.has_scheduled_frame());
}

#[test]
fn non_coalesced_request_still_works() {
    let mut s = Scheduler::new();
    s.request_frame();
    s.request_frame();
    s.request_frame();
    assert_eq!(s.priority_queue.len(), 3);
}

// ─── Clock-Based Frame Budget Tests ───────────────────────────────────────

#[test]
fn frame_budget_start_frame_at_uses_clock() {
    use std::time::Instant;
    let mut b = FrameBudget::new(60);
    let t = Instant::now();
    b.start_frame_at(t);
    assert!(b.current_frame_start.is_some());
}

#[test]
fn frame_budget_end_frame_at_uses_clock() {
    use std::time::Instant;
    let mut b = FrameBudget::new(60);
    let start = Instant::now();
    b.start_frame_at(start);
    // Simulate some elapsed time
    std::thread::sleep(Duration::from_millis(1));
    let end = start + Duration::from_millis(5);
    b.end_frame_at(end);
    assert!(b.last_frame_duration >= Duration::from_millis(4));
    assert!(b.last_frame_duration <= Duration::from_millis(6));
}

#[test]
fn frame_budget_remaining_budget_at() {
    use std::time::Instant;
    let mut b = FrameBudget::new(60); // 16ms frame time
    let start = Instant::now();
    b.start_frame_at(start);
    let check = start + Duration::from_millis(5);
    let remaining = b.remaining_budget_at(check);
    assert!(remaining > Duration::from_millis(10));
    assert!(remaining < Duration::from_millis(12));
}

// ─── Full Coalescing + Clock Integration ──────────────────────────────────

#[test]
fn coalescing_with_clock_full_cycle() {
    let mut clock = bettertui_engine::clock::ManualClock::new();
    let mut s = Scheduler::with_clock(60, Box::new(clock.clone()));

    s.request_render_coalesced();
    s.request_render_coalesced();
    s.request_render_coalesced_with_priority(Priority::High);
    assert_eq!(s.priority_queue.len(), 1);
    assert_eq!(s.highest_priority(), Some(Priority::High));

    clock.advance(20);
    assert!(s.begin_frame());
    assert_eq!(s.priority_queue.len(), 0);

    s.begin_render();
    s.request_render_coalesced();
    assert!(s.immediate_rerender_requested());
    let should_rerender = s.end_render();
    assert!(should_rerender);

    s.end_frame();
    assert_eq!(s.frame_count(), 1);
}
