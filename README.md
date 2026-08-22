# BetterTUI

> **Website:** [bettertui.dev](https://bettertui.dev)

A framework-agnostic terminal UI rendering engine powered by **Rust** and exposed to **TypeScript** with first-class **React** support.

BetterTUI is a **terminal UI framework**, not an application, IDE, or AI tool. It provides the rendering engine, layout engine, input system, and widget primitives that other frameworks build on top of.

## Architecture

```
Vanilla / Native TS App → @bettertui/core → napi-rs FFI → Rust Engine (bettertui-engine cdylib)
                                                              ↓
                                                    Terminal / PTY via crossterm + portable-pty
React App → @bettertui/react → @bettertui/core (auto-resolved)
```

- **Rust owns performance-critical work.** Rendering, layout, input parsing, event dispatch, text editing, and animation live in a native engine (`bettertui-engine`, `packages/core/crates/engine/`).
- **TypeScript owns developer experience.** `@bettertui/core` is the framework-agnostic public API (command protocol, reconciler, runtime, native bridge). `@bettertui/react` is the React adapter with hooks, providers, and components.
- **`@bettertui/shared`** is an internal type-only package (re-exported by core). Do not install it directly.

## Quick start

```bash
# React
npm install @bettertui/react

# Vanilla TypeScript
npm install @bettertui/core
```

### Build from source

```bash
pnpm install
pnpm build                          # TypeScript packages
pnpm --filter @bettertui/core build:native  # Rust addon (bettertui_engine.node)
```

### Run examples

```bash
pnpm --filter @bettertui/core build:native
pnpm --filter @bettertui/examples dev          # interactive browser
pnpm --filter @bettertui/examples dev <slug>    # single example
```

## Repo structure

```
packages/
├── core/          @bettertui/core     — framework-agnostic public API
│   └── crates/
│       ├── engine/     bettertui-engine   — Rust rendering engine (cdylib)
│       └── benchmark/  bettertui-benchmark — Rust benchmarks
├── shared/        @bettertui/shared   — type-only foundation (internal)
├── react/         @bettertui/react    — React adapter (reconciler, hooks, components)
├── solid/         @bettertui/solid    — SolidJS adapter (placeholder)
├── performance/   @bettertui/performance — Vitest benchmarks
└── examples/      @bettertui/examples — 64 TypeScript examples
apps/
└── website/       @bettertui/website  — Astro documentation site
docs/
├── architecture/  — Rust engine + protocol architecture docs
├── api/           — Package API references
└── guides/        — Getting started, theming, testing, terminal guides
```

## Packages

| Package | Status | Description |
|---------|--------|-------------|
| `@bettertui/core` | ✅ Implemented | Framework-agnostic: command protocol, reconciler, runtime, native bridge, DevTools, testing utilities, widgets, keymap |
| `@bettertui/react` | ✅ Implemented | React 19 adapter: `createRoot()`, reconciler, hooks (`useRuntime`, `useFocus`, `useKeyboard`, `useTheme`, `useTimeline`, `useTerminalDimensions`), JSX types, runtime context |
| `@bettertui/solid` | ⏳ Placeholder | SolidJS adapter — `packages/solid` is a placeholder directory with stub structure |
| `@bettertui/shared` | ✅ Implemented | Type-only foundation (internal, re-exported by core) |
| `@bettertui/performance` | ✅ Implemented | Vitest benchmark suite |
| `@bettertui/examples` | ✅ Implemented | 64 TypeScript examples runnable on `@bettertui/core` |

## Tech stack

- **Engine:** Rust (napi-rs, taffy, crossterm, ropey, portable-pty, slotmap)
- **Runtime:** Node.js
- **Language:** TypeScript (ES2022+)
- **Monorepo:** TurboRepo + pnpm
- **Formatting/Linting:** Biome (TS/JS/JSON), rustfmt + clippy (Rust)

## Documentation

- [Website](https://bettertui.dev)
- [Architecture](docs/architecture/README.md)
- [Documentation index](docs/README.md)
- [API reference](docs/api/README.md)
- [Guides](docs/guides/getting-started.md)
- [Roadmap](docs/ROADMAP.md)
- [Contributing](CONTRIBUTING.md)

## License

MIT
