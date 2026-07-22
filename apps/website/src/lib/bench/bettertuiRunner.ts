import type { BenchApp, BenchmarkMetrics, FrameworkId, FrameworkRunner } from "./bench.types";
import { MetricCollector } from "./metrics";

export class BetterTuiRunner implements FrameworkRunner {
  id: FrameworkId = "bettertui";
  private collector = new MetricCollector();

  async mount(_app: BenchApp): Promise<void> {
    throw new Error("BetterTUI runner blocked: @bettertui/* is not published to npm.");
  }

  async frame(): Promise<number> {
    return 0;
  }
  async dispatchInput(): Promise<number> {
    return 0;
  }
  async unmount(): Promise<void> {}

  report(app: BenchApp): BenchmarkMetrics {
    return { framework: this.id, app: app.id, ...this.collector.bundle() };
  }
}
