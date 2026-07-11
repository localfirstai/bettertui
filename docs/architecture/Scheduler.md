# Scheduler

The scheduler owns frame timing, a priority queue, frame budget tracking, idle callbacks, and animation frame scheduling. Code: `native/engine/src/scheduler/mod.rs` (~18 tests).

## Responsibilities

```mermaid
graph TD
    A[Scheduler] --> B[Frame timing: begin_frame / end_frame]
    A --> C[Priority queue: request_frame[_with_priority]]
    A --> D[Frame budget tracking: FrameBudget]
    A --> E[Idle callbacks: on_idle]
    A --> F[Animation frames: schedule_animation]
    A --> G[Stats: SchedulerStats]
```

## Key API

| Item | Purpose |
|------|---------|
| `enum Priority { Idle, Low, Normal, High }` | frame request priority |
| `enum FrameStatus { Idle, Pending, Due, Overdue }` | returned by `status()` / `should_render()` |
| `request_frame()`, `request_high_frame()`, `request_low_frame()`, `request_idle_frame()` | enqueue a frame |
| `begin_frame() -> bool`, `end_frame()` | bracket a frame; `end_frame` updates `avg_frame_time` (EMA) |
| `execute_idle_callbacks()` | run deferred idle work |
| `skip_frame()` | drop a frame |
| `set_fps()/fps()`, `time_until_next_frame()`, `frame_count`, `dropped_frames` | introspection |
| `SchedulerStats` | `frame_count`, `dropped_frames`, `avg_frame_time`, `frame_budget` |

## Wiring

- The `Renderer` owns a `Scheduler` and calls `begin_frame()`/`end_frame()` around `render()`.
- `NapiScheduler` (`bindings`) exposes `beginFrame`, `endFrame`, `requestFrame`, `shouldRender`, `fps`, `frameBudgetMs`, `isIdle`, `frameCount`, `droppedFrames`.

> Known issue (Phase 8 review): `begin_frame()` is called at the **end** of `Renderer::render()` rather than the start, and it clears the entire priority queue — losing intermediate frame requests. `Engine::frame_count` (command batching) and `Scheduler::frame_count` (rendering) are not synchronized. These are documented architecture issues, not yet fixed.
