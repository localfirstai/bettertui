// BetterTUI framework runner. BLOCKED until @bettertui/* is published to npm.
//
// Shape mirrors OpenTuiRunner. Once published (v1.0 work item #1), add:
//   "@bettertui/core", "@bettertui/native", "@bettertui/react"
// to apps/performance/package.json dependencies and implement mount() using
// @bettertui/react's render() + hooks.

import { type BenchmarkMetrics, MetricCollector } from "../metrics";
import type { BenchApp, FrameworkId, FrameworkRunner } from "./apps/definitions";

export class BetterTuiRunner implements FrameworkRunner {
  id: FrameworkId = "bettertui";
  private collector = new MetricCollector();

  async mount(_app: BenchApp): Promise<void> {
    throw new Error(
      "BetterTUI runner blocked: @bettertui/* is not published to npm. " +
        "See apps/performance/README.md (PACKAGE BLOCKER).",
    );
  }

  async frame(): Promise<number> {
    return 0;
  }
  async dispatchInput(): Promise<number> {
    return 0;
  }
  async unmount(): Promise<void> {}

  report(app: BenchApp): BenchmarkMetrics {
    return { framework: this.id, app: app.id, ...this.collector.bundle() } as BenchmarkMetrics;
  }
}
