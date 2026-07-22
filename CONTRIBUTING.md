# Contributing to BetterTUI

## Development setup

### Prerequisites

- Node.js >= 24.15.0
- pnpm >= 9 (pinned to `pnpm@9.15.0`)
- Rust (stable, with `cargo` and `rustup`)

### Getting started

```bash
git clone https://github.com/bettertui/bettertui.git
cd bettertui
pnpm install
pnpm build
pnpm --filter @bettertui/core build:native
pnpm lint
pnpm typecheck
```

### Rust development

```bash
cargo check --manifest-path packages/core/Cargo.toml
cargo test --manifest-path packages/core/Cargo.toml --lib
cargo fmt --all
cargo clippy --manifest-path packages/core/Cargo.toml -- -D warnings
```

## Project structure

- `packages/core/crates/engine/` — Rust rendering engine (`bettertui-engine`, lib + cdylib)
- `packages/core/crates/benchmark/` — Rust benchmarks (`bettertui-benchmark`)
- `packages/` — TypeScript packages:
  - `@bettertui/core` — framework-agnostic public API
  - `@bettertui/react` — React 19 adapter
  - `@bettertui/solid` — **placeholder**
  - `@bettertui/shared` — type-only foundation (internal)
  - `@bettertui/performance` — Vitest benchmarks
  - `@bettertui/examples` — TypeScript examples
- `apps/website/` — Astro/Starlight docs site
- `docs/` — canonical documentation

## Coding standards

### TypeScript

- ES2022+
- No `any` — use `unknown` or proper types
- Prefer `type` over `interface` for simple shapes
- Use `const` assertions where appropriate
- All exports must be typed
- Biome for formatting and linting (no ESLint or Prettier)

### Rust

- Edition 2024
- No `unwrap()` in production code
- Use `parking_lot` for synchronization
- Prefer `smallvec` for small collections
- Public APIs must have `///` doc comments
- rustfmt for formatting, clippy for linting
- All structs with `new()` must derive or implement `Default`

### General

- Feature-first architecture
- No dead code or TODO comments in committed code (track work in `tasks/`)
- Keep functions small and focused
- Write descriptive commit messages

## Pre-commit hooks

Husky runs `biome check --write --staged` then `cargo fmt --all` then `git update-index --again`.

## Editor setup

Install **Biome** (`biomejs.biome`) and **Rust Analyzer** (`rust-lang.rust-analyzer`) VS Code extensions. Format on save with Biome for TS/JS/JSON and rust-analyzer for Rust.

## Pull request process

1. Create a feature branch from `main`
2. Make changes following code standards
3. Ensure `pnpm lint && pnpm typecheck && pnpm build` passes
4. Ensure `cargo check && cargo test --manifest-path packages/core/Cargo.toml --lib` passes
5. Submit a pull request with a clear description

## Reporting issues

Use GitHub Issues. Include steps to reproduce, expected behaviour, actual behaviour, and environment details.

## License

By contributing, you agree your contributions are licensed under MIT.
