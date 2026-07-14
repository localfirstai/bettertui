# Contributing to BetterTUI

Thank you for your interest in contributing to BetterTUI.

## Development Setup

### Prerequisites

- Node.js >= 24.15.0
- pnpm >= 9 (the repo pins `pnpm@9.15.0`)
- Rust (stable, with `cargo` and `rustup`)
- napi CLI is **not** required — the native build uses Cargo build scripts

### Getting Started

```bash
# Clone the repository
git clone https://github.com/bettertui/bettertui.git
cd bettertui

# Install dependencies
pnpm install

# Build all TypeScript packages (does NOT compile Rust)
pnpm build

# Build the native Rust addon (required before running anything native)
cargo build -p bettertui-bindings --manifest-path packages/core/Cargo.toml

# Run linting
pnpm lint

# Run type checking
pnpm typecheck
```

### Rust Development

```bash
# Check the engine compiles
cargo check --workspace

# Run tests (720 Rust lib tests across engine/terminal/widgets crates, use --lib to skip integration tests)
cargo test -p bettertui-engine --lib

# Format code
cargo fmt --all

# Lint
cargo clippy --workspace -- -D warnings
```

## Project Structure

- `packages/core/crates/engine/` — Rust rendering engine (`bettertui-engine`, library)
- `packages/core/crates/bindings/` — napi-rs Node.js bindings (`bettertui-bindings`, cdylib)
- `packages/` — TypeScript packages (`shared`, `core`, `react`, `themes`, `devtools`, `benchmark`).
- `apps/website/` — Astro/Starlight docs + landing site (not part of the framework)
- `examples/` — 15 example apps (built on `@bettertui/core` + `@bettertui/react`, launched via the interactive launcher)
- `docs/` — the documentation you are reading (canonical source of truth)

## Test-Driven Development

BetterTUI is built test-first. Tests describe behavior before or alongside implementation, and every change is gated by automated checks. There is **no `@bettertui/testing` package** and no snapshot/headless harness — tests use [Vitest](https://vitest.dev/) directly, and React output is verified through `renderToStringAsync` in `packages/react/src/testing.ts`.

- **Write the test first.** For an engine change or a new API, add a failing test that pins the expected behavior.
- **Rust:** unit tests live next to the code in `#[cfg(test)] mod tests` within the engine, terminal, and widgets crates (720 lib tests in total, verified via `cargo test --lib`).
- **TypeScript:** co-locate tests as `src/**/*.test.ts` / `*.test.tsx` (see `vitest.shared.ts`); run them with `pnpm test`.
- **Keep the suite green.** Run `pnpm lint && pnpm typecheck && pnpm build && pnpm test` and `cargo test -p bettertui-engine --lib` before opening a PR.
- **No dead code or TODOs in committed source.** Track planned work in `tasks/`.

For commands and the full testing workflow, see [docs/guides/testing.md](docs/guides/testing.md) and [docs/testing.md](docs/testing.md).

## Code Standards

### TypeScript

- Use modern TypeScript (ES2022+)
- No `any` types — use `unknown` or proper types
- Prefer `type` over `interface` for simple shapes
- Use `const` assertions where appropriate
- All exports must be typed
- Formatting and linting enforced by **Biome** (no ESLint or Prettier)

### Rust

- Edition 2024
- No `unwrap()` in production code — use proper error handling
- Use `parking_lot` for synchronization primitives
- Prefer `smallvec` for small collections
- Document public APIs with `///` doc comments
- Formatting enforced by **rustfmt**, linting by **clippy**
- All structs with `new()` must derive or implement `Default` (clippy requirement)

### General

- Feature-first architecture
- No dead code — remove unused imports and variables
- No TODO comments in committed code (track work in `tasks/`)
- Keep functions small and focused
- Write descriptive commit messages

## Pre-commit Hooks

This repository uses **Husky** to automatically format code before every commit:

```bash
# TypeScript/JS/JSON: formatted with Biome (lint + organize imports)
# Rust: formatted with rustfmt
# Modified files are automatically re-staged
```

If pre-commit formatting fails, the commit is aborted. Fix the reported issues and try again.

## Editor Setup

### VS Code (recommended)

Install these extensions:

- **Biome** (`biomejs.biome`) — formatter and linter for TypeScript/JS/JSON
- **Rust Analyzer** (`rust-lang.rust-analyzer`) — Rust language support

Configure format on save in `.vscode/settings.json`:

```json
{
  "editor.formatOnSave": true,
  "[javascript]": { "editor.defaultFormatter": "biomejs.biome" },
  "[typescript]": { "editor.defaultFormatter": "biomejs.biome" },
  "[typescriptreact]": { "editor.defaultFormatter": "biomejs.biome" },
  "[json]": { "editor.defaultFormatter": "biomejs.biome" },
  "[jsonc]": { "editor.defaultFormatter": "biomejs.biome" },
  "[rust]": { "editor.defaultFormatter": "rust-lang.rust-analyzer" }
}
```

## Pull Request Process

1. Create a feature branch from `main`
2. Make your changes following the code standards
3. Ensure all checks pass: `pnpm lint && pnpm typecheck && pnpm build`
4. Ensure Rust checks pass: `cargo check --workspace && cargo test -p bettertui-engine --lib`
5. Submit a pull request with a clear description

## Reporting Issues

Use GitHub Issues to report bugs or request features. Include:

- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment details (OS, Node version, Rust version)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
IT License.
