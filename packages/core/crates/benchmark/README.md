# bettertui-benchmark

## Purpose

Consolidated end-to-end benchmarks for the BetterTUI Rust crates. All
Rust benchmarking lives in this crate. The public API surface of the
`engine`, `widgets`, and `terminal` crates is exercised through
[`criterion`] benchmark targets under `benches/`.

## Responsibilities

- **Criterion targets:** Three harnesses (`engine`, `widgets`, `terminal`)
  under `benches/` measure real-world throughput of the public APIs.
- **Crate-internal micro-benchmarks:** The `benchmark` module provides a
  lightweight timing/throughput harness migrated from the engine crate for
  micro-benchmarks that don't need the full criterion infrastructure.

## Public API

The crate re-exports three items from the `benchmark` module:

| Item | Description |
|------|-------------|
| `BenchmarkHarness` | Configurable timing harness with warmup, iterations, and mean/min/max latency reporting. |
| `BenchmarkResult` | Result statistics: name, total iterations, duration, mean, min, max, throughput (ops/s), and op count. |
| `OpCounter` | Helper for counting operations inside benchmark loops (e.g. `count_items`). |

### Example

```rust
use bettertui_benchmark::BenchmarkHarness;

let mut harness = BenchmarkHarness::new("my_bench")
    .with_iterations(1000)
    .with_warmup(100);

harness.run(|| {
    let x = 2 + 2;
    std::hint::black_box(x);
});

println!("{}", harness.report());
```

## Dependencies

- `bettertui-engine` — rendering engine under test (terminal I/O, VT, PTY, capabilities, and the widget host are modules within this crate)
- `criterion` (dev) — criterion harness with `html_reports`

## Consumers

This is a `publish = false` workspace-internal crate. It is not consumed by
other crates; it is run via cargo bench.

## Build & Run

```bash
# All benchmarks (criterion)
cargo bench -p bettertui-benchmark

# A single benchmark target
cargo bench -p bettertui-benchmark --bench engine
```

## Notes

- `[[bench]]` targets (`engine`, `widgets`, `terminal`) use
  `harness = false` so criterion drives them.
- The `benchmark` module is a standalone micro-benchmark utility and does
  not depend on criterion.
