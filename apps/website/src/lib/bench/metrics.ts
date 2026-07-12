import type { BenchmarkMetrics } from "./types";

export class MetricCollector {
  private layoutSamples: number[] = [];
  private renderSamples: number[] = [];
  private frameSamples: number[] = [];

  recordLayout(ms: number) {
    this.layoutSamples.push(ms);
  }
  recordRender(ms: number) {
    this.renderSamples.push(ms);
  }
  recordFrame(ms: number) {
    this.frameSamples.push(ms);
  }

  private avg(s: number[]): number {
    return s.length ? s.reduce((a, b) => a + b, 0) / s.length : 0;
  }

  bundle(): Omit<BenchmarkMetrics, "framework" | "app"> {
    const mem = process.memoryUsage();
    return {
      startupMs: 0,
      layoutMs: this.avg(this.layoutSamples),
      renderMs: this.avg(this.renderSamples),
      frameGenerateMs: this.avg(this.frameSamples),
      updateLatencyMs: 0,
      inputLatencyMs: 0,
      scrollLatencyMs: 0,
      fps: 0,
      animationSmoothness: 0,
      memoryRssMb: mem.rss / 1024 / 1024,
      memoryHeapMb: mem.heapUsed / 1024 / 1024,
      cpuPercent: 0,
      bundleKb: 0,
      largeTableMs: 0,
      largeTreeMs: 0,
      terminalThroughputCps: 0,
      continuousRenderCostMb: 0,
    };
  }
}

export function writeReport(results: BenchmarkMetrics[], path: string) {
  console.log(`[bench] ${results.length} result(s) -> ${path}`);
  for (const r of results) {
    console.log(
      `  ${r.framework.padEnd(9)} ${r.app.padEnd(16)} ` +
        `layout=${r.layoutMs.toFixed(2)}ms render=${r.renderMs.toFixed(2)}ms ` +
        `fps=${r.fps} mem=${r.memoryRssMb.toFixed(1)}MB`,
    );
  }
}
