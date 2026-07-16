export type TimerHandle = ReturnType<typeof globalThis.setTimeout>;

export interface Clock {
  now(): number;
  setTimeout(fn: () => void, delayMs: number): TimerHandle;
  clearTimeout(handle: TimerHandle): void;
  setInterval(fn: () => void, delayMs: number): TimerHandle;
  clearInterval(handle: TimerHandle): void;
}

export class SystemClock implements Clock {
  now(): number {
    return globalThis.performance.now();
  }

  setTimeout(fn: () => void, delayMs: number): TimerHandle {
    return globalThis.setTimeout(fn, delayMs);
  }

  clearTimeout(handle: TimerHandle): void {
    globalThis.clearTimeout(handle);
  }

  setInterval(fn: () => void, delayMs: number): TimerHandle {
    return globalThis.setInterval(fn, delayMs);
  }

  clearInterval(handle: TimerHandle): void {
    globalThis.clearInterval(handle);
  }
}

export class TestClock implements Clock {
  private _now = 0;
  private timeouts: Array<{ at: number; fn: () => void; id: number }> = [];
  private intervals: Array<{ at: number; fn: () => void; id: number; delayMs: number }> = [];
  private nextId = 1;

  now(): number {
    return this._now;
  }

  advance(ms: number): void {
    this._now += ms;
    this.tick();
  }

  setTime(time: number): void {
    this._now = time;
  }

  private tick(): void {
    const dueTimeouts = this.timeouts.filter((t) => t.at <= this._now);
    this.timeouts = this.timeouts.filter((t) => t.at > this._now);
    for (const t of dueTimeouts) {
      t.fn();
    }

    const dueIntervals = this.intervals.filter((t) => t.at <= this._now);
    for (const t of dueIntervals) {
      t.fn();
      t.at = this._now + t.delayMs;
    }
  }

  setTimeout(fn: () => void, delayMs: number): TimerHandle {
    const id = this.nextId++;
    this.timeouts.push({ at: this._now + delayMs, fn, id });
    return id as unknown as TimerHandle;
  }

  clearTimeout(handle: TimerHandle): void {
    const id = handle as unknown as number;
    this.timeouts = this.timeouts.filter((t) => t.id !== id);
  }

  setInterval(fn: () => void, delayMs: number): TimerHandle {
    const id = this.nextId++;
    this.intervals.push({ at: this._now + delayMs, fn, id, delayMs });
    return id as unknown as TimerHandle;
  }

  clearInterval(handle: TimerHandle): void {
    const id = handle as unknown as number;
    this.intervals = this.intervals.filter((t) => t.id !== id);
  }
}
