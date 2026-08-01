# BetterTUI Architecture

> Code-accurate architecture reference. Every statement is derived from the current implementation.

BetterTUI is a framework-agnostic terminal UI rendering engine. Rust owns performance-critical work (rendering, layout, input, animation, text editing, terminal emulation). TypeScript owns developer experience (typed APIs, framework bindings, theming). A napi-rs FFI boundary is the only coupling point and carries **commands** — never framework concepts.

```
Vanilla / Native TS App → @bettertui/core ──(napi-rs FFI)──▶ Rust Engine (bettertui-engine cdylib)
                                                              ↓
                                                    Terminal / PTY via crossterm + portable-pty
React App → @bettertui/react → @bettertui/core (auto-resolved)
```

`@bettertui/shared` is internal (re-exported by core) — do not install it directly.

## Document index

| Document                                    | What it describes                                                                           |
| ------------------------------------------- | ------------------------------------------------------------------------------------------- |
| [Overview](overview.md)                     | Repository layout, workspaces, build system, dependency direction                           |
| [Node Model](node-model.md)                 | Arena-allocated node tree (`tree` module)                                                   |
| [Protocol](protocol.md)                     | Command protocol between TypeScript and Rust (`protocol` module, napi surface in `napi.rs`) |
| [Rendering Pipeline](rendering-pipeline.md) | Render stages from arena → ANSI (`render/`, `framebuffer`, `dirty_diff`)                    |
| [Layout](layout.md)                         | Taffy-based flexbox layout adapted to terminal cells (`taffy.rs`)                           |
| [Frame Buffer](frame-buffer.md)             | Cell grid and dirty-region diffing (`framebuffer`, `dirty_diff`)                            |
| [Event System](event-system.md)             | Event dispatch and bubbling (`input` module)                                                |
| [Input System](input-system.md)             | Keyboard, mouse, paste, Kitty/CSI-u parsing (`input`, `ansi`)                               |
| [Animation](animation.md)                   | Tween/spring/keyframe engine (`animation` module)                                           |
| [Widget Model](widget-model.md)             | Widget host on the arena (`createWidgetHost`, engine's `widget` trait)                      |
| [Terminal](terminal.md)                     | Raw mode, VT emulation, screen buffers (engine `terminal/` modules)                         |
| [PTY](pty.md)                               | Embedded process spawning (`pty.rs`, `terminal/process.rs`)                                 |
| [Compositor](compositor.md)                 | Compositing primitives (`tree.rs`/`graphics.rs`), screen state (`terminal/screen.rs`)       |
| [Capabilities](capabilities.md)             | Terminal feature detection (`terminal/capabilities.rs`)                                     |
| [Text Editing](text-editing.md)             | Rope-based editor (`text` module, exposed as `NapiTextEngine`)                              |
| [Scheduler](scheduler.md)                   | Frame timing and priority scheduling (`scheduler` module)                                   |
| [DevTools](devtools.md)                     | In-core debug overlay and inspector modules (`src/devtools/`)                               |

### Engineering notes

| Document                                                      | What it describes                                    |
| ------------------------------------------------------------- | ---------------------------------------------------- |
| [Dirty Diff Audit](dirty-diff-audit.md)                       | Engineering audit of the dirty-region diffing system |
| [Viewport Culling Comparison](viewport-culling-comparison.md) | Reference vs BetterTUI viewport-culling comparison   |
| [Visibility Propagation](visibility-propagation.md)           | Visibility propagation design                        |

## Principles

- **BetterTUI is a terminal UI framework.** Never an application, IDE, AI framework, or editor.
- **Two first-class packages.** `@bettertui/core` for vanilla/native TypeScript; `@bettertui/react` for React.
- **Rust owns rendering.** TypeScript never renders; it emits commands.
- **Framework adapters are optional.** React is implemented. Vue, Solid, Svelte must be addable without touching Rust.
- **No business logic in the engine.** It is a rendering/layout/input framework only.

## Rust engine subsystems

All subsystems are modules **inside the `bettertui-engine` crate** (`packages/core/crates/engine/`). There is no separate `bettertui-widgets`, `bettertui-terminal`, or `bettertui-bindings` crate. The napi surface is a module (`napi.rs`) compiled with the `napi` feature.

| Subsystem                   | Module location                                                                          | Implemented?               |
| --------------------------- | ---------------------------------------------------------------------------------------- | -------------------------- |
| Arena / node tree           | `tree.rs`                                                                                | ✅                         |
| Command protocol            | `protocol.rs`                                                                            | ✅                         |
| Renderer / frame production | `render/` (`mod.rs`, `render.rs`, `effects.rs`), `framebuffer.rs`, `dirty_diff.rs`       | ✅                         |
| Frame buffer                | `framebuffer.rs`                                                                         | ✅                         |
| Dirty diff                  | `dirty_diff.rs`                                                                          | ✅                         |
| Layout (Taffy)              | `taffy.rs`                                                                               | ✅                         |
| Events                      | `input.rs`                                                                               | ✅                         |
| Input parsing               | `input.rs`, `ansi.rs`                                                                    | ✅                         |
| Animation                   | `animation.rs`                                                                           | ✅ (callback path partial) |
| Scheduler                   | `scheduler.rs`                                                                           | ✅                         |
| Capabilities                | `terminal/capabilities.rs`                                                               | ✅                         |
| Terminal I/O + VT           | `terminal/` (`mod.rs`, `vt.rs`, `screen.rs`, `scrollback.rs`, `query.rs`, `neovim.rs`)   | ✅                         |
| PTY / process               | `pty.rs`, `terminal/process.rs`                                                          | ✅                         |
| Compositing / screen        | `tree.rs`/`graphics.rs`, `terminal/screen.rs`                                            | ✅                         |
| Text editing (rope)         | `text/` (buffer, cursor, edit, search, selection, undo, unicode, wrap, viewport, styled) | ✅                         |
| Widget host                 | Native bridge `createWidgetHost`                                                         | ✅                         |
| napi FFI surface            | `napi.rs` (compiled with `napi` feature), `ffi.rs` (C-ABI)                               | ✅                         |
| Syntax highlighting         | `syntax.rs` (tree-sitter)                                                                | ✅                         |
| Font / glyph support        | `font/` (loader, metrics, provider, registry, ascii)                                     | ✅                         |

## TypeScript packages

All packages are currently `private` (not published to npm).

| Package                  | Role                                                                                                                                                                           | Status         |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------- |
| `@bettertui/core`        | Framework-agnostic — command protocol, reconciler, `CommandRuntime`, native bridge, DevTools                                                                                   | ✅ Implemented |
| `@bettertui/react`       | React 19 adapter — `createRoot()`, reconciler, hooks (`useRuntime`, `useFocus`, `useKeyboard`, `useTheme`, `useTimeline`, `useTerminalDimensions`), runtime context, JSX types | ✅ Implemented |
| `@bettertui/solid`       | SolidJS adapter — placeholder directory with stub structure                                                                                                                    | ⏳ Placeholder |
| `@bettertui/shared`      | Type-only foundation (zero runtime, internal, re-exported by core)                                                                                                             | ✅ Implemented |
| `@bettertui/performance` | Vitest benchmark suite                                                                                                                                                         | ✅ Implemented |
| `@bettertui/examples`    | 64 TypeScript examples running on `@bettertui/core`                                                                                                                            | ✅ Implemented |
