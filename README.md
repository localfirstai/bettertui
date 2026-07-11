# BetterTUI

A framework-agnostic terminal UI rendering engine powered by **Rust** and exposed to **TypeScript** with first-class **React** support.

BetterTUI is a **framework**, not an application, IDE, or AI tool. It provides the rendering engine, layout engine, input system, and widget primitives that other frameworks build on top of.

## Philosophy

- **Rust owns performance-critical work.** Rendering, layout, input parsing, event dispatch, and text editing live in a native engine.
- **TypeScript owns developer experience.** Public APIs, framework bindings, theming, and tooling are written in TypeScript.
- **The engine is framework-agnostic.** It knows nothing about React. Commands are the only boundary between a UI framework and the engine, so Vue, Solid, Svelte, and vanilla TypeScript adapters can be added without touching Rust.
- **No business logic in the engine.** It is a rendering framework, not an application runtime.

## Architecture

```
Framework Adapter (React, …)
        ↓ commands
  TypeScript packages (@bettertui/core, @bettertui/react, …)
        ↓ napi-rs FFI
  Rust Native Engine (bettertui-engine)
        ↓ crossterm
    Terminal
```

The Rust engine owns:

- **Rendering** — double-buffered, cell-based frame buffer with dirty-region diffing
- **Layout** — Taffy-based flexbox adapted to terminal cells
- **Input** — keyboard, mouse, and paste parsing
- **Terminal** — capability detection, raw mode, alternate screen
- **Scheduler** — frame timing and async task orchestration
- **Animation** — tween engine with keyframes
- **Text editing** — rope-based buffer with cursor, selection, and undo/redo
- **PTY & clipboard** — embedded terminal processes and system clipboard

TypeScript owns:

- **API** — typed, framework-agnostic interfaces
- **React bindings** — reconciler host config and components
- **Themes** — design-token system
- **Icons** — icon registry

See [`docs/architecture`](docs/architecture/README.md) for the full design.

## Project Layout

```
bettertui/
├── native/
│   ├── engine/       # Rust rendering engine
│   └── bindings/     # napi-rs Node.js bindings
├── packages/
│   ├── core/         # Node model, protocol, tree operations
│   ├── shared/       # Framework-agnostic type definitions
│   ├── reconciler/   # React reconciler host config
│   ├── react/        # Public React component API
│   ├── native/       # Engine/native runtime wrapper
│   ├── widgets/      # Widget library
│   ├── themes/       # Theme definitions
│   ├── icons/        # Icon registry
│   └── devtools/     # Developer tooling
├── examples/         # Example applications
├── docs/             # Architecture documentation
├── benchmarks/       # Performance harness
└── scripts/          # Build and maintenance scripts
```

## Package Overview

| Package | Description | Status |
|---------|-------------|--------|
| `@bettertui/shared` | Framework-agnostic type definitions | Types complete |
| `@bettertui/core` | Node model, protocol, tree operations | Types |
| `@bettertui/reconciler` | React reconciler host config | In progress |
| `@bettertui/react` | Public React component API | Stub components |
| `@bettertui/native` | napi bindings wrapper and runtime | Implemented |
| `@bettertui/widgets` | Reusable widget components | Interface only |
| `@bettertui/themes` | Theme definitions and utilities | Partial |
| `@bettertui/icons` | Icon registry and management | Scaffolded |
| `@bettertui/devtools` | Developer tools and debugging | Stub |

## Getting Started

```bash
# Install dependencies
pnpm install

# Build all packages (compiles the Rust engine via napi-rs)
pnpm build

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

- **Engine:** Rust (napi-rs, taffy, crossterm, ropey)
- **Runtime:** Node.js
- **Language:** TypeScript 5+
- **Monorepo:** TurboRepo + pnpm
- **React:** React 19 + custom reconciler
- **Formatting/Linting:** Biome (TS/JS/JSON), rustfmt + clippy (Rust)
- **Git Hooks:** Husky

## Current Status

The Rust engine and its napi-rs bindings are the most complete part of the project: the rendering pipeline, layout, frame buffer, events, input, animation, text engine, PTY, and Nerd Font support are implemented and covered by tests. The TypeScript side — React components, the reconciler, widgets, and examples — is still early and largely stubbed. Integration between the two layers is ongoing.

## Documentation

- [Architecture](docs/architecture/README.md) — engine and protocol design
- [Contributing](CONTRIBUTING.md)
- [Roadmap](ROADMAP.md)
- [Changelog](CHANGELOG.md)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). All contributions are licensed under MIT.

## License

MIT
