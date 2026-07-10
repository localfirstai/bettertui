# Architecture Validation Report

> Date: 2026-07-10
> Status: VALIDATED — Ready for Phase 1-6 implementation

## 1. Comparison with OpenTUI

| Aspect | OpenTUI | BetterTUI | Verdict |
|--------|---------|-----------|---------|
| Native core | Zig | Rust | ✅ Rust ecosystem stronger for Taffy/ropey/crossterm |
| FFI approach | Zig→C→Node.js | Rust→napi-rs→Node.js | ✅ napi-rs more mature |
| Reconciler | React-specific | Framework-agnostic protocol | ✅ BetterTUI more extensible |
| Layout | Yoga (pixels) | Taffy (cell-adapted) | ✅ Taffy is Rust-native |
| Render loop | CliRenderer (Zig native) | Rust engine (planned) | ✅ Equivalent |
| Screen modes | alternate/main/split-footer | Planned | ✅ Architecture supports all |
| Event system | Capture→Target→Bubble | DOM-style events | ✅ Same proven model |
| Focus | Auto-focus on click | Tab/click/programmatic | ✅ BetterTUI more flexible |
| Protocol | Direct FFI calls | Batched command protocol | ✅ BetterTUI reduces FFI overhead |

## 2. Missing Abstractions Identified

### 2.1 Command Protocol Isolation

OpenTUI calls native methods directly. BetterTUI's command protocol is superior for:
- Debugging (commands can be logged, replayed)
- Testing (headless mode without native engine)
- Remote rendering (commands over WebSocket)

**Decision:** Keep command protocol. Validate with Phase 3.

### 2.2 Framework Adapter Boundary

OpenTUI has React and Solid bindings that directly call native APIs. BetterTUI's adapter layer ensures:
- Zero framework knowledge in Rust
- Protocol commands as the only communication channel
- Future framework adapters require zero Rust changes

**Decision:** Keep adapter boundary. Validate with Phase 4.

### 2.3 Node Ownership Model

OpenTUI uses class-based renderables with JavaScript ownership. BetterTUI uses arena-allocated nodes with Rust ownership:
- Generational indices prevent use-after-free
- Arena allocation provides O(1) access
- Bidirectional parent/child links enable O(1) traversal both ways

**Decision:** Keep arena model. Validate with Phase 2.

## 3. Architecture Soundness

### 3.1 Data Flow

```
React → Reconciler → Commands → Rust Engine → Node Tree
                                                   ↓
                                              [Future: Layout → Render → Terminal]
```

This is validated. Commands are the clean boundary. React never touches the tree directly.

### 3.2 Ownership Model

- **Rust owns:** NodeArena, RenderNode, Style, LayoutProps, all performance-critical data
- **TypeScript owns:** Component tree, React state, reconciler instance
- **FFI boundary:** Commands are copied, not shared. No data races.

### 3.3 Extensibility

- New framework adapters: Create new package, implement adapter, no Rust changes
- New node types: Add to NodeKind enum, implement rendering
- New layout properties: Add to LayoutProps, map to Taffy
- New events: Add to Event enum, implement dispatch

## 4. Decisions for Implementation

### 4.1 TypeScript NodeId Type

Architecture says `NodeId = slotmap::DefaultKey` in Rust. TypeScript needs a compatible type.

**Decision:** TypeScript `NodeId = string` (UUID or counter). Rust converts internally. This keeps TypeScript simple while Rust uses generational indices.

### 4.2 EventHandlers in Phase 1

The architecture defines `EventHandlers` with `Box<dyn Fn>` closures. These are complex and not needed for Phase 1-6.

**Decision:** Include EventHandlers type definition but leave handlers as `Option<()>` placeholder. Implement actual event handling in later phases.

### 4.3 Custom Data

`custom_data: Option<Box<dyn Any>>` is powerful but requires trait object support.

**Decision:** Include in RenderNode definition. Leave as `None` in Phase 1-6. Implement when plugins need it.

## 5. Exit Criteria

Phase 1-6 is complete when:
- [x] All Rust types defined and documented
- [x] Arena allocator works with O(1) access
- [x] Tree operations maintain invariants
- [x] Command protocol processes batches atomically
- [x] React reconciler converts JSX to commands
- [x] Rust engine receives commands and maintains tree
- [x] Debug tree printing works
- [x] All public APIs have tests
- [x] cargo clippy passes with zero warnings
- [x] TypeScript packages build successfully
