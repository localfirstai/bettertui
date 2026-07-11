# Integration TODO

> **Status:** Active
> **Objective:** Bridge the Rust engine and TypeScript ecosystem into a production-ready framework.
> **Architecture is mature.** No new subsystems. Only integration.

---

## Dependency Graph

```
┌─────────────────────────────────────────────────────────────────────┐
│                    React Application                                │
│  (User code: components, hooks, state)                              │
└────────────────────────────┬────────────────────────────────────────┘
                             │ React.render()
                             ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    @bettertui/react                                  │
│  Box, Text, Flex, Spacer, Provider, etc.                            │
│  Real JSX components that emit element descriptors                   │
└────────────────────────────┬────────────────────────────────────────┘
                             │ uses react-reconciler
                             ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    @bettertui/reconciler                             │
│  react-reconciler host config                                      │
│  createInstance, appendChild, commitUpdate, resetAfterCommit        │
│  Translates React operations → CommandBuffer                        │
└────────────────────────────┬────────────────────────────────────────┘
                             │ CommandBuffer.drain()
                             ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    @bettertui/core                                   │
│  CommandBuffer → JSON serialization                                 │
│  Engine wrapper class (napi-rs binding)                              │
│  NodeId conversion (string ↔ u32)                                   │
└────────────────────────────┬────────────────────────────────────────┘
                             │ napi-rs FFI
                             ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    native/bindings (napi-rs)                         │
│  #[napi] Engine class with processCommands()                        │
│  #[napi] Renderer class with render()                               │
│  #[napi] EventBus, FocusManager, Scheduler                          │
│  #[napi] TextEngine, PtyRuntime                                     │
│  #[napi] CapabilityDetector                                         │
│  #[napi] Compositor                                                 │
└────────────────────────────┬────────────────────────────────────────┘
                             │ Rust FFI
                             ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    bettertui-engine (Rust)                           │
│  ┌───────────────────────────────────────────────────────────┐     │
│  │ CommandProcessor → NodeArena → Tree operations             │     │
│  └───────────────────────────────────────────────────────────┘     │
│  ┌───────────────────────────────────────────────────────────┐     │
│  │ LayoutEngine (Taffy) → Layout results                     │     │
│  └───────────────────────────────────────────────────────────┘     │
│  ┌───────────────────────────────────────────────────────────┐     │
│  │ Renderer → Painter → FrameBuffer → DirtyDiff → AnsiBackend│     │
│  └───────────────────────────────────────────────────────────┘     │
│  ┌───────────────────────────────────────────────────────────┐     │
│  │ EventBus → EventDispatcher → Widget event handling         │     │
│  └───────────────────────────────────────────────────────────┘     │
│  ┌───────────────────────────────────────────────────────────┐     │
│  │ Scheduler (priority queue, frame budget, animation)        │     │
│  └───────────────────────────────────────────────────────────┘     │
│  ┌───────────────────────────────────────────────────────────┐     │
│  │ FocusManager, CapabilityDetector, TextEngine, Compositor   │     │
│  │ PtyRuntime, NeovimProcess, GlyphCache, NerdFont            │     │
│  └───────────────────────────────────────────────────────────┘     │
└────────────────────────────┬────────────────────────────────────────┘
                             │ crossterm
                             ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    Terminal Emulator                                  │
│  stdout → ANSI escape sequences → display                           │
└─────────────────────────────────────────────────────────────────────┘
```

### Data Flow (Detailed)

```
React Component Tree
    ↓ [react-reconciler]
Host Config Operations (createInstance, appendChild, etc.)
    ↓ [CommandBuffer]
TypeScript Command[] (JSON)
    ↓ [napi-rs FFI]
Rust Command[] (native)
    ↓ [CommandProcessor]
NodeArena mutated
    ↓ [LayoutTreeSync → Taffy]
Layout results (position + size per node)
    ↓ [build_render_tree]
Render tree (visible nodes with resolved styles)
    ↓ [Painter]
FrameBuffer (cell grid)
    ↓ [DirtyDiff]
Dirty regions (changed cells)
    ↓ [AnsiBackend]
ANSI escape sequences (bytes)
    ↓ [stdout]
Terminal display
```

---

## Integration Plan

### Phase 1: napi-rs Bindings (Foundation)

**Goal:** Expose Rust engine to Node.js.

**Files to modify:**
- `native/bindings/src/lib.rs` — add #[napi] wrappers
- `native/bindings/Cargo.toml` — ensure all deps

**What to expose (in order of priority):**

| # | Rust Type | napi Class/Function | Key Methods |
|---|-----------|---------------------|-------------|
| 1 | `Engine` | `NapiEngine` class | `new()`, `processCommands(json)`, `beginFrame()`, `commitFrame()`, `nodeCount()`, `printTree()`, `treeSummary()`, `validate()`, `root()` |
| 2 | `Command` | JSON serialization | Convert TS Command → Rust Command via JSON |
| 3 | `NodeId` | `string` (u32 → string) | Bidirectional conversion |
| 4 | `Renderer` | `NapiRenderer` class | `new(w,h)`, `resize(w,h)`, `render(arena)`, `renderFull(arena)`, `requestFrame()`, `shouldRender()`, `dimensions()` |
| 5 | `Scheduler` | (via Renderer) | `withFps()`, `requestFrame()`, `status()` |
| 6 | `EventBus` | `NapiEventBus` class | `new()`, `pushKey()`, `pushMouse()`, `pushPaste()`, `pushResize()`, `drain()`, `len()`, `isEmpty()`, `clear()` |
| 7 | `FocusManager` | `NapiFocusManager` class | `new()`, `focus()`, `blur()`, `focused()`, `traverse()`, `setScope()` |
| 8 | `CapabilityDetector` | `NapiCapabilities` class | `detect()`, `brand()`, `supportsTrueColor()`, `terminalSize()`, `pixelSize()` |
| 9 | `TextEngine` | `NapiTextEngine` class | `new()`, `withText()`, `insertChar()`, `insertStr()`, `deleteChar()`, `undo()`, `redo()`, `text()`, `search()`, `lineCount()`, `charCount()` |
| 10 | `Compositor` | `NapiCompositor` class | `new(w,h)`, `resize(w,h)`, `addLayer()`, `removeLayer()`, `layerCount()`, `compositeToBuffer()` |
| 11 | `PtyRuntime` | `NapiPty` class | `new()`, `spawn(config)`, `read()`, `write()`, `resize()`, `kill()`, `isRunning()`, `exitStatus()` |
| 12 | `WidgetHost` | `NapiWidgetHost` class | `new()`, `register()`, `mount()`, `unmount()`, `handleEvent()`, `update()`, `widgetCount()` |
| 13 | `Global functions` | `detectCapabilities()`, `getVersion()` | Standalone functions |

**Command Serialization Strategy:**
- TypeScript sends commands as JSON arrays
- Rust deserializes via `serde_json`
- Commands are applied atomically via `CommandProcessor::process_batch()`
- Results returned as JSON (success count, errors, warnings)

**NodeId Strategy:**
- Rust: `slotmap::DefaultKey` (8 bytes = u32 index + u32 generation)
- TypeScript: `string` = `"index:generation"` format
- napi layer handles conversion in both directions

### Phase 2: TypeScript Runtime

**Goal:** Wire reconciler → command buffer → napi → Rust.

**Files created/modified:**
- `packages/reconciler/src/index.ts` — modified to accept CommandBuffer, emit commands on every operation
- `packages/native/src/index.ts` — created: exports createEngine, createEventBus, etc.
- `packages/native/src/types.ts` — created: TypeScript types mirroring napi bindings
- `packages/native/src/runtime.ts` — created: Runtime class orchestrating React → commands → native engine
- `packages/react/src/index.ts` — updated: real React components (Box, Text, Flex, Spacer, Provider)

**Key changes:**
1. `createReconciler(buffer: CommandBuffer)` now accepts a shared command buffer
2. Every reconciler operation emits a Command (CreateNode, AppendChild, RemoveNode, etc.)
3. Native wrapper (`@bettertui/native`) loads napi bindings and provides typed APIs
4. `createRuntime()` orchestrates: flush CommandBuffer → engine.processCommands() → engine.render()
5. NodeId = numeric string (matches Rust engine's temp IDs)

### Phase 3: React Components

**Goal:** Replace placeholder components.

**Files to modify:**
- `packages/react/src/index.ts` — real implementations

**Component implementations:**
- `Box` → renders `<box>` element with flex layout props
- `Text` → renders `<text>` element with style props
- `Flex` → renders `<box>` with flexDirection
- `Spacer` → renders `<spacer>` element
- `Provider` → provides theme context

All components emit element descriptors that the reconciler translates to commands.

### Phase 4: Renderer Lifecycle

**Goal:** Connect Terminal → Input → EventBus → Widget Tree → Render → Scheduler → Output.

**Files to create/modify:**
- `packages/core/src/loop.ts` — main render loop
- `packages/core/src/terminal.ts` — terminal raw mode, crossterm integration

**Render loop:**
1. Enter raw mode via crossterm
2. Read input events → push to EventBus
3. Process events → dispatch to widgets
4. Check if frame is due (Scheduler)
5. If due: render(arena) → get RenderFrame → write to stdout
6. Repeat until shutdown

### Phase 5: Input Integration

**Goal:** Connect input events to widget system.

**Files to modify:**
- `packages/core/src/input.ts` — input handling
- `packages/react/src/index.ts` — event handler props

**Input events to handle:**
- Keyboard → EventBus::push_key
- Mouse → EventBus::push_mouse
- Paste → EventBus::push_paste
- Resize → EventBus::push_resize + Renderer::resize
- Focus → FocusManager::focus/blur

### Phase 6: Widget State

**Goal:** Persist widget state between renders.

**Files to modify:**
- `packages/reconciler/src/index.ts` — state management
- `packages/react/src/index.ts` — useState/useReducer hooks

**State strategy:**
- React hooks manage state on TS side
- State changes trigger re-reconciliation
- Reconciler emits update commands to Rust
- Rust updates NodeArena

### Phase 7: Dirty Rendering

**Goal:** Complete dirty tracking.

**Files to modify:**
- `native/engine/src/dirty_diff/mod.rs` — improve collect_dirty_nodes

**Strategy:**
- Track which nodes changed in CommandProcessor
- Propagate dirty flags up ancestor chain
- Only layout/render dirty subtrees
- Skip unchanged regions in diff

### Phase 8: Example Applications

**Goal:** Upgrade examples to real applications.

**Files to modify:**
- `examples/counter/src/index.ts`
- `examples/dashboard/src/index.ts`
- `examples/mouse/src/index.ts`
- `examples/tree/src/index.ts`
- `examples/table/src/index.ts`
- `examples/text-editor/src/index.ts`

### Phase 9: Developer Tools

**Goal:** Implement inspector, FPS counter, paint flashing.

**Files to modify:**
- `packages/devtools/src/index.ts`
- `native/engine/src/engine/inspector.rs`

### Phase 10: Testing

**Goal:** Expand test coverage.

**Files to create/modify:**
- `native/bindings/src/tests/` — Rust binding tests
- `packages/reconciler/src/__tests__/` — reconciler tests
- `packages/react/src/__tests__/` — component tests
- Integration tests (Rust ↔ TypeScript)

---

## Phase Details

### Phase 1: napi-rs Bindings

**Estimated files modified:** 3
**Estimated files created:** 0

**Critical constraints:**
- All napi types must be `Send + Sync` (napi requirement)
- `#[napi]` structs cannot contain `&mut` references
- Use `napi::threadsafe_function` for callbacks (events Rust → TS)
- `NodeId` (slotmap::DefaultKey) is not napi-compatible — convert to u64 for FFI
- `Command` enum is not napi-compatible — serialize to JSON

**NodeId conversion:**
```rust
// Rust → JS: encode as u64 via transmute (both are 8-byte #[repr(transparent)])
fn node_id_to_u64(id: NodeId) -> u64 {
    unsafe { std::mem::transmute(id) }
}

// JS → Rust: decode from u64 via transmute
fn u64_to_node_id(val: u64) -> NodeId {
    unsafe { std::mem::transmute(val) }
}
```

**Command serialization:**
```rust
// In napi binding:
#[napi]
fn process_commands(&mut self, commands_json: String) -> String {
    let commands: Vec<CommandSerde> = serde_json::from_str(&commands_json).unwrap();
    let rust_commands: Vec<Command> = commands.into_iter().map(|c| c.into()).collect();
    let result = self.engine.process_commands(rust_commands);
    serde_json::to_string(&result).unwrap()
}
```

### Phase 2: TypeScript Runtime

**Estimated files modified:** 4
**Estimated files created:** 2

**New files:**
- `packages/core/src/engine.ts` — Engine wrapper class
- `packages/core/src/command-serializer.ts` — Command JSON serialization

**Engine wrapper:**
```typescript
import { NapiEngine } from 'bettertui-bindings';

export class Engine {
  private native: NapiEngine;
  
  constructor() {
    this.native = new NapiEngine();
  }
  
  processCommands(commands: Command[]): CommandResult {
    const json = JSON.stringify(commands);
    const resultJson = this.native.processCommands(json);
    return JSON.parse(resultJson);
  }
  
  beginFrame(): void { this.native.beginFrame(); }
  commitFrame(): void { this.native.commitFrame(); }
  nodeCount(): number { return this.native.nodeCount(); }
  printTree(): string { return this.native.printTree(); }
}
```

### Phase 3: React Components

**Estimated files modified:** 1

**Component pattern:**
```tsx
export function Box(props: BoxProps): JSX.Element {
  return createElement('box', {
    flexDirection: props.flexDirection ?? 'column',
    justifyContent: props.justifyContent ?? 'flex-start',
    alignItems: props.alignItems ?? 'stretch',
    padding: props.padding,
    margin: props.margin,
    width: props.width,
    height: props.height,
    style: props.style,
    children: props.children,
  });
}
```

The reconciler's host config translates these element descriptors into commands.

---

## File Impact Summary

| File | Action | Phase | Description |
|------|--------|-------|-------------|
| `native/bindings/src/lib.rs` | **MODIFY** | 1 | Expand from 6 lines to full napi bindings |
| `native/bindings/Cargo.toml` | **MODIFY** | 1 | Add serde, serde_json deps |
| `packages/core/src/engine.ts` | **CREATE** | 2 | Engine wrapper class |
| `packages/core/src/command-serializer.ts` | **CREATE** | 2 | Command JSON serialization |
| `packages/core/src/index.ts` | **MODIFY** | 2 | Export Engine, update types |
| `packages/reconciler/src/index.ts` | **MODIFY** | 2 | Add react-reconciler, emit commands |
| `packages/shared/src/index.ts` | **MODIFY** | 2 | Update NodeId, add command types |
| `packages/react/src/index.ts` | **MODIFY** | 3 | Real component implementations |
| `packages/core/src/loop.ts` | **CREATE** | 4 | Main render loop |
| `packages/core/src/terminal.ts` | **CREATE** | 4 | Terminal raw mode |
| `packages/core/src/input.ts` | **CREATE** | 5 | Input event handling |
| `packages/devtools/src/index.ts` | **MODIFY** | 9 | Inspector, FPS counter |
| `examples/*/src/index.ts` | **MODIFY** | 8 | Real example applications |

---

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| napi-rs `Send + Sync` requirement | Engine must be thread-safe | Use `parking_lot::Mutex<Engine>` |
| NodeId conversion overhead | Performance | Use u64 encoding, batch conversions |
| JSON serialization overhead | Performance | Use `simd-json` or binary protocol |
| react-reconciler complexity | Correctness | Follow Ink's pattern as reference |
| Terminal raw mode from TS | Platform compat | Use crossterm via napi, not Node.js |
| Event callbacks Rust → TS | Latency | Use threadsafe_function for async |

---

## Success Criteria

1. **napi bindings expose 15+ Rust types** to Node.js
2. **React components render to terminal** via reconciler → commands → napi → Rust
3. **Input events flow** from terminal → EventBus → widgets → re-render
4. **60fps rendering** with dirty diff and frame scheduling
5. **All 777 existing Rust tests pass** (no regressions)
6. **All 6 examples work** as real applications
7. **Developer tools** show live inspector, FPS, paint flashing

---

## Next Milestone

After integration is complete, the next milestone is:
- **Plugin system** for extending the engine
- **Remote rendering** via WebSocket
- **GPU acceleration** for high-density displays
- **Multi-window** terminal support
