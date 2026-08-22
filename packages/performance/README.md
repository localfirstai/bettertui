# @bettertui/performance

> **Website:** [bettertui.dev](https://bettertui.dev)

Micro and end-to-end benchmark suite for BetterTUI.

## Benchmarks

| File | Scope |
|------|-------|
| `commandBuffer.bench.ts` | Command buffer operations |
| `engineFrame.bench.ts` | Engine frame rendering |
| `layout.bench.ts` | Layout computation |
| `parser.bench.ts` | Input parsing |
| `reconciler.bench.ts` | Reconciler operations |
| `reconcilerUnit.bench.ts` | Reconciler unit benchmarks |
| `runtime.bench.ts` | Runtime frame loop |
| `spanFeed.bench.ts` | Span feed operations |
| `textBuffer.bench.ts` | Text buffer operations |
| `theme.bench.ts` | Theme resolution |

## Running

```bash
pnpm bench
pnpm bench:run
```
