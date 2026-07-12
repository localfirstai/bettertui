import type { LogEntry, LogLevel } from "./types";

const LOG_LEVEL_PRIORITY: Record<LogLevel, number> = {
  trace: 0,
  debug: 1,
  info: 2,
  warn: 3,
  error: 4,
};

export interface LoggerOptions {
  maxEntries?: number;
  minLevel?: LogLevel | undefined;
  onEntry?: ((entry: LogEntry) => void) | undefined;
}

export class Logger {
  private entries: LogEntry[] = [];
  private nextId = 0;
  private minLevel: LogLevel;
  private maxEntries: number;
  private onEntry: ((entry: LogEntry) => void) | undefined;

  constructor(options: LoggerOptions = {}) {
    this.maxEntries = options.maxEntries ?? 1000;
    this.minLevel = options.minLevel ?? "trace";
    this.onEntry = options.onEntry;
  }

  private shouldLog(level: LogLevel): boolean {
    return LOG_LEVEL_PRIORITY[level] >= LOG_LEVEL_PRIORITY[this.minLevel];
  }

  private record(level: LogLevel, category: string, message: string, data?: unknown): LogEntry {
    const entry: LogEntry = {
      id: this.nextId++,
      timestamp: performance.now(),
      level,
      category,
      message,
      data,
    };

    if (this.shouldLog(level)) {
      this.entries.push(entry);
      if (this.entries.length > this.maxEntries) {
        this.entries.shift();
      }
      this.onEntry?.(entry);
    }

    return entry;
  }

  trace(category: string, message: string, data?: unknown): LogEntry {
    return this.record("trace", category, message, data);
  }

  debug(category: string, message: string, data?: unknown): LogEntry {
    return this.record("debug", category, message, data);
  }

  info(category: string, message: string, data?: unknown): LogEntry {
    return this.record("info", category, message, data);
  }

  warn(category: string, message: string, data?: unknown): LogEntry {
    return this.record("warn", category, message, data);
  }

  error(category: string, message: string, data?: unknown): LogEntry {
    return this.record("error", category, message, data);
  }

  getEntries(): readonly LogEntry[] {
    return this.entries;
  }

  getEntriesByLevel(level: LogLevel): LogEntry[] {
    return this.entries.filter((e) => e.level === level);
  }

  getEntriesByCategory(category: string): LogEntry[] {
    return this.entries.filter((e) => e.category === category);
  }

  search(query: string): LogEntry[] {
    const lower = query.toLowerCase();
    return this.entries.filter(
      (e) => e.message.toLowerCase().includes(lower) || e.category.toLowerCase().includes(lower),
    );
  }

  clear(): void {
    this.entries = [];
  }

  get count(): number {
    return this.entries.length;
  }
}
