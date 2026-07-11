# Changelog

All notable changes to BetterTUI are documented here. This project adheres to [Semantic Versioning](https://semver.org/).

The project is pre-1.0; the public API is not yet stable.

## [Unreleased]

### Added
- napi-rs bindings exposing the Rust engine (`@bettertui/native`): engine, event bus, focus manager, text engine, scheduler, renderer, compositor, PTY, and capability detection.
- Pane management system in the Rust engine.
- Keymap system, animation enhancements, and color improvements.
- Framework widget components (15+ widgets) in the Rust engine.
- Website homepage with features, code examples, and terminal demo.

### Changed
- Switch website icons to Phosphor icons.
- Centralize TypeScript version via pnpm catalog.

### Fixed
- Husky pre-commit errors (deprecated shebang, node PATH).

## [0.0.0] - Initial scaffolding

### Added
- Rust workspace with rendering engine modules (tree, layout, render, framebuffer, terminal, input, events, animation, scheduler, graphics, protocol, selection, clipboard, editor, plugin, capabilities, ffi, benchmark, error).
- TypeScript packages: `core`, `shared`, `reconciler`, `react`, `widgets`, `themes`, `icons`, `devtools`.
- TurboRepo + pnpm monorepo configuration.
- Biome for TypeScript/JSON and rustfmt + clippy for Rust.
- Husky pre-commit hooks.
- Architecture documentation under `docs/architecture`.
