import type { SchedulerSnapshot } from "./devtools.types";

export interface SchedulerInspectorOptions {
  onFrameDrop?: ((droppedCount: number) => void) | undefined;
}

export class SchedulerInspector {
  private frameCount = 0;
  private droppedFrames = 0;
  private pendingFrames = 0;
  private isRunning = false;
  private isRendering = false;
  private hasScheduledRender = false;
  private highestPriority = "idle";
  private idleCallbacksPending = 0;
  private animationFramesPending = 0;
  private frameBudgetMs = 16.67;
  private utilization = 0;
  private onFrameDrop: ((droppedCount: number) => void) | undefined;

  constructor(options: SchedulerInspectorOptions = {}) {
    this.onFrameDrop = options.onFrameDrop;
  }

  updateState(state: Partial<SchedulerSnapshot>): void {
    if (state.isRunning !== undefined) this.isRunning = state.isRunning;
    if (state.isRendering !== undefined) this.isRendering = state.isRendering;
    if (state.hasScheduledRender !== undefined) this.hasScheduledRender = state.hasScheduledRender;
    if (state.frameCount !== undefined) this.frameCount = state.frameCount;
    if (state.pendingFrames !== undefined) this.pendingFrames = state.pendingFrames;
    if (state.highestPriority !== undefined) this.highestPriority = state.highestPriority;
    if (state.idleCallbacksPending !== undefined)
      this.idleCallbacksPending = state.idleCallbacksPending;
    if (state.animationFramesPending !== undefined)
      this.animationFramesPending = state.animationFramesPending;
    if (state.frameBudgetMs !== undefined) this.frameBudgetMs = state.frameBudgetMs;
    if (state.utilization !== undefined) this.utilization = state.utilization;
  }

  recordFrameDrop(): void {
    this.droppedFrames++;
    this.onFrameDrop?.(this.droppedFrames);
  }

  incrementFrameCount(): void {
    this.frameCount++;
  }

  getSnapshot(): SchedulerSnapshot {
    return {
      isRunning: this.isRunning,
      isRendering: this.isRendering,
      hasScheduledRender: this.hasScheduledRender,
      frameCount: this.frameCount,
      droppedFrames: this.droppedFrames,
      pendingFrames: this.pendingFrames,
      highestPriority: this.highestPriority,
      idleCallbacksPending: this.idleCallbacksPending,
      animationFramesPending: this.animationFramesPending,
      frameBudgetMs: this.frameBudgetMs,
      utilization: this.utilization,
    };
  }

  getDropRate(): number {
    if (this.frameCount === 0) return 0;
    return this.droppedFrames / this.frameCount;
  }

  clear(): void {
    this.frameCount = 0;
    this.droppedFrames = 0;
    this.pendingFrames = 0;
    this.isRunning = false;
    this.isRendering = false;
    this.hasScheduledRender = false;
    this.highestPriority = "idle";
    this.idleCallbacksPending = 0;
    this.animationFramesPending = 0;
    this.frameBudgetMs = 16.67;
    this.utilization = 0;
  }
}
