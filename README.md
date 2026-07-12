# BetterTUI

A framework-agnostic terminal UI rendering engine powered by **Rust** and exposed to **TypeScript** with first-class **React** support.

BetterTUI is a **framework**, not an application, IDE, or AI tool. It provides the rendering engine, layout engine, input system, and widget primitives that other frameworks build on top of.

## Philosophy

- **Rust owns performance-critical work.** Rendering, layout, input parsing, event dispatch, and text editing live in a native engine.
- **TypeScript owns developer experience.** Public APIs, framework bindings, theming, and tooling are written in TypeScript.
- **The engine is framework-agnostic.** It knows nothing about React. Commands are the only boundary between a UI framework and the engine, so Vue, Solid, Svelte, and vanilla TypeScript adapters can be added without touching Rust.
- **No business logic in the engine.** It is a rendering framework, not an application runtime.

## Architecture

```mermaid
graph TD
    A[Application] --> B[@bettertui/react]
    B --> C[@bettertui/core]
    C --> D[@bettertui/native]
    D -->|napi-rs FFI| E[Rust Engine bettertui-bindings]
    E --> F[bettertui-engine]
    F -->|crossterm + portable-pty| G[(Terminal / PTY)]
```

The Rust engine owns:

- **Rendering** — double-buffered, cell-based frame buffer with dirty-region diffing
- **Layout** — Taffy-based flexbox adapted to terminal cells
- **Input** — keyboard, mouse, and paste parsing (incl. Kitty / CSI-u)
- **Terminal** — capability detection, raw mode, alternate screen, VT emulation
- **Scheduler** — frame timing and priority scheduling
- **Animation** — tween/spring/keyframe engine
- **Text editing** — rope-based buffer with cursor, selection, undo/redo
- **PTY & clipboard** — embedded terminal processes and system clipboard

TypeScript owns:

- **API** — typed, framework-agnostic interfaces
- **React bindings** — reconciler host config and components
- **Themes** — design-token system
- **Hooks** — useTheme, useFocus, useKeyboard, useMouse, useAnimation, useTimeline, etc.

See [`docs/architecture`](docs/architecture/README.md) for the full design, and [`docs/`](docs/README.md) for the complete documentation index.

## Project Layout

```
bettertui/
├── native/
│   ├── engine/        # bettertui-engine (Rust library)
│   └── bindings/      # bettertui-bindings (napi-rs cdylib)
├── packages/
│   ├── shared/        # @bettertui/shared  — type-only foundation
│   ├── core/          # @bettertui/core    — command protocol, tree ops, runtime
│   ├── react/         # @bettertui/react   — React 19 adapter
│   ├── native/        # @bettertui/native  — napi bridge + runtime
│   ├── themes/        # @bettertui/themes  — theme defs + factory
│   └── icons/         # @bettertui/icons   — icon registry
├── apps/
│   └── website/       # @bettertui/website — Astro/Starlight docs + landing site
├── examples/          # 12 runnable examples
├── docs/              # this documentation
├── benchmarks/        # empty placeholder (no harness yet)
├── scripts/           # empty placeholder (scripts live in package.json/turbo.json)
├── Cargo.toml         # Rust workspace
├── package.json       # root TS manifest + pnpm scripts
├── pnpm-workspace.yaml
├── turbo.json
└── biome.json
```

## Package Overview

| Package | Description | Status |
|---------|-------------|--------|
| `@bettertui/shared` | Framework-agnostic type definitions | ✅ Types complete |
| `@bettertui/core` | Framework-agnostic runtime, command protocol, tree ops | ✅ Implemented |
| `@bettertui/react` | React adapter (renderer, hooks, components) | ✅ Reconciler + 50+ components |
| `@bettertui/native` | napi bindings wrapper and runtime | ✅ Implemented (needs native addon) |
| `@bettertui/themes` | Theme definitions and utilities | ✅ Partial (default theme + factory) |
| `@bettertui/icons` | Icon registry and management | ✅ Scaffolded (empty) |
| `@bettertui/devtools` | Developer tooling | ✅ Scaffolded (stub) |

## React Components

BetterTUI provides 50+ React components:

**Layout:** Box, Flex, Grid, Stack, Spacer, Separator
**Text:** Text, Heading, Label, Code, Blockquote
**Input:** Button, Input, Textarea, Checkbox, Radio, Switch, Slider, Select, Combobox
**Navigation:** Tabs, Accordion
**Data Display:** Badge, Progress, Spinner, List, Tree, Table, DataTable
**Overlays:** Tooltip, Modal, Popover, Dropdown, ContextMenu, Toast
**Status:** StatusLine, StatusBar
**Layout:** Pane, Viewport
**Specialized:** Calendar, Chart, ScrollArea, Markdown, CodeBlock, Diff
**AI-specific:** PromptComposer, ChatView, ThinkingIndicator
**Terminal:** Terminal, TerminalViewport, TerminalProcess

## React Hooks

- `useTheme` / `Provider` — theme context with dark theme default
- `useFocus` / `FocusProvider` — basic focus tracking by string ID
- `useKeyboard` — keyboard event handling
- `useMouse` — mouse event handling
- `useTerminal` / `TerminalProvider` — terminal size context
- `useFrame` — frame request mechanism
- `useClipboard` — clipboard operations
- `useAnimation` — animation with easing (21 easing functions)
- `useTimeline` — timeline-based animation sequencing
- `useSelection` / `SelectionProvider` — text selection tracking
- `useCapabilities` / `CapabilitiesProvider` — terminal capability detection

## Getting Started

```bash
# Install dependencies
pnpm install

# Build all TypeScript packages (does NOT compile Rust)
pnpm build

# Build the native Rust addon (required before running anything native)
cargo build -p bettertui-bindings

# Type-check and lint
pnpm typecheck
pnpm lint
```

### Rust engine

```bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

## Tech Stack

- **Engine:** Rust (napi-rs, taffy, crossterm, ropey, portable-pty, slotmap)
- **Runtime:** Node.js
- **Language:** TypeScript 5+
- **Monorepo:** TurboRepo + pnpm
- **React:** React 19 + custom reconciler
- **Formatting/Linting:** Biome (TS/JS/JSON), rustfmt + clippy (Rust)
- **Git Hooks:** Husky

## Current Status

The Rust engine and its napi-rs bindings are the most complete part: rendering, layout, frame buffer, events, input, animation, text engine, PTY, capability detection, VT emulation, and Nerd Font support are implemented and covered by ~1193 passing tests. The TypeScript side is implemented: `@bettertui/core` and `@bettertui/native` are implemented; `@bettertui/react` has a real reconciler + hooks + 50+ components; 12 runnable examples are available.

## Documentation

- [Architecture (root summary)](ARCHITECTURE.md)
- [Architecture (full reference)](docs/architecture/README.md)
- [Documentation index](docs/README.md)
- [Contributing](CONTRIBUTING.md)
- [Roadmap](ROADMAP.md)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). All contributions are licensed under MIT.

## License

MIT
