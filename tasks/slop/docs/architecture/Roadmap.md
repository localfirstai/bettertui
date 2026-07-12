# Roadmap (Historical Design Plan)

> **Status:** This is the original detailed phase design plan. It is preserved as a historical reference for the intended engineering breakdown.
> **Canonical status:** See [ROADMAP.md](../../../../ROADMAP.md) at the repository root for the current, code-accurate status.
>
> Phased, incremental, each phase building on the previous.

## Phase 0: Architecture (Current)

**Goal:** Design the architecture before writing any implementation code.

- [x] Architecture analysis
- [x] Node model design
- [x] Protocol design
- [x] Rendering pipeline design
- [x] Layout system design
- [x] Frame buffer design
- [x] Event system design
- [x] Input system design
- [x] Animation system design
- [x] Widget model design
- [x] Plugin API design
- [x] Threading model design
- [x] Memory model design
- [x] Performance strategy
- [x] This roadmap

**Exit criteria:** All architecture documents complete and reviewed.

## Phase 1: Engine Core

**Goal:** Basic terminal I/O and frame buffer rendering.

**Duration:** 2-3 weeks

### 1.1 Terminal Module

- [ ] Terminal size detection
- [ ] Raw mode enable/disable
- [ ] Alternate screen management
- [ ] Capability detection (true color, mouse, etc.)
- [ ] SIGWINCH handling

### 1.2 Frame Buffer

- [ ] Cell struct (char, fg, bg, attributes)
- [ ] FrameBuffer (width, height, cells, dirty)
- [ ] Double buffering
- [ ] Cell-level dirty tracking
- [ ] Clear, set, fill operations

### 1.3 ANSI Encoding

- [ ] Cursor positioning (ESC[{row};{col}H)
- [ ] SGR sequences (colors, bold, italic, etc.)
- [ ] Style coalescing
- [ ] Character output
- [ ] Full frame encoding

### 1.4 Basic Rendering

- [ ] Clear screen
- [ ] Write text at position
- [ ] Apply styles to text
- [ ] Cursor show/hide
- [ ] Full frame render (no diffing yet)

**Deliverable:** A Rust binary that can clear the screen, write styled text, and render a static frame.

## Phase 2: Node Tree

**Goal:** Arena-based node tree with basic operations.

**Duration:** 2-3 weeks

### 2.1 Arena

- [ ] NodeArena struct (slotmap-backed)
- [ ] NodeId (generational index)
- [ ] Insert, get, get_mut, remove
- [ ] Root node management
- [ ] Clear, len, is_empty

### 2.2 RenderNode

- [ ] NodeKind enum
- [ ] Style struct
- [ ] LayoutProps struct
- [ ] Text field
- [ ] Parent/children fields
- [ ] Default implementations

### 2.3 Tree Operations

- [ ] append_child
- [ ] insert_before
- [ ] move_node
- [ ] replace_node
- [ ] remove_subtree
- [ ] detach
- [ ] depth
- [ ] is_ancestor

### 2.4 Iteration

- [ ] children iterator
- [ ] descendants iterator
- [ ] ancestors iterator
- [ ] filter by kind

**Deliverable:** A Rust library that manages an arena-based node tree with CRUD operations.

## Phase 3: Layout Engine

**Goal:** Taffy-based flexbox layout for terminal grids.

**Duration:** 2-3 weeks

### 3.1 Taffy Integration

- [ ] Taffy tree mirroring
- [ ] Style mapping (LayoutProps → taffy::Style)
- [ ] Terminal size as root constraints
- [ ] Basic flexbox layout (row, column)

### 3.2 Layout Properties

- [ ] Flex direction
- [ ] Justify content
- [ ] Align items
- [ ] Flex grow/shrink
- [ ] Gap
- [ ] Padding/margin
- [ ] Width/height (fixed, auto, percentage)

### 3.3 Layout Caching

- [ ] Cache invalidation
- [ ] Incremental layout
- [ ] Cache hit/miss tracking

### 3.4 Layout Results

- [ ] LayoutResult struct (x, y, width, height)
- [ ] Store results on nodes
- [ ] Query layout results

**Deliverable:** A Rust library that computes flexbox layout for a node tree.

## Phase 4: Event System

**Goal:** Keyboard and mouse input handling.

**Duration:** 2 weeks

### 4.1 Keyboard Input

- [ ] Raw byte parsing
- [ ] Escape sequence parsing
- [ ] Key mapping
- [ ] Modifier detection
- [ ] Kitty keyboard protocol

### 4.2 Mouse Input

- [ ] X10 mouse protocol
- [ ] SGR mouse protocol
- [ ] Button detection
- [ ] Position tracking
- [ ] Scroll detection

### 4.3 Event Dispatch

- [ ] Event types (Key, Mouse, Resize)
- [ ] Capture → Target → Bubble propagation
- [ ] stopPropagation
- [ ] preventDefault

### 4.4 Focus Management

- [ ] Focus tracking (one node focused at a time)
- [ ] Tab navigation
- [ ] Focus/blur events
- [ ] Cursor positioning

**Deliverable:** A Rust binary that handles keyboard and mouse input, dispatches events, and manages focus.

## Phase 5: Protocol Layer

**Goal:** TypeScript ↔ Rust communication via command protocol.

**Duration:** 2-3 weeks

### 5.1 Command Codec

- [ ] Command enum (all variants)
- [ ] Binary encoding
- [ ] Binary decoding
- [ ] Version field

### 5.2 Batch Processing

- [ ] Command buffer (Vec<Command>)
- [ ] Batch encoding/decoding
- [ ] Atomic batch processing
- [ ] Error handling

### 5.3 napi-rs Bindings

- [ ] Process commands binding
- [ ] Query node tree binding
- [ ] Subscribe to events binding
- [ ] Resize binding

### 5.4 TypeScript Protocol

- [ ] @bettertui/protocol package
- [ ] Encoder/decoder
- [ ] Command types

**Deliverable:** TypeScript can send commands to Rust and receive events back.

## Phase 6: React Integration

**Goal:** Working React components that render to terminal.

**Duration:** 3-4 weeks

### 6.1 Reconciler

- [ ] Host config implementation
- [ ] createInstance
- [ ] appendChild/removeChild
- [ ] commitUpdate
- [ ] Text instance support

### 6.2 React Components

- [ ] Box component
- [ ] Text component
- [ ] Flex component
- [ ] Spacer component
- [ ] Provider component

### 6.3 Renderer

- [ ] createRenderer function
- [ ] render method
- [ ] Frame scheduling
- [ ] Event bridging

### 6.4 Hooks

- [ ] useRenderer
- [ ] useKeyboard
- [ ] useOnResize
- [ ] useTerminalDimensions

**Deliverable:** A React application that renders to the terminal using BetterTUI.

## Phase 7: Widgets

**Goal:** Reusable widget library.

**Duration:** 3-4 weeks

### 7.1 Basic Widgets

- [ ] Box
- [ ] Text
- [ ] Input (single-line)
- [ ] List
- [ ] Spacer
- [ ] Separator

### 7.2 Advanced Widgets

- [ ] Table
- [ ] Tree
- [ ] Scroll container
- [ ] Tab bar
- [ ] Modal dialog
- [ ] Progress bar
- [ ] Spinner

### 7.3 Widget System

- [ ] Widget trait
- [ ] Widget lifecycle
- [ ] Widget composition
- [ ] Widget state management

**Deliverable:** A widget library with 10+ reusable widgets.

## Phase 8: Rendering Pipeline

**Goal:** Complete rendering pipeline with diffing and optimization.

**Duration:** 2-3 weeks

### 8.1 Frame Diffing

- [ ] Cell comparison
- [ ] Dirty region detection
- [ ] Region merging
- [ ] Full-frame optimization

### 8.2 Optimized Output

- [ ] Style coalescing
- [ ] Move optimization
- [ ] Buffered writes
- [ ] Single syscall output

### 8.3 Render Tree

- [ ] Visibility filtering
- [ ] Style resolution (inheritance)
- [ ] Clip region computation
- [ ] Z-order sorting

**Deliverable:** A rendering pipeline that efficiently updates only changed cells.

## Phase 9: Theming

**Goal:** Theme system for consistent styling.

**Duration:** 1-2 weeks

### 9.1 Theme Model

- [ ] Theme struct (colors, borders, fonts)
- [ ] Color palette (primary, secondary, accent, etc.)
- [ ] Border styles
- [ ] Theme inheritance

### 9.2 Theme Application

- [ ] Resolve theme tokens to concrete values
- [ ] Apply theme to node tree
- [ ] Theme switching

### 9.3 Built-in Themes

- [ ] Dark theme
- [ ] Light theme
- [ ] High-contrast theme

**Deliverable:** A theme system with 3 built-in themes.

## Phase 10: Animation

**Goal:** Tween-based animations at 60fps.

**Duration:** 2-3 weeks

### 10.1 Animation Engine

- [ ] Animation struct
- [ ] Keyframe interpolation
- [ ] Easing functions
- [ ] Timeline management

### 10.2 Animatable Properties

- [ ] Style properties (fg, bg, opacity)
- [ ] Layout properties (width, height, padding)
- [ ] Transform properties (translate)

### 10.3 Animation API

- [ ] animate function
- [ ] chain_animations
- [ ] parallel_animations
- [ ] Spring animations

**Deliverable:** An animation system that can tween node properties over time.

## Phase 11: DevTools

**Goal:** Developer tools for debugging and inspection.

**Duration:** 2-3 weeks

### 11.1 Inspector

- [ ] Tree visualization
- [ ] Node properties display
- [ ] Layout visualization
- [ ] Style resolution display

### 11.2 Profiler

- [ ] Frame timing
- [ ] Layout timing
- [ ] Render timing
- [ ] Memory usage

### 11.3 Overlay

- [ ] Performance overlay
- [ ] Layout grid overlay
- [ ] Dirty region overlay
- [ ] Hit test overlay

**Deliverable:** DevTools overlay that shows tree structure, layout, and performance metrics.

## Phase 12: Advanced Features

**Goal:** Clipboard, selection, text editing, scroll.

**Duration:** 3-4 weeks

### 12.1 Clipboard

- [ ] Read clipboard
- [ ] Write clipboard
- [ ] Bracketed paste

### 12.2 Text Selection

- [ ] Selection model
- [ ] Selection rendering
- [ ] Copy to clipboard
- [ ] Word/line selection

### 12.3 Text Editing

- [ ] Rope-based buffer
- [ ] Cursor management
- [ ] Undo/redo
- [ ] Selection editing

### 12.4 Scrolling

- [ ] Scroll container
- [ ] Scrollbar rendering
- [ ] Scroll-to-view
- [ ] Smooth scrolling

**Deliverable:** Full text editing, selection, clipboard, and scroll support.

## Phase 13: Plugin System

**Goal:** Extensible architecture for community contributions.

**Duration:** 2-3 weeks

### 13.1 Plugin Core

- [ ] Plugin trait
- [ ] Plugin registry
- [ ] Extension points
- [ ] Plugin lifecycle

### 13.2 Extension Types

- [ ] Widget extensions
- [ ] Theme extensions
- [ ] Command extensions
- [ ] Animation extensions
- [ ] Input handler extensions

### 13.3 Plugin Loading

- [ ] NPM package discovery
- [ ] Plugin manifest
- [ ] Capability system

**Deliverable:** A plugin system that allows third-party extensions.

## Phase 14: Documentation & Examples

**Goal:** Comprehensive documentation and example applications.

**Duration:** 2-3 weeks

### 14.1 Documentation

- [ ] Getting started guide
- [ ] API reference
- [ ] Widget catalog
- [ ] Theme guide
- [ ] Plugin guide
- [ ] Migration guide (from Ink)

### 14.2 Examples

- [ ] Counter
- [ ] Dashboard
- [ ] Text editor
- [ ] Table with sorting
- [ ] Tree view
- [ ] Mouse interaction

**Deliverable:** Complete documentation and 6 working examples.

## Phase 15: Multi-Framework Support

**Goal:** Vue, Solid, Svelte adapters.

**Duration:** 4-6 weeks

### 15.1 Vue Adapter

- [ ] Vue reconciler
- [ ] Vue components
- [ ] Vue hooks

### 15.2 Solid Adapter

- [ ] Solid reconciler
- [ ] Solid components
- [ ] Solid hooks

### 15.3 Svelte Adapter

- [ ] Svelte integration
- [ ] Svelte components
- [ ] Svelte actions

**Deliverable:** Working adapters for Vue, Solid, and Svelte.

## Dependency Graph

```
Phase 0: Architecture
    ↓
Phase 1: Engine Core
    ↓
Phase 2: Node Tree ← Phase 3: Layout Engine
    ↓
Phase 4: Event System
    ↓
Phase 5: Protocol Layer
    ↓
Phase 6: React Integration ← Phase 7: Widgets
    ↓
Phase 8: Rendering Pipeline ← Phase 9: Theming
    ↓
Phase 10: Animation
    ↓
Phase 11: DevTools
    ↓
Phase 12: Advanced Features
    ↓
Phase 13: Plugin System
    ↓
Phase 14: Documentation & Examples
    ↓
Phase 15: Multi-Framework Support
```

## Risk Assessment

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Taffy doesn't work well for terminal grids | High | Low | Fork and adapt Taffy's internals |
| napi-rs performance is insufficient | High | Low | Profile early, optimize FFI boundary |
| React reconciler is too complex | Medium | Medium | Start with minimal implementation, iterate |
| Terminal compatibility issues | Medium | Medium | Test on multiple terminals (iTerm2, Kitty, Alacritty, Windows Terminal) |
| Memory leaks in Rust engine | High | Low | Extensive testing, sanitizer integration |
| Scope creep delays delivery | High | High | Strict phase boundaries, MVP mindset |
