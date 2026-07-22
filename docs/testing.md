# Testing

Testing is a first-class part of BetterTUI. The project follows test-driven development. See [Testing guide](guides/testing.md) for commands and workflow.

There is **no `@bettertui/testing` package**. Tests use [Vitest](https://vitest.dev/) directly.

## Layers

| Layer | Tool | Coverage |
|-------|------|----------|
| Rust engine | `cargo test` | Tree, layout, renderer, frame buffer, events, input, animation, text, PTY, terminal, VT emulation, capabilities |
| Rust fmt/lint | `cargo fmt` / `cargo clippy -D warnings` | Style and correctness |
| TypeScript | `vitest run` | Command buffer, reconciler, runtime, hooks, validation |
| TS fmt/lint/typecheck | Biome + tsc | Style, lint, types |

## Quality gates

| Gate | Status |
|------|--------|
| Rust engine unit tests | Co-located in `bettertui-engine`; run `cargo test --manifest-path packages/core/Cargo.toml` |
| Clippy `-D warnings` | clean |
| rustfmt | clean |
| `pnpm build` (turbo) | all TS packages |
| `pnpm lint` (Biome) | all TS packages |
| `pnpm typecheck` (tsc) | all TS packages |
| `pnpm test` (Vitest) | TS packages with `*.test.ts(x)` |

## Known issues

- The Rust test build currently has a compilation issue in `terminal/vt.rs` (missing import in test code). Run library tests with `cargo test --manifest-path packages/core/Cargo.toml --lib`.
- There is no `renderToStringAsync` utility — React component testing is done through the reconciler's host config and unit tests.
- There is no snapshot/headless harness package.
