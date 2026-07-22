# BetterTUI

A framework-agnostic terminal UI rendering engine powered by **Rust** and exposed to **TypeScript** with first-class **React**, **SolidJS** support.

BetterTUI is a **native terminal UI**, not an application, IDE, or AI tool. It provides the rendering engine, layout engine, input system, and widget primitives that other frameworks build on top of.

## Philosophy

- **Rust owns performance-critical work.** Rendering, layout, input parsing, event dispatch, and text editing live in a native engine.
- **TypeScript owns developer experience.** Public APIs, framework bindings, theming, and tooling are written in TypeScript.
- **The engine is framework-agnostic.** The native core exposes a C ABI and can be used from any language.

## Architecture

```
┌───────────────────────────────────────────────────────────────────────────┐
│                           BetterTUI Architecture                          │
└───────────────────────────────────────────────────────────────────────────┘

   ┌──────────────────┐     ┌──────────────────┐     ┌──────────────────┐
   │  Vanilla / Native│     │    React App     │     │   SolidJS App    │
   │    TypeScript    │     │                  │     │                  │
   └────────┬─────────┘     └────────┬─────────┘     └────────┬─────────┘
            │                        │                        │
            ▼                        ▼                        ▼
   ┌──────────────────┐     ┌───────────────────┐    ┌──────────────────┐
   │  @bettertui/core │     │  @bettertui/react │    │  @bettertui/solid│
   │  (public API)    │     │    (adapter)      │    │    (adapter)     │
   └────────┬─────────┘     └────────┬──────────┘    └────────┬─────────┘
            │                        │                        │
            │                        │                        │
            └────────────────────────┘────────────────────────┘
            │
            ▼
   ┌─────────────────────────┐
   │   napi-rs FFI bridge    │
   └────────┬────────────────┘
            ▼
   ┌──────────────────────────────────────────────────────────────┐
   │                      Rust Engine                             │
   │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────┐   │
   │  │   Rendering     │  │   Layout        │  │   Input     │   │
   │  │   (text buffer) │  │   (taffy)       │  │   (events)  │   │
   │  └─────────────────┘  └─────────────────┘  └─────────────┘   │
   │  ┌─────────────────┐  ┌─────────────────┐                    │
   │  │   Widgets       │  │   Text Editing │                     │
   │  │   (primitives)  │  │   (rope)        │                    │
   │  └─────────────────┘  └─────────────────┘                    │
   └────────────────────────────┬─────────────────────────────────┘
                                │
                                ▼
   ┌──────────────────────────────────────────────────────────────┐
   │               Terminal / PTY Layer                           │
   │            (crossterm + portable-pty)                        │
   └──────────────────────────────────────────────────────────────┘

   ┌──────────────────────────────────────────────────────────────┐
   │  Package Dependencies:                                       │
   │  • @bettertui/core     → Public API, framework-agnostic      │
   │  • @bettertui/react    → React adapter (depends on core)     │
   │  • @bettertui/solid    → SolidJS adapter (depends on core)   │
   └──────────────────────────────────────────────────────────────┘
```

The Rust engine owns:

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

Docs: https://bettertui.dev/docs/getting-started

Quick start with `create-tui`:

```bash
# npm
npm create tui

# pnpm
pnpm create tui

# bun
bun create tui
```

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

The React adapter (`@bettertui/react`) is not implemented yet. The `packages/react` directory is a
placeholder. Once built, React apps will install only `@bettertui/react` and call `render(<Box>…</Box>)`.

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
`CommandRuntime`, native bridge, testing utilities) and `@bettertui/examples` (vanilla / native
TypeScript demos runnable on `@bettertui/core`). The React adapter (`@bettertui/react`) is **not
implemented** — `packages/react` is a placeholder directory only. Vanilla / native TypeScript
examples live under `packages/examples/typescript/` and run directly on `@bettertui/core` (no React).

> The Rust unit-test suite is co-located in the engine crate. Run
> `cargo test --manifest-path packages/core/Cargo.toml --lib` to execute it. (At the time of
> writing the engine test build has a compilation issue in `terminal/vt.rs`; this must be fixed
> before the suite is green.)

## Documentation

- [Architecture](docs/architecture/README.md)
- [Documentation index](docs/README.md)
- [Roadmap](docs/ROADMAP.md)
- [Contributing](CONTRIBUTING.md)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). All contributions are licensed under MIT.

## License

MIT
