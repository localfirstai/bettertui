# IDE Platform Foundation

> **Status:** Active
> **Objective:** Make BetterTUI capable of powering a modern coding IDE.
> **Constraint:** Extend existing architecture. No duplicate systems. No redesign.

---

## Architecture Validation

### What Exists (DO NOT CHANGE)

```
bettertui-engine (Rust)
├── tree/          NodeArena, RenderNode, NodeKind, Style, LayoutProps
├── protocol/      Command (50+ variants), CommandProcessor, CommandBuffer
├── layout/        LayoutEngine (Taffy), LayoutTreeSync, LayoutResult
├── renderer/      Renderer, RenderBackend, AnsiBackend
├── painter/       Painter (paints RenderTree → FrameBuffer)
├── framebuffer/   FrameBuffer (cell grid, double buffered)
├── dirty_diff/    DirtyDiff, DirtyRegion
├── compositor/    Layer, CompositorRenderer (7 z-index levels)
├── events/        EventBus, EventDispatcher (capture/target/bubble)
├── scheduler/     Scheduler (priority queue, frame budget)
├── focus/         FocusManager, FocusScope, FocusTraversal
├── pty/           PtyRuntime, PtyProcess, PtyReader, PtyWriter
├── terminal/      Terminal (raw mode, alt screen, crossterm)
├── capabilities/  CapabilityDetector
├── text/          TextBuffer, TextCursor, TextSearch, TextUndo
├── input/         InputRuntime, KeyboardState, MouseState, ClipboardState
├── widgets/       Widget trait, WidgetHost, WidgetRegistry, WidgetContext
│   ├── BoxWidget, TextWidget, FlexWidget, SpacerWidget
│   ├── ScrollAreaWidget, ContainerWidget
│   ├── ChatView, PromptComposer, MarkdownRenderer
│   └── Pipeline, Reconciler, Theme
├── ansi/          AnsiEncoder, AnsiParser
├── glyph/         GlyphCache, GlyphMetrics
├── animation/     AnimationEngine (stub)
├── engine/        Engine (high-level API), Inspector
└── lib.rs         34 pub mod declarations
```

### What We Add (EXTEND, DON'T REPLACE)

```
New Rust Modules:
├── pane/           PaneManager, Pane, PaneSplit, PaneLayout
├── workspace/      Workspace, WorkspaceState, FolderEntry
├── command_ext/    CommandRegistry, CommandContext, CommandHistory
├── keybinding/     KeybindingEngine, Keymap, ChordState, BindingConflict
├── terminal_wid/   TerminalWidget (PTY + ANSI + ScreenBuffer → Widget)
├── virtual_view/   VirtualList, VirtualTree, VirtualTable, Viewport
├── filesystem/     FileSystem, DirEntry, FileWatcher, GitStatus
├── palette/        CommandPalette, FuzzySearch, PaletteEntry
├── plugin/         PluginHost, PluginManifest, PluginLifecycle
└── snapshot/       SnapshotTest, GoldenFile, SnapshotDiff

New TypeScript Packages:
├── @bettertui/pane        Pane system TypeScript API
├── @bettertui/workspace   Workspace runtime
├── @bettertui/commands    Command platform
├── @bettertui/keybindings Keybinding engine
├── @bettertui/terminal    Terminal widget wrapper
├── @bettertui/viewport    Virtual viewport
├── @bettertui/fs          Filesystem service
├── @bettertui/palette     Command palette
├── @bettertui/plugin      Plugin host
└── @bettertui/testing     Snapshot testing utilities
```

---

## Dependency Graph

```
Phase 1: Pane System
    │
    ├── Extends: tree/, layout/, focus/, events/
    └── Depends on: NodeArena, LayoutEngine, FocusManager
    │
Phase 2: Workspace Runtime
    │
    ├── Extends: (new TS module)
    └── Depends on: Pane System (for editor panes)
    │
Phase 3: Command Platform
    │
    ├── Extends: protocol/command.rs, events/
    └── Depends on: (standalone, extends existing)
    │
Phase 4: Keybinding Engine
    │
    ├── Extends: events/, input/
    └── Depends on: Command Platform (commands to execute)
    │
Phase 5: Terminal Widget
    │
    ├── Extends: widgets/, pty/, ansi/, framebuffer/
    └── Depends on: Widget trait, PtyRuntime, AnsiParser
    │
Phase 6: Virtual Viewport
    │
    ├── Extends: widgets/, renderer/, framebuffer/
    └── Depends on: Widget trait, FrameBuffer
    │
Phase 7: File System Platform
    │
    ├── Extends: (new TS module + Rust service)
    └── Depends on: Workspace Runtime
    │
Phase 8: Command Palette Foundation
    │
    ├── Extends: Command Platform, Keybinding Engine
    └── Depends on: Fuzzy search, Command registry
    │
Phase 9: Plugin Host
    │
    ├── Extends: widgets/registry, protocol/
    └── Depends on: Command Platform, Widget Registry
    │
Phase 10: Snapshot Testing
    │
    ├── Extends: (new testing infra)
    └── Depends on: Renderer, FrameBuffer
    │
Phase 11: Performance
    │
    └── Optimizes: All above
```

---

## Phase 1: Pane System

### Design

The pane system manages screen subdivisions. Each pane owns a widget tree. The renderer does not know about panes — it renders whatever widget tree is assigned to it.

```
PaneManager
├── root: Pane (always exists)
│   ├── split: Horizontal | Vertical | None
│   ├── children: [Pane, Pane] (if split)
│   ├── widget_tree: NodeId (root of widget subtree)
│   ├── size: (width, height)
│   ├── position: (x, y)
│   ├── focused: bool
│   └── history: Vec<FocusEvent>
```

### Rust Implementation

**File: `native/engine/src/pane/mod.rs`**

```rust
pub struct PaneManager {
    panes: NodeArena,  // Reuse existing arena
    root: PaneId,
    focused: PaneId,
    splits: HashMap<SplitId, PaneSplit>,
}

pub struct Pane {
    id: PaneId,
    parent: Option<PaneId>,
    children: SmallVec<[PaneId; 2]>,
    split: Option<PaneSplit>,
    widget_root: Option<NodeId>,  // Points into engine's NodeArena
    bounds: Rect,
    min_size: Size,
    focused: bool,
}

pub struct PaneSplit {
    id: SplitId,
    direction: SplitDirection,  // Horizontal | Vertical
    ratio: f32,                 // 0.0 - 1.0
    position: f32,              // In cells
}

pub enum SplitDirection { Horizontal, Vertical }
```

**Integration points:**
- `PaneManager` lives alongside `Engine`, not inside it
- Pane bounds feed into `LayoutEngine` as constraints
- Focus propagation goes through `FocusManager` scopes
- Each pane gets its own `FocusScope`

**Key operations:**
- `split_pane(pane_id, direction, ratio)` → creates two panes
- `remove_pane(pane_id)` → merges into sibling
- `resize_pane(pane_id, new_size)` → updates bounds, triggers relayout
- `focus_pane(pane_id)` → updates focus scope
- `swap_pane(pane_id_a, pane_id_b)` → swaps widget trees
- `get_pane_bounds(pane_id)` → returns Rect for layout constraints

**Tests:**
- Root pane always exists
- Split creates two panes with correct bounds
- Remove merges into sibling
- Resize respects min_size
- Focus switches between panes
- Nested splits work (split a split child)
- Widget tree stays attached to correct pane

### napi Bindings

Add `NapiPaneManager` class:
- `new()`, `split(paneId, direction, ratio)`, `remove(paneId)`, `resize(paneId, w, h)`, `focus(paneId)`, `focused()`, `panes()`, `bounds(paneId)`

---

## Phase 2: Workspace Runtime

### Design

Workspace = application context. Not tied to any specific editor.

```
Workspace
├── id: WorkspaceId
├── name: String
├── root_path: PathBuf
├── folders: Vec<FolderEntry>
├── open_editors: Vec<EditorEntry>
├── recent_files: Vec<PathBuf>
├── state: WorkspaceState
├── config: WorkspaceConfig
├── env: HashMap<String, String>
```

### TypeScript Implementation

**File: `packages/workspace/src/index.ts`**

```typescript
interface Workspace {
  id: string;
  name: string;
  rootPath: string;
  folders: FolderEntry[];
  openEditors: EditorEntry[];
  recentFiles: string[];
  config: WorkspaceConfig;
  env: Record<string, string>;
}

interface FolderEntry {
  path: string;
  name: string;
  visible: boolean;
}

interface EditorEntry {
  id: string;
  filePath: string;
  paneId: string;
  dirty: boolean;
  cursorPosition: Point;
  scrollTop: number;
}

interface WorkspaceConfig {
  tabSize: number;
  insertSpaces: boolean;
  lineNumbers: boolean;
  wordWrap: boolean;
  fontSize: number;
}
```

**Key operations:**
- `createWorkspace(rootPath)` → initializes workspace
- `addFolder(path)` / `removeFolder(path)`
- `openEditor(filePath, paneId)` → creates EditorEntry
- `closeEditor(editorId)`
- `switchEditor(editorId)` → focuses pane
- `saveWorkspaceState()` → serializes to disk
- `loadWorkspaceState(path)` → deserializes

### Rust Implementation

Minimal Rust side — workspace is mostly TS state. Only expose:
- `WorkspaceState` struct for serialization
- `FolderEntry` struct for filesystem hooks

---

## Phase 3: Command Platform

### Design

Every user action becomes a command. Extends existing `Command` enum.

```
CommandRegistry
├── commands: HashMap<CommandId, CommandDef>
├── history: Vec<CommandEntry>
├── contexts: HashMap<ContextId, ContextPredicate>
├── undo_stack: Vec<UndoEntry>
└── redo_stack: Vec<UndoEntry>
```

### Rust Implementation

**File: `native/engine/src/command_ext/mod.rs`**

```rust
pub struct CommandRegistry {
    commands: HashMap<CommandId, CommandDef>,
    history: Vec<CommandEntry>,
    undo_stack: Vec<UndoEntry>,
    redo_stack: Vec<UndoEntry>,
}

pub struct CommandDef {
    id: CommandId,
    name: String,
    category: String,
    description: String,
    keybinding: Option<KeybindingRef>,
    enabled: bool,
    visible: bool,
    handler: CommandHandler,  // Fn or enum variant
}

pub struct CommandEntry {
    command_id: CommandId,
    args: CommandArgs,
    timestamp: Instant,
    context: CommandContext,
}

pub struct UndoEntry {
    command_id: CommandId,
    inverse: Vec<Command>,  // Commands to undo this action
}
```

**Built-in commands:**
- File: Open, Save, SaveAs, Close, CloseAll
- Edit: Undo, Redo, Cut, Copy, Paste, Find, Replace
- View: ToggleSidebar, ToggleTerminal, ToggleMinimap, ZoomIn, ZoomOut
- Pane: SplitHorizontal, SplitVertical, FocusLeft, FocusRight, FocusUp, FocusDown, ClosePane
- Navigation: GoToLine, GoToDefinition, GoBack, GoForward
- Workspace: OpenFolder, AddFolder, NewFile, DeleteFile, RenameFile

**Integration:**
- Extends existing `Command` enum in `protocol/command.rs`
- Commands can carry args (JSON-serializable)
- Undo/redo stores inverse command sequences
- History is queryable (recent commands, command count)

---

## Phase 4: Keybinding Engine

### Design

Production-quality keybinding system. Integrates with existing EventBus.

```
KeybindingEngine
├── keymaps: Vec<Keymap>
├── chord_state: ChordState
├── context_evaluator: ContextEvaluator
└── conflict_detector: ConflictDetector

Keymap
├── name: String
├── priority: u32
├── platform: Option<Platform>
├── when: Option<String>  // Context expression
├── bindings: Vec<Binding>

Binding
├── keys: KeySequence
├── command: CommandId
├── when: Option<String>
├── args: Option<CommandArgs>

ChordState
├── pending: Option<KeySequence>
├── timeout: Duration
├── candidates: Vec<Binding>
```

### Rust Implementation

**File: `native/engine/src/keybinding/mod.rs`**

```rust
pub struct KeybindingEngine {
    keymaps: Vec<Keymap>,
    chord_state: ChordState,
    history: VecDeque<KeyEvent>,  // Last N events for chord matching
}

pub struct Keymap {
    name: String,
    priority: u32,
    platform: Option<Platform>,
    when: Option<ContextExpr>,
    bindings: Vec<Binding>,
}

pub struct Binding {
    keys: SmallVec<[Key; 4]>,  // Modifier + key(s)
    command: CommandId,
    when: Option<ContextExpr>,
    args: Option<serde_json::Value>,
}

pub enum ContextExpr {
    And(Box<ContextExpr>, Box<ContextExpr>),
    Or(Box<ContextExpr>, Box<ContextExpr>),
    Not(Box<ContextExpr>),
    Equals(String, String),
    Defined(String),
}

pub struct ChordState {
    pending: Option<SmallVec<[Key; 4]>>,
    started_at: Instant,
    timeout: Duration,
}
```

**Features:**
- Single key, modifier combos, chord sequences (Ctrl+K Ctrl+C)
- Context expressions: `when: "editorTextFocus && !terminalFocus"`
- Priority resolution: higher priority keymaps win
- Conflict detection at registration time
- Platform-specific overrides (Cmd on macOS, Ctrl on Linux)
- Runtime registration (plugins can add keymaps)
- History for chord matching

**Default keymaps:**
- `editor` — active when editor focused
- `terminal` — active when terminal focused
- `sidebar` — active when sidebar focused
- `global` — always active

---

## Phase 5: Terminal Widget

### Design

First platform widget. Wraps PTY + ANSI parser + screen buffer into a Widget.

```
TerminalWidget
├── pty: PtyRuntime
├── parser: AnsiParser
├── screen: ScreenBuffer
├── scrollback: Vec<ScreenRow>
├── selection: Option<Selection>
├── clipboard: ClipboardState
├── search: Option<SearchState>
```

### Rust Implementation

**File: `native/engine/src/widgets/terminal.rs`**

```rust
pub struct TerminalWidget {
    id: WidgetId,
    pty: PtyRuntime,
    parser: AnsiParser,
    screen: ScreenBuffer,        // Current visible screen
    scrollback: VecDeque<ScreenRow>,  // History
    cursor: CursorPosition,
    selection: Option<Selection>,
    search: Option<SearchState>,
    alt_screen: bool,
    focus: bool,
}

pub struct ScreenBuffer {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    cursor: (usize, usize),
    insert_mode: bool,
    origin_mode: bool,
    wrap_mode: bool,
}

impl Widget for TerminalWidget {
    fn render(&self, ctx: &mut WidgetContext) -> RenderResult {
        // Convert ScreenBuffer → render objects
        // Apply scrollback offset
        // Render cursor if focused
        // Render selection highlight
    }

    fn handle_event(&mut self, event: &Event, ctx: &mut WidgetContext) -> EventResult {
        match event {
            Event::Key(key) => {
                // Write key to PTY
                self.pty.write(key.to_bytes());
                EventResult::Consumed
            }
            Event::Mouse(mouse) => {
                // Handle scroll, selection, click
                EventResult::Consumed
            }
            _ => EventResult::Ignored,
        }
    }

    fn update(&mut self, ctx: &mut WidgetContext) {
        // Read PTY output
        if let Some(output) = self.pty.try_read() {
            self.parser.parse(&output, &mut self.screen);
            self.scrollback.extend(self.parser.drain_lines());
        }
    }
}
```

**Capabilities:**
- Shell spawning (bash, zsh, fish, powershell)
- Resize (SIGWINCH to PTY)
- Clipboard (OSC 52)
- Selection (mouse drag)
- Scrollback (configurable limit)
- Focus tracking
- Alt screen support
- Search (future phase)

---

## Phase 6: Virtual Viewport

### Design

Viewport virtualization. Only visible items rendered. Standard strategy for all list-like widgets.

```
VirtualViewport<T>
├── items: Vec<T>
├── viewport_height: usize
├── scroll_offset: usize
├── item_height: Fn(&T) -> usize  // Fixed or variable
├── overscan: usize  // Extra items above/below viewport
└── render_item: Fn(&T, usize) -> RenderObject
```

### Rust Implementation

**File: `native/engine/src/widgets/virtual_viewport.rs`**

```rust
pub struct VirtualViewport<T> {
    items: Vec<T>,
    viewport_height: usize,
    scroll_offset: usize,
    item_height: ItemHeightFn,
    overscan: usize,
    total_height: usize,
    cached_heights: Vec<usize>,  // For variable height
}

pub enum ItemHeightFn {
    Fixed(usize),
    Variable(Box<dyn Fn(&dyn Any) -> usize>),
}

impl<T: Renderable> VirtualViewport<T> {
    pub fn visible_range(&self) -> Range<usize> {
        let start = self.scroll_offset.saturating_sub(self.overscan);
        let end = (self.scroll_offset + self.viewport_height + self.overscan)
            .min(self.items.len());
        start..end
    }

    pub fn render_visible(&self, ctx: &mut WidgetContext) -> Vec<RenderObject> {
        let range = self.visible_range();
        let y_offset = self.y_offset_for_index(range.start);

        range.enumerate().map(|(i, item)| {
            let y = y_offset + self.height_for_index(i);
            self.render_item(item, i, y, ctx)
        }).collect()
    }

    pub fn scroll_to(&mut self, index: usize) {
        self.scroll_offset = index.min(self.items.len().saturating_sub(1));
    }

    pub fn scroll_by(&mut self, delta: isize) {
        self.scroll_offset = (self.scroll_offset as isize + delta)
            .max(0) as usize;
    }
}
```

**Supported modes:**
- List: fixed row height
- Tree: variable height with expand/collapse
- Table: fixed row height + column widths
- Chat: variable height messages
- Markdown: variable height blocks
- Log viewer: fixed row height + auto-scroll
- Editor: line-based with line numbers

**Target:** 100,000 rows → render only ~50 visible rows.

---

## Phase 7: File System Platform

### Design

Filesystem service. No UI. Pure data layer.

```
FileSystem
├── root: PathBuf
├── tree: DirTree
├── watcher: FileWatcher
├── git: Option<GitStatus>
├── cache: FileCache
└── ignore: IgnoreRules
```

### Rust Implementation

**File: `native/engine/src/filesystem/mod.rs`**

```rust
pub struct FileSystem {
    root: PathBuf,
    tree: DirTree,
    watcher: Option<FileWatcher>,
    git: Option<GitStatus>,
    cache: HashMap<PathBuf, FileEntry>,
    ignore: IgnoreRules,
}

pub struct DirTree {
    entries: Arena<DirEntry>,
    root: NodeId,
}

pub struct DirEntry {
    name: String,
    path: PathBuf,
    kind: FileKind,  // File | Directory | Symlink
    children: Option<Vec<NodeId>>,
    expanded: bool,
    git_status: Option<GitStatus>,
    metadata: Option<FileMetadata>,
}

pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
    rx: Receiver<FileEvent>,
}

pub enum FileEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
    Renamed(PathBuf, PathBuf),
}
```

**Features:**
- Lazy loading (children loaded on expand)
- File watching (notify crate)
- Git status (via git2 or command)
- Ignore patterns (.gitignore, .ignore)
- File metadata (size, modified, permissions)
- Icons (via nerd font / file extension mapping)
- Sorting (name, size, modified, type)
- Filtering (name, type, git status)
- Search hooks (grep, fuzzy find)

---

## Phase 8: Command Palette Foundation

### Design

Fuzzy search + command discovery + keyboard navigation.

```
CommandPalette
├── query: String
├── results: Vec<PaletteEntry>
├── selected: usize
├── sources: Vec<Box<dyn PaletteSource>>
├── recent: Vec<CommandId>
└── preview: Option<PreviewContent>
```

### Rust Implementation

**File: `native/engine/src/palette/mod.rs`**

```rust
pub struct CommandPalette {
    query: String,
    entries: Vec<PaletteEntry>,
    filtered: Vec<usize>,  // Indices into entries
    selected: usize,
    recent: VecDeque<CommandId>,
    sources: Vec<PaletteSource>,
}

pub struct PaletteEntry {
    id: EntryId,
    label: String,
    description: String,
    category: String,
    keybinding: Option<String>,
    score: f64,  // Fuzzy match score
    source: SourceType,
    icon: Option<char>,
}

pub enum SourceType {
    Command,
    File,
    Symbol,
    Recent,
    Plugin,
}

impl CommandPalette {
    pub fn fuzzy_search(&mut self, query: &str) {
        // Score each entry against query
        // Sort by score descending
        // Update filtered list
    }

    pub fn select_next(&mut self) { /* ... */ }
    pub fn select_prev(&mut self) { /* ... */ }
    pub fn execute_selected(&self) -> Option<CommandId> { /* ... */ }
    pub fn preview_selected(&self) -> Option<PreviewContent> { /* ... */ }
}
```

**Features:**
- Fuzzy matching (Smith-Waterman or similar)
- Command discovery (from CommandRegistry)
- File search (from FileSystem)
- Recently used commands (tracked)
- Plugin commands (from PluginHost)
- Context-aware filtering
- Keyboard navigation (up/down/enter/escape)
- Preview panel (file content, command description)

---

## Phase 9: Plugin Host

### Design

Plugin runtime. Extends existing systems, doesn't replace them.

```
PluginHost
├── plugins: HashMap<PluginId, PluginInstance>
├── manifests: HashMap<PluginId, PluginManifest>
├── lifecycle: PluginLifecycle
├── registry: PluginRegistry
└── capabilities: CapabilityNegotiation
```

### Rust Implementation

**File: `native/engine/src/plugin/mod.rs`**

```rust
pub struct PluginHost {
    plugins: HashMap<PluginId, PluginInstance>,
    registry: PluginRegistry,
    lifecycle: PluginLifecycle,
}

pub struct PluginManifest {
    id: PluginId,
    name: String,
    version: Version,
    description: String,
    author: String,
    engine_version: VersionRange,
    capabilities: Vec<Capability>,
    activation_events: Vec<ActivationEvent>,
}

pub struct PluginInstance {
    manifest: PluginManifest,
    state: PluginState,  // Installed | Activated | Deactivated
    commands: Vec<CommandId>,
    keybindings: Vec<Binding>,
    themes: Vec<ThemeId>,
    widgets: Vec<WidgetId>,
}

pub enum Capability {
    RegisterCommands,
    RegisterKeybindings,
    RegisterWidgets,
    RegisterThemes,
    AccessFileSystem,
    AccessTerminal,
    AccessNetwork,
}

pub enum ActivationEvent {
    OnStartup,
    OnCommand(String),
    OnFileOpen(String),
    OnWorkspaceOpen,
}
```

**Lifecycle:**
1. Discovery (scan plugin directories)
2. Manifest parsing + version check
3. Capability negotiation
4. Registration (commands, keybindings, widgets, themes)
5. Activation (on startup or on activation event)
6. Deactivation (on workspace close or explicit)
7. Disposal (cleanup)

**Integration:**
- Commands → `CommandRegistry`
- Keybindings → `KeybindingEngine`
- Widgets → `WidgetRegistry`
- Themes → `Theme`
- Each plugin gets isolated state

---

## Phase 10: Snapshot Testing

### Design

Widget → Render → FrameBuffer → Snapshot → Golden comparison.

```
SnapshotTest
├── name: String
├── widget: Box<dyn Widget>
├── size: (usize, usize)
├── golden: Option<Vec<u8>>
├── current: Vec<u8>
└── diff: Option<SnapshotDiff>
```

### Rust Implementation

**File: `native/engine/src/snapshot/mod.rs`**

```rust
pub struct SnapshotTest {
    name: String,
    width: usize,
    height: usize,
}

impl SnapshotTest {
    pub fn capture(widget: &dyn Widget, width: usize, height: usize) -> Snapshot {
        let mut ctx = WidgetContext::new(width, height);
        let render_result = widget.render(&mut ctx);
        let mut framebuffer = FrameBuffer::new(width, height);
        // Paint render result into framebuffer
        Snapshot {
            name: String::new(),
            cells: framebuffer.cells().to_vec(),
            width,
            height,
        }
    }

    pub fn compare(current: &Snapshot, golden: &Snapshot) -> SnapshotDiff {
        // Cell-by-cell comparison
        // Return diff regions
    }

    pub fn save(golden: &Snapshot, path: &Path) { /* ... */ }
    pub fn load(path: &Path) -> Option<Snapshot> { /* ... */ }
}

pub struct SnapshotDiff {
    pub regions: Vec<DiffRegion>,
    pub total_changed: usize,
}

pub struct DiffRegion {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub expected: Vec<Cell>,
    pub actual: Vec<Cell>,
}
```

**Workflow:**
1. `cargo test` runs snapshot tests
2. First run: creates golden files in `tests/snapshots/`
3. Subsequent runs: compares against golden
4. `UPDATE_GOLDEN=1 cargo test` updates golden files
5. CI checks golden files are up to date

---

## Phase 11: Performance

### Targets

| Metric | Target |
|--------|--------|
| Frame time | < 16ms (60fps) |
| Layout (1000 nodes) | < 2ms |
| Dirty diff (10% change) | < 1ms |
| ANSI encode (full frame) | < 3ms |
| PTY throughput | > 100KB/s |
| Virtual viewport (100K rows) | < 1ms visible range |
| Command execution | < 0.1ms |
| Keybinding lookup | < 0.01ms |
| Snapshot generation | < 5ms |
| Memory (1000 widgets) | < 50MB |

### Optimizations

1. **Pane rendering** — Only re-render dirty panes
2. **Workspace switching** — Cache workspace state, lazy load
3. **PTY throughput** — Batch reads, avoid per-byte processing
4. **Virtual viewport** — Binary search for variable heights
5. **Command execution** — Direct dispatch, no hash lookup
6. **Keybinding lookup** — Trie-based matching
7. **Snapshot generation** — Incremental snapshots
8. **Memory allocations** — Arena allocation, object pooling

---

## Testing Strategy

### Rust Tests (target: 1000+)

| Module | Current | Target |
|--------|---------|--------|
| pane/ | 0 | 50+ |
| command_ext/ | 0 | 40+ |
| keybinding/ | 0 | 60+ |
| terminal_wid/ | 0 | 40+ |
| virtual_view/ | 0 | 30+ |
| filesystem/ | 0 | 30+ |
| palette/ | 0 | 25+ |
| plugin/ | 0 | 30+ |
| snapshot/ | 0 | 20+ |
| Existing | 777 | 777 (no regression) |
| **Total** | **777** | **1100+** |

### Integration Tests

- Pane split/resize/focus end-to-end
- Command execute → undo → redo
- Keybinding chord sequence
- Terminal spawn → type → output
- Virtual viewport scroll → render
- File watch → tree update
- Palette search → select → execute
- Plugin load → activate → command

### TypeScript Tests

- Workspace state serialization
- Command palette search
- Keybinding parsing
- Plugin manifest validation

---

## Documentation Updates

### New Documents

| File | Content |
|------|---------|
| `docs/architecture/PaneSystem.md` | Pane model, splits, focus, integration |
| `docs/architecture/Workspace.md` | Workspace state, folders, editors |
| `docs/architecture/CommandPlatform.md` | Command registry, undo/redo, history |
| `docs/architecture/KeybindingEngine.md` | Keymaps, chords, context, conflicts |
| `docs/architecture/TerminalWidget.md` | PTY + ANSI + screen buffer integration |
| `docs/architecture/VirtualViewport.md` | Virtualization strategy, modes |
| `docs/architecture/FileSystem.md` | Directory tree, watching, git status |
| `docs/architecture/CommandPalette.md` | Fuzzy search, discovery, navigation |
| `docs/architecture/PluginHost.md` | Lifecycle, capabilities, extension points |
| `docs/architecture/SnapshotTesting.md` | Golden files, workflow, CI |

### Updated Documents

| File | Changes |
|------|---------|
| `docs/architecture/Architecture.md` | Add IDE platform layer diagram |
| `docs/architecture/Roadmap.md` | Add Phase 16-26 (IDE Platform) |
| `docs/architecture/TODO.md` | Replace with IDE Platform plan |
| `ARCHITECTURE.md` | Update high-level overview |
| `ROADMAP.md` | Add IDE platform phases |
| `README.md` | Update feature list |

---

## File Impact Summary

### New Rust Files

| File | Lines (est) | Purpose |
|------|-------------|---------|
| `pane/mod.rs` | ~400 | PaneManager, Pane, PaneSplit |
| `pane/tests.rs` | ~300 | Pane system tests |
| `command_ext/mod.rs` | ~350 | CommandRegistry, undo/redo |
| `command_ext/tests.rs` | ~250 | Command platform tests |
| `keybinding/mod.rs` | ~500 | KeybindingEngine, Keymap, chords |
| `keybinding/tests.rs` | ~400 | Keybinding tests |
| `widgets/terminal.rs` | ~450 | TerminalWidget |
| `widgets/terminal/tests.rs` | ~300 | Terminal widget tests |
| `widgets/virtual_viewport.rs` | ~350 | VirtualViewport |
| `widgets/virtual_viewport/tests.rs` | ~250 | Viewport tests |
| `filesystem/mod.rs` | ~400 | FileSystem, DirTree |
| `filesystem/tests.rs` | ~250 | Filesystem tests |
| `palette/mod.rs` | ~350 | CommandPalette, fuzzy search |
| `palette/tests.rs` | ~250 | Palette tests |
| `plugin/mod.rs` | ~400 | PluginHost, PluginManifest |
| `plugin/tests.rs` | ~250 | Plugin tests |
| `snapshot/mod.rs` | ~300 | SnapshotTest, golden files |
| `snapshot/tests.rs` | ~200 | Snapshot tests |
| **Total new** | **~5,700** | |

### New TypeScript Files

| File | Lines (est) | Purpose |
|------|-------------|---------|
| `packages/workspace/src/index.ts` | ~200 | Workspace API |
| `packages/workspace/src/types.ts` | ~100 | Workspace types |
| `packages/commands/src/index.ts` | ~150 | Command registry TS |
| `packages/keybindings/src/index.ts` | ~200 | Keybinding parser |
| `packages/terminal/src/index.ts` | ~150 | Terminal widget TS wrapper |
| `packages/viewport/src/index.ts` | ~150 | Virtual viewport TS |
| `packages/fs/src/index.ts` | ~200 | Filesystem service TS |
| `packages/palette/src/index.ts` | ~150 | Command palette TS |
| `packages/plugin/src/index.ts` | ~200 | Plugin host TS |
| `packages/testing/src/index.ts` | ~100 | Snapshot testing utils |
| **Total new** | **~1,600** | |

### Modified Files

| File | Changes |
|------|---------|
| `native/engine/src/lib.rs` | Add 9 new pub mod declarations |
| `native/engine/src/widgets/mod.rs` | Add TerminalWidget, VirtualViewport |
| `native/engine/src/protocol/command.rs` | Add IDE commands |
| `native/engine/src/events/types.rs` | Add IDE event types |
| `native/engine/src/focus/manager.rs` | Add pane scopes |
| `native/bindings/src/lib.rs` | Add NapiPaneManager, etc. |
| `packages/reconciler/src/index.ts` | Add widget reconciliation |
| `packages/react/src/index.ts` | Add Terminal, Pane components |
| `packages/shared/src/index.ts` | Add IDE types |
| `pnpm-workspace.yaml` | Add new packages |

---

## Risk Analysis

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Pane system breaks layout engine | High | Medium | Test with existing widgets first |
| PTY throughput insufficient | Medium | Low | Batch reads, async I/O |
| Virtual viewport complexity | Medium | Medium | Start with fixed-height lists |
| Plugin isolation too weak | High | Low | Capability negotiation, sandboxing later |
| Keybinding chord timeout race | Low | Low | State machine with timeout |
| Snapshot tests fragile | Medium | Medium | Deterministic rendering, fixed seeds |
| Memory growth with 100K items | Medium | Low | Virtual viewport + lazy loading |
| Git status slow on large repos | Medium | Medium | Cache, background refresh |
| Command palette fuzzy perf | Low | Low | Pre-filter by category |
| Workspace state corruption | High | Low | Atomic writes, backup files |

---

## Exit Criteria

1. All 11 phases implemented
2. 1000+ Rust tests passing
3. No existing tests broken
4. No duplicate runtime introduced
5. No existing architecture replaced
6. Every new file has clear purpose
7. Every integration point documented
8. Performance targets met
9. Snapshot testing working in CI
10. Documentation updated

---

## Next Milestone

After IDE Platform Foundation:
- **Editor Engine** — text editing, syntax highlighting, LSP integration
- **Integrated Terminal** — full terminal emulator with splits
- **Git Integration** — branch, commit, diff, merge UI
- **Remote Development** — SSH/WSL workspace support
- **Collaborative Editing** — real-time multi-cursor
