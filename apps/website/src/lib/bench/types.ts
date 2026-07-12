export type FrameworkId = "opentui" | "bettertui";

export interface BenchApp {
  id: string;
  label: string;
  description: string;
  scale?: number;
}

export interface BenchmarkMetrics {
  framework: FrameworkId;
  app: string;
  startupMs: number;
  layoutMs: number;
  renderMs: number;
  frameGenerateMs: number;
  updateLatencyMs: number;
  inputLatencyMs: number;
  scrollLatencyMs: number;
  fps: number;
  animationSmoothness: number;
  memoryRssMb: number;
  memoryHeapMb: number;
  cpuPercent: number;
  bundleKb: number;
  largeTableMs: number;
  largeTreeMs: number;
  terminalThroughputCps: number;
  continuousRenderCostMb: number;
}

export interface FrameworkRunner {
  id: FrameworkId;
  mount(app: BenchApp): Promise<void>;
  frame(): Promise<number>;
  dispatchInput(): Promise<number>;
  unmount(): Promise<void>;
}

export interface VisualReport {
  opentui: BenchmarkMetrics[];
  bettertui: BenchmarkMetrics[];
  generatedAt: string;
}
