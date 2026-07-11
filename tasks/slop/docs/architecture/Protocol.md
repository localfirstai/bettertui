# Protocol

> The Rust ↔ TypeScript command protocol is the heart of BetterTUI.
> Every framework adapter communicates with the Rust engine through this protocol.
> It must be fast, extensible, and debuggable.

## 1. Overview

The protocol is a **command-based, batch-oriented interface** between TypeScript and Rust. Framework adapters produce commands. The Rust engine consumes commands.

```
TypeScript (adapter)
    ↓ (queues commands)
Command Buffer
    ↓ (batched FFI call)
Rust Engine
    ↓ (processes commands)
Node Tree Updated
```

### 1.1 Why Command-Based?

**Alternative 1: Direct FFI calls per operation.** Each `appendChild`, `setStyle`, etc. is a separate FFI call. This is simple but slow — each FFI call costs ~100ns. With 1000 operations per frame, that's 100μs of pure FFI overhead.

**Alternative 2: Shared memory with lock-free queues.** TypeScript and Rust share a memory region and communicate via lock-free queues. This is fast but complex — it requires careful memory ordering, handle contention, and makes debugging difficult.

**Chosen approach: Batched commands.** TypeScript queues commands into a buffer. On commit, the entire buffer is serialized and sent to Rust in a single FFI call. This gives us:
- O(1) FFI calls per frame (amortized).
- Simple serialization (no shared memory complexity).
- Easy debugging (commands can be logged, replayed, and inspected).
- Natural batching (all mutations in a frame are processed atomically).

### 1.2 Why Not React Native's Bridge?

React Native's bridge is asynchronous and serialized. Commands are queued and processed on the next frame. This causes a "bridge delay" between user interaction and visual update.

BetterTUI's protocol is **synchronous**. When TypeScript calls `commit()`, the Rust engine processes all commands before returning. This ensures:
- No visual delay between mutation and render.
- Layout is always consistent after `commit()`.
- Event handlers can immediately see the results of mutations.

**Trade-off:** Synchronous processing blocks the Node.js event loop during command processing. We mitigate this by keeping command processing fast (typically <1ms for most frames).

## 2. Command Structure

### 2.1 Command Enum

```rust
pub enum Command {
    // Tree operations
    CreateNode { id: NodeId, kind: NodeKind },
    RemoveNode { id: NodeId },
    AppendChild { parent: NodeId, child: NodeId },
    InsertBefore { reference: NodeId, child: NodeId },
    MoveNode { node: NodeId, new_parent: NodeId },
    ReplaceNode { old: NodeId, new: NodeId },
    DetachNode { id: NodeId },

    // Style operations
    SetStyle { id: NodeId, style: Style },
    SetForeground { id: NodeId, color: Option<Color> },
    SetBackground { id: NodeId, color: Option<Color> },
    SetBold { id: NodeId, value: Option<bool> },
    SetItalic { id: NodeId, value: Option<bool> },
    SetUnderline { id: NodeId, value: Option<bool> },

    // Layout operations
    SetLayout { id: NodeId, layout: LayoutProps },
    SetFlexDirection { id: NodeId, direction: FlexDirection },
    SetJustifyContent { id: NodeId, justify: JustifyContent },
    SetAlignItems { id: NodeId, align: AlignItems },
    SetWidth { id: NodeId, width: Option<Sizing> },
    SetHeight { id: NodeId, height: Option<Sizing> },
    SetPadding { id: NodeId, padding: Option<RectValues> },
    SetMargin { id: NodeId, margin: Option<RectValues> },
    SetGap { id: NodeId, gap: Option<Gap> },
    SetFlexGrow { id: NodeId, grow: f32 },
    SetFlexShrink { id: NodeId, shrink: f32 },

    // Content operations
    SetText { id: NodeId, text: Box<str> },
    SetAttribute { id: NodeId, key: Box<str>, value: Box<str> },
    RemoveAttribute { id: NodeId, key: Box<str> },

    // Visibility operations
    SetVisibility { id: NodeId, visibility: Visibility },
    SetDisplay { id: NodeId, display: Display },
    SetOpacity { id: NodeId, opacity: f32 },
    SetClip { id: NodeId, clip: bool },

    // Transform operations
    SetTransform { id: NodeId, transform: Transform },
    SetTranslate { id: NodeId, x: i32, y: i32 },
    SetZIndex { id: NodeId, z_index: i32 },

    // Overflow operations
    SetOverflow { id: NodeId, overflow: Overflow },
    SetScrollOffset { id: NodeId, x: i32, y: i32 },

    // Focus operations
    FocusNode { id: NodeId },
    BlurNode { id: NodeId },
    SetTabIndex { id: NodeId, tab_index: Option<i32> },
    SetFocusable { id: NodeId, focusable: bool },

    // Cursor operations
    SetCursor { id: NodeId, cursor: Option<CursorProps> },
    SetCursorStyle { id: NodeId, style: CursorStyle },
    SetCursorPosition { id: NodeId, position: Option<Point> },

    // Event operations
    SetEventHandler { id: NodeId, event_type: EventType, handler_id: HandlerId },
    RemoveEventHandler { id: NodeId, event_type: EventType },

    // Metadata operations
    SetMetadata { id: NodeId, metadata: Metadata },
    SetAccessibility { id: NodeId, accessibility: Accessibility },

    // Rendering operations
    BeginFrame,
    CommitFrame,
    Invalidate,
    InvalidateNode { id: NodeId },
    InvalidateRect { rect: Rect },

    // Terminal operations
    Resize { width: u32, height: u32 },

    // Lifecycle operations
    Shutdown,
    Suspend,
    Resume,

    // Batch operations
    Batch(Vec<Command>),
}
```

### 2.2 Command Categories

| Category | Commands | Purpose |
|----------|----------|---------|
| Tree | CreateNode, RemoveNode, AppendChild, InsertBefore, MoveNode, ReplaceNode, DetachNode | Node tree mutations |
| Style | SetStyle, SetForeground, SetBackground, SetBold, etc. | Visual styling |
| Layout | SetLayout, SetFlexDirection, SetWidth, etc. | Layout properties |
| Content | SetText, SetAttribute, RemoveAttribute | Node content |
| Visibility | SetVisibility, SetDisplay, SetOpacity, SetClip | Visibility control |
| Transform | SetTransform, SetTranslate, SetZIndex | Visual transforms |
| Overflow | SetOverflow, SetScrollOffset | Overflow handling |
| Focus | FocusNode, BlurNode, SetTabIndex, SetFocusable | Focus management |
| Cursor | SetCursor, SetCursorStyle, SetCursorPosition | Cursor appearance |
| Events | SetEventHandler, RemoveEventHandler | Event handler registration |
| Metadata | SetMetadata, SetAccessibility | Node metadata |
| Rendering | BeginFrame, CommitFrame, Invalidate, InvalidateNode, InvalidateRect | Render control |
| Terminal | Resize | Terminal size changes |
| Lifecycle | Shutdown, Suspend, Resume | Application lifecycle |
| Batch | Batch | Group multiple commands |

### 2.3 Fine-Grained vs Bulk Commands

We provide both fine-grained commands (`SetForeground`, `SetBold`, etc.) and bulk commands (`SetStyle`, `SetLayout`).

**Fine-grained commands** are used when only one property changes. They are more efficient because:
- The engine only needs to update the specific property.
- Dirty flags can be set more precisely.
- Less data is serialized across the FFI.

**Bulk commands** are used when multiple properties change simultaneously. They are simpler for the adapter to produce.

**The engine internally normalizes all commands to fine-grained operations.** `SetStyle` is expanded into individual `SetForeground`, `SetBold`, etc. calls. This means the engine always works with the same granular operations regardless of how the adapter batches them.

## 3. Protocol Codec

### 3.1 TypeScript Side

```typescript
interface ProtocolCodec {
  encode(commands: Command[]): Uint8Array;
  decode(data: Uint8Array): Command[];
  estimateSize(commands: Command[]): number;
}
```

**encode:** Serializes a batch of commands into a flat byte buffer. Uses a compact binary format with:
- 1-byte command type tag.
- Variable-length encoded NodeId (4-8 bytes).
- Inline small values (colors, bools, etc.).
- Heap-allocated values (strings, metadata) referenced by offset.

**decode:** Deserializes a byte buffer back into commands. Used for debugging and replay.

**estimateSize:** Estimates the byte size of a command batch without encoding. Used to decide when to flush the command buffer.

### 3.2 Binary Layout

```
┌──────────────┬──────────────┬──────────────┬──────────────┐
│ Header       │ Command 1    │ Command 2    │ ...          │
├──────────────┼──────────────┼──────────────┼──────────────┤
│ version: u16 │ type: u8     │ type: u8     │              │
│ count: u32   │ id: [u8; 8]  │ id: [u8; 8]  │              │
│ flags: u8    │ payload: ... │ payload: ... │              │
└──────────────┴──────────────┴──────────────┴──────────────┘
```

**Header:**
- `version: u16` — Protocol version. Currently 1.
- `count: u32` — Number of commands in the batch.
- `flags: u8` — Batch flags (e.g., synchronous vs async).

**Command:**
- `type: u8` — Command type (maps to `Command` enum variant).
- `id: [u8; 8]` — NodeId as raw bytes.
- `payload: ...` — Command-specific payload. Variable length.

### 3.3 Versioning Strategy

The protocol uses a **version field** in the header. When the protocol changes:

1. **Additive changes** (new commands, new optional fields) — increment minor version. Old engines can safely ignore unknown commands.
2. **Breaking changes** (command restructuring, field removal) — increment major version. New adapters must check the version and reject incompatible engines.

### 3.4 Backwards Compatibility

- Engines must accept commands from adapters with the same or older protocol version.
- Adapters must reject commands from engines with a newer major version.
- Unknown command types are skipped with a warning, not an error.
- Unknown payload bytes are discarded.

## 4. Command Processing

### 4.1 Processing Pipeline

```
1. Receive command batch from FFI
2. Validate all commands (check node IDs exist, check permissions)
3. Process commands in order:
   a. Tree operations update the arena
   b. Style/Layout/Content operations update node fields
   c. Set dirty flags on affected nodes
4. Return processing result to TypeScript
```

### 4.2 Atomicity

All commands in a batch are processed **atomically**. If any command fails validation, the entire batch is rejected and an error is returned. This prevents partial updates that could leave the tree in an inconsistent state.

**Why atomicity matters:** Without atomicity, a failed `AppendChild` could leave the tree with a detached node. With atomicity, the tree is either fully updated or fully unchanged.

### 4.3 Error Handling

```rust
pub struct CommandResult {
    pub success: bool,
    pub errors: Vec<CommandError>,
    pub warnings: Vec<CommandWarning>,
}

pub enum CommandError {
    NodeNotFound(NodeId),
    ParentNotFound(NodeId),
    CycleDetected { node: NodeId, ancestor: NodeId },
    InvalidOperation { command: Command, reason: String },
}
```

Errors are returned per-command, not per-batch. This allows the adapter to see which specific commands failed while still processing the rest of the batch.

**Wait — this contradicts atomicity.** Actually, we support two modes:

- **Strict mode (default):** Any error rejects the entire batch.
- **Lenient mode (debug):** Errors are collected and returned, successful commands are applied.

Lenient mode is useful during development. Strict mode is used in production.

### 4.4 Performance

Command processing performance targets:

| Metric | Target | Measurement |
|--------|--------|-------------|
| 100 commands | <0.5ms | Batch processing time |
| 1000 commands | <2ms | Batch processing time |
| FFI overhead | <0.1ms | Per-call overhead |
| Serialization | <0.1ms | Per-batch encoding |

These targets are achievable because:
- Commands are simple structs with no heap allocation (except string payloads).
- The arena provides O(1) node access.
- Dirty flag propagation is O(depth), not O(n).

## 5. Event Protocol (Rust → TypeScript)

### 5.1 Event Types

```rust
pub enum Event {
    // Input events
    Key(KeyEvent),
    Mouse(MouseEvent),
    Paste(PasteEvent),

    // Focus events
    FocusChanged { node: Option<NodeId>, previous: Option<NodeId> },

    // Lifecycle events
    FrameComplete(FrameStats),
    Resize(ResizeEvent),
    Suspend,
    Resume,

    // Custom events
    Custom { type_id: u16, payload: Box<[u8]> },
}
```

### 5.2 Event Dispatch

Events flow from Rust to TypeScript:

```
1. Rust detects event (input, resize, etc.)
2. Rust dispatches event through event system
3. If event has a handler on the target node:
   a. Event serialized into bytes
   b. FFI callback to TypeScript
   c. TypeScript deserializes and invokes handler
   d. Handler may produce commands (e.g., scroll, focus change)
   e. Commands queued for next commit
4. If event bubbles, repeat for parent nodes
```

### 5.3 Event Serialization

Events are serialized using the same binary format as commands, but in reverse direction:

```
┌──────────────┬──────────────┐
│ Header       │ Event Payload│
├──────────────┼──────────────┤
│ type: u16    │ ...          │
│ timestamp: u64│             │
│ node_id: [u8; 8]│          │
│ flags: u8    │              │
└──────────────┴──────────────┘
```

### 5.4 Callback Mechanism

The FFI boundary supports two callback mechanisms:

**Synchronous callbacks** — used for events that must be handled before the next frame. The Rust engine blocks until the TypeScript handler returns. Used for keyboard events, focus changes.

**Asynchronous callbacks** — used for events that can be handled eventually. The Rust engine continues processing while the TypeScript handler runs in the background. Used for animations, timers, non-critical events.

## 6. Frame Protocol

### 6.1 Frame Lifecycle

```
1. TypeScript calls beginFrame()
2. Rust prepares for new frame:
   a. Clear render dirty flags
   b. Begin layout calculation
3. TypeScript queues mutation commands
4. TypeScript calls commitFrame()
5. Rust processes all queued commands
6. Rust performs layout (if layout_dirty nodes exist)
7. Rust renders dirty nodes to frame buffer
8. Rust diffs frame buffer against previous frame
9. Rust encodes dirty cells as ANSI
10. Rust writes to terminal stdout
11. Rust returns FrameStats to TypeScript
```

### 6.2 Frame Stats

```rust
pub struct FrameStats {
    pub frame_number: u64,
    pub duration_ms: f64,
    pub layout_time_ms: f64,
    pub render_time_ms: f64,
    pub diff_time_ms: f64,
    pub write_time_ms: f64,
    pub nodes_layouted: u32,
    pub nodes_rendered: u32,
    pub cells_changed: u32,
    pub bytes_written: u32,
}
```

**Purpose:** Provides performance metrics for each frame. Used by DevTools and performance monitoring.

### 6.3 Frame Skipping

If no nodes are dirty and no commands are queued, the engine skips the frame entirely. This is the "idle" state — the engine does no work until something changes.

```
Frame N:
  - No dirty nodes → skip render
  - Write time: 0ms
  - Total time: ~0.01ms (just the check)
```

## 7. Future Protocol Extensions

### 7.1 WebSocket Transport

The protocol can be transmitted over WebSocket for remote DevTools:

```
DevTools Browser ←→ WebSocket ←→ BetterTUI Application
```

Commands flow in both directions: DevTools can inspect and modify the tree, and the application sends frame updates and events.

### 7.2 IPC Transport

For multi-process applications, the protocol can use IPC (Unix sockets, named pipes):

```
Main Process ←→ Unix Socket ←→ Renderer Process
```

### 7.3 Protocol Compression

For large command batches (e.g., initial tree construction with 1000+ nodes), we can add optional compression:

- **Delta encoding:** Only send changed fields, not full commands.
- **Dictionary encoding:** Replace common patterns with short codes.
- **Variable-length integers:** Use varint encoding for small values.

### 7.4 Protocol Profiling

The protocol can emit profiling data:

```rust
pub struct ProtocolProfile {
    pub commands_sent: u64,
    pub bytes_sent: u64,
    pub events_received: u64,
    pub bytes_received: u64,
    pub average_batch_size: f64,
    pub largest_batch: u32,
}
```

This data helps identify performance bottlenecks in the FFI boundary.
