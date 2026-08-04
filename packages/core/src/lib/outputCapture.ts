import { EventEmitter } from "node:events";
import { Writable } from "node:stream";

export type CapturedOutput = {
  stream: "stdout" | "stderr";
  output: string;
};

export class Capture extends EventEmitter {
  private outputCache: CapturedOutput[] = [];

  get size(): number {
    return this.outputCache.length;
  }

  write(stream: "stdout" | "stderr", data: string): void {
    this.outputCache.push({ stream, output: data });
    this.emit("write", stream, data);
  }

  claimOutput(): string {
    const output = this.outputCache.map((o) => o.output).join("");
    this.clear();
    return output;
  }

  clear(): void {
    this.outputCache = [];
  }
}

export class CapturedWritableStream extends Writable {
  public isTTY = true;
  public columns: number = process.stdout?.columns || 80;
  public rows: number = process.stdout?.rows || 24;

  constructor(
    private stream: "stdout" | "stderr",
    private captureInstance: Capture,
  ) {
    super();
  }

  _write(
    chunk: unknown,
    _encoding: BufferEncoding,
    callback: (error?: Error | null) => void,
  ): void {
    const data = typeof chunk === "string" ? chunk : String(chunk);
    this.captureInstance.write(this.stream, data);
    callback();
  }

  getColorDepth(): number {
    return process.stdout?.getColorDepth?.() || 8;
  }
}
