import type { EventCategory, TimelineEntry } from "./types";

export interface TimelineOptions {
  maxEntries?: number | undefined;
  onEntry?: ((entry: TimelineEntry) => void) | undefined;
}

export class Timeline {
  private entries: TimelineEntry[] = [];
  private nextId = 0;
  private maxEntries: number;
  private onEntry: ((entry: TimelineEntry) => void) | undefined;

  constructor(options: TimelineOptions = {}) {
    this.maxEntries = options.maxEntries ?? 5000;
    this.onEntry = options.onEntry;
  }

  record(
    category: TimelineEntry["category"],
    type: string,
    duration?: number,
    data?: unknown,
  ): TimelineEntry {
    const entry: TimelineEntry = {
      id: this.nextId++,
      timestamp: performance.now(),
      category,
      type,
      duration,
      data,
    };

    this.entries.push(entry);
    if (this.entries.length > this.maxEntries) {
      this.entries.shift();
    }

    this.onEntry?.(entry);
    return entry;
  }

  recordRender(duration: number, data?: unknown): TimelineEntry {
    return this.record("render", "frame", duration, data);
  }

  recordCommand(type: string, duration?: number): TimelineEntry {
    return this.record("command", type, duration);
  }

  recordEvent(category: EventCategory, type: string, data?: unknown): TimelineEntry {
    return this.record(category, type, undefined, data);
  }

  getEntries(): readonly TimelineEntry[] {
    return this.entries;
  }

  getEntriesByCategory(category: TimelineEntry["category"]): TimelineEntry[] {
    return this.entries.filter((e) => e.category === category);
  }

  getEntriesInRange(start: number, end: number): TimelineEntry[] {
    return this.entries.filter((e) => e.timestamp >= start && e.timestamp <= end);
  }

  getRecent(count: number): TimelineEntry[] {
    return this.entries.slice(-count);
  }

  /** Get entries grouped by time windows */
  getGroupedByWindow(
    windowMs: number,
  ): Array<{ start: number; end: number; entries: TimelineEntry[] }> {
    if (this.entries.length === 0) return [];

    const groups: Array<{ start: number; end: number; entries: TimelineEntry[] }> = [];
    let currentGroup: { start: number; end: number; entries: TimelineEntry[] } | null = null;

    for (const entry of this.entries) {
      if (!currentGroup || entry.timestamp - currentGroup.start >= windowMs) {
        if (currentGroup) groups.push(currentGroup);
        currentGroup = { start: entry.timestamp, end: entry.timestamp, entries: [entry] };
      } else {
        currentGroup.entries.push(entry);
        currentGroup.end = entry.timestamp;
      }
    }

    if (currentGroup) groups.push(currentGroup);
    return groups;
  }

  clear(): void {
    this.entries = [];
  }

  get count(): number {
    return this.entries.length;
  }
}
