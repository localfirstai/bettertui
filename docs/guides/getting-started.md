# Getting Started

This guide gets you from a clone to a running example. It reflects the current build, including the Rust-native addon requirement.

## Prerequisites

- Node.js >= 20
- pnpm >= 9 (`packageManager` is pinned to `pnpm@9.15.0`)
- Rust stable (cargo + rustup)
- napi CLI is **not** required — the build uses the local `@napi-rs/cli` via Cargo build scripts

## Install & build

```bash
pnpm install
pnpm build            # turbo run build across all TS packages
pnpm --filter @bettertui/core build:native   # produces the bettertui_engine.node addon
```

> `@bettertui/core`'s native bridge calls `require("bettertui_engine")` at runtime. If you skip the native build step, native factories throw: `Failed to load native bindings. Run pnpm --filter @bettertui/core build:native first.`

The Rust workspace is independent of pnpm; the TS `build` task does not compile Rust. Build the addon explicitly (or wire it into your app's build) before running anything that touches the native bridge (`@bettertui/core`'s engine module at `packages/core/src/platform/`).

## Useful scripts

| Script | Command |
|--------|---------|
| `pnpm lint` | `turbo run lint` (Biome) |
| `pnpm typecheck` | `turbo run typecheck` |
| `pnpm format:check` | Biome format check |
| `pnpm check` | lint + format:check + typecheck + `cargo:check` |
| `cargo test --manifest-path packages/core/Cargo.toml --lib` | engine library tests (co-located in `bettertui-engine`) |
| `cargo clippy --workspace -- -D warnings` | lint, warnings are errors |
| `cargo fmt --all` | rustfmt |

## Running the examples

Vanilla / native TypeScript examples live in `examples/vanila/` and run directly on `@bettertui/core` via the native `CliRenderer`. Build the native addon first, then run the launcher:

```bash
pnpm --filter @bettertui/core build:native
pnpm --filter @bettertui/examples-vanila dev            # interactive browser
pnpm --filter @bettertui/examples-vanila dev hello-world # run a single example by slug
```

The vanilla examples under `examples/vanila/` (e.g. `hello-world`, `flex-layout`, `colors`, `capabilities`, `keyboard`, `performance`, `select-demo`) run directly on `@bettertui/core` and the native engine.

> The React component functions are thin wrappers that emit element descriptors; the live native render loop is not yet connected, so running an example exercises the API surface and reconciler rather than painting pixels to the terminal.

## Two ways to use BetterTUI

```
Vanilla / Native TS App ─┐
                         ├─▶ @bettertui/core ──(napi-rs FFI)──▶ Rust Engine (bettertui_engine.node)
React App ─▶ @bettertui/react ───────────────┘
                                                       │
                                                       ▼
                                         (Terminal / PTY via crossterm + portable-pty)
```

- **React app:** `npm install @bettertui/react` (this pulls in `@bettertui/core` automatically — you never install core by hand).
- **Vanilla / native TypeScript app:** `npm install @bettertui/core` and use the command protocol, runtime, and native bridge directly.

## Next steps

- Read [Architecture Overview](../architecture/overview.md) for the full layout.
- Read [Theming](../guides/theming.md) to style your UI.
- Read [Terminal & PTY](../guides/terminal.md) to embed a shell.
- See [Examples](../examples.md) for what is and isn't implemented yet.
