import { Keymap } from "./keymap";
import type { KeymapOptions } from "./keymap";
import type { BindingInfo, NapiKeymap } from "./native/types";

// ─── Mock Native Keymap for Testing ──

export interface TestBinding {
  layer: string;
  id: string;
  keys: string;
  command: string;
  description: string | null;
  priority: number;
  enabled: boolean;
}

interface PendingState {
  keys: string[];
  command: string | null;
}

export function createMockNativeKeymap(): NapiKeymap {
  const bindings: TestBinding[] = [];
  const parsedKeys = new Map<string, string>();
  let currentMode: string | null = null;
  let chordTimeout = 1000;
  let pending: PendingState | null = null;
  const history: string[] = [];

  function parseKey(keyStr: string): string {
    let cached = parsedKeys.get(keyStr);
    if (cached) return cached;
    cached = keyStr.trim().toLowerCase();
    parsedKeys.set(keyStr, cached);
    return cached;
  }

  // Mirror Rust KeyParser::parse_sequence semantics:
  // - Repeated single-char (e.g. "dd") → chord of same key
  // - Comma-separated (e.g. "ctrl+x,ctrl+s") → sequence
  // - Single token → single key
  function parseSequence(keysStr: string): string[] {
    const trimmed = keysStr.trim();
    if (trimmed.includes(",")) {
      return trimmed.split(",").map((s) => parseKey(s));
    }
    if (
      trimmed.length === 2 &&
      !trimmed.includes("+") &&
      !trimmed.includes("<") &&
      trimmed[0] === trimmed[1]
    ) {
      return [parseKey(trimmed[0]), parseKey(trimmed[1])];
    }
    return [parseKey(trimmed)];
  }

  return {
    addBinding(
      layer: string,
      id: string,
      keys: string,
      command: string,
      description: string | null,
      priority: number,
    ): boolean {
      bindings.push({ layer, id, keys, command, description, priority, enabled: true });
      return true;
    },

    setMode(mode: string): void {
      currentMode = mode;
    },
    currentMode(): string | null {
      return currentMode;
    },
    clearMode(): void {
      currentMode = null;
    },

    removeLayer(name: string): boolean {
      const before = bindings.length;
      for (let i = bindings.length - 1; i >= 0; i--) {
        if (bindings[i].layer === name) bindings.splice(i, 1);
      }
      return bindings.length < before;
    },

    setChordTimeout(ms: number): void {
      chordTimeout = ms;
    },
    chordTimeout(): number {
      return chordTimeout;
    },

    handleKey(keyStr: string): string | null {
      const parsed = parseKey(keyStr);

      // Check pending sequence first
      if (pending) {
        const expectedKey = pending.keys[0];
        if (parsed === expectedKey) {
          pending.keys.shift();
          if (pending.keys.length === 0) {
            const cmd = pending.command;
            pending = null;
            if (cmd) history.push(cmd);
            return cmd;
          }
          return null;
        }
        pending = null;
      }

      // Find matching binding
      for (const b of bindings) {
        if (!b.enabled) continue;
        const seq = parseSequence(b.keys);
        if (seq.length === 0) continue;
        if (seq[0] !== parsed) continue;

        if (seq.length === 1) {
          history.push(b.command);
          return b.command;
        }

        pending = { keys: seq.slice(1), command: b.command };
        return null;
      }

      return null;
    },

    hasPending(): boolean {
      return pending !== null;
    },
    clearPending(): void {
      pending = null;
    },
    pendingKeys(): string[] {
      return pending?.keys ?? [];
    },

    activeBindings(): BindingInfo[] {
      return bindings
        .filter((b) => b.enabled)
        .map((b) => ({
          id: b.id,
          keys: b.keys,
          command: b.command,
          description: b.description,
          enabled: b.enabled,
          layer: b.layer,
        }));
    },

    allBindings(): BindingInfo[] {
      return bindings.map((b) => ({
        id: b.id,
        keys: b.keys,
        command: b.command,
        description: b.description,
        enabled: b.enabled,
        layer: b.layer,
      }));
    },

    commandHistory(): string[] {
      return [...history];
    },
    clearHistory(): void {
      history.length = 0;
    },

    parseKey(keyStr: string): string | null {
      return parseKey(keyStr);
    },

    parseSequence(keyStr: string): string[] {
      return parseSequence(keyStr);
    },
  };
}

export function createTestKeymap(
  bindings?: Array<{
    layer?: string;
    id?: string;
    keys: string;
    command: string;
    description?: string;
    priority?: number;
  }>,
  options?: KeymapOptions,
): Keymap {
  const keymap = new Keymap(createMockNativeKeymap(), options);

  if (bindings) {
    for (const b of bindings) {
      keymap.addBinding(
        b.layer ?? "test",
        b.id ?? b.command,
        b.keys,
        b.command,
        b.description,
        b.priority ?? 0,
      );
    }
  }

  return keymap;
}
