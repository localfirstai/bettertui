# Testing

BetterTUI is developed test-first.

- **Rust:** unit tests live next to code in `#[cfg(test)] mod tests`; run with `cargo test --manifest-path packages/core/Cargo.toml --lib`
- **TypeScript:** co-locate tests as `src/**/*.test.ts`; run with `pnpm test`

## TypeScript

Configuration shared from `vitest.shared.ts`:

```bash
pnpm test            # turbo run test → vitest run in each package
pnpm test --filter @bettertui/core   # single package
pnpm lint            # biome check src/
pnpm typecheck       # tsc --noEmit per package
```

## Rust

```bash
cargo test --manifest-path packages/core/Cargo.toml            # all tests
cargo test --manifest-path packages/core/Cargo.toml --lib      # unit tests
cargo clippy --manifest-path packages/core/Cargo.toml -- -D warnings
cargo fmt --all
```

The `--lib` flag matters: integration tests in `crates/engine/tests/` have pre-existing failures.

## Coverage

Coverage via `@vitest/coverage-v8` (catalog dependency):

```bash
pnpm test:coverage --filter @bettertui/core
```

## Pre-commit

`.husky/pre-commit` runs `biome check --write --staged` then `cargo fmt --all`, then `git update-index --again`.
