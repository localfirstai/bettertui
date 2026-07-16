# BetterTUI Architecture

> The canonical, code-accurate architecture reference for BetterTUI.
> Every statement here is derived from the current implementation, not from design intent.

BetterTUI is a **framework-agnostic terminal UI rendering engine**. Rust owns all performance-critical work (rendering, layout, input, animation, text editing, terminal emulation). TypeScript owns the developer experience (typed APIs, framework bindings, theming). A napi-rs FFI boundary is the only coupling point between the two, and it carries **commands** — never framework concepts.

```
Vanilla / Native TS App ─┐
                         ├─▶ @bettertui/core ──(napi-rs FFI)──▶ Rust Engine (bettertui-engine cdylib)
React App ─▶ @bettertui/react ───────────────┘                       │
         │                                                            ▼
         └─────────────────────────────────────▶ bettertui-engine ──▶ (Terminal / PTY)
                                                                        via crossterm + portable-pty
Both @bettertui/core and @bettertui/react re-export the internal @bettertui/shared types.
```

## Two first-class packages

- **`@bettertui/core`** is a fully public, framework-agnostic package for vanilla / native TypeScript. It is the recommended entry point when you don't use React.
- **`@bettertui/react`** is the React adapter. React apps install **only** `@bettertui/react` — it depends on `@bettertui/core` and resolves it automatically.

`@bettertui/shared` is **internal** (re-exported by core and react) and must not be installed directly.

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
`CommandRuntime`, native bridge, testing utilities) and `@bettertui/react` (a real
`react-reconciler` host config, hooks, and 13 components). `@bettertui/themes` was removed; theme
types live in `@bettertui/shared` (internal, re-exported by core/react) and theme presets are
created in the native bridge. `@bettertui/devtools` is implemented. Vanilla examples under
`examples/vanila/` demonstrate the `@bettertui/core` API. See [ROADMAP.md](../../ROADMAP.md) for
the current code-accurate status.
