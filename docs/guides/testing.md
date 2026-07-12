# Testing

BetterTUI verifies at two layers: Rust (engine + bindings) and TypeScript (Biome + tsc).

## Rust

```bash
cargo test --workspace                 # all tests (engine ~1071 passing)
cargo test -p bettertui-engine --lib   # engine unit tests only (excludes tests/ dir)
cargo clippy --workspace -- -D warnings
cargo fmt --all
```

The `--lib` flag matters: `packages/core/crates/engine/tests/integration_test.rs` has pre-existing integration failures that would otherwise block CI. The CI job separates `cargo test --workspace` from the lib-target run.

## TypeScript

```bash
pnpm lint          # turbo run lint -> biome check src/
pnpm typecheck     # turbo run typecheck -> tsc --noEmit per package
pnpm format:check  # biome format check
```

All TypeScript packages are ESM + `dts` via `tsup`. Biome is the only TS formatter/linter (no Prettier/ESLint). VCS integration is on, so `.gitignore` is respected.

## Pre-commit

`.husky/pre-commit` runs `biome check --write --staged` then `cargo fmt --all`, then `git update-index --again`. If formatting fails, the commit is aborted.

## Testing Utilities

The `@bettertui/testing` package provides utilities for testing BetterTUI applications:

```typescript
import {
  MockCommandCollector,
  createPoint,
  createRect,
  createMockHandler,
  expectCommandBuffer,
} from "@bettertui/testing";
```

### MockCommandCollector

A mock command collector that records commands for testing:

```typescript
const collector = new MockCommandCollector();
collector.commandBuffer.push({ type: "Shutdown" });
expect(collector.getCommands()).toHaveLength(1);
expect(collector.getLastCommand()?.type).toBe("Shutdown");
```

### createMockHandler

Creates a mock event handler that records calls:

```typescript
const handler = createMockHandler<(x: number) => void>();
handler(1);
handler(2);
expect(handler.calls).toHaveLength(2);
handler.clear();
expect(handler.calls).toHaveLength(0);
```

### expectCommandBuffer

Assertion helper for checking command buffer contents:

```typescript
const buffer = new CommandBuffer();
buffer.push({ type: "CreateNode", id: "1", kind: "Box" });
expectCommandBuffer(buffer, { length: 1, types: ["CreateNode"] });
```

## Coverage reality

| Layer | Status |
|-------|--------|
| Rust engine unit tests | ~1071 passing |
| Clippy `-D warnings` | clean |
| rustfmt | clean |
| pnpm build | 23/23 (excluding website) |
| pnpm lint | 12/12 |
| pnpm typecheck | 12/12 |
| pnpm format:check | 11/11 |

The `@bettertui/testing` package is now available for testing BetterTUI applications.
