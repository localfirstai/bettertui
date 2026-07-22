# Repository Overview

This document describes the physical and logical structure of the BetterTUI repository as it exists today.

## Workspaces

BetterTUI is a single repository managed by three workspace systems:

- **pnpm workspace** (`pnpm-workspace.yaml`) — declares packages at `apps/*` and `packages/*`
- **TurboRepo** (`turbo.json`) — orchestrates `build`, `dev`, `lint`, `format`, `format:check`, `typecheck`, `clean`
- **Cargo workspace** (`packages/core/Cargo.toml`, resolver = "2") — two Rust crates:
  - `crates/engine` → `bettertui-engine` (lib + cdylib + `layout_e2e` bin)
  - `crates/benchmark` → `bettertui-benchmark` (lib, `publish = false`)

The napi-rs bindings live inside `bettertui-engine` as the `napi.rs` module, compiled only with the `napi` feature. There is no separate `bettertui-bindings`, `bettertui-terminal`, or `bettertui-widgets` crate.

## Directory layout

```
bettertui/
├── packages/
│   ├── shared/        @bettertui/shared   — type-only foundation (internal)
│   ├── core/          @bettertui/core     — command protocol, reconciler, runtime, native bridge
│   │   └── crates/
│   │       ├── engine/       bettertui-engine (lib + cdylib + bin) — the native addon
│   │       └── benchmark/    bettertui-benchmark (Rust bench harness, publish = false)
│   ├── react/         @bettertui/react    — React 19 adapter (reconciler, hooks, components)
│   ├── solid/         @bettertui/solid    — SolidJS adapter (placeholder)
│   ├── performance/   @bettertui/performance — Vitest benchmark suite
│   └── examples/
│       └── typescript/  @bettertui/examples — 64 TypeScript examples
├── apps/
│   └── website/       @bettertui/website  — Astro/Starlight docs site
├── docs/              — this documentation
├── scripts/           — repo automation
├── tasks/             — PRDs, reports, archived history (read-only)
├── package.json
├── pnpm-workspace.yaml
├── turbo.json
├── biome.json
└── tsconfig.json
```

## TypeScript package layout

All packages are ESM-only, built with `tsdown` (`dts: true`), and export from a single `src/index.ts`. All are currently `private`.

| Package | `private` | Depends on | Role |
|---------|-----------|------------|------|
| `@bettertui/shared` | yes | — | Pure type definitions (zero runtime) — internal, re-exported by core |
| `@bettertui/core` | yes | `shared` | Framework-agnostic — command buffer, reconciler, `CommandRuntime`, native bridge, DevTools, widgets, keymap |
| `@bettertui/react` | yes | `core`, `shared` | React 19 adapter — `createRoot()`, reconciler, hooks, JSX types |
| `@bettertui/solid` | yes | `core`, `shared` | **Placeholder** — SolidJS adapter not implemented |
| `@bettertui/performance` | yes | `core` | Vitest benchmarks |
| `@bettertui/examples` | yes | `core` | TypeScript examples |

```
shared → core → react
shared → solid (placeholder)
core → performance
core → examples
```

## Rust crate layout

The Rust workspace root is `packages/core/Cargo.toml`:

| Crate | Type | Purpose |
|-------|------|---------|
| `bettertui-engine` | `lib` + `cdylib` + `bin` | Core engine: all rendering, layout, input, animation, text, terminal, PTY, font, syntax highlighting, protocol, scheduler. With the `napi` feature builds as `bettertui_engine.node` addon. |
| `bettertui-benchmark` | `lib` | Rust benchmark harness (`publish = false`) |

The engine is a single crate with mostly flat source files and subdirectories for `render/`, `text/`, `font/`, and `terminal/`. The napi surface is a module (`napi.rs`), not a separate crate.

## Build system

```bash
pnpm install                    # install dependencies
pnpm build                      # turbo run build (TypeScript packages)
pnpm --filter @bettertui/core build:native  # napi build of bettertui_engine.node
```

Key root scripts:

| Script | Command |
|--------|---------|
| `pnpm build` | `turbo run build` |
| `pnpm lint` | `turbo run lint` (Biome) |
| `pnpm typecheck` | `turbo run typecheck` |
| `pnpm format` / `format:check` | Biome format (cache disabled) |
| `pnpm check` | lint + format:check + typecheck + `cargo:check` |
| `pnpm cargo:check/test/fmt/clippy` | Cargo passthroughs |

The Rust addon (`bettertui_engine.node`) is not declared in any `package.json`. `@bettertui/core` loads it lazily at runtime and throws a clear error if not yet built.

## Architecture layers

```mermaid
graph TD
    VR[Vanilla / Native TS App] --> C[@bettertui/core]
    AR[React App] --> R[@bettertui/react]
    R --> C
    C -->|napi-rs| E[bettertui-engine]
    E --> Term[(Terminal / PTY)]
    C --> Shared[Internal: @bettertui/shared]
```

- **Layer 1 (App-facing).** `@bettertui/core` for vanilla TS; `@bettertui/react` for React.
- **Layer 2 (Core TypeScript).** Command protocol, reconciler, `CommandRuntime`, native bridge.
- **Layer 3 (Rust engine).** Single crate — arena, layout (Taffy), rendering, events, input, animation, text, PTY, terminal I/O, VT emulation, capability detection, widget host.
- **Layer 4 (Terminal).** Raw bytes via crossterm, child processes via portable-pty.
