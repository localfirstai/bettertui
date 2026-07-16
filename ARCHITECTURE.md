# Architecture

This document is the root-level architecture summary. The **canonical, module-by-module** reference lives in [`docs/architecture/`](docs/architecture/README.md) and is generated from the implementation.

## What BetterTUI is

A **framework-agnostic terminal UI rendering engine**. Rust owns rendering, layout, input, animation, text editing, and terminal emulation. TypeScript owns the developer experience (typed APIs, framework bindings, theming). A napi-rs FFI boundary carries **commands** — never framework concepts.

BetterTUI is **never** an application, IDE, AI framework, or editor. It is infrastructure that applications are built on.

## Two ways to consume BetterTUI

All TypeScript packages are currently `private` (not published to npm). They are framework packages, not applications:

- **`@bettertui/core` (vanilla / native TS).** The framework package for TypeScript without React. Use it directly for CLI tools, daemons, and custom framework adapters — it owns the command protocol, tree ops, `CommandRuntime`, and the native bridge.
- **`@bettertui/react` (React).** Install **only** `@bettertui/react` for a React app. It depends on `@bettertui/core` and pulls it in automatically — you never install core by hand for a React project.

## Dependency flow (the rule)

```mermaid
graph TD
    VR[Vanilla / Native TS App] --> Core[@bettertui/core]
    AR[React App] --> React[@bettertui/react]
    React --> Core
    Core -->|napi-rs FFI| Engine[bettertui-engine cdylib]
    Engine --> Term[(crossterm / portable-pty)]
    Core --> Shared[Internal: @bettertui/shared]
    React --> Shared

```

- `bettertui-engine` never imports any JS framework.
- `@bettertui/core` never imports React.
- Only `@bettertui/react` imports React.
- The only boundary between a UI framework and the engine is the **command** — so Vue/Solid/Svelte/vanilla-TS adapters require no Rust changes.

## Layer model

```mermaid
graph TD
    subgraph L1[App-Facing Packages — TypeScript]
        VR[Vanilla / Native TS App]
        C[@bettertui/core — first-class, framework-agnostic]
        AR[React App]
        R[@bettertui/react — first-class, depends on core]
    end
    subgraph L2[Type Foundation]
        S[@bettertui/shared — internal, re-exported]
    end
    subgraph L3[Rust Engine]
        E[bettertui-engine — lib + cdylib (bettertui_engine.node)]
    end
    subgraph L4[Terminal]
        T[crossterm / portable-pty]
    end
    VR --> C
    AR --> R
    R --> C
    C -->|napi-rs| E
    E --> T
    C --> S
    R --> S
```

## Rust engine: what it owns

All subsystems below are modules **inside the `bettertui-engine` crate** (`packages/core/crates/engine`). There is no separate `bettertui-widgets`, `bettertui-terminal`, or `bettertui-bindings` crate — the engine crate is itself the `cdylib` (with the `napi` feature) that produces `bettertui_engine.node`.

| Subsystem | Module | Implemented? |
|-----------|--------|--------------|
| Arena / node tree | `tree` | ✅ |
| Command protocol | `protocol` | ✅ |
| Renderer / frame production | `render` (`Renderer`, `AnsiBackend`, `Painter`), `framebuffer`, `dirty_diff` | ✅ |
| Frame buffer | `framebuffer` | ✅ |
| Dirty diff | `dirty_diff` | ✅ |
| Layout (Taffy) | `taffy` | ✅ |
| Events | `input` | ✅ |
| Input parsing | `input`, `ansi` | ✅ |
| Animation | `animation` | ✅ (callback path partial) |
| Scheduler | `scheduler` | ✅ |
| Capabilities | `terminal/capabilities.rs` (engine) | ✅ |
| Terminal I/O + VT | `terminal/` (engine: `mod.rs`, `vt.rs`, `screen.rs`, `scrollback.rs`, `query.rs`, `neovim.rs`) | ✅ |
| PTY / process | `pty.rs` (engine), `terminal/process.rs` (engine) | ✅ |
| Compositor / screen | `tree`/`graphics` (engine), `terminal/screen.rs` (engine) | ✅ |
| Text editing (rope) | `text` | ✅ |
| Widget host | `createWidgetHost` (native bridge) | ✅ |
| napi FFI surface | `napi` module (compiled with the `napi` feature) + `ffi` (C-ABI) | ✅ |

> Note: raw terminal-byte parsing lives in `input` and `ansi`; the C-ABI surface is in `ffi`; the napi surface is in `napi`. There are no top-level `keyboard/`, `mouse/`, `editor/`, `clipboard/`, or `selection/` modules — those concerns live inside `input.rs` and `text/`.

## TypeScript: what it owns

All TypeScript packages are currently `private` (not published to npm).

| Package | Role | Status |
|---------|------|--------|
| `@bettertui/core` | Framework package for vanilla / native TypeScript — command protocol, tree ops, `CommandRuntime`, native Rust bridge | ✅ |
| `@bettertui/react` | React 19 adapter — install **only** this for React apps (auto-depends on `@bettertui/core`) | ✅ reconciler/hooks real; 13 components |
| `@bettertui/shared` | Type foundation (zero runtime code) — **internal, do not install directly** — re-exported via `@bettertui/core` and `@bettertui/react` | ✅ |
| `@bettertui/devtools` | DevTools | ✅ Implemented | `createDevTools` factory (inspectors, logger, export helpers) |
| `@bettertui/benchmark` | Vitest benchmarks | ✅ |

## Architecture principles

1. **Framework first.** The engine is framework-agnostic; React is only the first adapter.
2. **Two framework packages.** `@bettertui/core` is the entry point for vanilla / native TypeScript. `@bettertui/react` is the entry point for React apps and depends on core — React users install only `@bettertui/react`.
3. **Rust owns rendering.** TypeScript never renders; it emits commands.
4. **No business logic in the engine.** It is a rendering/layout/input framework.
5. **Future adapters (Vue, Solid, Svelte, Astro, vanilla TS) must not require Rust changes.**

## Further reading

- [docs/architecture/ — full design reference](docs/architecture/README.md)
- [docs/ — documentation index](docs/README.md)
- [ROADMAP.md](ROADMAP.md) — current, code-accurate status
- [CONTRIBUTING.md](CONTRIBUTING.md)
