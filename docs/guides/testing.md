# Testing

BetterTUI verifies at two layers: Rust (engine + bindings) and TypeScript (Biome + tsc).

## Rust

```bash
cargo test --workspace                 # all tests (engine ~1071 passing)
cargo test -p bettertui-engine --lib   # engine unit tests only (excludes tests/ dir)
cargo clippy --workspace -- -D warnings
cargo fmt --all
```

The `--lib` flag matters: `native/engine/tests/integration_test.rs` has pre-existing integration failures that would otherwise block CI. The CI job separates `cargo test --workspace` from the lib-target run.

## TypeScript

```bash
pnpm lint          # turbo run lint -> biome check src/
pnpm typecheck     # turbo run typecheck -> tsc --noEmit per package
pnpm format:check  # biome format check
```

All TypeScript packages are ESM + `dts` via `tsup`. Biome is the only TS formatter/linter (no Prettier/ESLint). VCS integration is on, so `.gitignore` is respected.

## Pre-commit

`.husky/pre-commit` runs `biome check --write --staged` then `cargo fmt --all`, then `git update-index --again`. If formatting fails, the commit is aborted.

## Coverage reality

| Layer | Status |
|-------|--------|
| Rust engine unit tests | ~1071 passing |
| Clippy `-D warnings` | clean |
| rustfmt | clean |
| pnpm build | 17/17 |
| pnpm lint | 11/11 |
| pnpm typecheck | 11/11 |
| pnpm format:check | 10/10 |

There is **no** dedicated `@bettertui/testing` package yet (proposed in the architecture), and `benchmarks/` is an empty placeholder — no Criterion/benchmark harness is implemented.
