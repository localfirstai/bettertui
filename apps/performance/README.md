# @bettertui/performance

Independent benchmark harness for **OpenTUI vs BetterTUI**.

This package is **NOT part of the BetterTUI runtime**. It consumes **published npm
packages exactly as end users do** — no local workspace references, no `workspace:*`
links. It exists only to validate parity and performance toward v1.0.

Target deploy: **performance.bettertui.com**

---

## ⚠️ PACKAGE BLOCKER (read before running)

The audit (`tasks/reports/opentui-gap-analysis.md`) found that **BetterTUI is not
published to npm**: every `packages/*/package.json` is `private: true` (or v0.0.0
with `workspace:*` deps) and the native addon `bettertui_bindings` is `require`d at
runtime but must be built manually via `cargo build -p bettertui-bindings`.

Consequence for this harness:

- `@opentui/core@0.4.3` + `@opentui/react@0.4.3` install and run **today** (OpenTUI
  ships per-platform native binaries as optional deps).
- `@bettertui/*` **cannot be installed from npm yet**. The BetterTUI benchmark apps
  under `src/bench/apps/bettertui/` are scaffolded but will fail to `import` until
  BetterTUI is published (see v1.0 work item #1 in the report).

**To unblock:** publish `@bettertui/core` (includes the native bridge formerly `@bettertui/native`), `@bettertui/react`
(with the napi addon as an optional platform dependency, mirroring
`@opentui/core-darwin-arm64`), then add them to this package's `dependencies`.

---

## Layout

```
apps/performance/
  package.json          # deps: @opentui/* only (BetterTUI blocked on publish)
  README.md             # this file
  tsconfig.json
  src/
    bench/
      metrics.ts        # MetricCollector: startup, bundle, memory, cpu, fps, layout, render, latency
      frameworks.ts     # FrameworkRunner abstraction (OpenTUI | BetterTUI)
      runner.ts         # runs identical workloads for both, collects + writes JSON
      apps/
        opentui/        # identical React apps using @opentui/react (RUNNABLE NOW)
        bettertui/      # identical React apps using @bettertui/react (BLOCKED ON PUBLISH)
        definitions.ts  # shared app list (hello-world, counter, large-list, ...)
    visual/
      side-by-side.tsx  # OpenTUI vs BetterTUI visual compare
      charts.tsx        # perf charts (timing / memory / fps / latency)
      frame-stats.tsx   # render pipeline + frame statistics
      history.tsx       # benchmark history
    deploy/
      astro.config.ts   # static site -> performance.bettertui.com
```

## Benchmarks (identical UI for both frameworks)

1. Hello World
2. Counter
3. Large List (10k rows)
4. Large Table (1k × 20)
5. Large Tree (5k nodes)
6. Dashboard
7. Markdown Viewer
8. Animation (tween / spring)
9. Terminal scroll
10. Stress Test (50k nodes)

## Metrics collected (identical workloads)

startup time, bundle size, memory (RSS/heap), CPU %, FPS, layout time, render time,
frame generation, update latency, input latency, scroll latency, animation smoothness,
large-table perf, large-tree perf, terminal throughput, continuous-render cost.

## Run

```bash
bun install
bun run bench        # OpenTUI results now; BetterTUI after publish
```

> Note: OpenTUI requires Bun (`engines.bun >=1.3.0`). BetterTUI's Node napi addon
> would run under Node. The runner detects the active framework per app.
