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
    C --> H[@bettertui/widgets]
    C --> I[@bettertui/themes]
    C --> J[@bettertui/shared]
    B --> J
    I --> J
    H --> C
```

## Document Index

| Document | What it describes |
|----------|-------------------|
| [Overview.md](Overview.md) | Repository layout, workspaces, build system, dependency direction |
| [NodeModel.md](NodeModel.md) | Arena-allocated node tree (`tree` module) |
| [Protocol.md](Protocol.md) | Command protocol between TypeScript and Rust (`protocol` module) |
| [RenderingPipeline.md](RenderingPipeline.md) | Render stages from arena → ANSI (`renderer`, `painter`, `dirty_diff`, `framebuffer`) |
| [Layout.md](Layout.md) | Taffy-based flexbox layout adapted to terminal cells (`layout` module) |
| [FrameBuffer.md](FrameBuffer.md) | Cell grid and dirty-region diffing (`framebuffer`, `dirty_diff`) |
| [EventSystem.md](EventSystem.md) | Event dispatch and bubbling (`events` module) |
| [InputSystem.md](InputSystem.md) | Keyboard, mouse, paste, Kitty/C SI-u parsing (`input`, `ansi`) |
| [Animation.md](Animation.md) | Tween/spring/keyframe engine (`animation` module) |
| [WidgetModel.md](WidgetModel.md) | Trait-based widget framework on the arena (`widgets` module) |
| [Terminal.md](Terminal.md) | Raw mode, VT emulation, screen buffers (`terminal`, `terminal/vt`) |
| [PTY.md](PTY.md) | Embedded process spawning (`pty`, `terminal_process`) |
| [Compositor.md](Compositor.md) | Layered compositing and screen state (`compositor`, `screen`) |
| [Capabilities.md](Capabilities.md) | Terminal feature detection (`capabilities` module) |
| [TextEditing.md](TextEditing.md) | Rope-based editor (`text` module, exposed as `textEngine`) |
| [Scheduler.md](Scheduler.md) | Frame timing and priority scheduling (`scheduler` module) |

## Principles (non-negotiable)

- **BetterTUI is a framework.** Never an application, IDE, AI framework, or editor.
- **Rust owns rendering.** TypeScript never renders; it emits commands.
- **Framework adapters are optional.** React is the first adapter. Vue, Solid, Svelte, vanilla TS must be addable without touching Rust.
- **No business logic in the engine.** It is a rendering/layout/input framework only.

## Status Reality Check

The Rust engine (`bettertui-engine`, **1,204 passing lib tests**) and its napi bindings (`bettertui-bindings`) are the most complete parts: rendering, layout, frame buffer, events, input, animation, text engine, PTY, capability detection, and Nerd Font support are implemented and tested.

The TypeScript side: `@bettertui/core` (command buffer, reconciler wrapper, runtime, native bridge) is implemented; `@bettertui/react` has a real `react-reconciler` host config, hooks, and 69 component exports — the reconciler and hooks are fully wired, but the component functions are thin wrappers that emit element descriptors and are not yet connected to a live native render loop. `@bettertui/themes` and `@bettertui/icons` are minimal; `@bettertui/devtools` is a stub. 14 example apps demonstrate the API. See [ROADMAP.md](../../ROADMAP.md) at the repo root for the current code-accurate status.
