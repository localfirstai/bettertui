import { type Command, CommandBuffer } from "@bettertui/reconciler";

export class Runtime {
  private buffer: CommandBuffer;
  private running = false;
  private frameHandle: ReturnType<typeof setTimeout> | null = null;
  private subscribers: Array<(commands: Command[]) => void> = [];

  constructor(buffer?: CommandBuffer) {
    this.buffer = buffer ?? new CommandBuffer();
  }

  get commandBuffer(): CommandBuffer {
    return this.buffer;
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

  startFrameLoop(intervalMs = 16): void {
    if (this.running) return;
    this.running = true;
    const tick = () => {
      if (!this.running) return;
      this.flush();
      this.frameHandle = setTimeout(tick, intervalMs);
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

  dispose(): void {
    this.stopFrameLoop();
    this.subscribers = [];
    this.buffer.clear();
  }
}
