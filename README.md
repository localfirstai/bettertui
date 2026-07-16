# BetterTUI

A framework-agnostic terminal UI rendering engine powered by **Rust** and exposed to **TypeScript** with first-class **React** support.

BetterTUI is a **framework**, not an application, IDE, or AI tool. It provides the rendering engine, layout engine, input system, and widget primitives that other frameworks build on top of.

## Philosophy

- **Rust owns performance-critical work.** Rendering, layout, input parsing, event dispatch, and text editing live in a native engine.
- **TypeScript owns developer experience.** Public APIs, framework bindings, theming, and tooling are written in TypeScript.
- **The engine is framework-agnostic.** It knows nothing about React. Commands are the only boundary between a UI framework and the engine, so Vue, Solid, Svelte, and vanilla TypeScript adapters can be added without touching Rust.
- **No business logic in the engine.** It is a rendering framework, not an application runtime.

## Two ways to use BetterTUI

BetterTUI is consumed through **two first-class packages**:

- **`@bettertui/core` — native / vanilla TypeScript (first-class).** Use it directly when you don't need React. It is a fully public, framework-agnostic package: command protocol, tree operations, runtime, and the native Rust bridge. This is the recommended path for CLI tools, daemons, and custom framework adapters.
- **`@bettertui/react` — React adapter (first-class).** Install **only** `@bettertui/react` for a React app. It depends on `@bettertui/core` and pulls it in automatically, so you never install core by hand for a React project.

> Rule of thumb: **React apps install `@bettertui/react` and nothing else. Vanilla/native TypeScript apps install `@bettertui/core`.**

## Architecture

```
Vanilla / Native TS App ─┐
                         ├─▶ @bettertui/core ──(napi-rs FFI)──▶ Rust Engine
React App ─▶ @bettertui/react ───────────────┘
                                                       │
                                                       ▼
                                              bettertui-engine (cdylib: bettertui_engine.node)
                                                       │
                                                       ▼
                                         (Terminal / PTY via crossterm + portable-pty)
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

- **API** — typed, framework-agnostic interfaces (`@bettertui/core`)
- **React bindings** — reconciler host config and components (`@bettertui/react`)
- **Themes** — design-token types (in `@bettertui/shared`; presets created in the native bridge)
- **Hooks** — `useTheme`, `useFocus`, `useKeyboard`, `useMouse`, `useAnimation`, `useTimeline`, etc.

See [`docs/architecture`](docs/architecture/README.md) for the full design, and [`docs/`](docs/README.md) for the complete documentation index.

## Project Layout

```
bettertui/
├── packages/
│   ├── shared/        # @bettertui/shared  — type-only foundation (internal, re-exported by core & react)
│   ├── core/          # @bettertui/core    — command protocol, tree ops, runtime, native bridge
│   │   └── crates/                    # Rust workspace (packages/core/Cargo.toml)
│   │       ├── engine/        # bettertui-engine (lib + cdylib + layout_e2e bin; the native addon)
│   │       ├── logger/        # bettertui-logger (tracing logger for native code)
│   │       └── benchmark/     # bettertui-benchmark (Rust bench harness)
│   ├── react/         # @bettertui/react   — React 19 adapter
│   ├── devtools/      # @bettertui/devtools — developer tooling
│   └── benchmark/     # @bettertui/benchmark — TS benchmark harness
├── apps/
│   └── website/       # @bettertui/website — Astro/Starlight docs + landing site
├── examples/
│   ├── vanila/        # Vanilla / native TypeScript examples (run on @bettertui/core)
│   ├── react/         # (reserved)
│   ├── rust/          # (reserved)
│   └── solid/         # (reserved)
├── docs/              # this documentation
├── tasks/             # PRDs, reports, archived history (read-only)
├── scripts/           # repo-level automation (doc checks, example smoke tests)
├── packages/core/Cargo.toml         # Rust workspace
├── package.json       # root TS manifest + pnpm scripts
├── pnpm-workspace.yaml
├── turbo.json
└── biome.json
```

All TypeScript packages are currently `private` (not published to npm). `@bettertui/core` is the
framework package for vanilla / native TypeScript; `@bettertui/react` is the framework package for
React and depends on `@bettertui/core`.

## Package Overview

| Package | Description | Status |
|---------|-------------|--------|
| `@bettertui/core` | Framework package for native/vanilla TypeScript — framework-agnostic runtime, command protocol, tree ops, and the native Rust bridge | Implemented |
| `@bettertui/react` | React 19 adapter — install **only** this for React apps (depends on `@bettertui/core`) | Implemented (reconciler + hooks; 13 components) |
| `@bettertui/shared` | Framework-agnostic type definitions — **internal, re-exported by `@bettertui/core`/`@bettertui/react`** | Types complete |
| `@bettertui/devtools` | Developer tooling (`createDevTools` factory) | Implemented |
| `@bettertui/benchmark` | Vitest benchmarks | Implemented |

## React Components

`@bettertui/react` exports 13 component functions (each emits an element descriptor consumed by the
reconciler):

`Box`, `Text`, `Code`, `Input`, `Textarea`, `Select`, `Slider`, `TabSelect`, `ScrollBar`,
`ScrollBox`, `Markdown`, `Diff`, `TextTable`.

## React Hooks

- `Provider` / `useTheme` — theme context (dark theme default)
- `FocusProvider` / `useFocus` — focus tracking by string ID
- `useKeyboard` — keyboard event handling
- `useMouse` — mouse event handling
- `TerminalProvider` / `useTerminal` — terminal size context
- `useResize` — resize handling
- `useFrame` — frame request mechanism
- `useClipboard` — clipboard operations
- `useAnimation` (with `easings`) — animation with easing functions
- `useTimeline` — timeline-based animation sequencing
- `SelectionProvider` / `useSelection` — text selection tracking
- `CapabilitiesProvider` / `useCapabilities` — terminal capability detection
- `KeymapProvider` / `useKeymap`, `useKeymapEvent`, `useActiveBindings`, `usePendingSequence`, `useCommand`, `useKeyIntercept`, `useKeymapMode` — keymap engine (chords, modes, commands, intercepts)
- `RuntimeProvider` / `useRuntime` — runtime access

## Getting Started

### Install

```bash
# React app — install ONLY @bettertui/react (it pulls in @bettertui/core)
npm install @bettertui/react

# Vanilla / native TypeScript app — install @bettertui/core directly
npm install @bettertui/core
```

### Build from source

```bash
# Install dependencies
pnpm install

# Build all TypeScript packages (does NOT compile Rust)
pnpm build

# Build the native Rust addon (required before running anything native)
pnpm --filter @bettertui/core build:native

# Type-check and lint
pnpm typecheck
pnpm lint
```

### React usage

```tsx
import { render, Box, Text } from "@bettertui/react";

render(
  <Box>
    <Text>hello</Text>
  </Box>,
);
```

### Vanilla / native TypeScript usage

```ts
import { createEngine, detectCapabilities, CliRenderer } from "@bettertui/core";

const engine = createEngine();
const caps = detectCapabilities();
// drive the engine directly — no React required
```

### Rust engine

```bash
cargo check --manifest-path packages/core/Cargo.toml
cargo test  --manifest-path packages/core/Cargo.toml
cargo clippy --manifest-path packages/core/Cargo.toml -- -D warnings
```

## Tech Stack

- **Engine:** Rust (napi-rs, taffy, crossterm, ropey, portable-pty, slotmap, tree-sitter)
- **Runtime:** Node.js
- **Language:** TypeScript (ES2022+)
- **Monorepo:** TurboRepo + pnpm
- **React:** React 19 + custom reconciler
- **Formatting/Linting:** Biome (TS/JS/JSON), rustfmt + clippy (Rust)
- **Git Hooks:** Husky

## Current Status

The Rust engine (`bettertui-engine`) is the most complete part: rendering, layout (Taffy), frame
buffer, events, input, animation, text engine, PTY, capability detection, VT emulation, and Nerd
Font support are implemented. Terminal I/O, VT emulation, PTY, capabilities, and the widget host
live as modules **inside** `bettertui-engine` (there is no separate `bettertui-widgets`,
`bettertui-terminal`, or `bettertui-bindings` crate). The engine is built as a `cdylib` with the
`napi` feature to produce the Node.js addon (`bettertui_engine.node`).

The TypeScript side is implemented: `@bettertui/core` (command buffer, reconciler wrapper,
`CommandRuntime`, native bridge, testing utilities) and `@bettertui/react` (a real
`react-reconciler` host config, hooks, and 13 components). Vanilla / native TypeScript examples
live under `examples/vanila/` and run directly on `@bettertui/core` (no React).

> The Rust unit-test suite is co-located in the engine crate. Run
> `cargo test --manifest-path packages/core/Cargo.toml --lib` to execute it. (At the time of
> writing the engine test build has a compilation issue in `terminal/vt.rs`; this must be fixed
> before the suite is green.)

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
