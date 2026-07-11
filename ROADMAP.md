# Roadmap

BetterTUI is built in layers. The Rust engine and its FFI bindings lead; the TypeScript framework bindings follow. This roadmap reflects what is implemented today and what remains.

## Status at a glance

| Layer | State | Notes |
|-------|-------|-------|
| Rust engine | Implemented | Rendering, layout, input, events, animation, text, PTY |
| napi-rs bindings | Implemented | `@bettertui/native` exposes the engine to Node.js |
| TypeScript core | Partial | Types and command model defined |
| React components | Stub | Component signatures exist; rendering not wired |
| Reconciler | In progress | Host config scaffolding present |
| Widgets | Interface only | Public API surface defined, not implemented |
| Examples | Scaffolded | Placeholder entry points |
| DevTools | Stub | Not implemented |

## Completed

- [x] Repository scaffolding (TurboRepo + pnpm workspace)
- [x] Rust workspace and engine modules
- [x] Node arena with generational indices
- [x] Taffy-based flexbox layout adapted to terminal cells
- [x] Double-buffered frame buffer with dirty-region diffing
- [x] ANSI encoding/decoding (CSI, OSC, DCS)
- [x] Keyboard, mouse, and paste input parsing
- [x] DOM-style event dispatch (capture → target → bubble)
- [x] Animation tween engine
- [x] Rope-based text engine with cursor, selection, undo/redo, search
- [x] Terminal capability detection
- [x] PTY runtime for embedded terminal processes
- [x] Nerd Font support
- [x] Compositor with layered output
- [x] napi-rs bindings exposing the engine to Node.js
- [x] Focus management
- [x] Clipboard integration
- [x] Architecture documentation

## In progress

- [ ] React reconciler host config producing commands
- [ ] React components emitting element descriptors
- [ ] Engine wrapper lifecycle (render loop, terminal raw mode)
- [ ] Input event → widget dispatch

## Planned

- [ ] Widget library (Box, Text, Input, List, Table, Tree, Tabs, Modal, …)
- [ ] Theme token system
- [ ] Icon registry with bundled icon sets
- [ ] Developer tools (inspector, profiler, error overlay)
- [ ] Example applications wired to the engine
- [ ] Additional framework adapters (Vue, Solid, Svelte, vanilla TypeScript)
- [ ] Plugin system
- [ ] Public API reference documentation per package

## Non-goals

BetterTUI is not an application, IDE, or AI framework. It does not ship a terminal emulator frontend, a code editor, or an agent runtime. Those are built by consumers on top of the engine.
