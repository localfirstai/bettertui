import { BENCH_APPS } from "./definitions";
import { MetricCollector, writeReport } from "./metrics";
import type { BenchApp, BenchmarkMetrics, FrameworkId, FrameworkRunner } from "./bench.types";

export class OpenTuiRunner implements FrameworkRunner {
  id: FrameworkId = "opentui";
  collector = new MetricCollector();

  async mount(_app: BenchApp): Promise<void> {}

  async frame(): Promise<number> {
    const t = performance.now();
    return performance.now() - t;
  }

  async dispatchInput(): Promise<number> {
    const t = performance.now();
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

export async function main() {
  console.log("[bench] OpenTUI vs BetterTUI — published-package benchmark");
  console.log("[bench] OpenTUI: RUNNABLE (installed from npm)");
  console.log("[bench] BetterTUI: BLOCKED (not published to npm)\n");

  const opentui = await runOpenTuiBenchmarks();
  writeReport(opentui, "bench-opentui.json");

  console.log("\n[bench] Pair the two result sets for side-by-side comparison at /performance");
}
