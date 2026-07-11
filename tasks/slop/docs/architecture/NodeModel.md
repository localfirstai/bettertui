# Node Model

> The internal UI node model is the foundation of BetterTUI.
> Every widget, every layout, every render operation flows through this model.
> It must be framework-agnostic, performant, and extensible.

## 1. Overview

BetterTUI uses an **arena-allocated, generational-indexed node tree** as its internal UI representation. Framework adapters translate their component trees into this generic model. The Rust engine operates exclusively on this model.

```
Framework Adapter
    ↓ (translates component tree)
Node Model (arena-allocated tree)
    ↓ (processed by)
Rust Engine (layout → render → terminal)
```

### 1.1 Why Not Use the Framework's Own Tree?

React has its virtual DOM. Vue has its proxy-based reactivity graph. Solid has its signal graph. These are all different data structures with different lifecycles and different update semantics.

BetterTUI cannot depend on any of these. It needs a **canonical, framework-agnostic representation** that:

1. Can be efficiently updated from any framework.
2. Can be traversed by the Rust engine without framework knowledge.
3. Supports incremental updates (not full-tree rebuilds).
4. Has predictable memory characteristics.

## 2. Core Types

### 2.1 NodeId

```rust
pub type NodeId = slotmap::DefaultKey;
// Equivalent to: GenerationalIndex { index: u32, generation: u32 }
```

**Purpose:** Uniquely identifies a node in the arena. Generational indices prevent use-after-free — if a node is removed and a new node is allocated at the same index, the generation mismatch catches stale references.

**Lifetime:** Created when a node is allocated. Invalidated when the node is removed.

**Mutability:** Immutable once created. Nodes are identified by ID, not by pointer.

**Memory:** 8 bytes (two u32 values).

**Why not UUID strings:** UUIDs are 16 bytes, require heap allocation, and comparison is slower. Generational indices are 8 bytes, stack-allocated, and compare in a single CPU instruction.

**Why not sequential integers:** Sequential integers have ABA problems — a removed node's ID could be reused for a new node, causing stale references. Generational indices catch this.

### 2.2 NodeKind

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeKind {
    Text,
    Box,
    Flex,
    Input,
    List,
    Table,
    Tree,
    Scroll,
    Tab,
    Modal,
    Spacer,
    Separator,
    Custom(u16),
}
```

**Purpose:** Identifies the type of node. The Rust engine uses this to determine rendering behavior, input handling, and layout strategy.

**Extensibility:** `Custom(u16)` allows plugins and widgets to register custom node types without modifying the core enum.

**Why an enum, not a string:** Enums are size-of-one with efficient pattern matching. Strings require heap allocation and comparison.

### 2.3 RenderNode

```rust
pub struct RenderNode {
    pub id: NodeId,
    pub kind: NodeKind,
    pub parent: Option<NodeId>,
    pub children: SmallVec<[NodeId; 4]>,
    pub style: Style,
    pub layout: LayoutProps,
    pub text: Option<Box<str>>,
    pub attrs: SmallVec<[Attribute; 4]>,
    pub visibility: Visibility,
    pub transform: Transform,
    pub overflow: Overflow,
    pub cursor: Option<CursorProps>,
    pub focus: FocusProps,
    pub events: EventHandlers,
    pub state: NodeState,
    pub metadata: Option<Box<Metadata>>,
    pub accessibility: Option<Box<Accessibility>>,
    pub custom_data: Option<Box<dyn Any>>,
}
```

**Purpose:** The complete data for a single UI node. Stored in the arena, accessed by `NodeId`.

**Ownership:** Owned by the arena. The arena is the sole owner of all nodes. References are by `NodeId`, not by pointer.

**Lifetime:** Lives as long as the arena. Individual nodes are freed when removed from the tree.

**Mutability:** Mutable through the arena. All mutations go through `arena.get_mut(id)`.

**Memory:** Approximately 128-256 bytes per node. The `SmallVec<[NodeId; 4]>` stores up to 4 children inline (32 bytes) without heap allocation. Most nodes have fewer than 4 children.

### 2.4 Style

```rust
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub underline_color: Option<Color>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub dim: Option<bool>,
    pub strikethrough: Option<bool>,
    pub inverse: Option<bool>,
    pub hidden: Option<bool>,
}
```

**Purpose:** Visual styling for a node. Applied during rendering.

**Design choice — Option\<bool\> instead of bitflags:** `Option<bool>` allows style inheritance. A child node can inherit its parent's `bold` value by having `bold: None`. If we used bitflags, we would need a separate "inherited" flag.

**Trade-off:** `Option<bool>` uses more memory than bitflags (1 byte per field vs 1 bit). For ~10,000 nodes, this is ~10KB extra. Acceptable for the clarity it provides.

**Inheritance:** Styles are resolved by merging parent and child. `None` means "inherit from parent." `Some(true)` or `Some(false)` overrides the parent.

### 2.5 Color

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Named(NamedColor),
    Indexed(u8),
    Rgb { r: u8, g: u8, b: u8 },
    Default,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamedColor {
    Black, Red, Green, Yellow, Blue, Magenta, Cyan, White,
    BrightBlack, BrightRed, BrightGreen, BrightYellow,
    BrightBlue, BrightMagenta, BrightCyan, BrightWhite,
}
```

**Purpose:** Represents a color with its intent (named, indexed, RGB, or default).

**Why store intent:** Different terminals support different color modes. A color defined as `Indexed(196)` should remain `Indexed(196)` even if the terminal supports true color — this preserves theme portability. Only when rendering do we resolve to the best available representation.

### 2.6 LayoutProps

```rust
pub struct LayoutProps {
    pub display: Display,
    pub position: Position,
    pub direction: FlexDirection,
    pub justify: JustifyContent,
    pub align: AlignItems,
    pub align_self: Option<AlignSelf>,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Option<Sizing>,
    pub gap: Option<Gap>,
    pub padding: Option<RectValues>,
    pub margin: Option<RectValues>,
    pub width: Option<Sizing>,
    pub height: Option<Sizing>,
    pub min_width: Option<Sizing>,
    pub min_height: Option<Sizing>,
    pub max_width: Option<Sizing>,
    pub max_height: Option<Sizing>,
    pub inset: Option<RectValues>,
}

#[derive(Debug, Clone, Copy)]
pub enum Sizing {
    Points(f32),
    Percent(f32),
    Auto,
}
```

**Purpose:** All layout properties for a node. Maps directly to CSS flexbox concepts.

**Why f32, not u16:** Terminal dimensions are integers, but flex calculations require fractional values. Taffy uses f32 internally. Final positions are rounded to integers only at the last step.

### 2.7 Visibility

```rust
pub struct Visibility {
    pub display: Display,
    pub opacity: f32,
    pub clip: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    Flex,
    None,
}
```

**Purpose:** Controls whether a node is rendered and how it affects layout.

- `display: None` — node removed from layout entirely (CSS `display: none`).
- `opacity` — multiplied with parent opacity during rendering.
- `clip` — whether to clip children that overflow the node's bounds.

### 2.8 Transform

```rust
pub struct Transform {
    pub translate_x: i32,
    pub translate_y: i32,
    pub z_index: i32,
}
```

**Purpose:** Visual offset and layer ordering without affecting layout.

**Why i32, not f32:** Terminal cells are integer positions. Fractional translations don't make sense in a cell-based renderer.

**z_index:** Higher z-index renders on top. Equal z-index renders in tree order (depth-first). Matches CSS stacking context behavior.

### 2.9 Overflow

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}
```

- `Visible` — children render outside bounds (may overlap siblings).
- `Hidden` — children are clipped at bounds.
- `Scroll` — children are clipped, scrollbar rendered, scroll offsets tracked.

### 2.10 CursorProps

```rust
pub struct CursorProps {
    pub style: CursorStyle,
    pub blink: bool,
    pub position: Option<Point>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorStyle {
    Block,
    Underline,
    Bar,
    None,
}
```

**Purpose:** Cursor appearance and position for input nodes.

**Why on the node, not global:** Terminal applications may have multiple input-like nodes. The cursor should visually appear at the focused input's position.

### 2.11 FocusProps

```rust
pub struct FocusProps {
    pub tab_index: Option<i32>,
    pub focusable: bool,
    pub focused: bool,
}
```

- `tab_index` — `None` = follow tree order. `Some(n)` = explicit order (lower first).
- `focusable` — whether this node can receive focus.
- `focused` — whether this node currently has focus.

### 2.12 EventHandlers

```rust
pub struct EventHandlers {
    pub on_key: Option<Box<dyn Fn(KeyEvent) -> EventResult>>,
    pub on_mouse: Option<Box<dyn Fn(MouseEvent) -> EventResult>>,
    pub on_focus: Option<Box<dyn Fn() -> EventResult>>,
    pub on_blur: Option<Box<dyn Fn() -> EventResult>>,
    pub on_resize: Option<Box<dyn Fn(ResizeEvent) -> EventResult>>,
    pub on_custom: Option<Box<dyn Fn(CustomEvent) -> EventResult>>,
}
```

**Design choice — callbacks on nodes vs event delegation:** We use callbacks on nodes (like React's `onClick`) because it matches how every UI framework works, allows per-node handler management, and integrates naturally with the reconciler lifecycle.

**Trade-off:** Each handler is a `Box<dyn Fn>` with heap allocation overhead. For nodes with no handlers (the majority), this is `None` — zero overhead.

### 2.13 NodeState

```rust
pub struct NodeState {
    pub scroll_x: i32,
    pub scroll_y: i32,
    pub content_width: u32,
    pub content_height: u32,
    pub dirty: bool,
    pub layout_dirty: bool,
    pub render_dirty: bool,
}
```

**Separation of dirty flags:**

- `layout_dirty` → triggers Taffy layout recalculation
- `render_dirty` → triggers frame buffer redraw
- `dirty` → generic flag for any change

### 2.14 Metadata

```rust
pub struct Metadata {
    pub key: Option<Box<str>>,
    pub test_id: Option<Box<str>>,
    pub aria_label: Option<Box<str>>,
    pub tooltip: Option<Box<str>>,
    pub user_data: Option<Box<dyn Any>>,
}
```

**Why boxed strings:** Most nodes don't have metadata. `Option<Box<Metadata>>` means zero overhead for nodes without metadata.

### 2.15 Accessibility

```rust
pub struct Accessibility {
    pub role: AriaRole,
    pub label: Option<Box<str>>,
    pub description: Option<Box<str>>,
    pub live: AriaLive,
    pub hidden: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaRole {
    Text, Button, Input, List, ListItem, Table, TableRow, TableCell,
    Tree, TreeItem, Tab, TabPanel, Dialog, Alert, Status, Custom(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaLive {
    Off, Polite, Assertive,
}
```

**Why on every node:** Screen readers need the full tree structure. Even non-interactive nodes may have accessibility roles.

## 3. The Arena

### 3.1 Arena Structure

```rust
pub struct NodeArena {
    nodes: slotmap::SlotMap<NodeId, RenderNode>,
    root: NodeId,
    generation: u64,
}
```

**`slotmap::SlotMap`:** O(1) insertion, O(1) access, O(1) removal. Generational indices prevent use-after-free.

**`root`:** The root node of the tree. All nodes are descendants of root. Root represents the terminal viewport.

**`generation`:** Incremented on every mutation. Used by the renderer to detect changes.

### 3.2 Arena Operations

```rust
impl NodeArena {
    pub fn new() -> Self;
    pub fn insert(&mut self, node: RenderNode) -> NodeId;
    pub fn remove(&mut self, id: NodeId) -> Option<RenderNode>;
    pub fn get(&self, id: NodeId) -> Option<&RenderNode>;
    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut RenderNode>;
    pub fn contains(&self, id: NodeId) -> bool;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn clear(&mut self);
    pub fn root(&self) -> NodeId;
    pub fn iter(&self) -> impl Iterator<Item = (NodeId, &RenderNode)>;
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (NodeId, &mut RenderNode)>;
    pub fn children(&self, id: NodeId) -> impl Iterator<Item = NodeId>;
    pub fn descendants(&self, id: NodeId) -> impl Iterator<Item = NodeId>;
    pub fn ancestors(&self, id: NodeId) -> impl Iterator<Item = NodeId>;
}
```

### 3.3 Tree Operations

```rust
impl NodeArena {
    pub fn append_child(&mut self, parent: NodeId, child: NodeId) -> Result<(), TreeError>;
    pub fn insert_before(&mut self, reference: NodeId, child: NodeId) -> Result<(), TreeError>;
    pub fn move_node(&mut self, node: NodeId, new_parent: NodeId) -> Result<(), TreeError>;
    pub fn replace_node(&mut self, old: NodeId, new: NodeId) -> Result<(), TreeError>;
    pub fn remove_subtree(&mut self, id: NodeId);
    pub fn detach(&mut self, id: NodeId);
    pub fn depth(&self, id: NodeId) -> u32;
    pub fn is_ancestor(&self, ancestor: NodeId, descendant: NodeId) -> bool;
}
```

### 3.4 Tree Invariant

The arena maintains a **tree invariant**: every node has exactly one parent (except root, which has none). Violations are caught at operation time and return `TreeError`.

Parent-child relationships are stored bidirectionally:
- `RenderNode.parent` points to the parent.
- `RenderNode.children` contains the children.

This allows O(1) access in both directions. The trade-off is that mutations must update both directions atomically.

## 4. Node Lifecycle

### 4.1 Creation

```
1. Framework adapter calls createNode(kind, props, style, layout)
2. Core TypeScript generates a NodeId
3. RenderNode created with default values
4. Node inserted into arena
5. Node marked as layout_dirty and render_dirty
6. NodeId returned to adapter
```

### 4.2 Update

```
1. Framework adapter calls updateNode(id, mutations)
2. Mutations applied to the RenderNode
3. Dirty flags set based on what changed:
   - Style change → render_dirty = true
   - Layout change → layout_dirty = true, render_dirty = true
   - Text change → render_dirty = true
4. Dirty flags propagated to ancestors
```

### 4.3 Removal

```
1. Framework adapter calls removeNode(id)
2. Node detached from parent's children list
3. All children recursively detached
4. Nodes removed from arena
5. Parent marked as layout_dirty and render_dirty
6. Event handlers released
7. Custom data dropped
```

### 4.4 Batch Updates

```
1. Framework adapter queues multiple mutations
2. Core TypeScript batches mutations into a single protocol command
3. Rust engine processes all mutations atomically
4. Layout and render invalidation happen once, not per-mutation
```

**Why batching matters:** Without batching, each mutation triggers a layout recalculation. With batching, 100 mutations trigger one layout recalculation. This is O(n) vs O(1) layout passes.

## 5. Ownership Model

### 5.1 Who Owns What

| Entity | Owner | Lifetime |
|--------|-------|----------|
| NodeArena | Rust engine | Entire render session |
| RenderNode | NodeArena (slotmap) | Until removed |
| Style | RenderNode | Same as node |
| LayoutProps | RenderNode | Same as node |
| Text | RenderNode (Box\<str\>) | Same as node |
| EventHandlers | RenderNode (Box) | Same as node |
| CustomData | RenderNode (Box\<dyn Any\>) | Same as node |
| Metadata | RenderNode (Option\<Box\>) | Same as node |
| Accessibility | RenderNode (Option\<Box\>) | Same as node |

### 5.2 Cross-FFI Ownership

When data crosses the FFI boundary (TypeScript → Rust):

1. TypeScript allocates the data structure.
2. Data serialized into a flat buffer (command protocol).
3. Rust deserializes into its own arena-based representation.
4. TypeScript retains its copy until explicitly released.

Data is **copied** across the FFI boundary, not shared. This prevents data races and simplifies ownership semantics.

### 5.3 Interior Mutability

- `NodeState.dirty` — set by any code that detects a change.
- `EventHandlers` — may need to update handler state without exclusive access.

We use `Cell<bool>` for simple flags and `RefCell<T>` for complex state.

## 6. Memory Strategy

### 6.1 Small Object Optimization

- `NodeId` is 8 bytes (stack-allocated).
- `Style` is ~16 bytes (stack-allocated).
- `LayoutProps` is ~48 bytes (stack-allocated).
- `SmallVec<[NodeId; 4]>` stores up to 4 children inline without heap allocation.
- `Option<Box<T>>` uses the null pointer optimization — no extra byte for `None`.

### 6.2 Allocation Minimization

- Nodes are arena-allocated, not individually heap-allocated.
- Text is `Box<str>` (not `String`) — immutable, no capacity overhead.
- Event handlers are only allocated for nodes that have handlers.
- Metadata and accessibility are only allocated for nodes that need them.

### 6.3 Pooling Strategy

- **Node pool:** The slotmap internally manages a free list.
- **Command buffer:** A pre-allocated `Vec<Command>` is reused across frames.
- **Frame buffer:** Two frame buffers are allocated once and swapped.

## 7. Framework Adapter Integration

### 7.1 How React Integrates

```
React component tree
    ↓ (React reconciler)
Reconciler host config methods called
    ↓ (adapter translates)
Node operations queued
    ↓ (batched into protocol command)
FFI call to Rust engine
    ↓
Rust arena updated
```

### 7.2 How Vue Would Integrate

```
Vue template compiled to render function
    ↓ (Vue's reactivity system)
Render function called reactively
    ↓ (adapter translates)
Same node operations as React
    ↓
Same protocol commands → Same Rust arena updates
```

### 7.3 How Solid Would Integrate

```
Solid component with signals
    ↓ (fine-grained reactivity)
Only changed signals trigger updates
    ↓ (adapter translates)
Minimal node operations (only changed nodes)
    ↓
Same protocol commands → Same Rust arena updates
```

**Key insight:** All frameworks produce the same node operations. The Rust engine doesn't care whether the update came from React's virtual DOM diff, Vue's proxy tracking, or Solid's signals.

## 8. Future Evolution

### 8.1 Node Pooling for High-Frequency Updates

For applications with thousands of nodes being created/destroyed per frame (e.g., scrolling lists), we can add a dedicated node pool that pre-allocates nodes and recycles them. This avoids repeated slotmap insertions.

### 8.2 Compressed Node Storage

For trees with many similar nodes (e.g., a list of 1000 items), we can add a "compressed" mode where nodes of the same kind share a common layout template and only store per-node overrides.

### 8.3 Snapshot and Diff

The arena can support snapshot-and-diff for DevTools: capture the full tree state, compare with previous snapshot, and visualize changes.
