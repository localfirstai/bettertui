# Command Protocol

The command protocol is the **only** interface between TypeScript and the Rust engine. Every framework adapter translates its tree operations into `Command`s.

## Why commands?

Batching reduces per-call FFI overhead, lets commands be logged/replayed for debugging, and lets the engine process a frame atomically.

## The Command enum (`protocol.rs`)

The Rust `Command` enum has ~41 variants grouped by category. The napi bindings accept the same set via a `CommandJson` serde enum:

| Category | Variants |
|----------|----------|
| Tree (7) | `CreateNode`, `RemoveNode`, `AppendChild`, `InsertBefore`, `MoveNode`, `ReplaceNode`, `DetachNode` |
| Style (8) | `SetStyle`, `SetForeground`, `SetBackground`, `SetBold`, `SetItalic`, `SetUnderline`, `SetStrikethrough`, `SetDim`, `SetInverse`, `SetHidden` |
| Layout (16) | `SetLayout`, `SetFlexDirection`, `SetJustifyContent`, `SetAlignItems`, `SetAlignSelf`, `SetWidth`, `SetHeight`, `SetMinWidth`, `SetMinHeight`, `SetMaxWidth`, `SetMaxHeight`, `SetFlexBasis`, `SetPadding`, `SetMargin`, `SetGap`, `SetFlexGrow`, `SetFlexShrink`, `SetPosition`, `SetInset` |
| Content (3) | `SetText`, `SetAttribute`, `RemoveAttribute` |
| Visibility (3) | `SetDisplay`, `SetOpacity`, `SetClip` |
| Transform (3) | `SetTranslateX`, `SetTranslateY`, `SetZIndex` |
| Overflow (1) | `SetOverflow` |
| Focus (3) | `FocusNode`, `BlurNode`, `SetTabIndex` |
| Frame (3) | `BeginFrame`, `CommitFrame`, `Invalidate` |
| Lifecycle (1) | `Shutdown` |

## Processing pipeline

`Command batch → CommandProcessor::process → validate NodeIds → apply to NodeArena → set dirty flags → generation++ → layout recalculated`.

## TypeScript side

`@bettertui/core` builds commands in a `CommandBuffer` and drains at frame boundaries. `CommandRuntime` owns the buffer and flushes to subscribers. `createRuntime()` (via the native bridge) ties a native engine + event bus + core `CommandBuffer` together.

The current implementation uses a **JSON command envelope** decoded in the engine's napi module (`napi.rs`), not a hand-rolled binary codec. There is no separate `@bettertui/protocol` package.
