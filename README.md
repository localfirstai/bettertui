# BetterTUI

High-performance terminal UI framework powered by Rust and TypeScript.

## Vision

BetterTUI aims to be the **React Native for Terminal Applications** — a framework-agnostic, Rust-powered engine with JavaScript bindings that enables developers to build rich terminal interfaces using familiar UI paradigms.

React is the first supported framework, but the architecture is designed from day one to support Vue, Solid, Svelte, Preact, and vanilla TypeScript without modifying the core engine.

## Architecture

```
Framework Adapter (React, Vue, Solid...)
        ↓
  @bettertui/core
        ↓
  Rust Native Engine
        ↓
    Terminal
```

The Rust engine owns all performance-critical operations:

- **Rendering** — GPU-accelerated frame buffer with dirty diffing
- **Layout** — Taffy-based flexbox/CSS-grid engine
- **Input** — Keyboard, mouse, and paste events
- **Terminal** — Detection, capabilities, and escape sequence handling
- **Scheduler** — Async task orchestration
- **Animation** — 60fps tween engine
- **Text Editing** — Rope-based buffer with cursor management
- **Clipboard** — System clipboard integration

TypeScript owns the developer experience:

- **API** — Clean, typed interfaces
- **React Bindings** — Custom reconciler (not Ink)
- **Components** — Declarative widget primitives
- **Themes** — Design token system
- **Plugins** — Extension architecture

## Project Layout

```
bettertui/
├── native/
│   ├── engine/       # Rust rendering engine
│   └── bindings/     # napi-rs Node.js bindings
│
├── packages/
│   ├── core/         # Framework-agnostic types
│   ├── reconciler/   # React reconciler host config
│   ├── react/        # Public React API
│   ├── widgets/      # Widget library
│   ├── themes/       # Theme definitions
│   ├── icons/        # Icon registry
│   ├── shared/       # Internal shared types
│   └── devtools/     # Developer tooling
│
├── examples/
│   ├── counter/
│   ├── dashboard/
│   ├── text-editor/
│   ├── table/
│   ├── tree/
│   └── mouse/
│
├── docs/
├── benchmarks/
└── scripts/
```

## Package Overview

| Package | Description |
|---------|-------------|
| `@bettertui/shared` | Framework-agnostic type definitions |
| `@bettertui/core` | Core types, tree diffing, node interfaces |
| `@bettertui/reconciler` | Custom React reconciler host config |
| `@bettertui/react` | Public React component API |
| `@bettertui/widgets` | Pre-built widget components |
| `@bettertui/themes` | Theme definitions and utilities |
| `@bettertui/icons` | Icon registry and management |
| `@bettertui/devtools` | Developer tools and debugging |

## Getting Started

```bash
# Install dependencies
pnpm install

# Build all packages
pnpm build

# Start development
pnpm dev
```

## Tech Stack

- **Engine**: Rust (napi-rs, taffy, crossterm)
- **Runtime**: Node.js
- **Language**: TypeScript 5+
- **Monorepo**: TurboRepo + pnpm
- **React**: React 19 + custom reconciler

## License

MIT
