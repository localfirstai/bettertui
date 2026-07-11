# Testing

See [Guides: Testing](guides/testing.md) for commands. Summary of the current quality gates:

| Gate | Result |
|------|--------|
| Rust engine unit tests | ~1071 passing |
| Clippy `-D warnings` | clean |
| rustfmt | clean |
| `pnpm build` | 17/17 |
| `pnpm lint` | 11/11 |
| `pnpm typecheck` | 11/11 |
| `pnpm format:check` | 10/10 |

## Notes

- `cargo test -p bettertui-engine --lib` excludes `native/engine/tests/` (pre-existing integration failures would block CI otherwise).
- Biome is the only TS formatter/linter.
- No `@bettertui/testing` package, no snapshot/headless harness, no `benchmarks/` implementation.
