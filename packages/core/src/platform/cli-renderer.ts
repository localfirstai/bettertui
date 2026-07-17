import { EventEmitter } from "node:events";
import { StdinParser } from "../lib/stdin-parser";
import type { NapiEngine, NapiKeymap, TerminalCapabilities } from "./binding";
import {
  createEngine,
  createKeymap,
  detectCapabilities,
  getVersion,
  loggerGetDiagnostics,
  loggerInit,
} from "./binding";
import type { DiagnosticSnapshot, LoggerConfig } from "./logger";
import type { ExternalOutputMode, ScreenMode } from "./types";

export interface KeyEvent {
  name: string;
  ctrl: boolean;
  shift: boolean;
  alt: boolean;
  meta: boolean;
  sequence: string;
}

export interface CliRendererOptions {
  width?: number;
  height?: number;
  exitOnCtrlC?: boolean;
  targetFps?: number;
  screenMode?: ScreenMode;
  footerHeight?: number;
  externalOutputMode?: ExternalOutputMode;
  logger?: LoggerConfig;
}

type KeyInputEvents = {
  keypress: [KeyEvent];
};

export class KeyInput extends EventEmitter<KeyInputEvents> {
  private stdinParser: StdinParser;
  private rawMode = false;
  private readonly onDataBound: (data: Buffer) => void;

  constructor() {
    super();
    this.stdinParser = new StdinParser({
      useKittyKeyboard: false, // Keep disabled to match old behavior
      onTimeoutFlush: () => {
        this.drain();
      },
    });
    this.onDataBound = this.onData.bind(this);
  }

  start(): void {
    const stdin = process.stdin;
    if (stdin.setRawMode && !this.rawMode) {
      stdin.setRawMode(true);
      this.rawMode = true;
    }
    stdin.resume();
    stdin.on("data", this.onDataBound);
  }

  stop(): void {
    process.stdin.off("data", this.onDataBound);
    if (this.rawMode && process.stdin.setRawMode) {
      process.stdin.setRawMode(false);
      this.rawMode = false;
    }
    process.stdin.pause();
    this.stdinParser.reset();
  }

  private onData(data: Buffer): void {
    this.stdinParser.push(new Uint8Array(data.buffer, data.byteOffset, data.byteLength));
    this.drain();
  }

  private drain(): void {
    this.stdinParser.drain((event) => {
      if (event.type === "key") {
        this.emit("keypress", {
          name: event.key.name,
          ctrl: event.key.ctrl,
          shift: event.key.shift,
          alt: event.key.option, // option acts as alt
          meta: event.key.meta,
          sequence: event.key.sequence,
        });
      }
    });
  }
}

export class CliRenderer {
  private engine: NapiEngine;
  private keymap: NapiKeymap;
  private _keyInput: KeyInput;
  private capabilities: TerminalCapabilities;
  private width: number;
  private height: number;
  private renderOffset: number;
  private _screenMode: ScreenMode;
  private _externalOutputMode: ExternalOutputMode;
  private externalOutputBuffer: string[] = [];
  private nodes: Map<number, { parent: number | null; children: number[] }> = new Map();
  private running = false;

  constructor(options: CliRendererOptions = {}) {
    if (options.logger) {
      // Default `dev` from NODE_ENV so file logging lands in the repo-root
      // `logs/` dir during development but stays off in production unless the
      // caller passes an explicit `file` path. An explicit `dev` still wins.
      loggerInit({ dev: process.env.NODE_ENV !== "production", ...options.logger });
    }
    this.capabilities = detectCapabilities();
    this.width = options.width ?? this.capabilities.columns;
    this.height = options.height ?? this.capabilities.rows;

    this._screenMode = options.screenMode ?? "alternate-screen";
    this._externalOutputMode =
      options.externalOutputMode ??
      (this._screenMode === "split-footer" ? "capture-stdout" : "passthrough");
    const footerHeight = options.footerHeight ?? 0;
    this.renderOffset =
      this._screenMode === "split-footer" ? Math.max(0, this.height - footerHeight) : 0;

    this.engine = createEngine(this.width, this.height);
    this.keymap = createKeymap();
    this._keyInput = new KeyInput();

    const rootId = this.engine.root();
    this.nodes.set(rootId, { parent: null, children: [] });
  }

  get terminalWidth(): number {
    return this.width;
  }

  get terminalHeight(): number {
    return this.height;
  }

  get viewportHeight(): number {
    return this._screenMode === "split-footer" ? this.renderOffset : this.height;
  }

  get screenMode(): ScreenMode {
    return this._screenMode;
  }

  get externalOutputMode(): ExternalOutputMode {
    return this._externalOutputMode;
  }

  get keyInput(): KeyInput {
    return this._keyInput;
  }

  get keyHandler(): KeyInput {
    return this._keyInput;
  }

  get version(): string {
    return getVersion();
  }

  get isRunning(): boolean {
    return this.running;
  }

  /**
   * Snapshot the engine's diagnostic counters (render calls, frame times, cache
   * hit/miss, etc.). Returns zeroed counters if the logger was never initialized.
   */
  getDiagnostics(): DiagnosticSnapshot {
    return loggerGetDiagnostics();
  }

  start(): void {
    if (this.running) return;
    this.running = true;

    if (this._screenMode === "alternate-screen") {
      this.enterAlternateScreen();
    }
    this._keyInput.start();
  }

  stop(): void {
    if (!this.running) return;
    this.running = false;
    this._keyInput.stop();

    if (this._screenMode === "split-footer") {
      this.flushExternalOutput();
    } else {
      this.exitAlternateScreen();
    }
    this.engine.shutdown();
  }

  /** Flush captured external output above the footer region. */
  private flushExternalOutput(): void {
    if (this.externalOutputBuffer.length === 0) return;
    // Move cursor above footer, replay output, restore footer position
    const output = this.externalOutputBuffer.join("");
    this.externalOutputBuffer = [];
    process.stdout.write(`\x1b[${this.renderOffset + 1};1H${output}`);
  }

  /** Capture or passthrough stdout writes depending on mode. */
  interceptStdoutWrite = (chunk: string | Uint8Array): boolean => {
    if (this._externalOutputMode === "capture-stdout") {
      this.externalOutputBuffer.push(
        typeof chunk === "string" ? chunk : Buffer.from(chunk).toString("utf8"),
      );
      return true;
    }
    return false;
  };

  createNode(kind: string): number {
    const id = this.engine.createNode(kind);
    this.nodes.set(id, { parent: null, children: [] });
    return id;
  }

  appendChild(parent: number, child: number): boolean {
    const result = this.engine.appendChild(parent, child);
    if (result) {
      const parentNode = this.nodes.get(parent);
      const childNode = this.nodes.get(child);
      if (parentNode && childNode) {
        parentNode.children.push(child);
        childNode.parent = parent;
      }
    }
    return result;
  }

  removeNode(id: number): void {
    const node = this.nodes.get(id);
    if (node) {
      if (node.parent !== null) {
        const parent = this.nodes.get(node.parent);
        if (parent) {
          parent.children = parent.children.filter((c) => c !== id);
        }
      }
      for (const child of node.children) {
        this.removeNode(child);
      }
      this.nodes.delete(id);
    }
    this.engine.removeNode(id);
  }

  setText(id: number, text: string): void {
    this.engine.setText(id, text);
  }

  clearTree(): void {
    const rootId = this.engine.root();
    const rootNode = this.nodes.get(rootId);
    if (rootNode) {
      for (const child of [...rootNode.children]) {
        this.removeNode(child);
      }
    }
  }

  /** Switch screen mode at runtime. */
  setScreenMode(mode: ScreenMode, footerHeight?: number): void {
    const oldMode = this._screenMode;
    this._screenMode = mode;

    if (mode === "split-footer") {
      this._externalOutputMode = "capture-stdout";
      this.renderOffset = Math.max(0, this.height - (footerHeight ?? 0));
      this.engine.setScreenMode("split-footer", footerHeight ?? 0);
      if (oldMode === "alternate-screen") {
        this.exitAlternateScreen();
      }
    } else if (mode === "alternate-screen") {
      this._externalOutputMode = "passthrough";
      this.renderOffset = 0;
      this.engine.setScreenMode("alternate-screen");
      this.enterAlternateScreen();
    } else {
      this._externalOutputMode = "passthrough";
      this.renderOffset = 0;
      this.engine.setScreenMode("main-screen");
      if (oldMode === "alternate-screen") {
        this.exitAlternateScreen();
      }
    }
  }

  render(): void {
    this.engine.beginFrame();
    const frame = this.engine.render();
    this.engine.commitFrame();

    if (frame.output_data) {
      const decoded = Buffer.from(frame.output_data, "base64");
      if (this._screenMode === "split-footer") {
        // Paint only in the viewport region (above footer)
        process.stdout.write(`\x1b[1;1H${decoded.toString()}`);
      } else {
        process.stdout.write(decoded);
      }
    }

    if (this._screenMode === "split-footer" && this.externalOutputBuffer.length > 0) {
      this.flushExternalOutput();
    }
  }

  renderFull(): void {
    this.engine.beginFrame();
    const frame = this.engine.renderFull();
    this.engine.commitFrame();

    if (frame.output_data) {
      const decoded = Buffer.from(frame.output_data, "base64");
      if (this._screenMode === "split-footer") {
        process.stdout.write(`\x1b[1;1H${decoded.toString()}`);
      } else {
        process.stdout.write(decoded);
      }
    }

    if (this._screenMode === "split-footer" && this.externalOutputBuffer.length > 0) {
      this.flushExternalOutput();
    }
  }

  clearScreen(): void {
    process.stdout.write("\x1b[2J\x1b[H");
  }

  write(text: string): void {
    process.stdout.write(text);
  }

  resize(width: number, height: number): void {
    this.width = width;
    this.height = height;
    if (this._screenMode === "split-footer") {
      // Keep footer height stable, adjust viewport
      this.engine.resize(width, this.viewportHeight);
    } else {
      this.engine.resize(width, height);
    }
  }

  private enterAlternateScreen(): void {
    process.stdout.write("\x1b[?1049h\x1b[?25l");
  }

  private exitAlternateScreen(): void {
    process.stdout.write("\x1b[?25h\x1b[?1049l");
  }

  handleKey(sequence: string): string | null {
    return this.keymap.handleKey(sequence);
  }

  addKeyBinding(
    layer: string,
    id: string,
    keys: string,
    command: string,
    description?: string,
    priority = 0,
  ): boolean {
    return this.keymap.addBinding(layer, id, keys, command, description ?? null, priority);
  }
}

export async function createCliRenderer(options: CliRendererOptions = {}): Promise<CliRenderer> {
  return new CliRenderer(options);
}

export { getVersion, detectCapabilities };
