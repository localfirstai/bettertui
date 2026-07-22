# Changelog

All notable changes to BetterTUI are documented here. This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

The project is pre-1.0; the public API is not yet stable.

## [Unreleased]

### Added
- napi-rs bindings exposing the Rust engine (internal to `@bettertui/core`): `NapiEngine`, `NapiEventBus`, `NapiFocusManager`, `NapiTextEngine`, `NapiScheduler`, `NapiKeymap`, `NapiCapabilities`, `detectCapabilities`, `getVersion`, `createRuntime`, `createEventLoop`
- `@bettertui/core` framework-agnostic command buffer, tree operations, reconciler wrapper, `CommandRuntime`, native bridge, DevTools, testing utilities, keymap
- `@bettertui/react` React 19 adapter: `createRoot()`, reconciler, hooks (`useRuntime`, `useFocus`, `useKeyboard`, `useTheme`, `useTimeline`, `useTerminalDimensions`), runtime context, JSX types
- `@bettertui/solid` — placeholder directory (not implemented)
- `@bettertui/shared` — type-only foundation
- `@bettertui/performance` — Vitest benchmark suite
- `@bettertui/examples` — 64 TypeScript examples running on `@bettertui/core`
- Rust engine subsystems: arena node model, command protocol, renderer, frame buffer, dirty diff, Taffy layout, events, input (keyboard/mouse/paste), ANSI parser, animation engine (tween/spring/keyframe), scheduler, capability detection, terminal I/O + VT emulation, PTY runtime, compositor, rope-based text engine, widget framework, tree-sitter syntax highlighting, font/glyph support

### Changed
- Architecture split: reconciler/runtime absorbed into `@bettertui/core` (framework-agnostic) and `@bettertui/react` (React-specific). `@bettertui/reconciler` and `@bettertui/runtime` packages removed.
- `@bettertui/native` package merged into `@bettertui/core` as an internal native bridge; Cargo workspace root moved to `packages/core/Cargo.toml`
- Cargo workspace simplified to 2 members (engine, benchmark) — no separate logger/bindings/terminal/widgets crate

### Fixed
- Husky pre-commit errors (deprecated shebang, node PATH)
