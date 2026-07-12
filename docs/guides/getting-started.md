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
cargo build -p bettertui-bindings   # produces the bettertui_bindings addon
```

> `@bettertui/core`'s native bridge calls `require("bettertui_bindings")` at runtime. If you skip the `cargo build` step, native factories throw: `Failed to load native bindings. Run cargo build -p bettertui-bindings first.`

The Rust workspace is independent of pnpm; the TS `build` task does not compile Rust. Build the addon explicitly (or wire it into your app's build) before running anything that touches the native bridge (`@bettertui/core`'s native module at `packages/core/src/native/`).

## Useful scripts

| Script | Command |
|--------|---------|
| `pnpm lint` | `turbo run lint` (Biome) |
| `pnpm typecheck` | `turbo run typecheck` |
| `pnpm format:check` | Biome format check |
| `pnpm check` | lint + format:check + typecheck + `cargo:check` |
| `cargo test --workspace` | all Rust tests (engine has 1,204 lib tests) |
| `cargo clippy --workspace -- -D warnings` | lint, warnings are errors |
| `cargo fmt --all` | rustfmt |

## Running the examples

All 14 examples under `examples/` have real implementations in `src/index.tsx` (built on `@bettertui/core` + `@bettertui/react`). To run one:

```bash
cd examples/counter
pnpm exec tsdown src/index.tsx --format esm
node dist/index.mjs
```

The example suite covers fundamentals (`counter`, `hello-world`, `layouts`, `forms`, `tables`, `tree`, `terminal`) and showcases (`dashboard`, `widget-gallery`, `markdown-viewer`, `system-monitor`, `capability-inspector`, `performance-lab`, `terminal-showcase`).

> The React component functions are thin wrappers that emit element descriptors; the live native render loop is not yet connected, so running an example exercises the API surface and reconciler rather than painting pixels to the terminal.

## Architecture at a glance

```mermaid
graph TD
    App[Your app] --> React[@bettertui/react]
    React --> Core[@bettertui/core: CommandBuffer + Runtime]
    Core --> Native[core native bridge: load bettertui_bindings]
    Native --> Engine[Rust engine]
    Engine --> Term[crossterm / portable-pty]
```

## Next steps

- Read [Architecture Overview](../architecture/Overview.md) for the full layout.
- Read [Theming](../guides/theming.md) to style your UI.
- Read [Terminal & PTY](../guides/terminal.md) to embed a shell.
- See [Examples](../examples.md) for what is and isn't implemented yet.
