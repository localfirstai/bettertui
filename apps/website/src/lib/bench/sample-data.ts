import type { BenchmarkMetrics, VisualReport } from "./types";

const opentuiBase: Record<string, Partial<BenchmarkMetrics>> = {
  "hello-world": {
    layoutMs: 0.18,
    renderMs: 0.22,
    frameGenerateMs: 0.41,
    fps: 240,
    memoryRssMb: 34.2,
    memoryHeapMb: 8.1,
    startupMs: 12,
    bundleKb: 1240,
    inputLatencyMs: 0.8,
  },
  counter: {
    layoutMs: 0.24,
    renderMs: 0.31,
    frameGenerateMs: 0.56,
    fps: 218,
    memoryRssMb: 35.8,
    memoryHeapMb: 8.9,
    startupMs: 14,
    bundleKb: 1240,
    inputLatencyMs: 1.1,
  },
  "large-list": {
    layoutMs: 4.82,
    renderMs: 6.14,
    frameGenerateMs: 11.2,
    fps: 78,
    memoryRssMb: 58.4,
    memoryHeapMb: 21.3,
    startupMs: 38,
    bundleKb: 1240,
    inputLatencyMs: 5.2,
  },
  "large-table": {
    layoutMs: 7.31,
    renderMs: 9.28,
    frameGenerateMs: 16.8,
    fps: 54,
    memoryRssMb: 64.1,
    memoryHeapMb: 26.7,
    startupMs: 42,
    bundleKb: 1240,
    inputLatencyMs: 7.8,
  },
  "large-tree": {
    layoutMs: 5.67,
    renderMs: 7.12,
    frameGenerateMs: 12.9,
    fps: 68,
    memoryRssMb: 61.3,
    memoryHeapMb: 24.1,
    startupMs: 36,
    bundleKb: 1240,
    inputLatencyMs: 6.4,
  },
  dashboard: {
    layoutMs: 2.14,
    renderMs: 3.08,
    frameGenerateMs: 5.31,
    fps: 112,
    memoryRssMb: 48.2,
    memoryHeapMb: 15.7,
    startupMs: 22,
    bundleKb: 1240,
    inputLatencyMs: 2.8,
  },
  "markdown-viewer": {
    layoutMs: 1.87,
    renderMs: 2.41,
    frameGenerateMs: 4.32,
    fps: 134,
    memoryRssMb: 42.6,
    memoryHeapMb: 13.2,
    startupMs: 19,
    bundleKb: 1240,
    inputLatencyMs: 2.2,
  },
  animation: {
    layoutMs: 0.92,
    renderMs: 1.34,
    frameGenerateMs: 2.28,
    fps: 168,
    memoryRssMb: 40.1,
    memoryHeapMb: 12.1,
    startupMs: 17,
    bundleKb: 1240,
    inputLatencyMs: 1.8,
  },
  scrolling: {
    layoutMs: 3.41,
    renderMs: 4.62,
    frameGenerateMs: 8.12,
    fps: 92,
    memoryRssMb: 52.8,
    memoryHeapMb: 18.9,
    startupMs: 28,
    bundleKb: 1240,
    inputLatencyMs: 4.1,
  },
  "stress-test": {
    layoutMs: 18.4,
    renderMs: 22.7,
    frameGenerateMs: 41.3,
    fps: 22,
    memoryRssMb: 118.5,
    memoryHeapMb: 52.3,
    startupMs: 89,
    bundleKb: 1240,
    inputLatencyMs: 19.2,
  },
};

const bettertuiImprovement = {
  layoutMs: 0.35,
  renderMs: 0.4,
  frameGenerateMs: 0.38,
  fps: 1.85,
  memoryRssMb: 0.62,
  memoryHeapMb: 0.48,
  startupMs: 0.45,
  bundleKb: 0.28,
  inputLatencyMs: 0.42,
};

export function generateOpentuiMetrics(): BenchmarkMetrics[] {
  const ids = Object.keys(opentuiBase);
  return ids.map((id) => {
    const base = opentuiBase[id];
    return {
      framework: "opentui" as const,
      app: id,
      startupMs: base.startupMs ?? 0,
      layoutMs: base.layoutMs ?? 0,
      renderMs: base.renderMs ?? 0,
      frameGenerateMs: base.frameGenerateMs ?? 0,
      updateLatencyMs: (base.inputLatencyMs ?? 0) * 0.8,
      inputLatencyMs: base.inputLatencyMs ?? 0,
      scrollLatencyMs: (base.inputLatencyMs ?? 0) * 1.4,
      fps: base.fps ?? 0,
      animationSmoothness: base.fps ? Math.min(base.fps / 240, 1) : 0,
      memoryRssMb: base.memoryRssMb ?? 0,
      memoryHeapMb: base.memoryHeapMb ?? 0,
      cpuPercent: base.fps ? base.fps / 10 : 0,
      bundleKb: base.bundleKb ?? 0,
      largeTableMs: id === "large-table" ? (base.frameGenerateMs ?? 0) : 0,
      largeTreeMs: id === "large-tree" ? (base.frameGenerateMs ?? 0) : 0,
      terminalThroughputCps: base.fps ? 80000 / (base.frameGenerateMs ?? 1) : 0,
      continuousRenderCostMb: (base.memoryRssMb ?? 0) * 0.15,
    };
  });
}

export function generateBettertuiMetrics(): BenchmarkMetrics[] {
  const opentui = generateOpentuiMetrics();
  return opentui.map((o) => {
    const newFps = Math.min(o.fps * bettertuiImprovement.fps, 300);
    return {
      ...o,
      framework: "bettertui" as const,
      layoutMs: o.layoutMs * bettertuiImprovement.layoutMs,
      renderMs: o.renderMs * bettertuiImprovement.renderMs,
      frameGenerateMs: o.frameGenerateMs * bettertuiImprovement.frameGenerateMs,
      fps: newFps,
      animationSmoothness: Math.min(newFps / 240, 1),
      memoryRssMb: o.memoryRssMb * bettertuiImprovement.memoryRssMb,
      memoryHeapMb: o.memoryHeapMb * bettertuiImprovement.memoryHeapMb,
      startupMs: o.startupMs * bettertuiImprovement.startupMs,
      bundleKb: o.bundleKb * bettertuiImprovement.bundleKb,
      inputLatencyMs: o.inputLatencyMs * bettertuiImprovement.inputLatencyMs,
      updateLatencyMs: o.updateLatencyMs * bettertuiImprovement.inputLatencyMs,
      scrollLatencyMs: o.scrollLatencyMs * bettertuiImprovement.inputLatencyMs,
      cpuPercent: o.cpuPercent * 0.5,
      largeTableMs: o.largeTableMs * bettertuiImprovement.frameGenerateMs,
      largeTreeMs: o.largeTreeMs * bettertuiImprovement.frameGenerateMs,
      terminalThroughputCps: o.terminalThroughputCps * (1 / bettertuiImprovement.frameGenerateMs),
      continuousRenderCostMb: o.continuousRenderCostMb * 0.4,
    };
  });
}

export const sampleReport: VisualReport = {
  opentui: generateOpentuiMetrics(),
  bettertui: generateBettertuiMetrics(),
  generatedAt: "2025-01-15T10:30:00Z",
};

export function getMetricForApp(
  report: VisualReport,
  appId: string,
  framework: "opentui" | "bettertui",
): BenchmarkMetrics | undefined {
  return report[framework].find((m) => m.app === appId);
}

export function computeImprovement(
  opentui: number,
  bettertui: number,
  lowerIsBetter: boolean,
): { ratio: number; percent: number; better: boolean } {
  if (opentui === 0) return { ratio: 0, percent: 0, better: false };
  const ratio = bettertui / opentui;
  const percent = Math.abs((1 - ratio) * 100);
  const better = lowerIsBetter ? bettertui < opentui : bettertui > opentui;
  return { ratio, percent, better };
}
