// ─── Types ───────────────────────────────────────────────────────────────────
export type {
  LogLevel,
  LogEntry,
  CommandType,
  RecordedCommand,
  EventCategory,
  RecordedEvent,
  FrameMetrics,
  PerformanceSnapshot,
  TreeNode,
  SchedulerSnapshot,
  FocusSnapshot,
  TerminalCapabilities,
  TimelineEntry,
  TreeSnapshot,
  SnapshotDiff,
  DiagnosticExport,
} from "./types";

// ─── Modules ─────────────────────────────────────────────────────────────────
export { Logger } from "./logger";
export type { LoggerOptions } from "./logger";
export { CommandInspector } from "./command-inspector";
export type { CommandInspectorOptions } from "./command-inspector";
export { EventInspector } from "./event-inspector";
export type { EventInspectorOptions } from "./event-inspector";
export { PerformanceTracker } from "./performance";
export type { PerformanceTrackerOptions } from "./performance";
export { TreeInspector } from "./tree-inspector";
export type { TreeInspectorOptions } from "./tree-inspector";
export { SchedulerInspector } from "./scheduler-inspector";
export type { SchedulerInspectorOptions } from "./scheduler-inspector";
export { FocusInspector } from "./focus-inspector";
export type { FocusInspectorOptions } from "./focus-inspector";
export { CapabilityInspector } from "./capability-inspector";
export type { CapabilityInspectorOptions } from "./capability-inspector";
export { Timeline } from "./timeline";
export type { TimelineOptions } from "./timeline";
export { SnapshotManager } from "./snapshot";
export type { SnapshotOptions } from "./snapshot";
export { createExport, exportToJson, createSummary } from "./export";
export type { ExportOptions, ExportData } from "./export";

// ─── DevTools Interface ──────────────────────────────────────────────────────

import type { ExportOptions } from "./export";
import type { DiagnosticExport, SchedulerSnapshot, TerminalCapabilities, TreeNode } from "./types";

import { CapabilityInspector } from "./capability-inspector";
import { CommandInspector } from "./command-inspector";
import { EventInspector } from "./event-inspector";
import { createExport, createSummary, exportToJson } from "./export";
import { FocusInspector } from "./focus-inspector";
import { Logger } from "./logger";
import { PerformanceTracker } from "./performance";
import { SchedulerInspector } from "./scheduler-inspector";
import { SnapshotManager } from "./snapshot";
import { Timeline } from "./timeline";
import { TreeInspector } from "./tree-inspector";

export interface DevTools {
  /** Whether DevTools is enabled */
  readonly enabled: boolean;

  /** Structured logger */
  readonly logger: Logger;

  /** Command inspector — records every command emitted */
  readonly commands: CommandInspector;

  /** Event inspector — tracks keyboard, mouse, focus, resize events */
  readonly events: EventInspector;

  /** Performance tracker — frame timing, FPS, metrics */
  readonly performance: PerformanceTracker;

  /** Tree inspector — render tree, props, styles, layout */
  readonly tree: TreeInspector;

  /** Scheduler inspector — frame budget, drops, callbacks */
  readonly scheduler: SchedulerInspector;

  /** Focus inspector — focused node, tab order, scopes */
  readonly focus: FocusInspector;

  /** Terminal capability inspector */
  readonly capabilities: CapabilityInspector;

  /** Timeline — chronological event recording */
  readonly timeline: Timeline;

  /** Snapshot manager — capture and compare tree states */
  readonly snapshots: SnapshotManager;

  /** Record a command being emitted */
  recordCommand(
    type: string,
    payload: Record<string, unknown>,
    duration?: number | undefined,
  ): void;

  /** Record a render frame */
  recordFrame(options: {
    duration: number;
    commandCount?: number | undefined;
    dirtyRegionCount?: number | undefined;
    renderDuration?: number | undefined;
    layoutDuration?: number | undefined;
    paintDuration?: number | undefined;
    ffiDuration?: number | undefined;
  }): void;

  /** Record a keyboard event */
  recordKeyboard(
    key: string,
    modifiers: { ctrl: boolean; shift: boolean; alt: boolean; meta: boolean },
    target?: string | undefined,
  ): void;

  /** Record a mouse event */
  recordMouse(
    type: string,
    x: number,
    y: number,
    button?: string | undefined,
    target?: string | undefined,
  ): void;

  /** Record a focus change */
  recordFocus(type: "focus" | "blur", nodeId: string): void;

  /** Record a resize event */
  recordResize(
    width: number,
    height: number,
    prevWidth?: number | undefined,
    prevHeight?: number | undefined,
  ): void;

  /** Update terminal capabilities */
  updateCapabilities(capabilities: Partial<TerminalCapabilities>): void;

  /** Update scheduler state */
  updateScheduler(state: Partial<SchedulerSnapshot>): void;

  /** Capture a tree snapshot */
  captureSnapshot(tree: TreeNode): number;

  /** Get a full diagnostic export */
  exportData(options?: ExportOptions | undefined): DiagnosticExport;

  /** Get export as JSON string */
  exportJson(options?: ExportOptions | undefined): string;

  /** Get a summary report */
  getSummary(): string;

  /** Reset all inspectors */
  reset(): void;

  /** Dispose all resources */
  dispose(): void;
}

// ─── No-op implementation ────────────────────────────────────────────────────

function createNoOpDevTools(): DevTools {
  const noop = () => {};
  const noopZero = () => 0;
  const noopExport = (): DiagnosticExport => ({
    version: "1.0.0",
    timestamp: 0,
    duration: 0,
    logs: [],
    commands: [],
    events: [],
    frames: [],
    performance: {
      fps: 0,
      avgFrameTime: 0,
      minFrameTime: 0,
      maxFrameTime: 0,
      totalFrames: 0,
      droppedFrames: 0,
      commandCount: 0,
      dirtyNodeCount: 0,
    },
    timeline: [],
    snapshots: [],
  });
  return {
    enabled: false,
    logger: new Logger(),
    commands: new CommandInspector(),
    events: new EventInspector(),
    performance: new PerformanceTracker(),
    tree: new TreeInspector(),
    scheduler: new SchedulerInspector(),
    focus: new FocusInspector(),
    capabilities: new CapabilityInspector(),
    timeline: new Timeline(),
    snapshots: new SnapshotManager(),
    recordCommand: noop,
    recordFrame: noop,
    recordKeyboard: noop,
    recordMouse: noop,
    recordFocus: noop,
    recordResize: noop,
    updateCapabilities: noop,
    updateScheduler: noop,
    captureSnapshot: noopZero,
    exportData: noopExport,
    exportJson: () => JSON.stringify(noopExport()),
    getSummary: () => "",
    reset: noop,
    dispose: noop,
  };
}

// ─── Factory ─────────────────────────────────────────────────────────────────

export interface CreateDevToolsOptions {
  enabled?: boolean | undefined;
  maxEvents?: number | undefined;
  logging?: boolean | undefined;
  logLevel?: ("debug" | "info" | "warn" | "error" | "trace") | undefined;
  timeline?: boolean | undefined;
  performance?: boolean | undefined;
  snapshots?: boolean | undefined;
}

/**
 * Create a DevTools instance.
 *
 * When `enabled` is false or omitted, returns a no-op implementation with
 * near-zero overhead. When `enabled: true`, returns a fully functional
 * DevTools instance that can record commands, events, performance metrics,
 * and more.
 */
export function createDevTools(options?: CreateDevToolsOptions): DevTools {
  if (!options?.enabled) {
    return createNoOpDevTools();
  }

  const maxEvents = options.maxEvents ?? 1000;
  const logger = new Logger({ maxEntries: maxEvents, minLevel: options.logLevel ?? "debug" });
  const commands = new CommandInspector({ maxCommands: maxEvents });
  const events = new EventInspector({ maxEvents });
  const performance = new PerformanceTracker({ maxFrames: maxEvents });
  const tree = new TreeInspector();
  const scheduler = new SchedulerInspector();
  const focus = new FocusInspector();
  const capabilities = new CapabilityInspector();
  const timeline = new Timeline({ maxEntries: maxEvents });
  const snapshots = new SnapshotManager();

  return {
    enabled: true,
    logger,
    commands,
    events,
    performance,
    tree,
    scheduler,
    focus,
    capabilities,
    timeline,
    snapshots,

    recordCommand(type, payload, duration) {
      commands.record(type, payload, duration);
      timeline.recordCommand(type, duration);
      logger.debug("command", `Command: ${type}`, payload);
    },

    recordFrame(opts) {
      performance.recordFrame({
        duration: opts.duration,
        commandCount: opts.commandCount ?? 0,
        dirtyRegionCount: opts.dirtyRegionCount ?? 0,
        renderDuration: opts.renderDuration,
        layoutDuration: opts.layoutDuration,
        paintDuration: opts.paintDuration,
        ffiDuration: opts.ffiDuration,
      });
      timeline.recordRender(opts.duration);
    },

    recordKeyboard(key, modifiers, target) {
      events.recordKeyboard(key, modifiers, target);
      /* c8 ignore next — ?? null fallback for undefined target */
      const kbdTarget: string | null = target ?? null;
      timeline.recordEvent("keyboard", "keydown", { key, modifiers, target: kbdTarget });
    },

    recordMouse(type, x, y, button, target) {
      events.recordMouse(type, x, y, button, target);
      /* c8 ignore next — ?? null fallback for undefined button */
      const safeButton: string | null = button ?? null;
      /* c8 ignore next — ?? null fallback for undefined target */
      const safeTarget: string | null = target ?? null;
      timeline.recordEvent("mouse", type, { x, y, button: safeButton, target: safeTarget });
    },

    recordFocus(type, nodeId) {
      events.recordFocus(type, nodeId);
      if (type === "focus") {
        focus.recordFocus(nodeId);
      } else {
        focus.recordBlur(nodeId);
      }
      timeline.recordEvent("focus", type, { nodeId });
    },

    recordResize(width, height, prevWidth, prevHeight) {
      events.recordResize(width, height, prevWidth, prevHeight);
      /* c8 ignore next — ?? null fallback for undefined prevWidth/prevHeight */
      const rPrevWidth: number | null = prevWidth ?? null;
      /* c8 ignore next — ?? null fallback for undefined prevWidth/prevHeight */
      const rPrevHeight: number | null = prevHeight ?? null;
      timeline.recordEvent("resize", "resize", {
        width,
        height,
        prevWidth: rPrevWidth,
        prevHeight: rPrevHeight,
      });
    },

    updateCapabilities(caps) {
      capabilities.update(caps);
    },

    updateScheduler(state) {
      scheduler.updateState(state);
    },

    captureSnapshot(tree) {
      const snap = snapshots.capture(tree);
      return snap.id;
    },

    exportData(options) {
      const root = tree.getRoot();
      return createExport(
        {
          logs: logger.getEntries(),
          commands: commands.getCommands(),
          events: events.getEvents(),
          frames: performance.getFrames(),
          performance: performance.getSnapshot(),
          ...(root !== null ? { tree: root } : {}),
          scheduler: scheduler.getSnapshot(),
          focus: focus.getSnapshot(),
          capabilities: capabilities.get(),
          timeline: timeline.getEntries(),
          snapshots: snapshots.getSnapshots(),
        },
        options,
      );
    },

    exportJson(options) {
      return exportToJson(this.exportData(options));
    },

    getSummary() {
      return createSummary(this.exportData());
    },

    reset() {
      logger.clear();
      commands.clear();
      events.clear();
      performance.clear();
      tree.clear();
      scheduler.clear();
      focus.clear();
      capabilities.clear();
      timeline.clear();
      snapshots.clear();
    },

    dispose() {
      this.reset();
    },
  };
}
