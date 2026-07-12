// Shared benchmark app definitions. Both frameworks implement each app to
// produce IDENTICAL UI. The harness runs the same workload for each.

export interface BenchApp {
  id: string;
  label: string;
  description: string;
  // Approximate scale used by stress/scale benchmarks.
  scale?: number;
}

export const BENCH_APPS: BenchApp[] = [
  { id: "hello-world", label: "Hello World", description: "Minimal render" },
  { id: "counter", label: "Counter", description: "State + re-render" },
  { id: "large-list", label: "Large List", description: "10k rows", scale: 10000 },
  { id: "large-table", label: "Large Table", description: "1k x 20", scale: 1000 },
  { id: "large-tree", label: "Large Tree", description: "5k nodes", scale: 5000 },
  { id: "dashboard", label: "Dashboard", description: "Mixed widgets" },
  { id: "markdown-viewer", label: "Markdown Viewer", description: "Text heavy" },
  { id: "animation", label: "Animation", description: "Tween/spring loop" },
  { id: "scrolling", label: "Terminal Scroll", description: "Scroll stress" },
  { id: "stress-test", label: "Stress Test", description: "50k nodes", scale: 50000 },
];

export type FrameworkId = "opentui" | "bettertui";

export interface FrameworkRunner {
  id: FrameworkId;
  // Load the framework's implementation of an app and mount it.
  mount(app: BenchApp): Promise<void>;
  // Advance one frame; returns measured frame time in ms.
  frame(): Promise<number>;
  // Simulate an input event and return update latency in ms.
  dispatchInput(): Promise<number>;
  unmount(): Promise<void>;
}
