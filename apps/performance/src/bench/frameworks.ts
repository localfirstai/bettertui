// OpenTUI framework runner. Uses the PUBLISHED @opentui/core + @opentui/react.
// Runnable today (OpenTUI ships native binaries as optional deps).
//
// NOTE: This file is a scaffold showing the integration shape. Fill the per-app
// mount logic from src/bench/apps/opentui/<app>.tsx as the suite grows.

import { type BenchmarkMetrics, MetricCollector } from "../metrics";
import { BENCH_APPS, type BenchApp, type FrameworkId, type FrameworkRunner } from "./definitions";

export class OpenTuiRunner implements FrameworkRunner {
  id: FrameworkId = "opentui";
  private collector = new MetricCollector();

  async mount(_app: BenchApp): Promise<void> {
    // e.g. const { createRoot } = await import("@opentui/react");
    // const root = createRoot(renderer); root.render(<App/>);
  }

  async frame(): Promise<number> {
    const t = performance.now();
    // drive one render frame
    return performance.now() - t;
  }

  async dispatchInput(): Promise<number> {
    const t = performance.now();
    // push a key event through the event bus
    return performance.now() - t;
  }

  async unmount(): Promise<void> {}

  report(app: BenchApp): BenchmarkMetrics {
    return {
      framework: this.id,
      app: app.id,
      ...this.collector.bundle(),
    } as BenchmarkMetrics;
  }
}

export async function runOpenTuiBenchmarks(): Promise<BenchmarkMetrics[]> {
  const runner = new OpenTuiRunner();
  const results: BenchmarkMetrics[] = [];
  for (const app of BENCH_APPS) {
    await runner.mount(app);
    for (let i = 0; i < 60; i++) runner.collector.recordFrame(await runner.frame());
    results.push(runner.report(app));
    await runner.unmount();
  }
  return results;
}
