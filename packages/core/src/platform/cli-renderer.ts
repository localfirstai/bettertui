import { EventEmitter } from "node:events";
import type { NapiEngine, NapiKeymap, TerminalCapabilities } from "./binding";
import { createEngine, createKeymap, detectCapabilities, getVersion } from "./binding";

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
}

const NAMED_KEYS: Record<string, string> = {
  "\x1b[A": "up",
  "\x1b[B": "down",
  "\x1b[C": "right",
  "\x1b[D": "left",
  "\x1b[5~": "pageup",
  "\x1b[6~": "pagedown",
  "\x1b[H": "home",
  "\x1b[F": "end",
  "\x1b[3~": "delete",
  "\x1b[Z": "tab",
  "\x1b": "escape",
  "\r": "enter",
  "\n": "enter",
  "\t": "tab",
  " ": "space",
  "\x7f": "backspace",
  "\x08": "backspace",
};

type KeyInputEvents = {
  keypress: [KeyEvent];
};

export class KeyInput extends EventEmitter<KeyInputEvents> {
  private buffer = "";
  private timeout: ReturnType<typeof setTimeout> | null = null;
  private rawMode = false;

  start(): void {
    const stdin = process.stdin;
    if (stdin.setRawMode && !this.rawMode) {
      stdin.setRawMode(true);
      this.rawMode = true;
    }
    stdin.resume();
    stdin.on("data", this.onData);
  }

  stop(): void {
    process.stdin.off("data", this.onData);
    if (this.rawMode && process.stdin.setRawMode) {
      process.stdin.setRawMode(false);
      this.rawMode = false;
    }
    process.stdin.pause();
    if (this.timeout) {
      clearTimeout(this.timeout);
      this.timeout = null;
    }
  }

  private onData = (data: Buffer): void => {
    this.buffer += data.toString("latin1");
    if (this.timeout) clearTimeout(this.timeout);

    const flush = () => {
      this.timeout = null;
      const data = this.buffer;
      this.buffer = "";
      this.dispatch(data);
    };

    if (this.buffer === "\x1b") {
      this.timeout = setTimeout(flush, 20);
      return;
    }
    flush();
  };

  private dispatch(data: string): void {
    const trimmed = data.replace(/\r?\n+$/u, "");
    const payload = trimmed.length > 0 ? trimmed : data;

    const isNamed = NAMED_KEYS[payload] !== undefined;
    const ctrl = !isNamed && payload.length === 1 && payload.charCodeAt(0) < 32;
    const alt = payload.startsWith("\x1b") && payload.length > 1;
    const shift = payload === "\x1b[Z";

    const known =
      NAMED_KEYS[payload] !== undefined ||
      (payload.length === 1 && payload.charCodeAt(0) >= 32) ||
      ctrl;

    if (!known) return;

    let key: string;
    if (NAMED_KEYS[payload]) {
      key = NAMED_KEYS[payload] ?? payload;
    } else if (ctrl && payload.length === 1 && payload.charCodeAt(0) < 32) {
      key = String.fromCharCode(payload.charCodeAt(0) + 96);
    } else if (payload.length === 1) {
      key = payload;
    } else {
      key = payload;
    }

    this.emit("keypress", {
      name: key,
      ctrl,
      shift,
      alt,
      meta: false,
      sequence: payload,
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
  private nodes: Map<number, { parent: number | null; children: number[] }> = new Map();
  private running = false;

  constructor(options: CliRendererOptions = {}) {
    this.capabilities = detectCapabilities();
    this.width = options.width ?? this.capabilities.columns;
    this.height = options.height ?? this.capabilities.rows;

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

  start(): void {
    if (this.running) return;
    this.running = true;
    this.enterAlternateScreen();
    this._keyInput.start();
  }

  stop(): void {
    if (!this.running) return;
    this.running = false;
    this._keyInput.stop();
    this.exitAlternateScreen();
    this.engine.shutdown();
  }

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

  render(): void {
    this.engine.beginFrame();
    const frame = this.engine.render();
    this.engine.commitFrame();

    if (frame.output_data) {
      const decoded = Buffer.from(frame.output_data, "base64");
      process.stdout.write(decoded);
    }
  }

  renderFull(): void {
    this.engine.beginFrame();
    const frame = this.engine.renderFull();
    this.engine.commitFrame();

    if (frame.output_data) {
      const decoded = Buffer.from(frame.output_data, "base64");
      process.stdout.write(decoded);
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
    this.engine.resize(width, height);
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
