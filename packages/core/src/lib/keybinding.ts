import { createKeymap as createNativeKeymap } from "../platform/binding";
import type { NapiKeymap } from "../platform/binding";

export interface BindingInfo {
  id: string;
  keys: string;
  command: string;
  description: string | null;
  enabled: boolean;
  layer: string;
}

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

export class Keymap {
  private native: NapiKeymap;
  private commands = new Map<string, CommandHandler>();
  private keyIntercepts: Array<{ priority: number; handler: InterceptHandler }> = [];
  private keyAfterIntercepts: Array<{ priority: number; handler: InterceptHandler }> = [];
  private listeners = new Map<string, Set<KeyListener>>();
  private runtimeData = new Map<string, unknown>();
  private bindings: BindingInfo[] = [];
  private layers = new Set<string>(["default"]);
  private currentModeValue: string | null = null;
  private chordTimeoutMsValue = 500;
  private pendingKeysValue: string[] = [];
  private commandHistoryValue: string[] = [];

  constructor(native?: NapiKeymap, options?: KeymapOptions) {
    this.native = native ?? createNativeKeymap();
    if (options?.chordTimeoutMs !== undefined) {
      this.chordTimeoutMsValue = options.chordTimeoutMs;
    }
    if (options?.mode !== undefined) {
      this.currentModeValue = options.mode;
    }
  }

  addBinding(
    layer: string,
    id: string,
    keys: string,
    command: string,
    description?: string,
    priority?: number,
  ): boolean {
    const result = this.native.addBinding(
      layer,
      id,
      keys,
      command,
      description ?? null,
      priority ?? 0,
    );
    if (result) {
      this.bindings.push({
        id,
        keys,
        command,
        description: description ?? null,
        enabled: true,
        layer,
      });
      this.layers.add(layer);
    }
    return result;
  }

  addSimpleBinding(keys: string, command: string, description?: string): boolean {
    return this.addBinding("default", command, keys, command, description, 0);
  }

  removeLayer(name: string): boolean {
    this.bindings = this.bindings.filter((b) => b.layer !== name);
    this.layers.delete(name);
    return true;
  }

  setChordTimeout(ms: number): void {
    this.chordTimeoutMsValue = ms;
  }

  chordTimeout(): number {
    return this.chordTimeoutMsValue;
  }

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
          // Swallow errors
        }
      }
    };
    fire(event);
    if (event !== "state") fire("state");
  }

  handleKey(keyStr: string): string | null {
    const event: KeymapEvent = {
      phase: "sequence-start",
      key: keyStr,
      command: null,
      keys: this.hasPending() ? [...this.pendingKeysValue, keyStr] : [keyStr],
    };

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
    const commandOrNull = command.length > 0 ? command : null;

    if (commandOrNull !== null) {
      event.phase = "binding-execute";
      event.command = commandOrNull;
      this.emit("dispatch", event);
      if (this.hasPending()) {
        this.emit("pendingSequence", event);
      }
      this.commandHistoryValue.push(commandOrNull);
      const handler = this.commands.get(commandOrNull);
      if (handler) {
        const ctx: CommandContext = {
          keymap: this,
          event,
          command: commandOrNull,
          data: Object.fromEntries(this.runtimeData),
        };
        handler(ctx);
      }
    } else if (this.hasPending()) {
      event.phase = "sequence-advance";
      this.pendingKeysValue.push(keyStr);
      this.emit("pendingSequence", event);
    } else {
      event.phase = "sequence-clear";
      this.emit("dispatch", event);
    }

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
    return commandOrNull;
  }

  setMode(mode: string): void {
    this.currentModeValue = mode;
  }

  currentMode(): string | null {
    return this.currentModeValue;
  }

  clearMode(): void {
    this.currentModeValue = null;
  }

  hasPending(): boolean {
    return this.native.hasPending();
  }

  clearPending(): void {
    this.native.clearPending();
    this.pendingKeysValue = [];
  }

  pendingKeys(): string[] {
    return this.pendingKeysValue;
  }

  activeBindings(): BindingInfo[] {
    return this.bindings.filter((b) => b.enabled);
  }

  allBindings(): BindingInfo[] {
    return [...this.bindings];
  }

  commandHistory(): string[] {
    return [...this.commandHistoryValue];
  }

  clearHistory(): void {
    this.commandHistoryValue = [];
  }

  setData(key: string, value: unknown): void {
    this.runtimeData.set(key, value);
  }

  getData(key: string): unknown {
    return this.runtimeData.get(key);
  }

  getCommandBindings(command: string): BindingInfo[] {
    return this.bindings.filter((b) => b.command === command);
  }

  getBindingsForCommands(commands: string[]): Map<string, BindingInfo[]> {
    const result = new Map<string, BindingInfo[]>();
    for (const cmd of commands) {
      result.set(
        cmd,
        this.bindings.filter((b) => b.command === cmd),
      );
    }
    return result;
  }

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
      ...(payload !== undefined ? { payload } : {}),
      data: Object.fromEntries(this.runtimeData),
    };
    handler(ctx);
    return true;
  }

  parseKey(keyStr: string): string | null {
    return keyStr.length > 0 ? keyStr : null;
  }

  parseSequence(keyStr: string): string[] {
    return keyStr.split(" ").filter((k) => k.length > 0);
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

  getNative(): NapiKeymap {
    return this.native;
  }
}
