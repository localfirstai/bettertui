# @bettertui/benchmark

Vitest benchmark harness for BetterTUI's TypeScript packages.

## What's inside

Benchmarks that measure the cost of core operations (command buffer, tree ops, rendering helpers) across `@bettertui/core` and `@bettertui/shared`.

## Running

```bash
pnpm bench         # vitest bench (watch)
pnpm bench:run     # vitest bench --run (CI)
```

Benchmarks are defined with Vitest's `bench` API; see `src/**/*.bench.ts`.

## Status

Implemented. This is a TS benchmark harness only — there is no `benchmarks/` Rust crate.

See [`docs/testing.md`](../../docs/testing.md).
