# Rendering Pipeline

> The rendering pipeline transforms a node tree into terminal output.
> Every stage has clear inputs, outputs, ownership, and performance characteristics.

## 1. Pipeline Overview

```
React Component Tree
    ↓ [1. Reconciliation]
Virtual DOM Diff
    ↓ [2. Adapter Translation]
Node Operations (create, append, setStyle, etc.)
    ↓ [3. Protocol Encoding]
Command Buffer (binary)
    ↓ [4. FFI Transfer]
Rust Command Queue
    ↓ [5. Command Processing]
Node Arena Updated
    ↓ [6. Layout Calculation]
Layout Results (position, size per node)
    ↓ [7. Render Tree Construction]
Render Tree (visible nodes with resolved styles)
    ↓ [8. Frame Buffer Rendering]
Frame Buffer (cell grid)
    ↓ [9. Frame Diffing]
Dirty Cell List
    ↓ [10. ANSI Encoding]
ANSI Escape Sequences
    ↓ [11. Terminal Output]
stdout
```

## 2. Stage Details

### 2.1 Reconciliation (Framework-Specific)

**Input:** React component tree (or Vue, Solid, etc.)
**Output:** Virtual DOM diff (added, removed, updated nodes)
**Owner:** Framework (React reconciler, Vue reactivity, etc.)
**Threading:** Main thread (Node.js event loop)
**Performance:** O(n) where n is the number of changed nodes

This stage is entirely framework-specific. React uses its reconciler. Vue uses its proxy-based reactivity. Solid uses fine-grained signals. BetterTUI does not control or depend on this stage.

### 2.2 Adapter Translation

**Input:** Virtual DOM diff
**Output:** Node operations (createNode, appendChild, setStyle, etc.)
**Owner:** Framework adapter (`@bettertui/react`, `@bettertui/vue`, etc.)
**Threading:** Main thread
**Performance:** O(n) where n is the number of changed nodes

The adapter translates framework-specific operations into generic node operations:

```
React: <Box flexDirection="row"><Text>Hello</Text></Box>
    ↓
Operations:
  createNode(boxId, "box")
  createNode(textId, "text")
  setStyle(textId, { bold: true })
  setLayout(boxId, { direction: "row" })
  setText(textId, "Hello")
  appendChild(rootId, boxId)
  appendChild(boxId, textId)
```

### 2.3 Protocol Encoding

**Input:** Node operations
**Output:** Command buffer (binary)
**Owner:** `@bettertui/core` (ProtocolCodec)
**Threading:** Main thread
**Performance:** O(n) where n is the number of operations, with small constant factor

Operations are batched and serialized into a compact binary format. See [Protocol.md](Protocol.md) for details.

### 2.4 FFI Transfer

**Input:** Command buffer (TypeScript Uint8Array)
**Output:** Command buffer (Rust Vec\<u8\>)
**Owner:** napi-rs binding layer
**Threading:** Main thread → crosses FFI boundary
**Performance:** ~0.1ms per call (amortized with batching)

napi-rs handles the memory transfer. TypeScript's Uint8Array is copied into Rust's Vec\<u8\>. This copy is necessary because JavaScript and Rust have different memory managers.

### 2.5 Command Processing

**Input:** Command buffer (Rust)
**Output:** Updated node arena
**Owner:** `bettertui-engine` (command processor)
**Threading:** Main thread (or dedicated command thread, see Threading.md)
**Performance:** O(n) where n is the number of commands

Commands are deserialized and applied to the node arena:

1. **Create commands** allocate new nodes in the arena.
2. **Tree commands** update parent-child relationships.
3. **Property commands** update node fields (style, layout, text, etc.).
4. **Dirty flags** are set on affected nodes and propagated to ancestors.

### 2.6 Layout Calculation

**Input:** Node arena (with dirty flags)
**Output:** Layout results (position and size for each node)
**Owner:** `bettertui-engine` (layout engine)
**Threading:** Main thread (or parallel, see Threading.md)
**Performance:** O(n) for full layout, O(k) for incremental where k is the number of dirty nodes

Layout uses Taffy (CSS flexbox engine) to calculate positions and sizes:

1. **Dirty subtree identification:** Only nodes with `layout_dirty = true` are recalculated.
2. **Constraint propagation:** Root receives terminal dimensions. Each node computes constraints for its children.
3. **Measurement:** Leaf nodes measure their content (text length, etc.).
4. **Flex calculation:** Flex containers distribute remaining space among children.
5. **Position computation:** Final positions are computed from sizes and alignment.

See [Layout.md](Layout.md) for detailed layout algorithm documentation.

### 2.7 Render Tree Construction

**Input:** Node arena + layout results
**Output:** Render tree (visible nodes with resolved styles)
**Owner:** `bettertui-engine` (render tree builder)
**Threading:** Main thread
**Performance:** O(n) where n is the number of visible nodes

The render tree is a filtered, flattened view of the node tree:

1. **Visibility filtering:** Nodes with `display: none` are excluded.
2. **Opacity resolution:** Parent opacity is propagated to children.
3. **Style resolution:** Inherited styles are resolved (child overrides parent).
4. **Clip region computation:** Overflow clipping regions are calculated.
5. **Z-order sorting:** Nodes with z-index > 0 are sorted for correct layering.

### 2.8 Frame Buffer Rendering

**Input:** Render tree + layout results
**Output:** Frame buffer (cell grid)
**Owner:** `bettertui-engine` (renderer)
**Threading:** Main thread (or parallel row rendering, see Threading.md)
**Performance:** O(n) where n is the number of visible cells

The renderer walks the render tree and writes cells to the frame buffer:

1. **Clear:** If the entire frame is dirty, clear the buffer.
2. **Background fill:** Fill node backgrounds (colors, borders).
3. **Text rendering:** Write text characters with their styles.
4. **Border rendering:** Draw borders (single, double, rounded, thick).
5. **Overlay rendering:** Render high z-index nodes on top.
6. **Cursor rendering:** Render the cursor at the focused input's position.
7. **Selection rendering:** Render text selection highlights.

See [FrameBuffer.md](FrameBuffer.md) for frame buffer details.

### 2.9 Frame Diffing

**Input:** Current frame buffer + previous frame buffer
**Output:** Dirty cell list (cells that changed)
**Owner:** `bettertui-engine` (frame differ)
**Threading:** Main thread (or parallel, see Threading.md)
**Performance:** O(w × h) where w × h is the terminal size

The differ compares current and previous frame buffers cell by cell:

1. **Cell comparison:** Each cell is compared for character, foreground, background, and attributes.
2. **Dirty cell collection:** Changed cells are collected into a list.
3. **Dirty region merging:** Adjacent dirty cells are merged into rectangular regions.
4. **Optimization:** If the entire frame is dirty, skip comparison and emit a full repaint.

**Optimization — early exit:** If the node arena's generation hasn't changed since the last frame, skip diffing entirely. No changes means no dirty cells.

### 2.10 ANSI Encoding

**Input:** Dirty cell list
**Output:** ANSI escape sequences
**Owner:** `bettertui-engine` (ANSI encoder)
**Threading:** Main thread
**Performance:** O(d) where d is the number of dirty cells

The encoder translates dirty cells into ANSI escape sequences:

1. **Cursor positioning:** Move cursor to the first dirty cell.
2. **Style application:** Emit SGR (Select Graphic Rendition) sequences for colors and attributes.
3. **Character output:** Emit the character for each dirty cell.
4. **Style reset:** Reset styles after each style change.
5. **Cursor hide/show:** Hide cursor during rendering, show at final position.

**Optimization — style coalescing:** Adjacent cells with the same style share a single SGR sequence. This reduces output volume significantly.

**Optimization — move optimization:** Cursor movements are minimized by processing cells in reading order (left-to-right, top-to-bottom) and using relative movements when possible.

### 2.11 Terminal Output

**Input:** ANSI escape sequences
**Output:** bytes written to stdout
**Owner:** `bettertui-engine` (terminal writer)
**Threading:** Main thread
**Performance:** O(b) where b is the number of bytes written

The terminal writer flushes ANSI sequences to stdout:

1. **Buffered write:** ANSI sequences are collected into a write buffer.
2. **Single write:** The entire buffer is written in one `write()` syscall.
3. **Flush:** stdout is flushed to ensure immediate display.

**Optimization — write coalescing:** Multiple small writes are coalesced into a single large write. This reduces syscall overhead.

## 3. Ownership Model

| Stage | Owns | Borrows |
|-------|------|---------|
| Reconciliation | React virtual DOM | — |
| Adapter Translation | Command buffer (TypeScript) | React virtual DOM |
| Protocol Encoding | Serialized bytes | Command buffer |
| FFI Transfer | Rust command buffer | Serialized bytes |
| Command Processing | Node arena mutations | Node arena |
| Layout | Layout results | Node arena |
| Render Tree | Render tree nodes | Node arena, layout results |
| Frame Buffer | Frame buffer cells | Render tree |
| Frame Diffing | Dirty cell list | Current + previous frame buffers |
| ANSI Encoding | ANSI byte buffer | Dirty cell list |
| Terminal Output | — | ANSI byte buffer |

## 4. Threading Model

### 4.1 Current: Single-Threaded

All stages run on the Node.js main thread. This is simple and correct but limits throughput.

```
Main Thread:
  [Reconcile] → [Encode] → [FFI] → [Process] → [Layout] → [Render] → [Diff] → [ANSI] → [Write]
```

### 4.2 Future: Multi-Threaded

Some stages can be parallelized:

```
Main Thread:          Worker Thread 1:      Worker Thread 2:
[Reconcile]          [Layout]              [Render]
[Encode]             [Diff]                [ANSI Encode]
[FFI]                [Write]
[Process]
```

**Layout parallelism:** Independent subtrees can be laid out in parallel. Taffy supports this via its `compute_layout` API.

**Render parallelism:** Different rows of the frame buffer can be rendered in parallel. Each row is independent.

**Diff parallelism:** Different regions of the frame buffer can be diffed in parallel.

**Encode parallelism:** Different rows of dirty cells can be encoded in parallel.

See [Threading.md](Threading.md) for detailed threading model documentation.

## 5. Performance Budget

### 5.1 Frame Budget

Target: 60fps = 16.67ms per frame.

| Stage | Budget | Typical |
|-------|--------|---------|
| Reconciliation | 2ms | 0.5ms |
| Adapter Translation | 1ms | 0.2ms |
| Protocol Encoding | 0.5ms | 0.1ms |
| FFI Transfer | 0.1ms | 0.05ms |
| Command Processing | 1ms | 0.3ms |
| Layout | 3ms | 1ms |
| Render Tree | 1ms | 0.3ms |
| Frame Buffer | 2ms | 0.5ms |
| Frame Diff | 1ms | 0.2ms |
| ANSI Encoding | 1ms | 0.3ms |
| Terminal Output | 2ms | 0.5ms |
| **Total** | **14.6ms** | **3.95ms** |

### 5.2 Idle Budget

When nothing changes, the engine should consume <0.1ms per frame check.

### 5.3 Worst Case

For a full-screen repaint (all cells dirty):
- Layout: 3ms (1000 nodes)
- Render: 5ms (10,000 cells)
- Diff: 2ms (10,000 cells compared)
- ANSI: 3ms (10,000 cells encoded)
- Write: 5ms (100KB of ANSI data)
- **Total: ~18ms** (just over 16.67ms budget)

We handle worst case by:
1. Splitting large writes across two frames.
2. Using progressive rendering (render visible area first, then offscreen).
3. Dropping to 30fps for complex frames.

## 6. Error Handling

### 6.1 Stage Errors

| Stage | Error | Recovery |
|-------|-------|----------|
| Reconciliation | React error boundary | Catch and report |
| Adapter Translation | Invalid operation | Log warning, skip |
| Protocol Encoding | Serialization error | Retry or abort |
| FFI Transfer | Memory error | Abort frame |
| Command Processing | Invalid node ID | Skip command, log error |
| Layout | Layout failure | Use previous layout |
| Render Tree | Style resolution error | Use default styles |
| Frame Buffer | Buffer overflow | Clip to bounds |
| Frame Diff | Diff failure | Full repaint |
| ANSI Encoding | Encoding error | Skip cell |
| Terminal Output | Write error | Retry or abort |

### 6.2 Error Propagation

Errors propagate upward: a layout failure causes the render to use stale data, which causes the diff to detect changes, which causes a full repaint on the next frame. The system is self-healing.

## 7. Future Considerations

### 7.1 GPU Acceleration

The rendering pipeline can be extended with a GPU backend:

```
[Render Tree] → [GPU Renderer] → [GPU Frame Buffer] → [GPU Diff] → [Terminal Output]
```

The GPU backend would use shaders for text rendering and alpha blending. The layout and event systems remain unchanged.

### 7.2 Remote Rendering

The pipeline can be split across network boundaries:

```
[Reconcile] → [Encode] → [Network] → [Process] → [Layout] → [Render] → [Diff] → [ANSI] → [Write]
```

The TypeScript side runs on the developer's machine. The Rust side runs on the remote server. The protocol is transmitted over WebSocket.

### 7.3 Incremental Rendering

For very large trees, we can render incrementally:

1. Render the visible viewport first.
2. Render offscreen content in the background.
3. Use a "progressive" rendering mode where the UI appears gradually.

This is useful for applications with thousands of nodes (e.g., log viewers, dashboards).
