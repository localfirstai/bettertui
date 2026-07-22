# Roadmap

BetterTUI is built in layers. The Rust engine and its FFI bindings lead; the TypeScript framework bindings follow.

## Status at a glance

| Layer | State | Notes |
|-------|-------|-------|
| Rust engine (`bettertui-engine`) | ✅ Implemented | Rendering, layout (Taffy), frame buffer, events, input, animation, text, PTY, VT, capabilities, syntax highlighting, font/glyph support. All modules inside a single crate. |
| napi-rs bindings | ✅ Implemented | `napi` module of `bettertui-engine` exposes engine, event bus, focus manager, text engine, scheduler, keymap, capabilities, `getVersion`, `detectCapabilities` via `bettertui_engine.node` |
| `@bettertui/shared` | ✅ Implemented | Type-only foundation (internal, re-exported by core) |
| `@bettertui/core` | ✅ Implemented | Command buffer, reconciler, `CommandRuntime`, native bridge, DevTools, testing utilities, widgets |
| `@bettertui/react` | ✅ Implemented | React 19 adapter — `createRoot()`, reconciler, hooks (`useRuntime`, `useFocus`, `useKeyboard`, `useTheme`, `useTimeline`, `useTerminalDimensions`), runtime context, JSX types, 10 test files |
| `@bettertui/solid` | ⏳ Placeholder | `packages/solid` has stub structure — not implemented |
| `@bettertui/performance` | ✅ Implemented | Vitest benchmark suite |
| `@bettertui/examples` | ✅ Implemented | 64 TypeScript examples |

## In progress

- [ ] Animation callback execution (`schedule_animation()`/callbacks not yet firing)
- [ ] Engine runtime lifecycle fixes (frame timing, scheduler/engine frame counters)
- [ ] Wiring `AnsiParser`/`VtMachine` into the production PTY read path

## Planned

- [ ] `@bettertui/solid` adapter — SolidJS adapter
- [ ] Theme presets (light, high-contrast)
- [ ] Additional framework adapters (Vue, Svelte)
- [ ] Plugin system
- [ ] React `useAnimation()` hook
- [ ] `renderToStringAsync` for headless React component testing
- [ ] Expanded React component library
- [ ] Public npm releases

## Non-goals

BetterTUI is not an application, IDE, or AI framework. It does not ship a terminal emulator frontend, a code editor, or an agent runtime.
