# BetterTUI Documentation

Welcome to the canonical BetterTUI documentation. This is the source of truth for the framework; everything here is derived from the implementation.

## Start here

- [Architecture Overview](architecture/Overview.md) — repository layout, workspaces, dependency direction
- [Getting Started](guides/getting-started.md) — install, build, run the one wired example

## Architecture (Rust engine + protocol)

| Doc | Scope |
|-----|-------|
| [Overview](architecture/Overview.md) | workspaces, build system, layers |
| [Node Model](architecture/NodeModel.md) | arena + `RenderNode` |
| [Protocol](architecture/Protocol.md) | command protocol (TS ↔ Rust) |
| [Rendering Pipeline](architecture/RenderingPipeline.md) | arena → ANSI |
| [Layout](architecture/Layout.md) | Taffy flexbox → cells |
| [Frame Buffer](architecture/FrameBuffer.md) | cell grid + dirty diff |
| [Event System](architecture/EventSystem.md) | capture/target/bubble |
| [Input System](architecture/InputSystem.md) | keyboard/mouse/paste/ANSI |
| [Animation](architecture/Animation.md) | tween/spring/keyframe |
| [Scheduler](architecture/Scheduler.md) | frame timing |
| [Capabilities](architecture/Capabilities.md) | terminal feature detection |
| [Terminal](architecture/Terminal.md) | raw mode + VT emulation |
| [PTY](architecture/PTY.md) | embedding a process |
| [Compositor](architecture/Compositor.md) | layered output + screen |
| [Text Editing](architecture/TextEditing.md) | rope editor |
| [Widget Model](architecture/WidgetModel.md) | trait-based widgets |

## Guides

- [Getting Started](guides/getting-started.md)
- [Theming](guides/theming.md)
- [Terminal & PTY](guides/terminal.md)
- [Animations](guides/animations.md)
- [Testing](guides/testing.md)

## API & Packages

- [API Index](api/README.md)
- Per-package: [shared](api/packages/shared.md), [core](api/packages/core.md), [react](api/packages/react.md), [native](api/packages/native.md), [widgets](api/packages/widgets.md), [themes](api/packages/themes.md), [icons](api/packages/icons.md), [devtools](api/packages/devtools.md)

## Subsystem docs

- [Runtime](runtime.md) · [Renderer](renderer.md) · [Widgets](widgets.md) · [Themes](themes.md)
- [Terminal](terminal.md) · [React](react.md) · [Native](native.md) · [DevTools](devtools.md)
- [Performance](performance.md) · [Testing](testing.md) · [Examples](examples.md)

## Repository root docs

- [README](../README.md)
- [CONTRIBUTING](../CONTRIBUTING.md)
- [ROADMAP](../ROADMAP.md)
- [ARCHITECTURE](../ARCHITECTURE.md)
- [CHANGELOG](../CHANGELOG.md)

## Obsolete docs

Older, aspirational design docs and AI iteration reports have been moved to `tasks/slop/docs/` (they described packages/modules that no longer match the code). They are preserved for history but are **not** canonical.
