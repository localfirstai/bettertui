# @bettertui/performance

End-to-end and native round-trip performance benchmarks for BetterTUI.

This package is **distinct from [`@bettertui/benchmark`](../benchmark/)**:

| Package | Scope | When to use |
|---|---|---|
| `@bettertui/benchmark` | TS-only micro-benchmarks over `@bettertui/core`'s pure-TS surfaces (CommandBuffer, Runtime, Reconciler, theme). No native dependency at runtime. | Quick CI perf regression checks. |
| `@bettertui/performance` (this package) | End-to-end benchmarks that round-trip through the Rust native engine, plus at-scale reconciler/parser/widget mounts. Requires `@bettertui/core` to be built (`pnpm build:native`) so `bettertui_engine.node` is loadable. | Deep perf analysis, OpenTUI parity comparison, release gating. |

## Layout

```
src/
├── engine-frame.bench.ts     — single-frame render through NativeEngine (napi + Rust)
├── layout.bench.ts           — tree-only layout via the Rust taffy integration
├── text-buffer.bench.ts      — NativeTextEngine insert/delete/undo/redo
├── span-feed.bench.ts        — NativeSpanFeed write/drain throughput
├── widget-mount.bench.ts     — mount every widget 1k times
├── parser.bench.ts           — stdin / keypress (xterm+kitty) / mouse parser throughput
└── reconciler.bench.ts       — createReconciler at 1k / 10k nodes
```

Each bench file documents its OpenTUI counterpart at the top so results can be cross-compared with `.opencode/references/opentui/packages/core/src/benchmark/`.

## Running

From repo root:

```bash
pnpm install
pnpm --filter @bettertui/core build            # ensure bettertui_engine.node exists
pnpm --filter @bettertui/performance bench     # watch mode
pnpm --filter @bettertui/performance bench:run # single run (CI)
```

Filter:

```bash
pnpm --filter @bettertui/performance bench:run -t "engine-frame"
```

## OpenTUI comparison

OpenTUI persists benchmark JSON to `latest-{quick,default,large,async,all}-bench-run.json`. The schema is consumed by their internal comparison tooling. When the BetterTUI numbers are stable, this package should emit the same JSON shape so they can be diffed directly.

OpenTUI's benchmark surface (`.opencode/references/opentui/packages/core/src/benchmark/`):

- `layout-benchmark.ts` (2,547 LoC) — counterpart of our `layout.bench.ts` + `reconciler.bench.ts`
- `box-draw-benchmark.ts` (1,042 LoC) — Rust covers this in `crates/benchmark/benches/engine.rs`
- `render-traversal-benchmark.ts` (928 LoC) — counterpart of our `engine-frame.bench.ts`
- `text-table-benchmark.ts` (948 LoC) + `text-table-width-benchmark.ts` — not yet ported (pending widget implementation work tracked in `tasks/reports/opentui-gap-analysis.md` W-7)
- `text-buffer-render-benchmark.ts` (874 LoC) — counterpart of our `text-buffer.bench.ts`
- `markdown-benchmark.ts` (1,796 LoC) — not yet ported (pending Markdown widget)
- `native-span-feed-benchmark.ts` (596 LoC) + async + compare variants — counterpart of our `span-feed.bench.ts`
- `attenuation-benchmark.ts`, `audio-playback-benchmark.ts`, `gain-benchmark.ts` — audio; **no counterpart** (BetterTUI has no audio subsystem; see gap analysis A-3 scope exclusion)
- `colormatrix-benchmark.ts` — covered by Rust `crates/benchmark/benches/engine.rs` color-matrix passes; TS surface not benchmarked

## Status

Scaffolded. The bench files in this package are the **first cut** — they exercise the APIs that already exist in `@bettertui/core`. As the React/Solid adapters (gaps A-1, A-2) and widget implementations (gap W-7) land, this package will grow cross-framework comparison benches.

See [`tasks/reports/opentui-gap-analysis.md`](../../tasks/reports/opentui-gap-analysis.md) §10–§12 for the full plan and gap tracker.
