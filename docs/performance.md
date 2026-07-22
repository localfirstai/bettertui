# Performance

Performance targets 60fps with dirty-region optimization.

## Strategy

`node change → dirty flags → layout dirty subtrees → render dirty nodes → diff changed cells → encode dirty regions`

## Benchmarking

- TypeScript: `@bettertui/performance` (Vitest `bench` files: command-buffer, reconciler, runtime, theme, etc.)
- Rust: `cargo bench` (criterion, in `crates/benchmark/`)

## Known issues

1. `Painter::paint()` clears every cell before repainting (O(n) per frame)
2. `DirtyDiff::find_dirty_cells` does a full scan every frame
3. Two frame counters (`Engine::frame_count` vs `Scheduler::frame_count`) unsynchronized
4. `begin_frame()` called at end of `render()`, clears priority queue

## Targets

| Metric | Target |
|--------|--------|
| Frame rate | 60fps (16.67ms) |
| Layout (1000 nodes) | <5ms |
| Render (10000 cells) | <10ms |
| Input → event | <1ms |
