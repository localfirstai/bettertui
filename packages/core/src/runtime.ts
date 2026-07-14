import type { Command } from "./command";
import { CommandBuffer } from "./command";

export interface CommandRuntimeOptions {
  frameIntervalMs?: number;
  autoStart?: boolean;
}

export class CommandRuntime {
  private buffer: CommandBuffer;
  private running = false;
  private frameHandle: ReturnType<typeof setTimeout> | null = null;
  private subscribers: Array<(commands: Command[]) => void> = [];
  private frameCallbacks: Array<(deltaMs: number) => void> = [];
  private lastFrameTime = 0;
  private frameIntervalMs: number;

  constructor(bufferOrOptions?: CommandBuffer | CommandRuntimeOptions) {
    if (bufferOrOptions instanceof CommandBuffer) {
      this.buffer = bufferOrOptions;
      this.frameIntervalMs = 16;
    } else {
      this.buffer = new CommandBuffer();
      this.frameIntervalMs = bufferOrOptions?.frameIntervalMs ?? 16;
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
    this.lastFrameTime = performance.now();
    const tick = () => {
      if (!this.running) return;
      const now = performance.now();
      const delta = now - this.lastFrameTime;
      this.lastFrameTime = now;
      for (const cb of this.frameCallbacks) {
        cb(delta);
      }
      this.flush();
      this.frameHandle = setTimeout(tick, this.frameIntervalMs);
    };
    tick();
  }

  stopFrameLoop(): void {
    this.running = false;
    if (this.frameHandle !== null) {
      clearTimeout(this.frameHandle);
      this.frameHandle = null;
    }
  }

  requestFrame(): void {
    if (!this.running) return;
    this.flush();
  }

  dispose(): void {
    this.stopFrameLoop();
    this.subscribers = [];
    this.frameCallbacks = [];
    this.buffer.clear();
  }
}
