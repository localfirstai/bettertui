import { bar, displayWidth, padEnd, sparkline } from "../ansi.utils";
import { DebugPanel, type Panel, type PanelContext } from "../panel.types";

/** Format a byte count into a short human string. */
function humanBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes}B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)}K`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)}M`;
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)}G`;
}

function labelled(label: string, value: string, width: number): string {
  const gap = Math.max(1, width - displayWidth(label) - displayWidth(value));
  return label + " ".repeat(gap) + value;
}

/**
 * Panel 1 (Performance) + Panel 6 (Render Statistics).
 *
 * Data-complete today: pulls FPS, frame time, render calls, bytes, cache
 * hit/miss, and allocations from engine diagnostics + the PerformanceTracker,
 * and memory from `process.memoryUsage()`.
 */
export const performancePanel: Panel = {
  id: DebugPanel.Performance,
  title: "Performance",

  render(ctx: PanelContext): string[] {
    const { diagnostics, devtools } = ctx;
    const w = ctx.maxWidth;
    const snapshot = devtools.getStats();
    const frames = devtools.performance.getFrames();
    const durations = frames.map((f) => f.duration);
    const mem = devtools.getMemoryStats();

    const fps = snapshot.fps > 0 ? snapshot.fps : diagnostics.fps;
    const avgFrame =
      snapshot.avgFrameTime > 0 ? snapshot.avgFrameTime : diagnostics.averageFrameTime;

    const cacheTotal = diagnostics.cacheHits + diagnostics.cacheMisses;
    const cacheRatio = cacheTotal === 0 ? 0 : diagnostics.cacheHits / cacheTotal;

    const lines: string[] = [];
    lines.push(labelled("FPS", fps.toFixed(1), w));
    lines.push(labelled("Frame", `${avgFrame.toFixed(2)}ms`, w));
    if (durations.length > 0) {
      lines.push(
        labelled(
          "min/max",
          `${snapshot.minFrameTime.toFixed(1)}/${snapshot.maxFrameTime.toFixed(1)}ms`,
          w,
        ),
      );
      const spark = sparkline(durations, w);
      if (spark) lines.push(spark);
    }
    lines.push(labelled("Frames", String(snapshot.totalFrames), w));
    lines.push(labelled("Dropped", String(snapshot.droppedFrames), w));

    lines.push("─".repeat(w));

    lines.push(labelled("Renders", String(diagnostics.renderCalls), w));
    lines.push(labelled("Bytes", humanBytes(diagnostics.renderBytes), w));
    lines.push(labelled("Layouts", String(diagnostics.layoutComputations), w));
    lines.push(labelled("Events", String(diagnostics.eventDispatches), w));
    lines.push(labelled("Allocs", String(diagnostics.allocations), w));

    const cachePct = `${(cacheRatio * 100).toFixed(0)}%`;
    lines.push(labelled("Cache", `${diagnostics.cacheHits}/${cacheTotal} ${cachePct}`, w));
    lines.push(padEnd(bar(cacheRatio, w), w));

    lines.push("─".repeat(w));

    lines.push(labelled("Heap", `${humanBytes(mem.heapUsed)}/${humanBytes(mem.heapTotal)}`, w));
    lines.push(labelled("RSS", humanBytes(mem.rss), w));

    return lines;
  },
};
