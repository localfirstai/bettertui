# Threading

> The threading model determines how BetterTUI uses CPU cores.
> It must be correct, performant, and simple to reason about.

## 1. Overview

BetterTUI currently runs on a **single thread** (Node.js main thread). The architecture is designed for future multi-threading without requiring fundamental changes.

```
Current: Single-threaded
┌─────────────────────────────────────┐
│ Main Thread (Node.js event loop)    │
│  [Reconcile] → [Layout] → [Render] │
│  → [Diff] → [Encode] → [Write]     │
└─────────────────────────────────────┘

Future: Multi-threaded
┌──────────────────┐  ┌──────────────┐  ┌──────────────┐
│ Main Thread      │  │ Worker 1     │  │ Worker 2     │
│ [Reconcile]      │  │ [Layout]     │  │ [Render]     │
│ [FFI]            │  │ [Diff]       │  │ [Encode]     │
│ [Event Dispatch] │  │ [Encode]     │  │ [Write]      │
└──────────────────┘  └──────────────┘  └──────────────┘
```

### 1.1 Why Single-Threaded First?

Multi-threading adds complexity:
- **Synchronization:** Shared state requires locks or message passing.
- **Race conditions:** Concurrent mutations can corrupt data.
- **Deadlocks:** Lock ordering bugs can freeze the application.
- **Debugging:** Multi-threaded code is harder to debug.

Starting single-threaded allows us to:
- Get the architecture right without threading concerns.
- Profile and identify actual bottlenecks.
- Add threading only where it provides measurable benefit.

## 2. Thread Safety

### 2.1 Send + Sync

All types that cross thread boundaries must be `Send + Sync`:

```rust
pub trait Send: Sized {}
pub trait Sync: Sync {}
```

- `Send` — safe to transfer ownership to another thread.
- `Sync` — safe to share references between threads.

### 2.2 Thread-Safe Types

| Type | Send | Sync | Notes |
|------|------|------|-------|
| `NodeId` | Yes | Yes | Copy type, no shared state |
| `RenderNode` | Yes | Yes | Owned data, no interior mutability |
| `NodeArena` | Yes | No | Mutable access requires exclusive ownership |
| `Style` | Yes | Yes | Pure data |
| `LayoutProps` | Yes | Yes | Pure data |
| `EventHandlers` | No | No | Contains `Box<dyn Fn>` (not `Send`) |
| `FrameBuffer` | Yes | No | Mutable access requires exclusive ownership |

### 2.3 Why EventHandlers Are Not Send

Event handlers contain closures that may capture non-Send types (like `Rc`, raw pointers). To make them Send, we would need:

1. **Restrict closures to `Send` closures.** This limits what handlers can capture.
2. **Use `Arc<Mutex<T>>` for shared state.** This adds synchronization overhead.
3. **Use message passing.** Handlers send messages instead of capturing state.

For now, event handlers stay on the main thread. This is acceptable because event handling is fast (<0.1ms) and doesn't benefit from parallelism.

## 3. Parallelism Opportunities

### 3.1 Layout Parallelism

Taffy supports parallel layout calculation:

```rust
// Sequential (current)
taffy.compute_layout(root, constraints)?;

// Parallel (future)
taffy.compute_layout_parallel(root, constraints)?;
```

**When to parallelize:** For trees with >500 nodes, parallel layout provides measurable speedup. For smaller trees, the overhead of thread synchronization exceeds the benefit.

### 3.2 Render Parallelism

Different rows of the frame buffer can be rendered in parallel:

```rust
// Sequential
for y in 0..height {
    render_row(y, &render_tree, &mut buffer);
}

// Parallel
(0..height).into_par_iter().for_each(|y| {
    render_row(y, &render_tree, &mut buffer);
});
```

**Caveat:** Rows must be independent. If a node spans multiple rows, its rendering must be synchronized.

### 3.3 Diff Parallelism

Different regions of the frame buffer can be diffed in parallel:

```rust
// Divide frame buffer into tiles
let tiles = divide_into_tiles(&front, &back, 8, 8);

// Diff each tile in parallel
let dirty_tiles: Vec<_> = tiles.into_par_iter()
    .map(|tile| diff_tile(tile))
    .filter(|tile| !tile.is_clean())
    .collect();
```

### 3.4 Encode Parallelism

Different rows of dirty cells can be encoded in parallel:

```rust
// Sequential
let mut output = Vec::new();
for row in dirty_rows {
    encode_row(row, &mut output);
}

// Parallel
let row_outputs: Vec<_> = dirty_rows.into_par_iter()
    .map(|row| {
        let mut output = Vec::new();
        encode_row(row, &mut output);
        output
    })
    .collect();

// Concatenate (maintaining order)
let output: Vec<u8> = row_outputs.into_iter().flatten().collect();
```

## 4. Synchronization

### 4.1 Mutex

```rust
use parking_lot::Mutex;

let arena = Mutex::new(NodeArena::new());

// Exclusive access
{
    let mut arena = arena.lock();
    arena.insert(node);
} // lock released here

// Shared access
let node = arena.lock().get(id).cloned();
```

### 4.2 RwLock

```rust
use parking_lot::RwLock;

let arena = RwLock::new(NodeArena::new());

// Shared read (multiple readers allowed)
{
    let arena = arena.read();
    let node = arena.get(id);
} // read lock released here

// Exclusive write (only one writer)
{
    let mut arena = arena.write();
    arena.insert(node);
} // write lock released here
```

**When to use RwLock:** When reads are much more frequent than writes. The frame buffer is a good candidate — it's read during diffing and encoding, but only written during rendering.

### 4.3 Channel

```rust
use tokio::sync::mpsc;

let (tx, rx) = mpsc::channel(100);

// Sender (main thread)
tx.send(event).await?;

// Receiver (worker thread)
while let Some(event) = rx.recv().await {
    process_event(event);
}
```

**When to use channels:** For event-driven communication between threads. The input thread sends events to the main thread via a channel.

### 4.4 Atomic

```rust
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

let dirty = AtomicBool::new(false);
let generation = AtomicU64::new(0);

// Check
if dirty.load(Ordering::Relaxed) {
    // Process dirty nodes
}

// Set
dirty.store(true, Ordering::Relaxed);
generation.fetch_add(1, Ordering::Relaxed);
```

**When to use atomics:** For simple flags and counters that are read/written from multiple threads without complex synchronization.

## 5. Message Passing

### 5.1 Command Channel

Commands from TypeScript are sent to Rust via a channel:

```rust
// TypeScript side (napi-rs)
fn process_commands(commands: Vec<Command>) {
    // Commands are serialized and sent via FFI
    // (napi-rs handles the channel internally)
}

// Rust side
fn command_loop(rx: Receiver<Command>, arena: &mut NodeArena) {
    while let Some(command) = rx.recv() {
        process_command(command, arena);
    }
}
```

### 5.2 Event Channel

Events from Rust are sent to TypeScript via a callback:

```rust
// Rust side
fn event_loop(event_tx: Sender<Event>) {
    loop {
        let event = read_input();
        event_tx.send(event).unwrap();
    }
}

// TypeScript side (napi-rs callback)
fn on_event(event: Event) {
    // Process event in TypeScript
}
```

### 5.3 Render Channel

Render commands are sent to a dedicated render thread:

```rust
// Main thread
let render_commands = collect_render_commands(arena);
render_tx.send(render_commands).unwrap();

// Render thread
while let Some(commands) = render_rx.recv() {
    render_frame(commands);
}
```

## 6. Future Threading Model

### 6.1 Proposed Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Main Thread                           │
│  [Event Loop] → [Command Processing] → [Orchestration]  │
└──────────┬──────────────┬──────────────┬────────────────┘
           │              │              │
     ┌─────▼─────┐  ┌────▼─────┐  ┌────▼─────┐
     │ Layout    │  │ Render   │  │ I/O      │
     │ Thread    │  │ Thread   │  │ Thread   │
     │           │  │          │  │          │
     │ [Taffy]   │  │ [Buffer] │  │ [stdin]  │
     │ [Cache]   │  │ [Diff]   │  │ [stdout] │
     │           │  │ [ANSI]   │  │ [signal] │
     └───────────┘  └──────────┘  └──────────┘
```

### 6.2 Thread Responsibilities

| Thread | Responsibilities | Data Access |
|--------|-----------------|-------------|
| Main | Event loop, command processing, orchestration | Full access (exclusive) |
| Layout | Taffy layout calculation | Read-only arena |
| Render | Frame buffer rendering, diffing, ANSI encoding | Read-only arena, exclusive frame buffer |
| I/O | Terminal input/output, signal handling | None (sends events/commands via channels) |

### 6.3 Synchronization Protocol

1. **Main thread** processes commands and updates the arena.
2. **Main thread** signals layout thread: "arena is ready for layout."
3. **Layout thread** calculates layout (read-only on arena).
4. **Layout thread** signals main thread: "layout is complete."
5. **Main thread** signals render thread: "render this frame."
6. **Render thread** renders and writes to terminal.
7. **I/O thread** reads input and sends events to main thread.

### 6.4 Double Buffering for Arena

To allow concurrent read/write access to the arena:

```
Frame N:
  Write arena: Main thread processes commands
  Read arena: Layout thread reads for layout

Frame N+1:
  Write arena: Main thread processes commands
  Read arena: Layout thread reads for layout
```

The arena is **double-buffered**: one copy for writing, one copy for reading. After layout is complete, the write buffer is copied to the read buffer.

**Trade-off:** Doubling arena memory. For 1000 nodes × 256 bytes = 256KB per buffer, total = 512KB. Acceptable.

## 7. Error Handling

### 7.1 Thread Panic

If a worker thread panics:

1. The panic is caught by the thread wrapper.
2. An error is sent to the main thread.
3. The main thread logs the error and recreates the worker thread.
4. The application continues (with degraded performance).

### 7.2 Deadlock Detection

Use `parking_lot`'s deadlock detection:

```rust
#[cfg(debug_assertions)]
let config = parking_lot::Mutex::new(config).deadlock_detection(true);
```

### 7.3 Data Race Prevention

Rust's type system prevents data races at compile time:

- `Send + Sync` bounds ensure safe cross-thread sharing.
- `Mutex` and `RwLock` provide runtime synchronization.
- `Cell` and `RefCell` prevent shared mutable state within a thread.

## 8. Performance

### 8.1 Single-Thread Performance

| Operation | Time | Notes |
|-----------|------|-------|
| Command processing | 0.3ms | 100 commands |
| Layout | 1ms | 1000 nodes |
| Render | 0.5ms | 10,000 cells |
| Diff | 0.2ms | 10,000 cells |
| ANSI encode | 0.3ms | 10,000 cells |
| Terminal write | 0.5ms | 100KB |
| **Total** | **2.8ms** | Single-threaded |

### 8.2 Multi-Thread Performance (Projected)

| Operation | Time | Notes |
|-----------|------|-------|
| Command processing | 0.3ms | Main thread |
| Layout | 0.3ms | 4 threads, 3x speedup |
| Render | 0.2ms | 4 threads, 2.5x speedup |
| Diff | 0.1ms | 4 threads, 2x speedup |
| ANSI encode | 0.1ms | 4 threads, 3x speedup |
| Terminal write | 0.5ms | I/O bound, no speedup |
| **Total** | **1.5ms** | Multi-threaded |

### 8.3 When to Add Threading

Add threading when:
- Profiling shows >50% CPU usage on a single core.
- Layout or render time exceeds 5ms consistently.
- The application targets high-refresh-rate displays (120fps+).

Do NOT add threading preemptively. It adds complexity without measurable benefit for most applications.

## 9. Future Considerations

### 9.1 GPU Thread

A dedicated GPU thread for hardware-accelerated rendering:

```
Main Thread → GPU Thread → Terminal
```

The GPU thread uses Vulkan, Metal, or DirectX for rendering. The terminal receives pre-rendered frames as ANSI or image protocols (iTerm2, Kitty).

### 9.2 Plugin Thread

Plugins can run in dedicated threads for isolation:

```
Main Thread → Plugin Thread → Plugin
```

This prevents buggy plugins from crashing the main application.

### 9.3 Network Thread

A dedicated network thread for remote rendering:

```
Main Thread → Network Thread → WebSocket → Remote Client
```

The network thread handles serialization and transmission of render commands.
