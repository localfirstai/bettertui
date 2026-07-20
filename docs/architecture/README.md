# BetterTUI Architecture

> The canonical, code-accurate architecture reference for BetterTUI.
> Every statement here is derived from the current implementation, not from design intent.

BetterTUI is a **framework-agnostic terminal UI rendering engine**. Rust owns all performance-critical work (rendering, layout, input, animation, text editing, terminal emulation). TypeScript owns the developer experience (typed APIs, framework bindings, theming). A napi-rs FFI boundary is the only coupling point between the two, and it carries **commands** — never framework concepts.

```
Vanilla / Native TS App ─▶ @bettertui/core ──(napi-rs FFI)──▶ Rust Engine (bettertui-engine cdylib)
                                                       │
                                                       ▼
                                              bettertui-engine ──▶ (Terminal / PTY)
                                                       via crossterm + portable-pty
React App ─▶ @bettertui/react (planned adapter, not yet implemented)
```

## Two first-class packages

- **`@bettertui/core`** is a fully public, framework-agnostic package for vanilla / native TypeScript. It is the recommended entry point when you don't use React. **Implemented.**
- **`@bettertui/react`** is the planned React adapter. React apps will install **only** `@bettertui/react` — it depends on `@bettertui/core` and resolves it automatically. **Not implemented yet** — `packages/react` is a placeholder.

`@bettertui/shared` is **internal** (re-exported by core) and must not be installed directly.

## Document Index

| Document | What it describes |
|----------|-------------------|
| [Overview](overview.md) | Repository layout, workspaces, build system, dependency direction |
| [Node Model](node-model.md) | Arena-allocated node tree (`tree` module) |
| [Protocol](protocol.md) | Command protocol between TypeScript and Rust (`protocol` module) |
| [Rendering Pipeline](rendering-pipeline.md) | Render stages from arena → ANSI (`renderer`, `painter`, `dirty_diff`, `framebuffer`) |
| [Layout](layout.md) | Taffy-based flexbox layout adapted to terminal cells (`layout` module) |
| [Frame Buffer](frame-buffer.md) | Cell grid and dirty-region diffing (`framebuffer`, `dirty_diff`) |
| [Event System](event-system.md) | Event dispatch and bubbling (`input` module) |
| [Input System](input-system.md) | Keyboard, mouse, paste, Kitty/C SI-u parsing (`input`, `ansi`) |
| [Animation](animation.md) | Tween/spring/keyframe engine (`animation` module) |
| [Widget Model](widget-model.md) | Widget host on the arena (engine `createWidgetHost` / native bridge) |
| [Terminal](terminal.md) | Raw mode, VT emulation, screen buffers (engine `terminal/` modules) |
| [PTY](pty.md) | Embedded process spawning (`pty`, `terminal/process.rs`) |
| [Compositor](compositor.md) | Layered compositing and screen state (`tree`/`graphics`, engine `terminal/`) |
| [Capabilities](capabilities.md) | Terminal feature detection (engine `terminal/capabilities.rs`) |
| [Text Editing](text-editing.md) | Rope-based editor (`text` module, exposed as `NapiTextEngine`) |
| [Scheduler](scheduler.md) | Frame timing and priority scheduling (`scheduler` module) |

## Engineering notes

| Document | What it describes |
|----------|-------------------|
| [Dirty Diff Audit](dirty-diff-audit.md) | Engineering audit of the dirty-region diffing system |
| [Viewport Culling Comparison](viewport-culling-comparison.md) | OpenTUI vs BetterTUI viewport-culling comparison |
| [Visibility Propagation](visibility-propagation.md) | Visibility propagation design |

## Principles (non-negotiable)

- **BetterTUI is a framework.** Never an application, IDE, AI framework, or editor.
- **Two first-class packages.** `@bettertui/core` is the public entry point for vanilla / native TypeScript; `@bettertui/react` is the public entry point for React and depends on core (React users install only `@bettertui/react`).
- **Rust owns rendering.** TypeScript never renders; it emits commands.
- **Framework adapters are optional.** React is the first adapter. Vue, Solid, Svelte, vanilla TS must be addable without touching Rust.
- **No business logic in the engine.** It is a rendering/layout/input framework only.

## Status Reality Check

The Rust engine (`bettertui-engine`) is the most complete part: rendering, layout (Taffy), frame
buffer, events, input, animation, text engine, PTY, capability detection, VT emulation, and Nerd
Font support are implemented. Terminal I/O, VT emulation, PTY, capabilities, and the widget host
are **modules inside `bettertui-engine`** — there is no separate `bettertui-widgets`,
`bettertui-terminal`, or `bettertui-bindings` crate. The crate is built as a `cdylib` with the
`napi` feature to produce the Node.js addon (`bettertui_engine.node`).

The TypeScript side is implemented: `@bettertui/core` (command buffer, reconciler wrapper,
`CommandRuntime`, native bridge, testing utilities) and `@bettertui/examples` (vanilla / native
TypeScript demos runnable on `@bettertui/core`). The React adapter (`@bettertui/react`) is **not
implemented** — `packages/react` is a placeholder directory only. Vanilla examples under
`packages/examples/typescript/` demonstrate the `@bettertui/core` API. See
[ROADMAP.md](../ROADMAP.md) for the current code-accurate status.

## Dependency flow

```mermaid
graph TD
    VR[Vanilla / Native TS App] --> Core[@bettertui/core]
    AR[React App] --> React[@bettertui/react]
    React --> Core
    Core -->|napi-rs FFI| Engine[bettertui-engine cdylib]
    Engine --> Term[(crossterm / portable-pty)]
    Core --> Shared[Internal: @bettertui/shared]
    React --> Shared
```

- `bettertui-engine` never imports any JS framework.
- `@bettertui/core` never imports React.
- Only `@bettertui/react` (when built) imports React.
- The only boundary between a UI framework and the engine is the **command** — so Vue/Solid/Svelte/vanilla-TS adapters require no Rust changes.

## Rust engine subsystems

All subsystems below are modules **inside the `bettertui-engine` crate** (`packages/core/crates/engine`). There is no separate `bettertui-widgets`, `bettertui-terminal`, or `bettertui-bindings` crate — the engine crate is itself the `cdylib` (with the `napi` feature) that produces `bettertui_engine.node`.

| Subsystem | Module | Implemented? |
|-----------|--------|--------------|
| Arena / node tree | `tree` | ✅ |
| Command protocol | `protocol` | ✅ |
| Renderer / frame production | `render` (`Renderer`, `AnsiBackend`, `Painter`), `framebuffer`, `dirty_diff` | ✅ |
| Frame buffer | `framebuffer` | ✅ |
| Dirty diff | `dirty_diff` | ✅ |
| Layout (Taffy) | `taffy` | ✅ |
| Events | `input` | ✅ |
| Input parsing | `input`, `ansi` | ✅ |
| Animation | `animation` | ✅ (callback path partial) |
| Scheduler | `scheduler` | ✅ |
| Capabilities | `terminal/capabilities.rs` (engine) | ✅ |
| Terminal I/O + VT | `terminal/` (engine: `mod.rs`, `vt.rs`, `screen.rs`, `scrollback.rs`, `query.rs`, `neovim.rs`) | ✅ |
| PTY / process | `pty.rs` (engine), `terminal/process.rs` (engine) | ✅ |
| Compositor / screen | `tree`/`graphics` (engine), `terminal/screen.rs` (engine) | ✅ |
| Text editing (rope) | `text` | ✅ |
| Widget host | `createWidgetHost` (native bridge) | ✅ |
| napi FFI surface | `napi` module (compiled with the `napi` feature) + `ffi` (C-ABI) | ✅ |

> Note: raw terminal-byte parsing lives in `input` and `ansi`; the C-ABI surface is in `ffi`; the napi surface is in `napi`. There are no top-level `keyboard/`, `mouse/`, `editor/`, `clipboard/`, or `selection/` modules — those concerns live inside `input.rs` and `text/`.

## TypeScript packages

All TypeScript packages are currently `private` (not published to npm).

| Package | Role | Status |
|---------|------|--------|
| `@bettertui/core` | Framework package for vanilla / native TypeScript — command protocol, tree ops, `CommandRuntime`, native Rust bridge, in-core debug tooling (`createDevTools`, debug overlay) | ✅ Implemented |
| `@bettertui/react` | React adapter — install **only** this for React apps (auto-depends on `@bettertui/core`) | ⏳ Not implemented (placeholder) |
| `@bettertui/shared` | Type foundation (zero runtime code) — **internal, do not install directly** — re-exported via `@bettertui/core` | ✅ Implemented |
| `@bettertui/benchmark` | Vitest benchmarks | ✅ Implemented |
| `@bettertui/examples` | Interactive CLI example browser runnable on `@bettertui/core` (no React) | ✅ Implemented |
