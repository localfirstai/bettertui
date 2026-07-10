# Event System

> The event system handles all user input and application events.
> It supports bubbling, capture, and cancellation like browser DOM events.

## 1. Overview

The event system is modeled after browser DOM events. Events propagate through the tree in three phases:

```
1. Capture phase (root → target)
2. Target phase (at the target node)
3. Bubble phase (target → root)
```

```
Root
├── NodeA
│   ├── NodeB (target)
│   └── NodeC
└── NodeD

Event flow for NodeB:
  Capture: Root → NodeA → NodeB
  Target:  NodeB
  Bubble:  NodeB → NodeA → Root
```

### 1.1 Why DOM-Style Events?

DOM events are a well-understood, battle-tested event model. Every web developer knows how `onClick`, `onKeyDown`, and `stopPropagation` work. By using the same model, BetterTUI leverages existing developer knowledge.

**Alternative considered:** EventEmitter pattern (like Node.js). Rejected because it doesn't support capture/bubble phases, making it difficult to implement event delegation and interception.

## 2. Event Types

### 2.1 Event Enum

```rust
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Paste(PasteEvent),
    Focus(FocusEvent),
    Blur(BlurEvent),
    Resize(ResizeEvent),
    Custom(CustomEvent),
    Frame(FrameEvent),
    Lifecycle(LifecycleEvent),
}
```

### 2.2 KeyEvent

```rust
pub struct KeyEvent {
    pub key: Key,
    pub modifiers: Modifiers,
    pub phase: EventPhase,
    pub default_prevented: bool,
}

pub enum Key {
    Character(char),
    Enter,
    Escape,
    Backspace,
    Delete,
    Tab,
    Space,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    PageUp,
    PageDown,
    F(u8),
    Ctrl(char),
    Alt(char),
    Shift(char),
    Unknown(String),
}

pub struct Modifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
}
```

### 2.3 MouseEvent

```rust
pub struct MouseEvent {
    pub button: MouseButton,
    pub position: Point,
    pub modifiers: Modifiers,
    pub phase: EventPhase,
    pub default_prevented: bool,
}

pub enum MouseButton {
    Left,
    Right,
    Middle,
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
    None,
}
```

### 2.4 PasteEvent

```rust
pub struct PasteEvent {
    pub text: Box<str>,
    pub phase: EventPhase,
    pub default_prevented: bool,
}
```

### 2.5 FocusEvent

```rust
pub struct FocusEvent {
    pub target: NodeId,
    pub previous: Option<NodeId>,
    pub phase: EventPhase,
}
```

### 2.6 ResizeEvent

```rust
pub struct ResizeEvent {
    pub width: u16,
    pub height: u16,
    pub previous_width: u16,
    pub previous_height: u16,
}
```

### 2.7 CustomEvent

```rust
pub struct CustomEvent {
    pub type_id: u16,
    pub payload: Box<[u8]>,
    pub phase: EventPhase,
}
```

### 2.8 FrameEvent

```rust
pub struct FrameEvent {
    pub frame_number: u64,
    pub delta_ms: f64,
}
```

### 2.9 LifecycleEvent

```rust
pub enum LifecycleEvent {
    Mount,
    Unmount,
    Suspend,
    Resume,
}
```

### 2.10 EventPhase

```rust
pub enum EventPhase {
    Capture,
    Target,
    Bubble,
}
```

## 3. Event Propagation

### 3.1 Propagation Algorithm

```
function dispatchEvent(event, targetNode):
    // Capture phase: root → target
    ancestors = getAncestors(targetNode)
    for node in ancestors:
        event.phase = Capture
        if node.hasHandler(event.type):
            node.handler(event)
        if event.propagationStopped:
            return

    // Target phase: at target
    event.phase = Target
    if targetNode.hasHandler(event.type):
        targetNode.handler(event)
    if event.propagationStopped:
        return

    // Bubble phase: target → root
    for node in ancestors.reverse():
        event.phase = Bubble
        if node.hasHandler(event.type):
            node.handler(event)
        if event.propagationStopped:
            return
```

### 3.2 Propagation Control

```rust
impl Event {
    /// Stop event propagation (no further handlers will be called).
    pub fn stop_propagation(&mut self) {
        self.propagation_stopped = true;
    }

    /// Prevent the default behavior (e.g., scrolling on arrow keys).
    pub fn prevent_default(&mut self) {
        self.default_prevented = true;
    }

    /// Stop propagation AND prevent default.
    pub fn stop_immediate(&mut self) {
        self.propagation_stopped = true;
        self.default_prevented = true;
    }
}
```

### 3.3 Event Delegation

Events can be handled at any ancestor, not just the target node. This enables event delegation:

```rust
// Handle all key events on the root, filter by target
root.on_key(|event| {
    if event.target == input_id {
        handle_input_key(event);
    }
});
```

**Why event delegation matters:** In large trees with many interactive nodes, attaching handlers to every node is expensive. Event delegation allows a single handler at the root to manage events for all descendants.

## 4. Hit Testing

### 4.1 Mouse Hit Testing

When a mouse event occurs, the system must determine which node the mouse is over:

```
1. Start at the root node
2. Check if mouse position is within the node's layout rect
3. If yes, check children (in reverse order for z-index)
4. Recursively check until a leaf node is found
5. The deepest matching node is the hit target
```

### 4.2 Hit Test Algorithm

```rust
pub fn hit_test(node_id: NodeId, position: Point, arena: &NodeArena) -> Option<NodeId> {
    let node = arena.get(node_id)?;

    // Check if position is within this node's bounds
    let layout = node.layout_result?;
    if !rect_contains(layout.rect(), position) {
        return None;
    }

    // Check children in reverse order (highest z-index first)
    for &child_id in node.children.iter().rev() {
        if let Some(hit) = hit_test(child_id, position, arena) {
            return Some(hit);
        }
    }

    // No child matched, this node is the hit target
    Some(node_id)
}
```

### 4.3 Hit Test Optimization

For large trees, hit testing can be optimized with:

1. **Spatial hashing:** Divide the terminal into grid cells and index nodes by their cell positions.
2. **Bounding box caching:** Cache each node's bounding box to avoid recomputing layout.
3. **Early exit:** If a node's bounding box doesn't contain the point, skip its children.

### 4.4 Hit Test for Scroll Containers

For scroll containers, the hit test must account for scroll offset:

```rust
fn hit_test_scroll(node_id: NodeId, position: Point, arena: &NodeArena) -> Option<NodeId> {
    let node = arena.get(node_id)?;
    let layout = node.layout_result?;

    // Adjust position by scroll offset
    let adjusted = Point {
        x: position.x + node.state.scroll_x as u16,
        y: position.y + node.state.scroll_y as u16,
    };

    // Standard hit test with adjusted position
    hit_test_children(node_id, adjusted, arena)
}
```

## 5. Keyboard Events

### 5.1 Key Parsing

Raw terminal input bytes are parsed into `KeyEvent`s:

```
Raw bytes → Parser → KeyEvent

Examples:
  "a"        → KeyEvent { key: Key::Character('a'), modifiers: none }
  "\x1b[A"   → KeyEvent { key: Key::ArrowUp, modifiers: none }
  "\x1b[1;2A" → KeyEvent { key: Key::ArrowUp, modifiers: { shift: true } }
  "\x03"     → KeyEvent { key: Key::Ctrl('c'), modifiers: { ctrl: true } }
```

### 5.2 Kitty Keyboard Protocol

BetterTUI supports the Kitty keyboard protocol for enhanced key reporting:

```
Enable:  ESC[>31u
Disable: ESC[<u
```

The Kitty protocol provides:
- Key release events (not just press).
- Modifier state in every event.
- Multi-key sequences.
- ESC as a distinct key (not mixed with escape sequences).

### 5.3 Key Mapping

Keys can be remapped by the application:

```rust
pub struct Keymap {
    pub bindings: Vec<KeyBinding>,
}

pub struct KeyBinding {
    pub key: Key,
    pub modifiers: Modifiers,
    pub command: Box<dyn Fn()>,
    pub when: Option<Box<dyn Fn() -> bool>>,
}
```

## 6. Mouse Events

### 6.1 Mouse Protocol

BetterTUI supports two mouse protocols:

**X10 protocol (basic):**
```
Enable: ESC[?9h
Disable: ESC[?9l
Format: ESC[MCbCx Cy
```

**SGR protocol (extended):**
```
Enable: ESC[?1006h
Disable: ESC[?1006l
Format: ESC[<Cb;Cx;Cy M/m
```

SGR is preferred because it supports coordinates > 223 and reports button release.

### 6.2 Mouse Event Flow

```
1. Raw mouse bytes received from stdin
2. Parser decodes bytes into MouseEvent
3. Hit test determines target node
4. Event dispatched through capture → target → bubble
5. Target node receives onMouseDown, onMouseUp, etc.
6. If auto-focus enabled, target node receives focus
```

### 6.3 Mouse Drag

Mouse drag is tracked by the event system:

```
1. onMouseDown on node A → start drag
2. onMouseMove → if button pressed, emit onMouseDrag on node A
3. onMouseUp → end drag, emit onMouseDrop on node A
```

### 6.4 Hover Events

Hover events are emitted when the mouse enters or leaves a node:

```
1. onMouseMove → hit test → new target node B
2. If previous target was node A:
   a. Emit onMouseOut on node A
   b. Emit onMouseOver on node B
3. Update hover target to node B
```

## 7. Focus Events

### 7.1 Focus Model

Focus is tracked globally. Only one node can be focused at a time:

```rust
pub struct FocusManager {
    focused: Option<NodeId>,
    focus_history: Vec<NodeId>,
}
```

### 7.2 Focus Navigation

**Tab:** Move focus to the next focusable node (in tree order).

**Shift+Tab:** Move focus to the previous focusable node.

**Arrow keys:** Move focus between sibling nodes (direction depends on layout).

### 7.3 Focus Events Flow

```
1. Focus requested (Tab, click, programmatic)
2. Current focused node receives BlurEvent
3. Focus manager updates focused node
4. New focused node receives FocusEvent
5. Cursor is repositioned to new focused node's cursor position
```

### 7.4 Focus Traversal

```rust
impl FocusManager {
    pub fn next_focus(&mut self, arena: &NodeArena) -> Option<NodeId> {
        let current = self.focused?;
        let focusable_nodes = arena.descendants(arena.root())
            .filter(|id| arena.get(id).map_or(false, |n| n.focus.focusable))
            .collect::<Vec<_>>();

        let current_index = focusable_nodes.iter().position(|&id| id == current)?;
        let next_index = (current_index + 1) % focusable_nodes.len();
        Some(focusable_nodes[next_index])
    }

    pub fn previous_focus(&mut self, arena: &NodeArena) -> Option<NodeId> {
        // Similar to next_focus, but in reverse order
    }
}
```

## 8. Resize Events

### 8.1 Resize Detection

Resize is detected via:

1. **SIGWINCH signal:** Unix signals when terminal size changes.
2. **crossterm poll:** Poll for resize events in the event loop.

### 8.2 Resize Handling

```
1. Terminal resize detected
2. New size queried from terminal
3. ResizeEvent dispatched to root node
4. Root node updates its layout constraints
5. Full layout recalculation triggered
6. Frame buffer resized
7. Full repaint triggered
```

## 9. Custom Events

### 9.1 Custom Event Registration

Plugins and applications can register custom event types:

```rust
pub fn register_event_type(type_id: u16, name: Box<str>) -> Result<(), EventError>;
pub fn unregister_event_type(type_id: u16) -> Result<(), EventError>;
```

### 9.2 Custom Event Dispatch

```rust
pub fn dispatch_custom(
    arena: &NodeArena,
    target: NodeId,
    type_id: u16,
    payload: Box<[u8]>,
) -> Result<(), EventError>;
```

Custom events follow the same capture → target → bubble propagation as built-in events.

## 10. Lifecycle Events

### 10.1 Mount

Emitted when a node is added to the tree and becomes visible.

### 10.2 Unmount

Emitted when a node is removed from the tree.

### 10.3 Suspend

Emitted when the application is suspended (e.g., terminal alternate screen exited).

### 10.4 Resume

Emitted when the application is resumed.

## 11. Event Queue

### 11.1 Queued Events

Events are queued and processed in order:

```rust
pub struct EventQueue {
    events: VecDeque<Event>,
    handlers: HashMap<EventType, Vec<Box<dyn Fn(&Event)>>>,
}
```

### 11.2 Event Processing

```
1. Event received from input parser
2. Event added to queue
3. Event loop processes queue:
   a. Dequeue next event
   b. Hit test (for mouse events)
   c. Dispatch through capture → target → bubble
   d. Process any state changes
   e. If state changed, trigger render
4. Repeat until queue is empty
```

### 11.3 Event Coalescing

Multiple rapid events (e.g., mouse moves) can be coalesced:

```rust
// Instead of processing 100 mouse move events:
// Process only the last one (most recent position)
if let Some(last_mouse) = self.events.iter().rfind(|e| e.is_mouse_move()) {
    self.process_event(last_mouse);
}
```

## 12. Error Handling

### 12.1 Event Errors

```rust
pub enum EventError {
    NodeNotFound(NodeId),
    HandlerError(String),
    PropagationError(String),
}
```

### 12.2 Error Recovery

- **Node not found:** Skip the event (node was removed between hit test and dispatch).
- **Handler error:** Log the error and continue processing other events.
- **Propagation error:** Reset propagation state and continue.

## 13. Performance

### 13.1 Event Processing Budget

- Keyboard events: <0.1ms per event.
- Mouse events: <0.5ms per event (includes hit testing).
- Resize events: <1ms per event (includes layout recalculation).

### 13.2 Hit Testing Performance

- Simple tree (100 nodes): <0.01ms.
- Complex tree (1000 nodes): <0.1ms.
- With spatial hashing: O(1) average case.

### 13.3 Event Queue Performance

- Enqueue: O(1).
- Dequeue: O(1).
- Coalescing: O(n) where n is the number of queued events.
