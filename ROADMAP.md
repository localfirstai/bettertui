# Roadmap

BetterTUI is built in layers. The Rust engine and its FFI bindings lead; the TypeScript framework bindings follow. This roadmap reflects what is implemented today and what remains.

## Status at a glance

| Layer | State | Notes |
|-------|-------|-------|
| Rust engine (`bettertui-engine`) | ✅ Implemented | 720 passing Rust lib tests (verified via `cargo test --lib`) across the engine, terminal, and widgets crates; rendering, layout, input, events, animation, text, PTY, VT, capabilities |
| napi-rs bindings (`bettertui-bindings`) | ✅ Implemented | `NapiEngine`, `NapiEventBus`, `NapiFocusManager`, `NapiTextEngine`, `NapiScheduler`, `NapiCapabilities`, `getVersion`, `detectCapabilities` |
| `@bettertui/shared` | ✅ Implemented | Type-only foundation |
| `@bettertui/core` | ✅ Implemented | Command buffer, tree ops, reconciler wrapper, runtime, native bridge |
| `@bettertui/core` (testing utilities) | ✅ Implemented | `createTestRenderer`, `createMockKeys`, `createMockMouse`, test streams, terminal capabilities mocks |
| `@bettertui/react` (reconciler/hooks) | ✅ Implemented | Host config + `render()` + hooks |
| `@bettertui/react` (components) | 🟡 Thin wrappers | 53 component functions exported; emit element descriptors, not yet wired to live native render loop |
| `@bettertui/devtools` | ✅ Implemented | `createDevTools()` factory (inspectors, logger, export helpers) |
| `@bettertui/benchmark` | ✅ Implemented | Vitest `bench` harness for TS packages |
| `examples/vanila` | ✅ Implemented | Interactive CLI example browser using native Rust engine directly via `@bettertui/core` (no React) |
| Examples (React) | ✅ Wired | 15 example apps in `@bettertui/examples` (`examples/src/examples/<category>/<slug>.tsx`), launched via `node dist/index.mjs <slug>`) |
| Benchmarks | ✅ Implemented | `packages/benchmark` (Vitest `bench` harness) |

## Completed

- [x] Repository scaffolding (TurboRepo + pnpm workspace, Cargo workspace)
- [x] Rust engine modules: tree (arena + generational indices), layout (Taffy), render pipeline, framebuffer, dirty diff, input (keyboard/mouse/paste), ansi parser, capabilities (terminal crate), animation, scheduler, compositor/screen (tree/graphics + terminal crate), pty, terminal process, vt emulation, text (rope), widgets
- [x] napi-rs bindings exposing the engine to Node.js (`bettertui_bindings`)
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
- [x] React reconciler host config producing commands
- [x] `@bettertui/core` command buffer + runtime
- [x] `@bettertui/core` native bridge (factories, runtime, event loop; merged from `@bettertui/native`)
- [x] `@bettertui/core` testing utilities (`createTestRenderer`, `createMockKeys`, `createMockMouse`, test streams, terminal capabilities mocks)
- [x] `examples/vanila` - Interactive CLI example browser using native Rust engine directly (no React)


## In progress

- [x] `@bettertui/devtools` — `createDevTools()` factory with command/event/performance/tree/scheduler/focus/capability inspectors, timeline, snapshot manager, and export helpers
- [ ] Wiring `AnsiParser`/`VtMachine` into the production PTY read path
- [ ] Animation callback execution (`schedule_animation`/callbacks not yet firing)
- [ ] Engine runtime lifecycle fixes (frame timing, scheduler/engine frame counters)

## Planned

- [ ] React `Terminal` component wrapping the embedded PTY
- [ ] Theme presets (light, high-contrast)
- [ ] Developer tools (inspector, profiler, error overlay)
- [ ] Example applications wired to the engine (dashboard, mouse, table, text-editor, tree)
- [ ] Additional framework adapters (Vue, Solid, Svelte, vanilla TypeScript) — require no Rust changes
- [ ] Plugin system
- [ ] Per-package test coverage: expand Vitest suites in each package (mock input, snapshot assertions via `renderToStringAsync`); no separate `@bettertui/testing` package
- [ ] Benchmark harness (`benchmarks/`)
- [ ] Public API reference documentation per package (see `docs/api/`)

## Non-goals

BetterTUI is not an application, IDE, or AI framework. It does not ship a terminal emulator frontend, a code editor, or an agent runtime. Those are built by consumers on top of the engine.
