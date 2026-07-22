# BetterTUI Documentation

Welcome to the canonical BetterTUI documentation.

## Start here

- [Architecture Overview](architecture/overview.md) — repository layout, workspaces, dependency direction
- [Getting Started](guides/getting-started.md) — install, build, run examples

## Two ways to use BetterTUI

- **Vanilla / native TypeScript — `@bettertui/core` (implemented).** Framework-agnostic. Build CLI tools, daemons, and custom adapters on it directly.
- **React — `@bettertui/react` (implemented).** React 19 adapter with reconciler, hooks, and JSX support. Install only `@bettertui/react`; it depends on `@bettertui/core` and pulls it in automatically.
- **SolidJS — `@bettertui/solid` (placeholder).** Not yet implemented.

`@bettertui/shared` is internal (re-exported by core) — do not install it directly.

## Architecture (Rust engine + protocol)

| Doc | Scope |
|-----|-------|
| [Overview](architecture/overview.md) | workspaces, build system, layers |
| [Node Model](architecture/node-model.md) | arena + `RenderNode` |
| [Protocol](architecture/protocol.md) | command protocol (TS ↔ Rust) |
| [Rendering Pipeline](architecture/rendering-pipeline.md) | arena → ANSI |
| [Layout](architecture/layout.md) | Taffy flexbox → cells |
| [Frame Buffer](architecture/frame-buffer.md) | cell grid + dirty diff |
| [Event System](architecture/event-system.md) | capture/target/bubble |
| [Input System](architecture/input-system.md) | keyboard/mouse/paste/ANSI |
| [Animation](architecture/animation.md) | tween/spring/keyframe |
| [Scheduler](architecture/scheduler.md) | frame timing |
| [Capabilities](architecture/capabilities.md) | terminal feature detection |
| [Terminal](architecture/terminal.md) | raw mode + VT emulation |
| [PTY](architecture/pty.md) | embedding a process |
| [Compositor](architecture/compositor.md) | compositing primitives |
| [Text Editing](architecture/text-editing.md) | rope editor |
| [Widget Model](architecture/widget-model.md) | trait-based widgets |

## Guides

- [Getting Started](guides/getting-started.md)
- [Theming](guides/theming.md)
- [Terminal & PTY](guides/terminal.md)
- [Animations](guides/animations.md)
- [Testing](guides/testing.md)

## API & Packages

- [API Index](api/README.md)
- Per-package: [core](api/packages/core.md), [shared](api/packages/shared.md), [react](api/packages/react.md), [native](api/packages/native.md), [widgets](api/packages/widgets.md), [themes](api/packages/themes.md), [devtools](api/packages/devtools.md)

## Subsystem docs

- [Runtime](runtime.md) · [Renderer](renderer.md) · [Widgets](widgets.md) · [Themes](themes.md)
- [Terminal](terminal.md) · [React](react.md) · [Native](native.md) · [DevTools](devtools.md)
- [Performance](performance.md) · [Testing](testing.md) · [Examples](examples.md)

## Repository root docs

- [README](../README.md)
- [CONTRIBUTING](../CONTRIBUTING.md)
- [Roadmap](../ROADMAP.md)
- [CHANGELOG](../CHANGELOG.md)
