import { type CellAttributes, CellMatrix } from "./cell-matrix";

export interface TestTerminalOptions {
  width?: number;
  height?: number;
}

export interface CapturedFrame {
  textFrame: string;
  ansiFrame: string;
  width: number;
  height: number;
  timestamp: number;
}

export type TerminalEventListener = (event: string, data: unknown) => void;

export class TestTerminal {
  public readonly matrix: CellMatrix;
  private listeners: Map<string, TerminalEventListener[]> = new Map();
  private focusedNodeId: string | null = null;
  private frameCount = 0;

  constructor(options: TestTerminalOptions = {}) {
    const width = options.width ?? 80;
    const height = options.height ?? 24;
    this.matrix = new CellMatrix(width, height);
  }

  public get width(): number {
    return this.matrix.width;
  }

  public get height(): number {
    return this.matrix.height;
  }

  public resize(width: number, height: number): void {
    this.matrix.resize(width, height);
    this.emit("resize", { width, height });
  }

  public getFocusedNodeId(): string | null {
    return this.focusedNodeId;
  }

  public setFocusedNodeId(nodeId: string | null): void {
    this.focusedNodeId = nodeId;
    this.emit("focusChange", { focusedNodeId: nodeId });
  }

  public on(event: string, listener: TerminalEventListener): () => void {
    const existing = this.listeners.get(event) || [];
    existing.push(listener);
    this.listeners.set(event, existing);
    return () => {
      const current = this.listeners.get(event) || [];
      this.listeners.set(
        event,
        current.filter((l) => l !== listener),
      );
    };
  }

  public emit(event: string, data: unknown): void {
    const handlers = this.listeners.get(event) || [];
    for (const handler of handlers) {
      handler(event, data);
    }
  }

  public captureFrame(): CapturedFrame {
    this.frameCount++;
    return {
      textFrame: this.matrix.renderTextFrame(),
      ansiFrame: this.matrix.renderAnsiFrame(),
      width: this.width,
      height: this.height,
      timestamp: Date.now(),
    };
  }

  public getCell(x: number, y: number): CellAttributes | undefined {
    return this.matrix.getCell(x, y);
  }

  public clear(): void {
    this.matrix.clear();
  }
}
