import type { EventCategory, RecordedEvent } from "./types";

export interface EventInspectorOptions {
  maxEvents?: number | undefined;
  onEvent?: ((event: RecordedEvent) => void) | undefined;
}

export class EventInspector {
  private events: RecordedEvent[] = [];
  private nextId = 0;
  private maxEvents: number;
  private onEvent: ((event: RecordedEvent) => void) | undefined;
  private categoryCounts = new Map<string, number>();

  constructor(options: EventInspectorOptions = {}) {
    this.maxEvents = options.maxEvents ?? 5000;
    this.onEvent = options.onEvent;
  }

  record(
    category: EventCategory,
    type: string,
    target?: string,
    data?: unknown,
    propagation?: "captured" | "target" | "bubbled",
  ): RecordedEvent {
    const event: RecordedEvent = {
      id: this.nextId++,
      timestamp: performance.now(),
      category,
      type,
      target,
      data,
      propagation,
    };

    this.events.push(event);
    if (this.events.length > this.maxEvents) {
      this.events.shift();
    }

    this.categoryCounts.set(category, (this.categoryCounts.get(category) ?? 0) + 1);
    this.onEvent?.(event);

    return event;
  }

  recordKeyboard(
    key: string,
    modifiers: { ctrl: boolean; shift: boolean; alt: boolean; meta: boolean },
    target?: string,
  ): RecordedEvent {
    return this.record("keyboard", "keydown", target, { key, modifiers });
  }

  recordMouse(type: string, x: number, y: number, button?: string, target?: string): RecordedEvent {
    return this.record("mouse", type, target, { x, y, button });
  }

  recordFocus(type: "focus" | "blur", nodeId: string): RecordedEvent {
    return this.record("focus", type, nodeId);
  }

  recordResize(
    width: number,
    height: number,
    prevWidth?: number,
    prevHeight?: number,
  ): RecordedEvent {
    return this.record("resize", "resize", undefined, { width, height, prevWidth, prevHeight });
  }

  recordLifecycle(type: string, data?: unknown): RecordedEvent {
    return this.record("lifecycle", type, undefined, data);
  }

  getEvents(): readonly RecordedEvent[] {
    return this.events;
  }

  getEventsByCategory(category: EventCategory): RecordedEvent[] {
    return this.events.filter((e) => e.category === category);
  }

  getEventsByType(type: string): RecordedEvent[] {
    return this.events.filter((e) => e.type === type);
  }

  getEventsInRange(start: number, end: number): RecordedEvent[] {
    return this.events.filter((e) => e.timestamp >= start && e.timestamp <= end);
  }

  getCategoryCounts(): Map<string, number> {
    return new Map(this.categoryCounts);
  }

  getRecent(count: number): RecordedEvent[] {
    return this.events.slice(-count);
  }

  clear(): void {
    this.events = [];
    this.categoryCounts.clear();
  }

  get count(): number {
    return this.events.length;
  }
}
