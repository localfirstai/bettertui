# Scheduler

The scheduler owns frame timing, a priority queue, frame budget tracking, idle callbacks, and animation frame scheduling. Code: `packages/core/crates/engine/src/scheduler.rs`.

## Key API

| Item | Purpose |
|------|---------|
| `enum Priority { Idle, Low, Normal, High }` | frame request priority |
| `enum FrameStatus { Idle, Pending, Due, Overdue }` | returned by `status()` / `should_render()` |
| `request_frame()`, `request_high_frame()`, etc. | enqueue a frame |
| `begin_frame() -> bool`, `end_frame()` | bracket a frame; `end_frame` updates `avg_frame_time` (EMA) |
| `execute_idle_callbacks()` | run deferred idle work |
| `skip_frame()` | drop a frame |
| `set_fps()/fps()`, `time_until_next_frame()`, `frame_count`, `dropped_frames` | introspection |
| `SchedulerStats` | `frame_count`, `dropped_frames`, `avg_frame_time`, `frame_budget` |

## Wiring

- The `Renderer` owns a `Scheduler` and calls `begin_frame()`/`end_frame()` around `render()`.
- The napi bindings expose `NapiScheduler` with `beginFrame`, `endFrame`, `requestFrame`, `shouldRender`, `fps`, `frameBudgetMs`, `isIdle`, `frameCount`, `droppedFrames`.

> Known issues: `begin_frame()` is called at the **end** of `Renderer::render()` rather than the start, and clears the entire priority queue, losing intermediate frame requests. `Engine::frame_count` (command batching) and `Scheduler::frame_count` (rendering) are not synchronized.
