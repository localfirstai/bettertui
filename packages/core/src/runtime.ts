import type { Command } from "./command";
import { CommandBuffer } from "./command";
import { SystemClock } from "./lib/clock";
import type { Clock, TimerHandle } from "./lib/clock";
import type { NapiEngine } from "./platform/binding";
import { detectCapabilities } from "./platform/binding";

export interface CommandRuntimeOptions {
  frameIntervalMs?: number;
  autoStart?: boolean;
  engine?: NapiEngine;
  clock?: Clock;
}

export class CommandRuntime {
  private buffer: CommandBuffer;
  private running = false;
  private frameHandle: TimerHandle | null = null;
  private subscribers: Array<(commands: Command[]) => void> = [];
  private frameCallbacks: Array<(deltaMs: number) => void> = [];
  private lastFrameTime = 0;
  private frameIntervalMs: number;
  private engine: NapiEngine | undefined;
  private width: number;
  private height: number;
  private clock: Clock;

  constructor(bufferOrOptions?: CommandBuffer | CommandRuntimeOptions) {
    this.clock = new SystemClock();
    if (bufferOrOptions instanceof CommandBuffer) {
      this.buffer = bufferOrOptions;
      this.frameIntervalMs = 16;
      this.width = 80;
      this.height = 24;
    } else {
      this.buffer = new CommandBuffer();
      this.frameIntervalMs = bufferOrOptions?.frameIntervalMs ?? 16;
      this.engine = bufferOrOptions?.engine;
      if (bufferOrOptions?.clock) {
        this.clock = bufferOrOptions.clock;
      }
      const caps = detectCapabilities();
      this.width = caps.columns;
      this.height = caps.rows;
      if (bufferOrOptions?.autoStart) {
        this.startFrameLoop();
      }
    }
  }

  get commandBuffer(): CommandBuffer {
    return this.buffer;
  }

  get isRunning(): boolean {
    return this.running;
  }

  get terminalWidth(): number {
    return this.width;
  }

  get terminalHeight(): number {
    return this.height;
  }

  drain(): Command[] {
    return this.buffer.drain();
  }

  flush(): void {
    const commands = this.drain();
    if (commands.length > 0) {
      for (const sub of this.subscribers) {
        sub(commands);
      }
    }
  }

  subscribe(fn: (commands: Command[]) => void): () => void {
    this.subscribers.push(fn);
    return () => {
      this.subscribers = this.subscribers.filter((s) => s !== fn);
    };
  }

  onFrame(callback: (deltaMs: number) => void): () => void {
    this.frameCallbacks.push(callback);
    return () => {
      this.frameCallbacks = this.frameCallbacks.filter((cb) => cb !== callback);
    };
  }

  startFrameLoop(intervalMs?: number): void {
    if (this.running) return;
    this.running = true;
    if (intervalMs !== undefined) {
      this.frameIntervalMs = intervalMs;
    }
    this.lastFrameTime = this.clock.now();
    const tick = () => {
      if (!this.running) return;
      const now = this.clock.now();
      const delta = now - this.lastFrameTime;
      this.lastFrameTime = now;
      for (const cb of this.frameCallbacks) {
        cb(delta);
      }
      this.flush();
      if (this.running) {
        this.frameHandle = this.clock.setTimeout(tick, this.frameIntervalMs);
      }
    };
    tick();
  }

  stopFrameLoop(): void {
    this.running = false;
    if (this.frameHandle !== null) {
      this.clock.clearTimeout(this.frameHandle);
      this.frameHandle = null;
    }
  }

  requestFrame(): void {
    if (!this.running) return;
    this.flush();
  }

  resize(width: number, height: number): void {
    this.width = width;
    this.height = height;
    this.engine?.resize(width, height);
  }

  render(): { outputData: Buffer; width: number; height: number } | null {
    if (!this.engine) return null;
    this.engine.beginFrame();
    const frame = this.engine.render();
    this.engine.commitFrame();
    if (frame.output_data) {
      return {
        outputData: Buffer.from(frame.output_data, "base64"),
        width: frame.width,
        height: frame.height,
      };
    }
    return null;
  }

  dispose(): void {
    this.stopFrameLoop();
    this.subscribers = [];
    this.frameCallbacks = [];
    this.buffer.clear();
    this.engine?.shutdown();
  }
}
