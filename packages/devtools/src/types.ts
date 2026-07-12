// ─── Log Levels ──────────────────────────────────────────────────────────────

export type LogLevel = "debug" | "info" | "warn" | "error" | "trace";

export interface LogEntry {
  id: number;
  timestamp: number;
  level: LogLevel;
  category: string;
  message: string;
  data?: unknown | undefined;
}

// ─── Command Types ───────────────────────────────────────────────────────────

export type CommandType =
  | "CreateNode"
  | "RemoveNode"
  | "AppendChild"
  | "InsertBefore"
  | "MoveNode"
  | "ReplaceNode"
  | "DetachNode"
  | "SetText"
  | "SetStyle"
  | "SetLayout"
  | "SetAttribute"
  | "RemoveAttribute"
  | "BeginFrame"
  | "CommitFrame"
  | "Invalidate"
  | "Shutdown"
  | string;

export interface RecordedCommand {
  id: number;
  timestamp: number;
  type: CommandType;
  payload: Record<string, unknown>;
  duration?: number | undefined;
}

// ─── Event Types ─────────────────────────────────────────────────────────────

export type EventCategory =
  | "keyboard"
  | "mouse"
  | "focus"
  | "resize"
  | "lifecycle"
  | "clipboard"
  | "animation"
  | "scheduler";

export interface RecordedEvent {
  id: number;
  timestamp: number;
  category: EventCategory;
  type: string;
  target?: string | undefined;
  data?: unknown | undefined;
  propagation?: ("captured" | "target" | "bubbled") | undefined;
}

// ─── Performance Types ───────────────────────────────────────────────────────

export interface FrameMetrics {
  frameNumber: number;
  timestamp: number;
  duration: number;
  commandCount: number;
  dirtyRegionCount: number;
  renderDuration?: number | undefined;
  layoutDuration?: number | undefined;
  paintDuration?: number | undefined;
  ffiDuration?: number | undefined;
}

export interface PerformanceSnapshot {
  fps: number;
  avgFrameTime: number;
  minFrameTime: number;
  maxFrameTime: number;
  totalFrames: number;
  droppedFrames: number;
  commandCount: number;
  dirtyNodeCount: number;
  memoryUsage?:
    | {
        heapUsed: number;
        heapTotal: number;
        external: number;
      }
    | undefined;
}

// ─── Tree Types ──────────────────────────────────────────────────────────────

export interface TreeNode {
  id: string;
  type: string;
  props: Record<string, unknown>;
  style?: Record<string, unknown> | undefined;
  layout?:
    | {
        x: number;
        y: number;
        width: number;
        height: number;
      }
    | undefined;
  children: TreeNode[];
  parent?: string | undefined;
  dirty?: boolean | undefined;
  visible?: boolean | undefined;
  zIndex?: number | undefined;
}

// ─── Scheduler Types ─────────────────────────────────────────────────────────

export interface SchedulerSnapshot {
  isRunning: boolean;
  isRendering: boolean;
  hasScheduledRender: boolean;
  frameCount: number;
  droppedFrames: number;
  pendingFrames: number;
  highestPriority: string;
  idleCallbacksPending: number;
  animationFramesPending: number;
  frameBudgetMs: number;
  utilization: number;
}

// ─── Focus Types ─────────────────────────────────────────────────────────────

export interface FocusSnapshot {
  focusedNodeId: string | null;
  previousNodeId: string | null;
  focusableNodes: string[];
  tabOrder: string[];
  currentScope: string | null;
}

// ─── Terminal Capability Types ───────────────────────────────────────────────

export interface TerminalCapabilities {
  trueColor: boolean;
  kittyKeyboard: boolean;
  mouseSupport: boolean;
  osc52: boolean;
  osc8: boolean;
  pixelSupport: boolean;
  alternateScreen: boolean;
  terminalBrand: string;
  terminalSize: { columns: number; rows: number };
  syncUpdate: boolean;
  bracketedPaste: boolean;
  focusEvents: boolean;
  strikethrough: boolean;
  underlineColor: boolean;
  cursorStyle: boolean;
  hyperlinks: boolean;
  inlineImages: boolean;
  sixel: boolean;
}

// ─── Timeline Types ──────────────────────────────────────────────────────────

export interface TimelineEntry {
  id: number;
  timestamp: number;
  category: EventCategory | "render" | "command" | "layout" | "paint";
  type: string;
  duration?: number | undefined;
  data?: unknown | undefined;
}

// ─── Snapshot Types ──────────────────────────────────────────────────────────

export interface TreeSnapshot {
  id: number;
  timestamp: number;
  tree: TreeNode;
  nodeCount: number;
}

export interface SnapshotDiff {
  added: string[];
  removed: string[];
  changed: Array<{ id: string; field: string; old: unknown; new: unknown }>;
}

// ─── Export Types ────────────────────────────────────────────────────────────

export interface DiagnosticExport {
  version: string;
  timestamp: number;
  duration: number;
  logs: readonly LogEntry[];
  commands: readonly RecordedCommand[];
  events: readonly RecordedEvent[];
  frames: readonly FrameMetrics[];
  performance: PerformanceSnapshot;
  tree?: TreeNode | undefined;
  scheduler?: SchedulerSnapshot | undefined;
  focus?: FocusSnapshot | undefined;
  capabilities?: TerminalCapabilities | undefined;
  timeline: readonly TimelineEntry[];
  snapshots: readonly TreeSnapshot[];
}
