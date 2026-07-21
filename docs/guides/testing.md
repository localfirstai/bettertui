# Testing

BetterTUI is developed test-first. Behavior is described by tests before or alongside implementation, and every merge is gated by automated checks at two layers: **Rust** (engine + bindings) and **TypeScript** (Vitest + Biome + tsc).

There is **no `@bettertui/testing` package** and no snapshot/headless harness. Tests use [Vitest](https://vitest.dev/) directly; React output is verified through `renderToStringAsync` (see below).

## Test-driven development

- **Write the test first.** For an engine change or a new API surface, add a failing test that pins the expected behavior.
- **Rust:** unit tests live next to the code in `#[cfg(test)] mod tests`; the engine crate (`bettertui-engine`) carries the Rust test suite, run with `cargo test --manifest-path packages/core/Cargo.toml --lib`.
- **TypeScript:** co-locate tests beside source as `src/**/*.test.ts` / `*.test.tsx`. The glob is configured in the shared Vitest config.
- **Keep the suite green.** `pnpm test` and `cargo test --manifest-path packages/core/Cargo.toml --lib` must pass before a change is committed.
- **No TODO-driven dead code.** Track planned work in `tasks/`, not in the test or source tree.

## TypeScript

Configuration is shared from the repo root:

```ts
// vitest.shared.ts
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    environment: "node",
    include: ["src/**/*.test.{ts,tsx}"],
    globals: false,
  },
});
```

Each package merges the shared config and overrides what it needs (for example, `@bettertui/react` uses `jsdom` with a setup file and `globals: true`):

```ts
// packages/react/vitest.config.ts
import { defineProject, mergeConfig } from "vitest/config";
import configShared from "../../vitest.shared";

export default mergeConfig(
  configShared,
  defineProject({
    test: {
      name: "react",
      environment: "jsdom",
      globals: true,
      setupFiles: ["./src/__tests__/setup.ts"],
    },
  }),
);
```

Run the suite:

```bash
pnpm test            # turbo run test -> vitest run in each package (depends on ^build)
pnpm test --filter @bettertui/core   # a single package
```

Quality gates are run separately (Biome is the only TS formatter/linter; no Prettier/ESLint):

```bash
pnpm lint          # turbo run lint -> biome check src/
pnpm typecheck     # turbo run typecheck -> tsc --noEmit per package
pnpm format:check  # biome format check
```

## Rust

```bash
cargo test --manifest-path packages/core/Cargo.toml            # all engine tests
cargo test --manifest-path packages/core/Cargo.toml --lib      # engine unit tests
cargo clippy --manifest-path packages/core/Cargo.toml -- -D warnings
cargo fmt --all
```

The `--lib` flag matters: `packages/core/crates/engine/tests/integration_test.rs` has pre-existing integration failures that would otherwise block CI. The CI job separates the full run from the lib-target run.

## Testing React output

`@bettertui/react` ships a real `react-reconciler` host config. `packages/react/src/testing.ts` exposes `renderToStringAsync`, which mounts a React tree, drains the emitted `Command`s into the Rust engine, renders a full frame, and returns the ANSI output as a string. This is the canonical way to assert rendered output without a live terminal:

```ts
import { renderToStringAsync } from "@bettertui/react";
import { Box, Text } from "@bettertui/react";

const out = await renderToStringAsync(
  <Box><Text>hello</Text></Box>,
  { width: 80, height: 24 },
);
expect(out).toContain("hello");
```

Manual smoke scripts also exist under `scripts/` (`smoke-render.cjs`, `verify-content.cjs`) for ad-hoc verification of the render pipeline.

## Coverage

Coverage is enabled through the `v8` provider in the shared Vitest config (`vitest.shared.ts`). Run it per package:

```bash
pnpm test:coverage --filter @bettertui/core   # text + html report in coverage/
pnpm test --coverage                          # any package
```

The shared config excludes test files, `*.bench.ts`, `*.d.ts`, and `__tests__/` helpers from the report. `@vitest/coverage-v8` is a catalog dependency so every testable package can produce a report. Coverage from the most recent core work: 100% statements/functions/lines on `core/src`, with the only gaps in unreachable native-error paths.

## Pre-commit

`.husky/pre-commit` runs `biome check --write --staged` then `cargo fmt --all`, then `git update-index --again`. If formatting fails, the commit is aborted.

## Current coverage

| Layer | Status |
|-------|--------|
| Rust engine unit tests | Co-located in `bettertui-engine`; run with `cargo test --manifest-path packages/core/Cargo.toml --lib` |
| Clippy `-D warnings` | enforced in CI |
| rustfmt | clean |
| `pnpm build` (turbo) | all TS packages |
| `pnpm lint` (Biome) | all TS packages |
| `pnpm typecheck` (tsc) | all TS packages |
| `pnpm format:check` (Biome) | all TS packages |
| `pnpm test` (Vitest) | 5 TS packages covered by `*.test.ts(x)`; `benchmark` covered by `*.bench.ts` |

There is no `@bettertui/testing` package and no separate snapshot/headless harness.
