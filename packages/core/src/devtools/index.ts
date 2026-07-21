// ─── Types ───────────────────────────────────────────────────────────────────
export type {
  LogLevel as DevToolsLogLevel,
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
  TerminalCapabilities as DevToolsTerminalCapabilities,
  TimelineEntry,
  TreeSnapshot,
  SnapshotDiff,
  DiagnosticExport,
} from "./devtools.types";

// ─── Modules ─────────────────────────────────────────────────────────────────
export { Logger as DevToolsLogger } from "./logger";
export type { LoggerOptions } from "./logger";
export { CommandInspector } from "./commandInspector";
export type { CommandInspectorOptions } from "./commandInspector";
export { EventInspector } from "./eventInspector";
export type { EventInspectorOptions } from "./eventInspector";
export { PerformanceTracker } from "./performance";
export type { PerformanceTrackerOptions } from "./performance";
export { TreeInspector } from "./treeInspector";
export type { TreeInspectorOptions } from "./treeInspector";
export { SchedulerInspector } from "./schedulerInspector";
export type { SchedulerInspectorOptions } from "./schedulerInspector";
export { FocusInspector } from "./focusInspector";
export type { FocusInspectorOptions } from "./focusInspector";
export { CapabilityInspector } from "./capabilityInspector";
export type { CapabilityInspectorOptions } from "./capabilityInspector";
export { Timeline } from "./timeline";
export type { TimelineOptions } from "./timeline";
export { SnapshotManager } from "./snapshot";
export type { SnapshotOptions } from "./snapshot";
export { createExport, exportToJson, createSummary } from "./export";
export type { ExportOptions, ExportData } from "./export";

// ─── Overlay ─────────────────────────────────────────────────────────────────
export { OverlayHost } from "./overlay/overlayHost";
export type { OverlayHostOptions, OverlayCorner } from "./overlay/overlayHost";
export { DebugPanel } from "./overlay/panel.types";
export type { Panel, PanelContext } from "./overlay/panel.types";
export * as ansi from "./overlay/ansi.utils";

// ─── DevTools Interface ──────────────────────────────────────────────────────

import type {
  DiagnosticExport,
  FrameMetrics,
  LogEntry,
  PerformanceSnapshot,
  RecordedEvent,
  SchedulerSnapshot,
  TerminalCapabilities,
  TreeNode,
} from "./devtools.types";
import type { ExportOptions } from "./export";
import { DebugPanel } from "./overlay/panel.types";

import { CapabilityInspector } from "./capabilityInspector";
import { CommandInspector } from "./commandInspector";
import { EventInspector } from "./eventInspector";
import { createExport, createSummary, exportToJson } from "./export";
import { FocusInspector } from "./focusInspector";
import { Logger } from "./logger";
import { PerformanceTracker } from "./performance";
import { SchedulerInspector } from "./schedulerInspector";
import { SnapshotManager } from "./snapshot";
import { Timeline } from "./timeline";
import { TreeInspector } from "./treeInspector";

/** Memory statistics captured from the Node.js process. */
export interface MemoryStats {
  heapUsed: number;
  heapTotal: number;
  external: number;
  rss: number;
  arrayBuffers: number;
}

/** A lightweight console surface backed by the DevTools logger. */
export interface DebugConsole {
  log(...args: unknown[]): void;
  info(...args: unknown[]): void;
  warn(...args: unknown[]): void;
  error(...args: unknown[]): void;
  debug(...args: unknown[]): void;
  /** Recent console entries, most-recent last. */
  entries(): readonly LogEntry[];
  clear(): void;
}

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

  /** Lightweight console surface backed by the logger */
  readonly console: DebugConsole;

  // ─── Panel control (§6.3) ────────────────────────────────────────────────

  /** Panels the host overlay should currently render. */
  readonly visiblePanels: ReadonlySet<DebugPanel>;

  /** Show a debug panel. */
  show(panel: DebugPanel): void;

  /** Hide a debug panel. */
  hide(panel: DebugPanel): void;

  /** Toggle a debug panel's visibility. Returns the new visibility. */
  toggle(panel: DebugPanel): boolean;

  /** Whether a given panel is currently visible. */
  isVisible(panel: DebugPanel): boolean;

  // ─── Queries (§6.3) ──────────────────────────────────────────────────────

  /** Current performance snapshot. */
  getStats(): PerformanceSnapshot;

  /** Begin capturing a profiling window. */
  startProfiling(): void;

  /** Stop the current profiling window and return the frames captured. */
  stopProfiling(): readonly FrameMetrics[];

  /** Inspect a node by id; returns its recorded tree node if known. */
  inspect(nodeId: string): TreeNode | undefined;

  /** Highlight a node (records the highlight target for the overlay). */
  highlight(nodeId: string): void;

  /** Clear any active highlight. */
  clearHighlight(): void;

  /** The currently highlighted node id, if any. */
  readonly highlightedNodeId: string | null;

  /** Enable or disable live event tracing. */
  traceEvents(enabled: boolean): void;

  /** Recorded event log. */
  getEventLog(): readonly RecordedEvent[];

  /** Layout box for a node, if recorded. */
  inspectLayout(nodeId: string): TreeNode["layout"] | undefined;

  /** Show the dirty-region panel (stats-level). */
  showDirtyRegions(enabled: boolean): void;

  /** Current process memory usage. */
  getMemoryStats(): MemoryStats;

  /** Capture a heap snapshot summary (memory stats point-in-time). */
  takeHeapSnapshot(): MemoryStats;

  // ─── Recording (existing surface) ─────────────────────────────────────────

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

const EMPTY_PANELS: ReadonlySet<DebugPanel> = new Set<DebugPanel>();

function createNoOpConsole(): DebugConsole {
  const noop = () => {};
  return {
    log: noop,
    info: noop,
    warn: noop,
    error: noop,
    debug: noop,
    entries: () => [],
    clear: noop,
  };
}

function noOpMemoryStats(): MemoryStats {
  return { heapUsed: 0, heapTotal: 0, external: 0, rss: 0, arrayBuffers: 0 };
}

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
  const noopSnapshot = (): PerformanceSnapshot => ({
    fps: 0,
    avgFrameTime: 0,
    minFrameTime: 0,
    maxFrameTime: 0,
    totalFrames: 0,
    droppedFrames: 0,
    commandCount: 0,
    dirtyNodeCount: 0,
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
    console: createNoOpConsole(),
    visiblePanels: EMPTY_PANELS,
    highlightedNodeId: null,
    show: noop,
    hide: noop,
    toggle: () => false,
    isVisible: () => false,
    getStats: noopSnapshot,
    startProfiling: noop,
    stopProfiling: () => [],
    inspect: () => undefined,
    highlight: noop,
    clearHighlight: noop,
    traceEvents: noop,
    getEventLog: () => [],
    inspectLayout: () => undefined,
    showDirtyRegions: noop,
    getMemoryStats: noOpMemoryStats,
    takeHeapSnapshot: noOpMemoryStats,
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

/** Options accepted by the `debug` field of `CliRendererOptions`. */
export type DevToolsOptions = CreateDevToolsOptions;

function readMemoryStats(): MemoryStats {
  /* c8 ignore start — process.memoryUsage shape depends on the host runtime */
  if (typeof process !== "undefined" && typeof process.memoryUsage === "function") {
    const m = process.memoryUsage();
    return {
      heapUsed: m.heapUsed,
      heapTotal: m.heapTotal,
      external: m.external,
      rss: m.rss,
      arrayBuffers: m.arrayBuffers ?? 0,
    };
  }
  return noOpMemoryStats();
  /* c8 ignore stop */
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

  const visiblePanels = new Set<DebugPanel>();
  let highlightedNodeId: string | null = null;
  let tracingEnabled = true;
  let profiling = false;
  let profileStart = 0;

  const toArgString = (args: unknown[]): string =>
    args.map((a) => (typeof a === "string" ? a : safeStringify(a))).join(" ");

  const consoleSurface: DebugConsole = {
    log: (...args) => logger.info("console", toArgString(args)),
    info: (...args) => logger.info("console", toArgString(args)),
    warn: (...args) => logger.warn("console", toArgString(args)),
    error: (...args) => logger.error("console", toArgString(args)),
    debug: (...args) => logger.debug("console", toArgString(args)),
    entries: () => logger.getEntriesByCategory("console"),
    clear: () => logger.clear(),
  };

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
    console: consoleSurface,

    get visiblePanels() {
      return visiblePanels;
    },

    get highlightedNodeId() {
      return highlightedNodeId;
    },

    show(panel) {
      visiblePanels.add(panel);
    },

    hide(panel) {
      visiblePanels.delete(panel);
    },

    toggle(panel) {
      if (visiblePanels.has(panel)) {
        visiblePanels.delete(panel);
        return false;
      }
      visiblePanels.add(panel);
      return true;
    },

    isVisible(panel) {
      return visiblePanels.has(panel);
    },

    getStats() {
      return performance.getSnapshot();
    },

    startProfiling() {
      profiling = true;
      profileStart = performance.count;
    },

    stopProfiling() {
      if (!profiling) return [];
      profiling = false;
      return performance.getFrames().slice(profileStart);
    },

    inspect(nodeId) {
      return tree.getNode(nodeId);
    },

    highlight(nodeId) {
      highlightedNodeId = nodeId;
    },

    clearHighlight() {
      highlightedNodeId = null;
    },

    traceEvents(enabled) {
      tracingEnabled = enabled;
    },

    getEventLog() {
      return events.getEvents();
    },

    inspectLayout(nodeId) {
      return tree.getNode(nodeId)?.layout;
    },

    showDirtyRegions(enabled) {
      if (enabled) {
        visiblePanels.add(DebugPanel.DirtyRegions);
      } else {
        visiblePanels.delete(DebugPanel.DirtyRegions);
      }
    },

    getMemoryStats() {
      return readMemoryStats();
    },

    takeHeapSnapshot() {
      return readMemoryStats();
    },

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
      if (!tracingEnabled) return;
      events.recordKeyboard(key, modifiers, target);
      /* c8 ignore next — ?? null fallback for undefined target */
      const kbdTarget: string | null = target ?? null;
      timeline.recordEvent("keyboard", "keydown", { key, modifiers, target: kbdTarget });
    },

    recordMouse(type, x, y, button, target) {
      if (!tracingEnabled) return;
      events.recordMouse(type, x, y, button, target);
      /* c8 ignore next — ?? null fallback for undefined button */
      const safeButton: string | null = button ?? null;
      /* c8 ignore next — ?? null fallback for undefined target */
      const safeTarget: string | null = target ?? null;
      timeline.recordEvent("mouse", type, { x, y, button: safeButton, target: safeTarget });
    },

    recordFocus(type, nodeId) {
      if (!tracingEnabled) return;
      events.recordFocus(type, nodeId);
      if (type === "focus") {
        focus.recordFocus(nodeId);
      } else {
        focus.recordBlur(nodeId);
      }
      timeline.recordEvent("focus", type, { nodeId });
    },

    recordResize(width, height, prevWidth, prevHeight) {
      if (!tracingEnabled) return;
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
      visiblePanels.clear();
      highlightedNodeId = null;
    },

    dispose() {
      this.reset();
    },
  };
}

// ─── Local helpers ─────────────────────────────────────────────────────────────

function safeStringify(value: unknown): string {
  try {
    return JSON.stringify(value);
  } catch {
    /* c8 ignore next — circular/unserialisable values fall back to String() */
    return String(value);
  }
}
