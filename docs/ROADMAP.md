# Roadmap

BetterTUI is built in layers. The Rust engine and its FFI bindings lead; the TypeScript framework bindings follow. This roadmap reflects what is implemented today and what remains.

## Status at a glance

| Layer | State | Notes |
|-------|-------|-------|
| Rust engine (`bettertui-engine`) | ✅ Implemented | rendering, layout (Taffy), frame buffer, events, input, animation, text, PTY, VT, capabilities. Terminal I/O, VT, PTY, capabilities, and the widget host are modules inside this single crate (no separate `bettertui-widgets`/`bettertui-terminal`/`bettertui-bindings` crates). |
| napi-rs bindings | ✅ Implemented | The `napi` module of `bettertui-engine` (built with the `napi` feature) exposes `NapiEngine`, `NapiEventBus`, `NapiFocusManager`, `NapiTextEngine`, `NapiScheduler`, `NapiKeymap`, `NapiCapabilities`, `getVersion`, `detectCapabilities` via the `bettertui_engine.node` addon. |
| `@bettertui/shared` | ✅ Implemented | Type-only foundation |
| `@bettertui/core` | ✅ Implemented | Command buffer, tree ops, reconciler wrapper, `CommandRuntime`, native bridge |
| `@bettertui/core` (testing utilities) | ✅ Implemented | `createTestRenderer`, `createMockKeys`, `createMockMouse`, test streams, terminal capabilities mocks |
| `@bettertui/core` (debug tooling) | ✅ Implemented | In-core `createDevTools()` factory (inspectors, logger, export helpers) + debug overlay (`CliRenderer` `debug` option) |
| `@bettertui/benchmark` | ✅ Implemented | Vitest `bench` harness for TS packages |
| `@bettertui/react` | ⏳ Not implemented | `packages/react` is a placeholder directory. The React adapter (host config, `render()`, hooks, components) is planned but not written. |
| `@bettertui/solid` | ⏳ Not implemented | `packages/solid` is a placeholder directory. The SolidJS adapter is planned but not written. |

## Completed

- [x] Repository scaffolding (TurboRepo + pnpm workspace, Cargo workspace)
- [x] Rust engine modules: tree (arena + generational indices), layout (Taffy), render pipeline, framebuffer, dirty diff, input (keyboard/mouse/paste), ansi parser, capabilities (engine `terminal/` modules), animation, scheduler, compositor/screen (tree/graphics + engine `terminal/`), pty, terminal process, vt emulation, text (rope), widget host
- [x] napi-rs bindings exposing the engine to Node.js (the engine's `napi` module, built as `bettertui_engine.node`)
- [x] Node arena with generational indices (`NodeId = slotmap::DefaultKey`, 8 bytes)
- [x] Taffy-based flexbox layout adapted to terminal cells
- [x] Double-buffered frame buffer with dirty-region diffing
- [x] ANSI encoding/decoding (CSI, OSC, DCS)
- [x] Keyboard, mouse, and paste input parsing (Kitty / CSI-u supported)
- [x] DOM-style event dispatch (capture → target → bubble)
- [x] Animation tween/spring/keyframe engine
- [x] Rope-based text engine with cursor, selection, undo/redo, search
- [x] Terminal capability detection
- [x] PTY runtime for embedded terminal processes
- [x] VT emulation state machine (`VtMachine`)
- [x] Nerd Font support
- [x] Compositor with layered output
- [x] Focus management
- [x] Clipboard capability detection (OSC 52/8)
- [x] `@bettertui/core` command buffer + runtime
- [x] `@bettertui/core` native bridge (factories, runtime, event loop; merged from `@bettertui/native`)
- [x] `@bettertui/core` testing utilities (`createTestRenderer`, `createMockKeys`, `createMockMouse`, test streams, terminal capabilities mocks)
- [x] `@bettertui/examples` — interactive CLI example browser using the native Rust engine directly (no React)

## In progress

- [x] In-core debug tooling — `createDevTools()` factory (in `@bettertui/core`) with command/event/performance/tree/scheduler/focus/capability inspectors, timeline, snapshot manager, export helpers, and a `CliRenderer` debug overlay
- [ ] Wiring `AnsiParser`/`VtMachine` into the production PTY read path
- [ ] Animation callback execution (`schedule_animation`/callbacks not yet firing)
- [ ] Engine runtime lifecycle fixes (frame timing, scheduler/engine frame counters)

## Planned

- [ ] `@bettertui/react` adapter — `react-reconciler` host config, `render()`, hooks, and components (when built, React apps install only `@bettertui/react`, which depends on `@bettertui/core`)
- [ ] `@bettertui/solid` adapter — SolidJS adapter (when built, SolidJS apps install only `@bettertui/solid`, which depends on `@bettertui/core`)
- [ ] React `Terminal` component wrapping the embedded PTY
- [ ] Theme presets (light, high-contrast)
- [ ] Example applications wired to the engine (dashboard, mouse, table, text-editor, tree)
- [ ] Additional framework adapters (Vue, Svelte, vanilla TypeScript) — require no Rust changes
- [ ] Plugin system
- [ ] Per-package test coverage: expand Vitest suites in each package (mock input, snapshot assertions); no separate `@bettertui/testing` package
- [ ] Public API reference documentation per package (see `api/packages/`)

## Non-goals

BetterTUI is not an application, IDE, or AI framework. It does not ship a terminal emulator frontend, a code editor, or an agent runtime. Those are built by consumers on top of the engine.
