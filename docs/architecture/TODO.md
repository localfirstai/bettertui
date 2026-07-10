# Implementation TODO

## Phase 1: Framework Agnostic UI Model (Rust Types)

### 1.1 Core Types
- [ ] `NodeId` — generational index type alias
- [ ] `NodeKind` — enum (Text, Box, Flex, Input, List, Table, Tree, Scroll, Tab, Modal, Spacer, Separator, Custom)
- [ ] `RenderNode` — complete node struct with all fields
- [ ] `Style` — visual styling (fg, bg, bold, italic, etc.)
- [ ] `Color` — color representation (Named, Indexed, Rgb, Default)
- [ ] `NamedColor` — 16 named terminal colors

### 1.2 Layout Types
- [ ] `LayoutProps` — flexbox layout properties
- [ ] `Sizing` — width/height values (Points, Percent, Auto)
- [ ] `Display` — display mode (Flex, None)
- [ ] `FlexDirection` — row/column/reverse
- [ ] `JustifyContent` — main axis alignment
- [ ] `AlignItems` — cross axis alignment
- [ ] `AlignSelf` — per-child cross axis override
- [ ] `Gap` — gap between children
- [ ] `RectValues` — padding/margin/inset values
- [ ] `Position` — relative/absolute

### 1.3 Visual Types
- [ ] `Visibility` — display, opacity, clip
- [ ] `Transform` — translate_x, translate_y, z_index
- [ ] `Overflow` — visible/hidden/scroll
- [ ] `CursorProps` — cursor style and position
- [ ] `CursorStyle` — block/underline/bar/none
- [ ] `Point` — x, y coordinates
- [ ] `Size` — width, height
- [ ] `Rect` — x, y, width, height

### 1.4 Interaction Types
- [ ] `FocusProps` — tab_index, focusable, focused
- [ ] `EventHandlers` — placeholder for future event callbacks
- [ ] `NodeState` — scroll offsets, dirty flags, content size
- [ ] `UpdateFlags` — bitflags for dirty propagation

### 1.5 Metadata Types
- [ ] `Metadata` — key, test_id, aria_label, tooltip
- [ ] `Accessibility` — role, label, description, live, hidden
- [ ] `AriaRole` — accessibility roles
- [ ] `AriaLive` — live region modes

### 1.6 Tests
- [ ] Type sizes match architecture spec
- [ ] Default implementations produce valid initial state
- [ ] Style inheritance works (None = inherit)

## Phase 2: Rust Runtime (Arena + Tree) ✅

### 2.1 Arena
- [x] `NodeArena` struct with slotmap backing
- [x] `new()` — create arena with root node
- [x] `insert()` — allocate node, return NodeId
- [x] `get()` / `get_mut()` — O(1) node access
- [x] `remove()` — free node, return it
- [x] `contains()` — check if node exists
- [x] `len()` / `is_empty()` — node count
- [x] `clear()` — remove all nodes
- [x] `root()` — get root NodeId
- [x] `iter()` / `iter_mut()` — iterate all nodes

### 2.2 Tree Operations
- [x] `append_child()` — add child to parent
- [x] `insert_before()` — insert before reference node
- [x] `move_node()` — move node to new parent
- [x] `replace_node()` — replace one node with another
- [x] `remove_subtree()` — recursively remove node and descendants
- [x] `detach()` — remove node from parent (keep in arena)
- [x] `depth()` — compute node depth
- [x] `is_ancestor()` — check ancestor relationship

### 2.3 Iterators
- [x] `children()` — direct children of a node
- [x] `descendants()` — all descendants (DFS)
- [x] `ancestors()` — parent chain to root
- [x] `descendant_count()` — count all descendants

### 2.4 Validation
- [x] Tree invariant maintained (one parent per node except root)
- [x] No cycles in tree
- [x] All children exist in arena
- [x] Parent references are consistent

### 2.5 Tests
- [x] Insert and retrieve nodes
- [x] Parent-child relationships
- [x] Move node between parents
- [x] Remove subtree cleans up all descendants
- [x] Arena reuse after removal (generational indices)
- [x] Tree validation catches inconsistencies
- [x] Deep tree traversal
- [x] Large tree performance (1000+ nodes)

## Phase 3: Protocol (Command System) ✅

### 3.1 Command Types
- [x] `Command` enum with all variants
- [x] Tree commands: CreateNode, RemoveNode, AppendChild, InsertBefore, MoveNode, ReplaceNode, DetachNode
- [x] Style commands: SetStyle, SetForeground, SetBackground, SetBold, etc.
- [x] Layout commands: SetLayout, SetFlexDirection, SetWidth, etc.
- [x] Content commands: SetText, SetAttribute, RemoveAttribute
- [x] Visibility commands: SetDisplay, SetOpacity, SetClip
- [x] Transform commands: SetTranslateX, SetTranslateY, SetZIndex
- [x] Overflow commands: SetOverflow
- [x] Focus commands: FocusNode, BlurNode, SetTabIndex
- [x] Frame commands: BeginFrame, CommitFrame, Invalidate
- [x] Lifecycle commands: Shutdown

### 3.2 Command Processing
- [x] `CommandProcessor` struct
- [x] `process_batch()` — process Vec<Command> atomically
- [x] `process_single()` — process one Command
- [x] Validation before application
- [x] Dirty flag propagation after mutations
- [x] Error collection (CommandError, CommandWarning)

### 3.3 Command Results
- [x] `CommandResult` — success, errors, warnings
- [x] `CommandError` — NodeNotFound, CycleDetected, InvalidOperation
- [x] `CommandWarning` — non-fatal issues

### 3.4 Batch Management
- [x] `CommandBuffer` — pre-allocated Vec<Command>
- [x] `clear()` / `push()` / `drain()`
- [x] Size estimation

### 3.5 Tests
- [x] CreateNode allocates in arena
- [x] AppendChild maintains parent-child
- [x] RemoveNode cleans up subtree
- [x] Invalid commands produce errors
- [x] Batch processing is atomic
- [x] Dirty flags propagate correctly
- [x] 1000 commands process in <2ms

## Phase 4: React Reconciler (HostConfig) ✅

### 4.1 Host Config
- [x] `createInstance()` — create node from type + props
- [x] `createTextInstance()` — create text node
- [x] `appendChild()` — add child to parent
- [x] `removeChild()` — remove child from parent
- [x] `insertBefore()` — insert before reference
- [x] `prepareUpdate()` — compute update payload
- [x] `commitUpdate()` — apply update payload
- [x] `commitTextUpdate()` — update text content
- [x] `finalizeInitialChildren()` — post-creation setup
- [x] `resetAfterCommit()` — flush batch to engine

### 4.2 Command Generation
- [x] Command type definitions
- [x] CommandBuffer class for batching commands

### 4.3 Tests
- [x] TypeScript typecheck passes
- [x] Biome lint passes

## Phase 5: Rust Engine (Command Receiver) ✅

### 5.1 Engine
- [x] `Engine` struct with arena + command buffer
- [x] `process_commands()` — receive and apply commands
- [x] `print_tree()` — debug tree printing
- [x] `node_count()` — current node count
- [x] `validate()` — tree integrity check
- [x] `create_node()` — create node and return ID
- [x] `append_child()` — append child to parent
- [x] `remove_node()` — remove node and descendants
- [x] `set_text()` — set text content
- [x] `set_style()` — set style
- [x] `set_layout()` — set layout
- [x] `begin_frame()` / `commit_frame()` — frame management
- [x] `tree_summary()` — summary of tree state

### 5.2 Debug Output
- [x] Tree printing with indentation
- [x] Node properties display
- [x] Style display
- [x] Layout props display
- [x] Tree summary with node counts and depth

### 5.3 Tests
- [x] Engine processes CreateNode correctly
- [x] Engine processes AppendChild correctly
- [x] Engine processes RemoveNode correctly
- [x] Engine maintains tree invariants
- [x] Debug output is human-readable
- [x] 1000 commands produce correct tree

## Phase 6: Developer Inspector ✅

### 6.1 Tree Inspector
- [x] `print_tree_detail()` — formatted tree with indentation
- [x] `tree_summary()` — node count, depth, kind distribution

### 6.2 Protocol Logger
- [x] `log_command()` — log commands with timestamps
- [x] `recent_commands()` — get recent commands
- [x] `commands_for_node()` — get commands for a specific node

### 6.3 Mutation Logger
- [x] `log_mutation()` — track tree mutations
- [x] `mutation_log()` — get mutation log

### 6.4 Tests
- [x] Tree inspector output is correct
- [x] Protocol logger captures all commands
- [x] Mutation logger tracks changes
