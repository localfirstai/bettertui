# Changelog

All notable changes to BetterTUI are documented here. This project adheres to [Semantic Versioning](https://semver.org/).

The project is pre-1.0; the public API is not yet stable.

## [Unreleased]

### Added
- napi-rs bindings exposing the Rust engine (`@bettertui/native`): engine, event bus, focus manager, text engine, scheduler, capability detection (`createEngine`, `createEventBus`, `createFocusManager`, `createTextEngine`, `createScheduler`, `detectCapabilities`, `getVersion`, `createRuntime`, `createEventLoop`).
- `@bettertui/core` framework-agnostic command buffer, tree operations, reconciler wrapper, and `Runtime`.
- React 19 adapter (`@bettertui/react`): `react-reconciler` host config, `render()`, `RuntimeProvider`/`useRuntime`, and hooks (`Provider`/`useTheme`, `FocusProvider`/`useFocus`, `useKeyboard`, `TerminalProvider`/`useTerminal`, `useFrame`, `useClipboard`, `useAnimation`).
- Rust engine subsystems: arena node model, command protocol, renderer, frame buffer, dirty diff, Taffy layout, events, input (keyboard/mouse/paste), ANSI parser, animation engine, scheduler, capability detection, terminal I/O + VT emulation, PTY runtime, compositor, rope-based text engine, widget framework.
- `@bettertui/themes` default theme and `createTheme()` factory.
- `@bettertui/icons` in-memory icon registry.
- `examples/counter` real implementation (`src/index.tsx`).

### Changed
- Architecture split: reconciler/runtime absorbed into `@bettertui/core` (framework-agnostic) and `@bettertui/react` (React-specific). `@bettertui/reconciler` and `@bettertui/runtime` packages removed.
- Documentation regenerated to reflect the implementation; obsolete Phase-0 design docs archived to `tasks/slop/docs/`.

### Fixed
- Husky pre-commit errors (deprecated shebang, node PATH).

## [0.0.0] - Initial scaffolding

### Added
- Rust workspace with rendering engine modules (tree, layout, render, framebuffer, terminal, input, events, animation, scheduler, graphics, protocol, selection, clipboard, editor, plugin, capabilities, ffi, benchmark, error).
- TypeScript packages: `core`, `shared`, `react`, `widgets`, `themes`, `icons`, `devtools`.
- TurboRepo + pnpm monorepo configuration.
- Biome for TypeScript/JSON and rustfmt + clippy for Rust.
- Husky pre-commit hooks.
- Architecture documentation under `docs/architecture`.
