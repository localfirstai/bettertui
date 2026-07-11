// Visual side-by-side comparison entry (OpenTUI vs BetterTUI).
// Renders performance charts, timing, memory, FPS, latency, frame statistics,
// and benchmark history. Deployed independently at performance.bettertui.com.

import type { BenchmarkMetrics } from "../bench/metrics.ts";

export interface VisualReport {
  opentui: BenchmarkMetrics[];
  bettertui: BenchmarkMetrics[]; // empty until BetterTUI is published
  generatedAt: string;
}

export function renderSideBySide(report: VisualReport): string {
  const fmt = (r?: BenchmarkMetrics) =>
    r
      ? `layout=${r.layoutMs.toFixed(2)}ms render=${r.renderMs.toFixed(2)}ms fps=${r.fps} mem=${r.memoryRssMb.toFixed(1)}MB`
      : "n/a (not published)";

  const lines: string[] = ["# OpenTUI vs BetterTUI — Performance"];
  for (const ot of report.opentui) {
    const bt = report.bettertui.find((b) => b.app === ot.app);
    lines.push("");
    lines.push(`## ${ot.app}`);
    lines.push(`- OpenTUI:   ${fmt(ot)}`);
    lines.push(`- BetterTUI: ${fmt(bt)}`);
  }
  lines.push("");
  lines.push(`Generated ${report.generatedAt}`);
  return lines.join("\n");
}

// Astro page wrapper would import this and render <charts>, <frame-stats>,
// <history> components. Kept dependency-free here.
export default renderSideBySide;
