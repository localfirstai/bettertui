# Changelog

All notable changes to BetterTUI are documented here. This project adheres to [Semantic Versioning](https://semver.org/).

The project is pre-1.0; the public API is not yet stable.

## [Unreleased]

### Added
- napi-rs bindings exposing the Rust engine (originally `@bettertui/native`, now internal to `@bettertui/core`): engine, event bus, focus manager, text engine, scheduler, capability detection (`createEngine`, `createEventBus`, `createFocusManager`, `createTextEngine`, `createScheduler`, `detectCapabilities`, `getVersion`, `createRuntime`, `createEventLoop`).
- `@bettertui/core` framework-agnostic command buffer, tree operations, reconciler wrapper, native bridge, and `Runtime`.
- React 19 adapter (`@bettertui/react`): `react-reconciler` host config, `render()`, `RuntimeProvider`/`useRuntime`, hooks (`Provider`/`useTheme`, `FocusProvider`/`useFocus`, `useKeyboard`, `useMouse`, `TerminalProvider`/`useTerminal`, `useFrame`, `useClipboard`, `useAnimation`, `useTimeline`, `SelectionProvider`/`useSelection`, `CapabilitiesProvider`/`useCapabilities`), and 69 component functions (thin wrappers emitting element descriptors).
- Rust engine subsystems: arena node model, command protocol, renderer, frame buffer, dirty diff, Taffy layout, events, input (keyboard/mouse/paste), ANSI parser, animation engine (tween/spring/keyframe), scheduler, capability detection, terminal I/O + VT emulation, PTY runtime, compositor, rope-based text engine, widget framework.
- `@bettertui/themes` default theme and `createTheme()` factory.
- `@bettertui/benchmark` Vitest benchmark harness for the TypeScript packages.
- 14 example applications under `examples/fundamentals/*` and `examples/showcase/*`.
- Full keymap engine in `@bettertui/core` (`Keymap` class: layered bindings, chord sequences, modes, named commands, pre/post key intercepts, and event subscriptions) plus a React binding suite in `@bettertui/react` (`KeymapProvider`, `useKeymap`, `useKeymapEvent`, `useActiveBindings`, `usePendingSequence`, `useCommand`, `useKeyIntercept`, `useKeymapMode`). Provides 100% OpenTUI parity on key handling.

### Changed
- Architecture split: reconciler/runtime absorbed into `@bettertui/core` (framework-agnostic) and `@bettertui/react` (React-specific). `@bettertui/reconciler` and `@bettertui/runtime` packages removed.
- `@bettertui/native` package merged into `@bettertui/core` as an internal native bridge; Cargo workspace root moved to `packages/core/Cargo.toml`.
- Documentation regenerated to reflect the implementation; obsolete Phase-0 design docs and AI iteration reports archived to `tasks/slop/`.

### Fixed
- Husky pre-commit errors (deprecated shebang, node PATH).

## [0.0.0] - Initial scaffolding

### Added
- Rust workspace with rendering engine modules (tree, layout, render, framebuffer, terminal, input, events, animation, scheduler, graphics, protocol, selection, clipboard, editor, plugin, capabilities, ffi, benchmark, error).
- TypeScript packages: `core`, `shared`, `react`, `themes`, `devtools`, `benchmark` (testing is per-package via Vitest, no `@bettertui/testing` package).
- TurboRepo + pnpm monorepo configuration.
- Biome for TypeScript/JSON and rustfmt + clippy for Rust.
- Husky pre-commit hooks.
- Architecture documentation under `docs/architecture`.
