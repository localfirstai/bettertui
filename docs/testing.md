# Testing

Testing is a first-class part of BetterTUI. The project follows **test-driven development (TDD)**: engine and API behavior are described by tests before or alongside implementation, and every change is gated by automated checks. See [Guides: Testing](guides/testing.md) for commands and the TDD workflow.

There is **no `@bettertui/testing` package**. Tests use [Vitest](https://vitest.dev/) directly. React component rendering is exercised through `renderToStringAsync` in `packages/react/src/testing.ts`, which drives the real reconciler against the Rust engine.

## Layers

| Layer | Tool | What it covers |
|-------|------|----------------|
| Rust engine | `cargo test` | Tree, layout, renderer, frame buffer, events, input, animation, text, PTY, terminal, capabilities |
| Rust fmt/lint | `cargo fmt` / `cargo clippy -D warnings` | Style and correctness |
| TypeScript | `vitest run` (per package) | Command buffer, reconciler, hooks, components, themes, shared types |
| TS fmt/lint/typecheck | Biome + `tsc --noEmit` | Style, lint, types |

## Current quality gates

| Gate | Result |
|------|--------|
| Rust engine unit tests | 1,204 lib tests passing |
| Clippy `-D warnings` | clean |
| rustfmt | clean |
| `pnpm build` (turbo) | 17/17 packages |
| `pnpm lint` (Biome) | 11/11 packages |
| `pnpm typecheck` (tsc) | 11/11 packages |
| `pnpm format:check` (Biome) | 10/10 packages |
| `pnpm test` (Vitest) | 6 TS packages with `.test.ts`/`.test.tsx` |

## TypeScript test inventory

| Package | Test files | Scope |
|---------|-----------|-------|
| `@bettertui/core` | 4 | `CommandBuffer`, `Runtime`, tree ops, native bridge |
| `@bettertui/shared` | 1 | Type definitions |
| `@bettertui/themes` | 1 | `defaultTheme`, `createTheme()` |
| `@bettertui/devtools` | 1 | `createDevTools()` |
| `@bettertui/react` | 7 | Public API, hooks, renderer, `renderToString`, runtime provider, easings |

## Notes

- `cargo test -p bettertui-engine --lib` excludes `packages/core/crates/engine/tests/` (pre-existing integration failures would otherwise block CI). The CI job separates the lib-target run from `cargo test --workspace`.
- Biome is the only TS formatter/linter (no Prettier/ESLint).
- There is no snapshot/headless harness package and no `benchmarks/` crate. Benchmarks live in `@bettertui/benchmark` via Vitest `bench`.
