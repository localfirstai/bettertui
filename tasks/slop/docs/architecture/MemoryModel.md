# Memory Model

> The memory model defines how BetterTUI allocates, manages, and frees memory.
> It must be predictable, efficient, and leak-free.

## 1. Overview

BetterTUI uses a combination of:

- **Arena allocation** for nodes (bulk allocation, bulk deallocation).
- **Stack allocation** for small, fixed-size data.
- **Pool allocation** for frequently created/destroyed objects.
- **Garbage collection** (Node.js) for TypeScript objects.

```
TypeScript (GC-managed)
    ↓ FFI boundary
Rust (manual/arena-managed)
```

### 1.1 Memory Goals

| Metric | Target |
|--------|--------|
| Per-node memory | <256 bytes |
| Frame buffer memory | <1MB |
| Total engine memory | <10MB |
| Memory churn per frame | <100KB |
| Memory leaks | Zero |

## 2. Arena Allocation

### 2.1 Node Arena

The primary memory structure is the `NodeArena`, backed by a `slotmap::SlotMap`:

```rust
pub struct NodeArena {
    nodes: slotmap::SlotMap<NodeId, RenderNode>,
    // ...
}
```

**Characteristics:**
- **Bulk allocation:** All nodes are allocated in a contiguous region.
- **Bulk deallocation:** When the arena is dropped, all nodes are freed at once.
- **O(1) access:** Slotmap provides constant-time access by `NodeId`.
- **Generational indices:** Prevent use-after-free bugs.

### 2.2 Arena Sizing

The arena grows dynamically as nodes are added:

```
Initial: 256 node slots (~64KB)
Growth: 2x when full
Typical: 1000-5000 nodes (~256KB-1.28MB)
```

### 2.3 Arena Compaction

When many nodes are removed, the arena may have fragmented free slots. The slotmap handles this internally by reusing freed slots. No explicit compaction is needed.

## 3. Stack Allocation

### 3.1 Small Types on Stack

These types are always stack-allocated:

- `NodeId` — 8 bytes
- `Point` — 4 bytes (two u16)
- `Size` — 4 bytes (two u16)
- `Rect` — 8 bytes (x, y, width, height)
- `Color` — 5 bytes (enum)
- `Style` — ~16 bytes
- `LayoutProps` — ~48 bytes
- `Visibility` — 6 bytes
- `Transform` — 12 bytes
- `Overflow` — 1 byte
- `CursorProps` — ~8 bytes
- `FocusProps` — 6 bytes
- `CellAttributes` — 1 byte

### 3.2 Stack vs Heap

**Rule of thumb:** If a type is <128 bytes and has a known size at compile time, put it on the stack.

**Why:** Stack allocation is ~100x faster than heap allocation. For a 1000-node tree, stack allocation saves ~100μs per frame.

## 4. Heap Allocation

### 4.1 Box<T>

Used for:

- `Box<str>` — immutable strings (text content, metadata labels).
- `Box<Metadata>` — optional metadata.
- `Box<Accessibility>` — optional accessibility data.
- `Box<dyn Any>` — custom data.
- `Box<dyn Fn(...)>` — event handlers.

**Why Box, not String:** `Box<str>` is immutable and has no capacity overhead. A `String` has 24 bytes of overhead (pointer, length, capacity). A `Box<str>` has 16 bytes (pointer, length). For thousands of text nodes, this saves ~8KB.

### 4.2 SmallVec<T>

Used for:

- `SmallVec<[NodeId; 4]>` — children list (inline up to 4 children).
- `SmallVec<[Attribute; 4]>` — attributes list (inline up to 4 attributes).

**Why SmallVec:** Most nodes have <4 children. `SmallVec` stores these inline without heap allocation. Only nodes with >4 children allocate on the heap.

**Trade-off:** `SmallVec` is larger than `Vec` (inline storage + discriminant). For a `SmallVec<[NodeId; 4]>`, the size is 32 bytes (4 × 8 bytes for inline storage) + overhead. This is acceptable because the benefit of avoiding heap allocation outweighs the size increase.

### 4.3 Vec<T>

Used for:

- `Vec<Command>` — command buffer (pre-allocated, reused across frames).
- `Vec<DirtyRegion>` — dirty region list (pre-allocated).
- `Vec<u8>` — ANSI output buffer (pre-allocated).
- `Vec<RenderNode>` — render tree nodes (rebuilt each frame).

### 4.4 String vs Box<str>

| Type | Size | Mutable | Capacity | Use Case |
|------|------|---------|----------|----------|
| `&str` | 16 bytes | No | N/A | Borrowed string slices |
| `Box<str>` | 16 bytes | No | N/A | Owned immutable strings |
| `String` | 24 bytes | Yes | Yes | Mutable strings being built |

**Rule:** Use `Box<str>` for strings that won't change. Use `String` only when building strings (e.g., ANSI encoding).

## 5. Object Pooling

### 5.1 Node Pool

The slotmap internally manages a free list. When a node is removed, its slot is added to the free list. When a new node is created, a free slot is reused.

**Benefit:** No system allocation for node creation after the initial allocation.

### 5.2 Command Buffer Pool

```rust
pub struct CommandBufferPool {
    buffers: Vec<Vec<Command>>,
}

impl CommandBufferPool {
    pub fn acquire(&mut self) -> Vec<Command> {
        self.buffers.pop().unwrap_or_else(|| Vec::with_capacity(256))
    }

    pub fn release(&mut self, mut buffer: Vec<Command>) {
        buffer.clear();
        if self.buffers.len() < 16 {
            self.buffers.push(buffer);
        }
    }
}
```

**Benefit:** Avoids allocating a new `Vec<Command>` for every frame.

### 5.3 Frame Buffer Pool

```rust
pub struct FrameBufferPool {
    buffers: Vec<FrameBuffer>,
}
```

**Benefit:** Avoids allocating new frame buffers when resizing the terminal.

### 5.4 ANSI Buffer Pool

```rust
pub struct AnsiBufferPool {
    buffers: Vec<Vec<u8>>,
}
```

**Benefit:** Avoids allocating a new `Vec<u8>` for every frame's ANSI output.

## 6. Memory Layout

### 6.1 Node Memory Layout

A typical `RenderNode` has this memory layout:

```
Offset  Size  Field
0       8     id (NodeId)
8       1     kind (NodeKind)
9       8     parent (Option<NodeId>)
17      32    children (SmallVec<[NodeId; 4]>)
49      16    style (Style)
65      48    layout (LayoutProps)
113     16    text (Option<Box<str>>)
129     32    attrs (SmallVec<[Attribute; 4]>)
161     6     visibility (Visibility)
167     12    transform (Transform)
179     1     overflow (Overflow)
180     8     cursor (Option<CursorProps>)
188     6     focus (FocusProps)
194     48    events (EventHandlers)
242     13    state (NodeState)
255     8     metadata (Option<Box<Metadata>>)
263     8     accessibility (Option<Box<Accessibility>>)
271     8     custom_data (Option<Box<dyn Any>>)
─────────────
Total: ~279 bytes
```

### 6.2 Cell Memory Layout

A typical `Cell` has this memory layout:

```
Offset  Size  Field
0       4     char (CellChar enum)
4       5     fg (Color)
9       5     bg (Color)
14      5     underline_color (Color)
19      1     attributes (CellAttributes)
─────────────
Total: ~20 bytes
```

### 6.3 Cache Line Optimization

For hot paths (node iteration, cell comparison), we want data to fit in cache lines:

- **L1 cache line:** 64 bytes
- **L2 cache line:** 128 bytes

A cell (20 bytes) fits 3 cells per cache line. A node (~279 bytes) fits ~0.23 nodes per cache line.

**Optimization:** For node iteration, sort nodes by layout order (top-to-bottom, left-to-right) to improve spatial locality.

## 7. Ownership Rules

### 7.1 Rust Ownership

```
NodeArena owns all RenderNodes.
RenderNodes own their Style, LayoutProps, Text, etc.
Option<Box<T>> is owned by the node.
EventHandlers are owned by the node.
```

### 7.2 Borrowing Rules

```
&RenderNode — shared borrow (read-only)
&mut RenderNode — exclusive borrow (read-write)
```

**Rule:** Never hold a `&mut RenderNode` while iterating the arena. This causes borrow conflicts. Instead:

```rust
// Wrong
for node in arena.iter_mut() {
    node.style.bold = Some(true); // conflicts with arena borrow
}

// Right
let dirty_nodes: Vec<NodeId> = arena.iter()
    .filter(|(_, node)| node.layout_dirty)
    .map(|(id, _)| id)
    .collect();

for id in dirty_nodes {
    if let Some(node) = arena.get_mut(id) {
        node.style.bold = Some(true);
    }
}
```

### 7.3 Cross-FFI Ownership

Data crossing the FFI boundary is **copied**, not shared:

```
TypeScript → [serialize] → [copy] → Rust → [deserialize] → Rust arena
```

This prevents data races and simplifies ownership semantics.

## 8. Memory Leaks

### 8.1 Leak Prevention

- **Arena drop:** When the arena is dropped, all nodes are freed.
- **Box drop:** When a `Box<T>` is dropped, the heap memory is freed.
- **Vec drop:** When a `Vec<T>` is dropped, the heap memory is freed.
- **Slotmap drop:** When the slotmap is dropped, all slots are freed.

### 8.2 Leak Detection

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_memory_leaks() {
        let mut arena = NodeArena::new();
        let mut allocator = AllocatorTracker::new();

        // Create 1000 nodes
        for _ in 0..1000 {
            let node = RenderNode::default();
            arena.insert(node);
        }

        // Remove all nodes
        let root = arena.root();
        arena.remove_subtree(root);

        // Check for leaks
        assert_eq!(allocator.allocated(), 0);
    }
}
```

### 8.3 Leak Budget

| Source | Budget | Notes |
|--------|--------|-------|
| Node arena | 0 bytes | Freed when dropped |
| Frame buffer | 0 bytes | Freed when dropped |
| Command buffer | 0 bytes | Freed when dropped |
| ANSI buffer | 0 bytes | Freed when dropped |
| Event handlers | 0 bytes | Freed when dropped |
| **Total** | **0 bytes** | No leaks allowed |

## 9. Performance

### 9.1 Allocation Performance

| Operation | Time | Notes |
|-----------|------|-------|
| Stack allocation | ~1ns | Compiler-optimized |
| Arena insert | ~100ns | Slotmap allocation |
| Box allocation | ~50ns | System allocator |
| Vec allocation | ~100ns | System allocator |
| Slotmap access | ~10ns | O(1) index |

### 9.2 Deallocation Performance

| Operation | Time | Notes |
|-----------|------|-------|
| Stack deallocation | ~0ns | Compiler-optimized |
| Arena drop (all) | ~1ms | Bulk deallocation |
| Box drop | ~50ns | System allocator |
| Vec drop | ~50ns | System allocator |

### 9.3 Memory Bandwidth

For a 1000-node tree:
- Node iteration: ~280KB (1000 × 279 bytes)
- L2 cache size: 256KB-1MB
- Nodes fit in L2: ~900-3600 nodes

For most trees, node data fits in L2 cache. This is important for iteration performance.

## 10. Future Considerations

### 10.1 Custom Allocator

Replace the system allocator with a custom allocator for better performance:

- **Arena allocator:** For bulk allocation of small objects.
- **Bump allocator:** For frame-local allocations.
- **Pool allocator:** For fixed-size objects.

### 10.2 Memory-Mapped Storage

For very large trees (100,000+ nodes), use memory-mapped files:

```
Node storage → mmap file → disk
```

This allows trees larger than available RAM.

### 10.3 Compression

Compress node data in memory:

- **Delta encoding:** Store only differences from a template.
- **Dictionary encoding:** Replace common patterns with short codes.
- **Bit packing:** Pack boolean fields into bits.

### 10.4 Garbage Collection Integration

For TypeScript objects that reference Rust data, integrate with Node.js garbage collector:

```rust
#[napi]
struct NodeRef {
    arena: Arc<Mutex<NodeArena>>,
    id: NodeId,
}

impl Drop for NodeRef {
    fn drop(&mut self) {
        // Notify Rust that this reference is no longer needed
        self.arena.lock().release_ref(self.id);
    }
}
```

This prevents Rust from holding references to nodes that TypeScript has garbage collected.
