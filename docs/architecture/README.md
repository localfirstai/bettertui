# BetterTUI Architecture

> The canonical, code-accurate architecture reference for BetterTUI.
> Every statement here is derived from the current implementation, not from design intent.

BetterTUI is a **framework-agnostic terminal UI rendering engine**. Rust owns all performance-critical work (rendering, layout, input, animation, text editing, terminal emulation). TypeScript owns the developer experience (typed APIs, framework bindings, theming). A napi-rs FFI boundary is the only coupling point between the two, and it carries **commands** — never framework concepts.

```mermaid
graph TD
    A[Application] --> B[@bettertui/react]
    B --> C[@bettertui/core]
    C -->|napi-rs FFI| E[Rust Engine bettertui-bindings]
    E --> F[bettertui-engine]
    F -->|crossterm + portable-pty| G[(Terminal / PTY)]
    C --> J[Internal: @bettertui/shared]
    B --> J
```

## Document Index

| Document | What it describes |
|----------|-------------------|
| [Overview](overview.md) | Repository layout, workspaces, build system, dependency direction |
| [Node Model](node-model.md) | Arena-allocated node tree (`tree` module) |
| [Protocol](protocol.md) | Command protocol between TypeScript and Rust (`protocol` module) |
| [Rendering Pipeline](rendering-pipeline.md) | Render stages from arena → ANSI (`renderer`, `painter`, `dirty_diff`, `framebuffer`) |
| [Layout](layout.md) | Taffy-based flexbox layout adapted to terminal cells (`layout` module) |
| [Frame Buffer](frame-buffer.md) | Cell grid and dirty-region diffing (`framebuffer`, `dirty_diff`) |
| [Event System](event-system.md) | Event dispatch and bubbling (`events` module) |
| [Input System](input-system.md) | Keyboard, mouse, paste, Kitty/C SI-u parsing (`input`, `ansi`) |
| [Animation](animation.md) | Tween/spring/keyframe engine (`animation` module) |
| [Widget Model](widget-model.md) | Trait-based widget framework on the arena (`widgets` module) |
| [Terminal](terminal.md) | Raw mode, VT emulation, screen buffers (`terminal`, `terminal/vt`) |
| [PTY](pty.md) | Embedded process spawning (`pty`, `terminal_process`) |
| [Compositor](compositor.md) | Layered compositing and screen state (`compositor`, `screen`) |
| [Capabilities](capabilities.md) | Terminal feature detection (`capabilities` module) |
| [Text Editing](text-editing.md) | Rope-based editor (`text` module, exposed as `textEngine`) |
| [Scheduler](scheduler.md) | Frame timing and priority scheduling (`scheduler` module) |

## Engineering notes

| Document | What it describes |
|----------|-------------------|
| [Dirty Diff Audit](dirty-diff-audit.md) | Engineering audit of the dirty-region diffing system |
| [Viewport Culling Comparison](viewport-culling-comparison.md) | OpenTUI vs BetterTUI viewport-culling comparison |
| [Visibility Propagation](visibility-propagation.md) | Visibility propagation design |

## Principles (non-negotiable)

- **BetterTUI is a framework.** Never an application, IDE, AI framework, or editor.
- **Rust owns rendering.** TypeScript never renders; it emits commands.
- **Framework adapters are optional.** React is the first adapter. Vue, Solid, Svelte, vanilla TS must be addable without touching Rust.
- **No business logic in the engine.** It is a rendering/layout/input framework only.

## Status Reality Check

The Rust engine (`bettertui-engine`, **1,332 passing lib tests**, verified via `cargo test --lib`) and its napi bindings (`bettertui-bindings`) are the most complete parts: rendering, layout, frame buffer, events, input, animation, text engine, PTY, capability detection, and Nerd Font support are implemented and tested.

The TypeScript side: `@bettertui/core` (command buffer, reconciler wrapper, runtime, native bridge) is implemented; `@bettertui/react` has a real `react-reconciler` host config, hooks, and 53 component exports — the reconciler and hooks are fully wired, but the component functions are thin wrappers that emit element descriptors and are not yet connected to a live native render loop. `@bettertui/themes` was removed (theme system moved to Rust engine + `@bettertui/shared` — internal package, re-exported by `@bettertui/core`/`@bettertui/react`); `@bettertui/devtools` is a stub. 15 example apps in `@bettertui/examples` demonstrate the API. See [ROADMAP.md](../../ROADMAP.md) at the repo root for the current code-accurate status.
