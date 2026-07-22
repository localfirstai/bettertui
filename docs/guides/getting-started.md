# Getting Started

## Prerequisites

- Node.js >= 20
- pnpm >= 9 (pinned to `pnpm@9.15.0`)
- Rust stable (cargo + rustup)

## Install & build

```bash
pnpm install
pnpm build
pnpm --filter @bettertui/core build:native   # produces bettertui_engine.node
```

Without the native addon, native factories throw: `Failed to load native bindings. Run pnpm --filter @bettertui/core build:native first.`

## Useful scripts

| Script | Command |
|--------|---------|
| `pnpm lint` | `turbo run lint` (Biome) |
| `pnpm typecheck` | `turbo run typecheck` |
| `pnpm format:check` | Biome format check |
| `pnpm check` | lint + format:check + typecheck + `cargo:check` |
| `cargo test --manifest-path packages/core/Cargo.toml --lib` | engine unit tests |

## Running examples

```bash
pnpm --filter @bettertui/core build:native
pnpm --filter @bettertui/examples dev            # interactive browser
pnpm --filter @bettertui/examples dev <slug>     # single example
```

## Two ways to use BetterTUI

```
Vanilla / Native TS App ──▶ @bettertui/core ──▶ Rust Engine (bettertui_engine.node)
React App ──▶ @bettertui/react ──▶ @bettertui/core (auto-resolved)
```

- **React app:** `npm install @bettertui/react` (auto-pulls `@bettertui/core`)
- **Vanilla TypeScript app:** `npm install @bettertui/core`

## Next steps

- [Architecture Overview](../architecture/overview.md)
- [Theming guide](theming.md)
- [Testing guide](testing.md)
- [Terminal & PTY guide](terminal.md)
