import type {
  DiagnosticExport,
  FocusSnapshot,
  FrameMetrics,
  LogEntry,
  PerformanceSnapshot,
  RecordedCommand,
  RecordedEvent,
  SchedulerSnapshot,
  TerminalCapabilities,
  TimelineEntry,
  TreeNode,
  TreeSnapshot,
} from "./types";

export interface ExportOptions {
  /** Include logs in the export */
  includeLogs?: boolean | undefined;
  /** Include commands in the export */
  includeCommands?: boolean | undefined;
  /** Include events in the export */
  includeEvents?: boolean | undefined;
  /** Include frame metrics in the export */
  includeFrames?: boolean | undefined;
  /** Include timeline in the export */
  includeTimeline?: boolean | undefined;
  /** Include snapshots in the export */
  includeSnapshots?: boolean | undefined;
}

export interface ExportData {
  /** Logs to include */
  logs?: readonly LogEntry[] | undefined;
  /** Commands to include */
  commands?: readonly RecordedCommand[] | undefined;
  /** Events to include */
  events?: readonly RecordedEvent[] | undefined;
  /** Frame metrics to include */
  frames?: readonly FrameMetrics[] | undefined;
  /** Performance snapshot */
  performance?: PerformanceSnapshot | undefined;
  /** Render tree */
  tree?: TreeNode | undefined;
  /** Scheduler snapshot */
  scheduler?: SchedulerSnapshot | undefined;
  /** Focus snapshot */
  focus?: FocusSnapshot | undefined;
  /** Terminal capabilities */
  capabilities?: TerminalCapabilities | undefined;
  /** Timeline entries */
  timeline?: readonly TimelineEntry[] | undefined;
  /** Tree snapshots */
  snapshots?: readonly TreeSnapshot[] | undefined;
}

/** Create a diagnostic export from collected data */
export function createExport(data: ExportData, options: ExportOptions = {}): DiagnosticExport {
  const {
    includeLogs = true,
    includeCommands = true,
    includeEvents = true,
    includeFrames = true,
    includeTimeline = true,
    includeSnapshots = true,
  } = options;

  return {
    version: "1.0.0",
    timestamp: performance.now(),
    duration: 0,
    logs: includeLogs ? (data.logs ?? []) : [],
    commands: includeCommands ? (data.commands ?? []) : [],
    events: includeEvents ? (data.events ?? []) : [],
    frames: includeFrames ? (data.frames ?? []) : [],
    performance: data.performance ?? {
      fps: 0,
      avgFrameTime: 0,
      minFrameTime: 0,
      maxFrameTime: 0,
      totalFrames: 0,
      droppedFrames: 0,
      commandCount: 0,
      dirtyNodeCount: 0,
    },
    tree: data.tree,
    scheduler: data.scheduler,
    focus: data.focus,
    capabilities: data.capabilities,
    timeline: includeTimeline ? (data.timeline ?? []) : [],
    snapshots: includeSnapshots ? (data.snapshots ?? []) : [],
  };
}

/** Serialize a diagnostic export to JSON */
export function exportToJson(exportData: DiagnosticExport): string {
  return JSON.stringify(exportData, null, 2);
}

/** Create a summary report from a diagnostic export */
export function createSummary(exportData: DiagnosticExport): string {
  const lines: string[] = [
    "BetterTUI DevTools Diagnostic Report",
    "=====================================",
    "",
    `Version: ${exportData.version}`,
    `Duration: ${(exportData.duration / 1000).toFixed(2)}s`,
    "",
    "## Performance",
    `  FPS: ${exportData.performance.fps.toFixed(1)}`,
    `  Avg Frame Time: ${exportData.performance.avgFrameTime.toFixed(2)}ms`,
    `  Dropped Frames: ${exportData.performance.droppedFrames}/${exportData.performance.totalFrames}`,
    `  Total Commands: ${exportData.performance.commandCount}`,
    `  Avg Commands/Frame: ${exportData.performance.totalFrames > 0 ? (exportData.commands.length / exportData.performance.totalFrames).toFixed(2) : "0.00"}`,
    "",
    "## Activity",
    `  Logs: ${exportData.logs.length}`,
    `  Commands: ${exportData.commands.length}`,
    `  Events: ${exportData.events.length}`,
    `  Timeline Entries: ${exportData.timeline.length}`,
    `  Snapshots: ${exportData.snapshots.length}`,
  ];

  if (exportData.scheduler) {
    lines.push(
      "",
      "## Scheduler",
      `  Frame Count: ${exportData.scheduler.frameCount}`,
      `  Dropped: ${exportData.scheduler.droppedFrames}`,
      `  Utilization: ${(exportData.scheduler.utilization * 100).toFixed(1)}%`,
    );
  }

  if (exportData.focus) {
    lines.push(
      "",
      "## Focus",
      `  Focused: ${exportData.focus.focusedNodeId ?? "none"}`,
      `  Focusable Nodes: ${exportData.focus.focusableNodes.length}`,
    );
  }

  if (exportData.capabilities) {
    lines.push(
      "",
      "## Terminal",
      `  Brand: ${exportData.capabilities.terminalBrand}`,
      `  Size: ${exportData.capabilities.terminalSize.columns}x${exportData.capabilities.terminalSize.rows}`,
      `  True Color: ${exportData.capabilities.trueColor}`,
    );
  }

  return lines.join("\n");
}
