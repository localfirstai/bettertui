export type LogLevel = "trace" | "debug" | "info" | "warn" | "error";

export interface LoggerConfig {
  level?: LogLevel;
  color?: boolean;
  timestamp?: boolean;
  module?: boolean;
  thread?: boolean;
  /**
   * Explicit log file path. When set, file logging is enabled in any mode and
   * writes to exactly this path. Overridden by the `BETTERTUI_LOG_DIR` env var.
   */
  file?: string;
  maxFileSize?: number;
  maxFiles?: number;
  /**
   * Development mode. When `true` and no explicit `file` is given, logs are
   * written to a daily file under the repo-root `logs/` directory (and still
   * mirrored to the terminal). When `false` (production), file logging stays
   * OFF unless an explicit `file` path (or `BETTERTUI_LOG_DIR`) is provided.
   *
   * `CliRenderer` defaults this to `process.env.NODE_ENV !== "production"` when
   * the caller does not set it explicitly.
   */
  dev?: boolean;
}

export interface DiagnosticSnapshot {
  renderCalls: number;
  renderBytes: number;
  eventDispatches: number;
  layoutComputations: number;
  cacheHits: number;
  cacheMisses: number;
  allocations: number;
  averageFrameTime: number;
  fps: number;
}

export interface Logger {
  init(config: LoggerConfig): void;
  setLevel(level: LogLevel): void;
  getLevel(): LogLevel;
  setModuleFilter(include?: string[], exclude?: string[]): void;
  getDiagnostics(): DiagnosticSnapshot;
  flush(): void;
}

export function cacheHitRatio(snapshot: DiagnosticSnapshot): number {
  const total = snapshot.cacheHits + snapshot.cacheMisses;
  return total === 0 ? 0 : snapshot.cacheHits / total;
}
