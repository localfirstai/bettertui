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
    C -->|napi-rs FFI| D[Rust Engine packages/core/crates/bindings]
    D --> E[bettertui-engine]
    E -->|crossterm + portable-pty| F[(Terminal / PTY)]
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
├── packages/
│   ├── shared/        # @bettertui/shared  — type-only foundation
│   ├── core/          # @bettertui/core    — command protocol, tree ops, runtime, native bridge
│   │   └── crates/
│   │       ├── engine/        # bettertui-engine (Rust library)
│   │       └── bindings/      # bettertui-bindings (napi-rs cdylib)
│   ├── react/         # @bettertui/react   — React 19 adapter
│   ├── themes/        # @bettertui/themes  — theme defs + factory
│   ├── devtools/      # @bettertui/devtools — devtools stub
│   └── benchmark/     # @bettertui/benchmark — TS benchmark harness
├── apps/
│   ├── website/       # @bettertui/website — Astro/Starlight docs + landing site
│   └── performance/   # @bettertui/performance — OpenTUI vs BetterTUI benchmark site
├── examples/          # 14 example apps (fundamentals/ + showcase/)
├── docs/              # this documentation
├── tasks/             # PRDs, reports, archived slop
├── scripts/           # empty placeholder (scripts live in package.json/turbo.json)
├── packages/core/Cargo.toml         # Rust workspace
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
| `@bettertui/react` | React adapter (renderer, hooks, 69 components) | ✅ Reconciler + hooks real; components thin wrappers |
| `@bettertui/core` (native bridge) | Internal napi bindings bridge (merged from `@bettertui/native`) | ✅ Implemented (needs native addon) |
| `@bettertui/themes` | Theme definitions and utilities | ✅ Partial (default theme + factory) |
| `@bettertui/devtools` | Developer tooling | ❌ Stub (`createDevTools` → `null`) |
| `@bettertui/benchmark` | Vitest benchmarks | ✅ Implemented |
| `@bettertui/widgets` | Widget library (proposed) | 🔜 Not yet a package |
| `@bettertui/icons` | Icon registry (proposed) | 🔜 Not yet a package |

## React Components

BetterTUI provides 69 React component functions (thin wrappers that emit element descriptors; not yet wired to a live native render loop):

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
cargo build -p bettertui-bindings --manifest-path packages/core/Cargo.toml

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
- **Language:** TypeScript 7+
- **Monorepo:** TurboRepo + pnpm
- **React:** React 19 + custom reconciler
- **Formatting/Linting:** Biome (TS/JS/JSON), rustfmt + clippy (Rust)
- **Git Hooks:** Husky

## Current Status

The Rust engine and its napi-rs bindings are the most complete part: rendering, layout, frame buffer, events, input, animation, text engine, PTY, capability detection, VT emulation, and Nerd Font support are implemented and covered by **1,204 passing lib tests**. The TypeScript side is implemented: `@bettertui/core` (including the former `@bettertui/native` bridge) is implemented; `@bettertui/react` has a real `react-reconciler` host config, hooks, and 69 component exports — though the component functions are thin wrappers not yet wired to a live native render loop; **14 runnable example apps** are available under `examples/`.

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
