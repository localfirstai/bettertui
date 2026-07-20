import { displayWidth, sparkline } from "../ansi.utils";
import { DebugPanel, type Panel, type PanelContext } from "../panel.types";

function labelled(label: string, value: string, width: number): string {
  const gap = Math.max(1, width - displayWidth(label) - displayWidth(value));
  return label + " ".repeat(gap) + value;
}

/**
 * Panel 5 — Dirty Region Visualizer (stats-level).
 *
 * The true per-cell dirty highlight needs napi cell-buffer access, which the
 * shipped engine does not expose (see the task plan). This pass reports the
 * dirty-region count per frame and its recent trend — everything reachable from
 * `RenderResult.dirty_region_count` today.
 */
export const dirtyRegionsPanel: Panel = {
  id: DebugPanel.DirtyRegions,
  title: "Dirty Regions",

  render(ctx: PanelContext): string[] {
    const { devtools } = ctx;
    const w = ctx.maxWidth;
    const frames = devtools.performance.getFrames();
    const dirtyCounts = frames.map((f) => f.dirtyRegionCount);
    const total = dirtyCounts.reduce((a, b) => a + b, 0);

    const lines: string[] = [];
    lines.push(labelled("Current", String(ctx.dirtyRegionCount), w));
    if (dirtyCounts.length > 0) {
      const avg = total / dirtyCounts.length;
      lines.push(labelled("Avg", avg.toFixed(1), w));
      lines.push(labelled("Max", String(Math.max(...dirtyCounts)), w));
      const spark = sparkline(dirtyCounts, w);
      if (spark) lines.push(spark);
    }
    lines.push("");
    lines.push("per-cell highlight:");
    lines.push("needs Rust napi (deferred)");
    return lines;
  },
};
