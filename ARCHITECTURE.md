# Architecture

This document is the root-level architecture summary. The **canonical, module-by-module** reference lives in [`docs/architecture/`](docs/architecture/README.md) and is generated from the implementation.

## What BetterTUI is

A **framework-agnostic terminal UI rendering engine**. Rust owns rendering, layout, input, animation, text editing, and terminal emulation. TypeScript owns the developer experience (typed APIs, framework bindings, theming). A napi-rs FFI boundary carries **commands** — never framework concepts.

BetterTUI is **never** an application, IDE, AI framework, or editor. It is infrastructure that applications are built on.

## Dependency flow (the rule)

```mermaid
graph TD
    App[Application] --> React[@bettertui/react]
    React --> Core[@bettertui/core]
    Core -->|napi-rs FFI| Bindings[packages/core/crates/bindings]
    Bindings --> Engine[bettertui-engine]
    Engine --> Term[(crossterm / portable-pty)]
    Core --> Shared[@bettertui/shared]
    React --> Shared
    Themes[@bettertui/themes] --> Shared
    Widgets[@bettertui/widgets] --> Core
    Widgets --> Shared
```

- `bettertui-engine` never imports any JS framework.
- `@bettertui/core` never imports React.
- Only `@bettertui/react` imports React.
- The only boundary between a UI framework and the engine is the **command** — so Vue/Solid/Svelte/vanilla-TS adapters require no Rust changes.

## Layer model

```mermaid
graph TD
    subgraph L1[Framework Adapter — TypeScript]
        R[@bettertui/react]
    end
    subgraph L2[Core TypeScript]
        C[@bettertui/core]
        S[@bettertui/shared]
    end
    subgraph L3[Rust Engine]
        B[packages/core/crates/bindings]
        E[bettertui-engine]
    end
    subgraph L4[Terminal]
        T[crossterm / portable-pty]
    end
    R --> C
    C -->|napi-rs| B
    B --> E
    E --> T
```

## Rust engine: what it owns

| Subsystem | Module | Implemented? |
|-----------|--------|--------------|
| Arena / node tree | `tree` | ✅ |
| Command protocol | `protocol` | ✅ |
| Renderer / frame production | `renderer`, `render_object`, `painter` | ✅ |
| Frame buffer | `framebuffer` | ✅ |
| Dirty diff | `dirty_diff` | ✅ |
| Layout (Taffy) | `layout` | ✅ |
| Events | `events` | ✅ |
| Input parsing | `input`, `ansi` | ✅ |
| Animation | `animation` | ✅ (callback path partial) |
| Scheduler | `scheduler` | ✅ |
| Capabilities | `capabilities` | ✅ |
| Terminal I/O + VT | `terminal`, `terminal/vt` | ✅ |
| PTY / process | `pty`, `terminal_process` | ✅ |
| Compositor / screen | `compositor`, `screen` | ✅ |
| Text editing (rope) | `text` | ✅ |
| Widgets | `widgets` | ✅ (~200 tests) |
| FFI surface | `bindings` (cdylib) | ✅ |

> Note: the top-level `keyboard/`, `mouse/`, `editor/`, `clipboard/`, `selection/`, `ffi/` engine modules are **stubs**; the real logic lives in `input/`, `text/`, `capabilities/`, `terminal/vt`, and the `bindings` crate.

## TypeScript: what it owns

| Package | Role | Status |
|---------|------|--------|
| `@bettertui/shared` | Type foundation (zero runtime code) | ✅ |
| `@bettertui/core` | Command protocol, tree ops, runtime | ✅ |
| `@bettertui/react` | React adapter (reconciler + hooks + 69 components) | ⚠️ reconciler/hooks real; component fns are thin wrappers not yet wired to live render loop |
| `@bettertui/core` (native bridge) | Internal napi bridge (merged from `@bettertui/native`) | ✅ (needs native addon) |
| `@bettertui/themes` | Theme defs + factory | ✅ |
| `@bettertui/devtools` | DevTools | ❌ stub (`createDevTools` → `null`) |
| `@bettertui/benchmark` | Vitest benchmarks | ✅ |

## Architecture principles

1. **Framework first.** The engine is framework-agnostic; React is only the first adapter.
2. **Rust owns rendering.** TypeScript never renders; it emits commands.
3. **No business logic in the engine.** It is a rendering/layout/input framework.
4. **Future adapters (Vue, Solid, Svelte, Astro, vanilla TS) must not require Rust changes.**

## Further reading

- [docs/architecture/ — full design reference](docs/architecture/README.md)
- [docs/ — documentation index](docs/README.md)
- [ROADMAP.md](ROADMAP.md) — current, code-accurate status
- [CONTRIBUTING.md](CONTRIBUTING.md)
