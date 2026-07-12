# @bettertui/benchmark

Performance benchmarks for BetterTUI TypeScript packages.

## Usage

```bash
# Run all benchmarks
pnpm vitest bench --run

# Run in watch mode
pnpm vitest --watch=false
```

## Adding Benchmarks

Create files with `.bench.ts` suffix in the `src/` directory:

```typescript
import { describe, it, bench } from "vitest";

describe("My Benchmark", () => {
  bench("operation", () => {
    // Code to benchmark
  });
});
```

## What We Benchmark

- Command buffer operations
- Tree manipulation (create, append, remove)
- Reconciliation performance
- Style computation
- Layout constraint processing

## Dependencies

- `vitest` - Test and benchmark runner
- `@bettertui/core` - Core package being benchmarked
