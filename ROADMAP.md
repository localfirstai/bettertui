# Roadmap

BetterTUI is built in layers. The Rust engine and its FFI bindings lead; the TypeScript framework bindings follow. This roadmap reflects what is implemented today and what remains.

## Status at a glance

| Layer | State | Notes |
|-------|-------|-------|
| Rust engine (`bettertui-engine`) | ✅ Implemented | 1,332 passing lib tests (verified via `cargo test --lib`); rendering, layout, input, events, animation, text, PTY, VT, capabilities |
| napi-rs bindings (`bettertui-bindings`) | ✅ Implemented | `NapiEngine`, `NapiEventBus`, `NapiFocusManager`, `NapiTextEngine`, `NapiScheduler`, `NapiCapabilities`, `getVersion`, `detectCapabilities` |
| `@bettertui/shared` | ✅ Implemented | Type-only foundation |
| `@bettertui/core` | ✅ Implemented | Command buffer, tree ops, reconciler wrapper, runtime |
| `@bettertui/core` (native bridge) | ✅ Implemented | Bridge + runtime + event loop (merged from `@bettertui/native`, requires native addon) |
| `@bettertui/themes` | 🟡 Partial | `defaultTheme` + `createTheme`; no preset themes |
| `@bettertui/react` (reconciler/hooks) | ✅ Implemented | Host config + `render()` + hooks |
| `@bettertui/react` (components) | 🟡 Thin wrappers | 53 component functions exported; emit element descriptors, not yet wired to live native render loop |
| `@bettertui/devtools` | ❌ Stub | `createDevTools` returns `null` |
| `@bettertui/benchmark` | ✅ Implemented | Vitest `bench` harness for TS packages |
| Examples | ✅ Wired | 15 example apps in `@bettertui/examples` (`examples/src/*.tsx`), launched via `node dist/index.mjs <slug>`) |
| Benchmarks | ✅ Implemented | `packages/benchmark` (Vitest `bench` harness) |
| `@bettertui/widgets` | 🔜 Proposed | Not a package yet; TS widget surface planned (Rust widgets exist) |
| `@bettertui/icons` | 🔜 Proposed | Not a package yet; icon registry planned (Phosphor preferred) |

## Completed

- [x] Repository scaffolding (TurboRepo + pnpm workspace, Cargo workspace)
- [x] Rust engine modules: tree (arena + generational indices), layout (Taffy), renderer, framebuffer, dirty diff, events, input (keyboard/mouse/paste), ansi parser, capabilities, animation, scheduler, compositor/screen, pty, terminal_process, terminal/vt, text (rope), widgets
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
- [x] `@bettertui/themes` default theme + factory

## In progress

- [ ] React components emitting element descriptors (currently stubs)
- [ ] Wiring `AnsiParser`/`VtMachine` into the production PTY read path
- [ ] Animation callback execution (`schedule_animation`/callbacks not yet firing)
- [ ] Engine runtime lifecycle fixes (frame timing, scheduler/engine frame counters)

## Planned

- [ ] Widget library on the TypeScript side (`@bettertui/widgets`)
- [ ] React `Terminal` component wrapping the embedded PTY
- [ ] Theme presets (light, high-contrast)
- [ ] Icon registry with bundled icon sets (Phosphor preferred per project taste)
- [ ] Developer tools (inspector, profiler, error overlay)
- [ ] Example applications wired to the engine (dashboard, mouse, table, text-editor, tree)
- [ ] Additional framework adapters (Vue, Solid, Svelte, vanilla TypeScript) — require no Rust changes
- [ ] Plugin system
- [ ] Per-package test coverage: expand Vitest suites in each package (mock input, snapshot assertions via `renderToStringAsync`); no separate `@bettertui/testing` package
- [ ] Benchmark harness (`benchmarks/`)
- [ ] Public API reference documentation per package (see `docs/api/`)

## Non-goals

BetterTUI is not an application, IDE, or AI framework. It does not ship a terminal emulator frontend, a code editor, or an agent runtime. Those are built by consumers on top of the engine.
