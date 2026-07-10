# Performance

> Performance is a first-class feature, not an afterthought.
> BetterTUI must be fast enough for 60fps interactive applications.

## 1. Performance Goals

### 1.1 Frame Rate

| Target | Condition |
|--------|-----------|
| 60fps (16.67ms) | Normal operation |
| 30fps (33.33ms) | Complex layouts (>1000 nodes) |
| 15fps (66.67ms) | Extreme cases (>5000 nodes) |

### 1.2 Latency

| Metric | Target | Notes |
|--------|--------|-------|
| Input to render | <16ms | Keystroke appears on screen |
| Input to event | <1ms | Event reaches handler |
| Layout calculation | <5ms | For 1000 nodes |
| Full frame render | <10ms | For 10000 cells |
| Terminal write | <5ms | For 100KB of ANSI data |

### 1.3 Memory

| Metric | Target | Notes |
|--------|--------|-------|
| Per-node | <256 bytes | Including all fields |
| Frame buffer | <1MB | For 200×50 terminal |
| Total engine | <10MB | For typical application |
| Memory churn/frame | <100KB | Allocations + deallocations |

## 2. Optimization Strategies

### 2.1 Dirty Tracking

Only process what has changed:

```
Node changed → set dirty flag → propagate to ancestors
    ↓
Layout recalculation (only dirty subtrees)
    ↓
Render (only dirty nodes)
    ↓
Diff (only changed cells)
    ↓
Encode (only dirty regions)
```

**Impact:** Reduces work from O(n) to O(k) where k is the number of changed elements.

### 2.2 Layout Caching

Cache layout results and only recalculate when inputs change:

```
Layout cache hit → skip calculation
Layout cache miss → calculate + cache
```

**Impact:** For 1000 nodes with 5 changes, reduces layout time from 1ms to 0.05ms.

### 2.3 Batch Operations

Group multiple operations into single FFI calls:

```
100 individual FFI calls: 100 × 100ns = 10μs
1 batched FFI call: 100ns + processing = ~0.5ms
```

**Impact:** 20x reduction in FFI overhead.

### 2.4 Frame Buffer Diffing

Only write changed cells to the terminal:

```
Full repaint: 200 × 50 = 10,000 cells → 100KB ANSI data
Diff repaint: 100 cells → 1KB ANSI data
```

**Impact:** 100x reduction in terminal I/O for typical frames.

### 2.5 Style Coalescing

Merge adjacent cells with the same style:

```
Without coalescing: 10 SGR sequences for 10 cells
With coalescing: 1 SGR sequence for 10 cells
```

**Impact:** 10x reduction in ANSI output size.

### 2.6 Early Exit

Skip work when nothing has changed:

```
No dirty nodes → skip layout
No dirty cells → skip diff
No diff changes → skip encode + write
```

**Impact:** Reduces idle frame time from 2ms to 0.01ms.

## 3. Profiling

### 3.1 Frame Timing

```rust
pub struct FrameProfiler {
    frame_start: Instant,
    layout_start: Option<Instant>,
    render_start: Option<Instant>,
    diff_start: Option<Instant>,
    encode_start: Option<Instant>,
    write_start: Option<Instant>,
    stats: FrameStats,
}

impl FrameProfiler {
    pub fn start_frame(&mut self) {
        self.frame_start = Instant::now();
    }

    pub fn start_layout(&mut self) {
        self.layout_start = Some(Instant::now());
    }

    pub fn end_layout(&mut self) {
        if let Some(start) = self.layout_start.take() {
            self.stats.layout_time_ms = start.elapsed().as_secs_f64() * 1000.0;
        }
    }

    // ... similar for render, diff, encode, write

    pub fn end_frame(&mut self) {
        self.stats.duration_ms = self.frame_start.elapsed().as_secs_f64() * 1000.0;
    }
}
```

### 3.2 Profiling Points

| Point | What It Measures |
|-------|-----------------|
| Command processing | Time to process all commands in a batch |
| Layout | Time to calculate layout for dirty nodes |
| Render tree | Time to build the render tree |
| Frame buffer | Time to render nodes to the frame buffer |
| Diff | Time to diff current and previous frames |
| Encode | Time to encode dirty cells as ANSI |
| Write | Time to write ANSI data to terminal |
| **Total** | Total frame time |

### 3.3 Profiling Output

```json
{
  "frame": 1234,
  "duration_ms": 3.2,
  "layout_ms": 1.1,
  "render_ms": 0.5,
  "diff_ms": 0.2,
  "encode_ms": 0.3,
  "write_ms": 0.8,
  "nodes_layouted": 150,
  "nodes_rendered": 200,
  "cells_changed": 50,
  "bytes_written": 1024
}
```

### 3.4 Performance Budgets

| Stage | Budget | Over Budget Action |
|-------|--------|-------------------|
| Command processing | 1ms | Log warning |
| Layout | 3ms | Log warning |
| Render | 2ms | Log warning |
| Diff | 1ms | Log warning |
| Encode | 1ms | Log warning |
| Write | 2ms | Log warning |
| **Total** | **10ms** | Drop to 30fps |

## 4. Benchmarking

### 4.1 Micro-Benchmarks

```rust
#[bench]
fn bench_node_creation(b: &mut Bencher) {
    let mut arena = NodeArena::new();
    b.iter(|| {
        arena.insert(RenderNode::default());
    });
}

#[bench]
fn bench_node_access(b: &mut Bencher) {
    let mut arena = NodeArena::new();
    let id = arena.insert(RenderNode::default());
    b.iter(|| {
        arena.get(id);
    });
}

#[bench]
fn bench_layout_1000_nodes(b: &mut Bencher) {
    let arena = create_tree(1000);
    let mut layout_engine = LayoutEngine::new();
    b.iter(|| {
        layout_engine.compute_layout(&arena);
    });
}
```

### 4.2 Integration Benchmarks

```rust
#[bench]
fn bench_full_frame_100_nodes(b: &mut Bencher) {
    let mut engine = create_engine(100);
    b.iter(|| {
        engine.process_commands();
        engine.compute_layout();
        engine.render_frame();
        engine.diff_and_encode();
    });
}
```

### 4.3 Regression Benchmarks

Run benchmarks on every commit to detect performance regressions:

```yaml
# .github/workflows/benchmark.yml
on: [push, pull_request]
jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo bench --workspace 2>&1 | tee bench.txt
      - uses: benchmark-action/github-action-benchmark@v1
        with:
          tool: cargo
          output-file-path: bench.txt
          auto-push: true
```

## 5. Optimization Opportunities

### 5.1 SIMD for Cell Comparison

Compare multiple cells at once using SIMD:

```rust
#[cfg(target_arch = "x86_64")]
fn cells_equal_simd(a: &[Cell], b: &[Cell]) -> bool {
    unsafe {
        let a_ptr = a.as_ptr() as *const __m256i;
        let b_ptr = b.as_ptr() as *const __m256i;
        let len = a.len() / 4; // 4 cells per 256-bit register

        for i in 0..len {
            let va = _mm256_loadu_si256(a_ptr.add(i));
            let vb = _mm256_loadu_si256(b_ptr.add(i));
            let eq = _mm256_cmpeq_epi8(va, vb);
            if _mm256_movemask_epi8(eq) != 0xFFFF {
                return false;
            }
        }
        true
    }
}
```

**Impact:** 4x faster cell comparison for large frame buffers.

### 5.2 Parallel Layout

Use rayon for parallel layout calculation:

```rust
use rayon::prelude::*;

fn parallel_layout(arena: &NodeArena, roots: &[NodeId]) {
    roots.par_iter().for_each(|&root| {
        // Each subtree is laid out independently
        compute_subtree_layout(arena, root);
    });
}
```

**Impact:** 2-4x speedup for trees with multiple independent subtrees.

### 5.3 Incremental Diff

Only diff regions that were written to:

```rust
fn incremental_diff(
    front: &FrameBuffer,
    back: &FrameBuffer,
    written_regions: &[Rect],
) -> Vec<DirtyRegion> {
    let mut dirty = Vec::new();
    for region in written_regions {
        dirty.extend(diff_region(front, back, *region));
    }
    dirty
}
```

**Impact:** Reduces diff time from O(w×h) to O(written_cells).

### 5.4 ANSI Output Compression

Compress ANSI output for large frames:

```rust
fn compress_ansi(output: &[u8]) -> Vec<u8> {
    // Use run-length encoding for repeated characters
    // Use delta encoding for repeated styles
    // Use dictionary encoding for common sequences
}
```

**Impact:** Reduces terminal I/O for large frames.

## 6. Performance Anti-Patterns

### 6.1 Layout Thrashing

**Problem:** Reading layout values before writing causes recalculation.

```rust
// Wrong
let height = node.layout.height;
node.layout.height = height + 1; // triggers recalculation

// Right
node.layout.height += 1; // single write
```

### 6.2 Excessive Allocations

**Problem:** Creating new objects every frame.

```rust
// Wrong
let buffer = vec![0u8; 1024]; // allocated every frame

// Right
static BUFFER: RefCell<Vec<u8>> = RefCell::new(Vec::new());
let mut buffer = BUFFER.borrow_mut();
buffer.clear();
buffer.resize(1024, 0);
```

### 6.3 Unnecessary Cloning

**Problem:** Cloning large data structures.

```rust
// Wrong
let node = arena.get(id).unwrap().clone(); // clones entire node

// Right
let style = arena.get(id).unwrap().style; // copies small struct
```

### 6.4 Blocking the Event Loop

**Problem:** Long-running operations on the main thread.

```rust
// Wrong
let data = std::fs::read_to_string("large_file.txt")?; // blocks event loop

// Right
let data = tokio::fs::read_to_string("large_file.txt").await?; // async
```

## 7. Performance Monitoring

### 7.1 Runtime Metrics

```rust
pub struct PerformanceMetrics {
    pub frames_per_second: f64,
    pub average_frame_time_ms: f64,
    pub max_frame_time_ms: f64,
    pub layout_time_ms: f64,
    pub render_time_ms: f64,
    pub memory_usage_bytes: usize,
    pub node_count: usize,
    pub dirty_node_count: usize,
}
```

### 7.2 Metrics Collection

```rust
impl PerformanceMetrics {
    pub fn collect(&mut self, profiler: &FrameProfiler, arena: &NodeArena) {
        self.frames_per_second = 1000.0 / profiler.stats.duration_ms;
        self.average_frame_time_ms = profiler.average_frame_time();
        self.max_frame_time_ms = profiler.max_frame_time();
        self.node_count = arena.len();
        self.dirty_node_count = arena.iter()
            .filter(|(_, n)| n.state.dirty)
            .count();
    }
}
```

### 7.3 Metrics Export

Metrics can be exported to:

- **Console:** For debugging.
- **File:** For post-analysis.
- **Network:** For remote monitoring (DevTools).
- **Prometheus:** For production monitoring.

## 8. Future Considerations

### 8.1 Adaptive Quality

Automatically reduce quality when frame budget is exceeded:

```rust
fn adaptive_quality(frame_time_ms: f64) -> QualityLevel {
    if frame_time_ms < 10.0 {
        QualityLevel::High
    } else if frame_time_ms < 16.0 {
        QualityLevel::Medium
    } else {
        QualityLevel::Low
    }
}
```

Quality levels affect:
- **High:** Full rendering, all animations.
- **Medium:** Reduced animations, simplified borders.
- **Low:** Minimal rendering, no animations.

### 8.2 Predictive Prefetching

Predict which nodes will be needed and prefetch them:

```rust
fn prefetch_nodes(arena: &NodeArena, viewport: &Rect) {
    // Prefetch nodes that will enter the viewport soon
    let predicted_viewport = viewport.offset(scroll_velocity);
    for node_id in arena.descendants(arena.root()) {
        if intersects(predicted_viewport, node_id.layout.rect()) {
            prefetch(node_id);
        }
    }
}
```

### 8.3 JIT Compilation

For performance-critical paths, JIT compile render routines:

```rust
fn compile_render_routine(nodes: &[NodeId]) -> Box<dyn Fn(&NodeArena, &mut FrameBuffer)> {
    // Generate optimized machine code for rendering specific node patterns
}
```

This is a far-future optimization for when BetterTUI has thousands of users and performance is critical.
