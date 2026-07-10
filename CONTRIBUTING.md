# Contributing to BetterTUI

Thank you for your interest in contributing to BetterTUI.

## Development Setup

### Prerequisites

- Node.js >= 20
- pnpm >= 9
- Rust (stable, with `cargo` and `rustup`)
- napi CLI (`npm install -g @napi-rs/cli`)

### Getting Started

```bash
# Clone the repository
git clone https://github.com/bettertui/bettertui.git
cd bettertui

# Install dependencies
pnpm install

# Build all packages
pnpm build

# Run linting
pnpm lint

# Run type checking
pnpm typecheck
```

### Rust Development

```bash
# Check the engine compiles
cargo check --workspace

# Run tests
cargo test --workspace

# Format code
cargo fmt --all

# Lint
cargo clippy --workspace -- -D warnings
```

## Project Structure

- `native/engine/` — Rust rendering engine
- `native/bindings/` — napi-rs Node.js bindings
- `packages/` — TypeScript packages (core, reconciler, react, etc.)
- `examples/` — Example applications
- `docs/` — Documentation

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

### General

- Feature-first architecture
- No dead code — remove unused imports and variables
- No TODO comments in committed code
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
4. Ensure Rust checks pass: `cargo check --workspace && cargo test --workspace`
5. Submit a pull request with a clear description

## Reporting Issues

Use GitHub Issues to report bugs or request features. Include:

- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment details (OS, Node version, Rust version)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
