import { createKeymap as createNativeKeymap } from "./native";
import type { BindingInfo, NapiKeymap } from "./native/types";

// ─── Types ──────────────────────────────────────────────────────

export interface KeymapEvent {
  phase:
    | "sequence-start"
    | "sequence-advance"
    | "sequence-clear"
    | "binding-execute"
    | "binding-reject";
  key: string;
  command: string | null;
  keys: string[];
}

export type CommandHandler = (ctx: CommandContext) => boolean | undefined;

export interface CommandContext {
  keymap: Keymap;
  event: KeymapEvent;
  command: string;
  payload?: Record<string, unknown>;
  data: Record<string, unknown>;
}

export interface CommandEntry {
  name: string;
  handler: CommandHandler;
}

export type InterceptHandler = (ctx: InterceptContext) => boolean | undefined;

export interface InterceptContext {
  key: string;
  event: KeymapEvent;
  preventDefault(): void;
  stopPropagation(): void;
  defaultPrevented: boolean;
  propagationStopped: boolean;
}

export type KeyListener = (event: KeymapEvent) => void;

export type KeymapOptions = {
  chordTimeoutMs?: number;
  mode?: string;
};

export interface ActiveKeyInfo {
  keys: string;
  command: string;
  description: string | null;
  layer: string;
  id: string;
}

// ─── Keymap ──────────────────────────────────────────────────────

export class Keymap {
  private native: NapiKeymap;
  private commands = new Map<string, CommandHandler>();
  private keyIntercepts: Array<{ priority: number; handler: InterceptHandler }> = [];
  private keyAfterIntercepts: Array<{ priority: number; handler: InterceptHandler }> = [];
  private listeners = new Map<string, Set<KeyListener>>();
  private runtimeData = new Map<string, unknown>();

  constructor(native?: NapiKeymap, options?: KeymapOptions) {
    this.native = native ?? createNativeKeymap();
    if (options?.chordTimeoutMs !== undefined) {
      this.native.setChordTimeout(options.chordTimeoutMs);
    }
    if (options?.mode !== undefined) {
      this.native.setMode(options.mode);
    }
  }

  // ── Binding Registration ──

  addBinding(
    layer: string,
    id: string,
    keys: string,
    command: string,
    description?: string,
    priority?: number,
  ): boolean {
    return this.native.addBinding(layer, id, keys, command, description ?? null, priority ?? 0);
  }

  addSimpleBinding(keys: string, command: string, description?: string): boolean {
    return this.addBinding("default", command, keys, command, description, 0);
  }

  removeLayer(name: string): boolean {
    return this.native.removeLayer(name);
  }

  // ── Command Registry ──

  registerCommand(name: string, handler: CommandHandler): void {
    this.commands.set(name, handler);
  }

  unregisterCommand(name: string): boolean {
    return this.commands.delete(name);
  }

  getCommand(name: string): CommandHandler | undefined {
    return this.commands.get(name);
  }

  hasCommand(name: string): boolean {
    return this.commands.has(name);
  }

  getCommands(): CommandEntry[] {
    const entries: CommandEntry[] = [];
    for (const [name, handler] of this.commands) {
      entries.push({ name, handler });
    }
    return entries;
  }

  // ── Intercepts ──

  intercept(name: "key" | "key:after", handler: InterceptHandler, priority?: number): () => void {
    const pri = priority ?? 0;
    const entry = { priority: pri, handler };
    if (name === "key") {
      this.keyIntercepts.push(entry);
      this.keyIntercepts.sort((a, b) => b.priority - a.priority);
    } else {
      this.keyAfterIntercepts.push(entry);
      this.keyAfterIntercepts.sort((a, b) => b.priority - a.priority);
    }
    return () => {
      if (name === "key") {
        this.keyIntercepts = this.keyIntercepts.filter((e) => e.handler !== handler);
      } else {
        this.keyAfterIntercepts = this.keyAfterIntercepts.filter((e) => e.handler !== handler);
      }
    };
  }

  // ── Event Listeners ──

  on(event: "state" | "pendingSequence" | "dispatch", listener: KeyListener): () => void {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, new Set());
    }
    this.listeners.get(event)?.add(listener);
    return () => {
      this.listeners.get(event)?.delete(listener);
    };
  }

  off(event: "state" | "pendingSequence" | "dispatch", listener: KeyListener): void {
    this.listeners.get(event)?.delete(listener);
  }

  private emit(event: string, data: KeymapEvent): void {
    const fire = (name: string) => {
      const set = this.listeners.get(name);
      if (!set) return;
      for (const listener of set) {
        try {
          listener(data);
        } catch {
          // Listener error - swallow to prevent cascade
        }
      }
    };
    fire(event);
    if (event !== "state") fire("state");
  }

  // ── Key Dispatch ──

  handleKey(keyStr: string): string | null {
    const event: KeymapEvent = {
      phase: "sequence-start",
      key: keyStr,
      command: null,
      keys: this.hasPending() ? [...this.pendingKeys(), keyStr] : [keyStr],
    };

    // Run key intercepts
    for (const intercept of this.keyIntercepts) {
      const ctx: InterceptContext = {
        key: keyStr,
        event,
        preventDefault() {
          ctx.defaultPrevented = true;
        },
        stopPropagation() {
          ctx.propagationStopped = true;
        },
        defaultPrevented: false,
        propagationStopped: false,
      };
      intercept.handler(ctx);
      if (ctx.defaultPrevented || ctx.propagationStopped) {
        return null;
      }
    }

    const command = this.native.handleKey(keyStr);

    if (command !== null) {
      event.phase = "binding-execute";
      event.command = command;

      this.emit("dispatch", event);
      if (this.hasPending()) {
        this.emit("pendingSequence", event);
      }

      // Run the command handler
      const handler = this.commands.get(command);
      if (handler) {
        const ctx: CommandContext = {
          keymap: this,
          event,
          command,
          data: Object.fromEntries(this.runtimeData),
        };
        handler(ctx);
      }
    } else if (this.hasPending()) {
      event.phase = "sequence-advance";
      this.emit("pendingSequence", event);
    } else {
      event.phase = "sequence-clear";
      this.emit("dispatch", event);
    }

    // Run key:after intercepts
    for (const intercept of this.keyAfterIntercepts) {
      const ctx: InterceptContext = {
        key: keyStr,
        event,
        preventDefault() {
          ctx.defaultPrevented = true;
        },
        stopPropagation() {
          ctx.propagationStopped = true;
        },
        defaultPrevented: false,
        propagationStopped: false,
      };
      intercept.handler(ctx);
    }

    this.emit("state", event);
    return command;
  }

  // ── Mode Management ──

  setMode(mode: string): void {
    this.native.setMode(mode);
  }

  currentMode(): string | null {
    return this.native.currentMode();
  }

  clearMode(): void {
    this.native.clearMode();
  }

  // ── Pending Sequence ──

  hasPending(): boolean {
    return this.native.hasPending();
  }

  clearPending(): void {
    this.native.clearPending();
  }

  pendingKeys(): string[] {
    return this.native.pendingKeys();
  }

  // ── Query Bindings ──

  activeBindings(): BindingInfo[] {
    return this.native.activeBindings();
  }

  allBindings(): BindingInfo[] {
    return this.native.allBindings();
  }

  // ── Command History ──

  commandHistory(): string[] {
    return this.native.commandHistory();
  }

  clearHistory(): void {
    this.native.clearHistory();
  }

  // ── Runtime Data ──

  setData(key: string, value: unknown): void {
    this.runtimeData.set(key, value);
  }

  getData(key: string): unknown {
    return this.runtimeData.get(key);
  }

  // ── Command Bindings Query ──

  getCommandBindings(command: string): BindingInfo[] {
    return this.native.allBindings().filter((b) => b.command === command);
  }

  getBindingsForCommands(commands: string[]): Map<string, BindingInfo[]> {
    const result = new Map<string, BindingInfo[]>();
    const all = this.native.allBindings();
    for (const cmd of commands) {
      result.set(
        cmd,
        all.filter((b) => b.command === cmd),
      );
    }
    return result;
  }

  // ── Run Command Directly ──

  runCommand(command: string, payload?: Record<string, unknown>): boolean {
    const handler = this.commands.get(command);
    if (!handler) return false;
    const event: KeymapEvent = {
      phase: "binding-execute",
      key: "",
      command,
      keys: [],
    };
    const ctx: CommandContext = {
      keymap: this,
      event,
      command,
      payload,
      data: Object.fromEntries(this.runtimeData),
    };
    handler(ctx);
    return true;
  }

  // ── Parsing / Formatting ──

  parseKey(keyStr: string): string | null {
    return this.native.parseKey(keyStr);
  }

  parseSequence(keyStr: string): string[] {
    return this.native.parseSequence(keyStr);
  }

  formatKeySequence(keys: string[]): string {
    return keys.join(" ");
  }

  stringifyKeySequence(
    keys: string[],
    options?: { preferDisplay?: boolean; separator?: string },
  ): string {
    return keys.join(options?.separator ?? " ");
  }

  formatBinding(binding: BindingInfo): string {
    const parts = [binding.keys];
    if (binding.description) {
      parts.push(`- ${binding.description}`);
    }
    return parts.join(" ");
  }

  formatCommandBindings(entries: Array<{ command: string; bindings: BindingInfo[] }>): string[] {
    return entries.map((entry) => {
      const keys = entry.bindings.map((b) => b.keys).join(", ");
      return `${entry.command}: ${keys}`;
    });
  }

  // ── Access Native Instance ──

  getNative(): NapiKeymap {
    return this.native;
  }
}
