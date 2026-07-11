# Node Model

The node tree is the single source of truth the Rust renderer reads from. It lives in the `tree` module of `bettertui-engine` and is arena-allocated with generational indices.

## Storage

```mermaid
classDiagram
    class NodeArena {
        +SlotMap~NodeId, RenderNode~ nodes
        +u64 generation
        +insert(node) NodeId
        +get(id) Option~&RenderNode~
        +get_mut(id) Option~&mut RenderNode~
        +remove(id) Option~RenderNode~
        +append_child(parent, child) Result~(), TreeError~
        +insert_before(ref, child) Result~(), TreeError~
        +move_node(node, parent) Result~(), TreeError~
        +replace_node(old, new) Result~(), TreeError~
        +remove_subtree(id)
        +descendants(id) Iterator
        +ancestors(id) Iterator
        +validate()
    }
    class RenderNode {
        +NodeId id
        +NodeKind kind
        +Option~NodeId~ parent
        +SmallVec~[NodeId; 4]~ children
        +Style style
        +LayoutProps layout
        +Option~Box~str~~ text
        +Visibility visibility
        +Transform transform
        +Overflow overflow
        +FocusProps focus
        +EventHandlers events
        +NodeState state
        +Metadata metadata
        +Accessibility accessibility
    }
    NodeArena "1" o-- "*" RenderNode
```

- `NodeId = slotmap::DefaultKey` — 8-byte generational index. Stored as `u64` across the FFI boundary (transmuted, since both are 8-byte `#[repr(transparent)]`).
- `NodeArena` wraps a `SlotMap`. Incrementing `generation` on every mutation lets the renderer short-circuit when nothing changed.
- `RenderNode` is ~256–320 bytes. Up to 4 children stored inline via `SmallVec<[NodeId; 4]>` (no heap allocation for the common case).

## NodeKind

```rust
pub enum NodeKind {
    Box, Text, Flex, Input, List, Table, Tree,
    Scroll, Tab, Modal, Spacer, Separator, Custom(u16),
}
```

Used by the renderer and widgets to decide behavior. `Custom(u16)` allows widget-defined node types.

## Key Sub-types

| Type | File | Notes |
|------|------|-------|
| `Style` | `tree/style.rs` | `Option<bool>` per attribute to allow inheritance |
| `Color { Named, Indexed, Rgb, Default }` | `tree/color.rs` | Stores intent; resolved at render time |
| `LayoutProps` | `tree/layout.rs` | Maps to Taffy flexbox |
| `Visibility { display, opacity, clip }` | `tree/visual.rs` | |
| `Transform { translate_x, translate_y, z_index }` | `tree/visual.rs` | integer offsets (cell grid) |
| `Overflow { Visible, Hidden, Scroll }` | `tree/node_kind.rs` / layout | |
| `FocusProps` | `tree/interaction.rs` | `tab_index`, `focusable`, `focused` |
| `EventHandlers` | `tree/interaction.rs` | per-node handler state |
| `NodeState { scroll_*, dirty flags }` | `tree/interaction.rs` | `layout_dirty`, `render_dirty`, `dirty` |
| `Metadata` / `Accessibility` | `tree/metadata.rs` | boxed; present only when set |

## Tree Operations

All mutations go through `NodeArena` and are bidirectional (parent pointer + children list kept in sync atomically). The tree invariant — every node has exactly one parent except `root` — is enforced at operation time via `Result<(), TreeError>`.

```mermaid
flowchart TD
    A[append_child parent,child] --> B{child has parent?}
    B -- yes --> C[Err TreeError]
    B -- no --> D[set child.parent = parent]
    D --> E[push child into parent.children]
    E --> F[generation += 1]
```

## Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Created: insert()
    Created --> Mutated: update style/layout/text
    Mutated --> Mutated: more updates
    Mutated --> Removed: remove_subtree()
    Removed --> [*]: arena drops node
```

## Integration with Framework Adapters

A framework adapter never touches `NodeArena` directly. It emits `Command`s (see [Protocol.md](Protocol.md)) which are applied to the arena by the command processor. This is what keeps the engine framework-agnostic.

```mermaid
sequenceDiagram
    participant F as React adapter
    participant CB as CommandBuffer (core)
    participant P as CommandProcessor (engine)
    participant A as NodeArena (engine)
    F->>CB: createInstance(kind)
    CB->>CB: push CreateNode command
    F->>CB: appendChild(parent, child)
    CB->>CB: push AppendChild command
    CB->>P: drain() at frame boundary
    P->>A: apply CreateNode, AppendChild
    A-->>P: arena mutated, generation++
```
