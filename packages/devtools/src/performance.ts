import type { FrameMetrics, PerformanceSnapshot } from "./types";

export interface PerformanceTrackerOptions {
  maxFrames?: number | undefined;
  onFrame?: ((metrics: FrameMetrics) => void) | undefined;
}

export class PerformanceTracker {
  private frames: FrameMetrics[] = [];
  private nextFrameNumber = 0;
  private maxFrames: number;
  private onFrame: ((frame: FrameMetrics) => void) | undefined;
  private frameStart = 0;
  private commandCountAtStart = 0;

  constructor(options: PerformanceTrackerOptions = {}) {
    this.maxFrames = options.maxFrames ?? 300;
    this.onFrame = options.onFrame;
  }

  /** Call at the start of a frame */
  beginFrame(commandCount: number): void {
    this.frameStart = performance.now();
    this.commandCountAtStart = commandCount;
  }

  /** Call at the end of a frame with metrics */
  endFrame(
    options: {
      dirtyRegionCount?: number;
      renderDuration?: number;
      layoutDuration?: number;
      paintDuration?: number;
      ffiDuration?: number;
    } = {},
  ): FrameMetrics {
    const now = performance.now();
    const metrics: FrameMetrics = {
      frameNumber: this.nextFrameNumber++,
      timestamp: now,
      duration: now - this.frameStart,
      commandCount: this.commandCountAtStart,
      dirtyRegionCount: options.dirtyRegionCount ?? 0,
      renderDuration: options.renderDuration,
      layoutDuration: options.layoutDuration,
      paintDuration: options.paintDuration,
      ffiDuration: options.ffiDuration,
    };

    this.frames.push(metrics);
    if (this.frames.length > this.maxFrames) {
      this.frames.shift();
    }

    this.onFrame?.(metrics);
    return metrics;
  }

  /** Record a frame with all metrics at once */
  recordFrame(metrics: Partial<FrameMetrics> & { duration: number }): FrameMetrics {
    const { timestamp: _ignored, ...rest } = metrics;
    const frame: FrameMetrics = {
      frameNumber: this.nextFrameNumber++,
      timestamp: performance.now(),
      commandCount: 0,
      dirtyRegionCount: 0,
      ...rest,
    };

    this.frames.push(frame);
    if (this.frames.length > this.maxFrames) {
      this.frames.shift();
    }

    this.onFrame?.(frame);
    return frame;
  }

  getFrames(): readonly FrameMetrics[] {
    return this.frames;
  }

  getRecentFrames(count: number): FrameMetrics[] {
    return this.frames.slice(-count);
  }

  /** Calculate current FPS based on recent frames */
  getFps(sampleSize = 60): number {
    const recent = this.frames.slice(-sampleSize);
    if (recent.length < 2) return 0;

    const first = recent[0];
    const last = recent[recent.length - 1];
    if (first === undefined || last === undefined) return 0;
    const elapsed = last.timestamp - first.timestamp;

    if (elapsed <= 0) return 0;
    return ((recent.length - 1) / elapsed) * 1000;
  }

  /** Get a full performance snapshot */
  getSnapshot(): PerformanceSnapshot {
    const frames = this.frames;
    const durations = frames.map((f) => f.duration);

    const fps = this.getFps();
    const avgFrameTime =
      durations.length > 0 ? durations.reduce((a, b) => a + b, 0) / durations.length : 0;
    const minFrameTime = durations.length > 0 ? Math.min(...durations) : 0;
    const maxFrameTime = durations.length > 0 ? Math.max(...durations) : 0;
    const totalFrames = frames.length;
    const droppedFrames = frames.filter((f) => f.duration > 33.33).length; // >2 frames at 60fps
    const commandCount = frames.reduce((sum, f) => sum + f.commandCount, 0);
    const dirtyNodeCount = frames.reduce((sum, f) => sum + f.dirtyRegionCount, 0);

    let memoryUsage: PerformanceSnapshot["memoryUsage"] | undefined;
    if (typeof globalThis !== "undefined" && "performance" in globalThis) {
      const perf = globalThis.performance as {
        memory?: { usedJSHeapSize: number; jsHeapSizeLimit: number; totalJSHeapSize: number };
      };
      if (perf.memory) {
        memoryUsage = {
          heapUsed: perf.memory.usedJSHeapSize,
          heapTotal: perf.memory.jsHeapSizeLimit,
          external: perf.memory.totalJSHeapSize,
        };
      }
    }

    return {
      fps,
      avgFrameTime,
      minFrameTime,
      maxFrameTime,
      totalFrames,
      droppedFrames,
      commandCount,
      dirtyNodeCount,
      memoryUsage,
    };
  }

  clear(): void {
    this.frames = [];
    this.nextFrameNumber = 0;
  }

  get count(): number {
    return this.frames.length;
  }
}
