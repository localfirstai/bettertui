// Internal keyboard input for the examples package. BetterTUI's public `useKeyboard`
// hook binds to DOM `keydown` events, which never fire in a Node TTY, so the
// interactive launcher and examples consume keypresses through this manager instead.
// This is internal to `@bettertui/examples` and must not leak into framework APIs.

import { pathToFileURL } from "node:url";
import type { KeyEvent } from "@bettertui/shared";

type KeyHandler = (event: KeyEvent) => void;

// Named keys that arrive as escape sequences or have dedicated meanings.
const NAMED_KEYS: Record<string, string> = {
  "\x1b[A": "ArrowUp",
  "\x1b[B": "ArrowDown",
  "\x1b[C": "ArrowRight",
  "\x1b[D": "ArrowLeft",
  "\x1b[5~": "PageUp",
  "\x1b[6~": "PageDown",
  "\x1b[H": "Home",
  "\x1b[F": "End",
  "\x1b[3~": "Delete",
  "\x1b[Z": "Tab", // shift+tab
  "\x1b": "Escape",
  "\r": "Enter",
  "\n": "Enter",
  "\t": "Tab",
  " ": " ",
  "\x7f": "Backspace",
  "\x08": "Backspace",
};

function buildKeyEvent(raw: string, ctrl: boolean, shift: boolean, alt: boolean): KeyEvent {
  let key: string;
  let code = "";

  if (NAMED_KEYS[raw] !== undefined) {
    key = NAMED_KEYS[raw] as string;
  } else if (raw.length === 1) {
    key = raw;
    code = `Key${raw.toUpperCase()}`;
  } else {
    key = raw;
  }

  // When ctrl is held with a letter key, normalise the control byte (\x03 etc.)
  // back to the readable letter so consumers can check `event.key === "c"`.
  if (ctrl && raw.length === 1 && raw.charCodeAt(0) < 32) {
    const letter = String.fromCharCode(raw.charCodeAt(0) + 96);
    key = letter;
    code = `Key${letter.toUpperCase()}`;
  } else if (ctrl && key.length === 1) {
    const lower = key.toLowerCase();
    if ("a" <= lower && lower <= "z") code = `Key${lower.toUpperCase()}`;
  }

  return { key, code, ctrl, shift, alt, meta: false };
}

export class KeyInput {
  private handlers = new Set<KeyHandler>();
  private buffer = "";
  private timeout: ReturnType<typeof setTimeout> | null = null;
  private rawMode = false;
  private onData: (chunk: Buffer) => void;

  constructor() {
    this.onData = (chunk: Buffer) => this.handleChunk(chunk.toString("latin1"));
  }

  on(handler: KeyHandler): () => void {
    this.handlers.add(handler);
    return () => this.handlers.delete(handler);
  }

  off(handler: KeyHandler): void {
    this.handlers.delete(handler);
  }

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
    const stdin = process.stdin;
    stdin.off("data", this.onData);
    if (this.rawMode) {
      stdin.setRawMode(false);
      this.rawMode = false;
    }
    stdin.pause();
    if (this.timeout) {
      clearTimeout(this.timeout);
      this.timeout = null;
    }
  }

  private emit(event: KeyEvent): void {
    for (const handler of [...this.handlers]) {
      handler(event);
    }
  }

  private handleChunk(chunk: string): void {
    // Accumulate escape sequences: wait a tick to see if more bytes follow.
    this.buffer += chunk;
    if (this.timeout) clearTimeout(this.timeout);

    const flush = () => {
      this.timeout = null;
      const data = this.buffer;
      this.buffer = "";
      this.dispatch(data);
    };

    // A bare ESC prefix may be followed by more bytes, so debounce briefly.
    // A complete named escape (e.g. \x1b[A, \x1b[Z) is already whole — flush now.
    if (this.buffer === "\x1b") {
      this.timeout = setTimeout(flush, 20);
      return;
    }
    flush();
  }

  private dispatch(data: string): void {
    // Piped (non-TTY) input arrives line-buffered with a trailing newline/CR;
    // strip it so "q\n" behaves like a raw "q" keypress.
    const trimmed = data.replace(/\r?\n+$/u, "");
    const payload = trimmed.length > 0 ? trimmed : data;

    const isNamed = NAMED_KEYS[payload] !== undefined;
    // Control chars (byte < 32) indicate ctrl+letter (e.g. \x03 = ctrl+c),
    // but only when the payload is NOT a recognised named key — Enter (\r=13),
    // Tab (\t=9), etc. have their own semantics and must not get ctrl:true.
    const ctrl = !isNamed && payload.length === 1 && payload.charCodeAt(0) < 32;
    const alt = payload.startsWith("\x1b") && payload.length > 1;

    // A key is "known" if it is a named sequence, a printable char, or a control
    // char (< 32, e.g. ctrl+c = \x03).
    const known =
      NAMED_KEYS[payload] !== undefined ||
      (payload.length === 1 && payload.charCodeAt(0) >= 32) ||
      ctrl;
    if (!known) return;

    const event = buildKeyEvent(
      payload,
      ctrl,
      payload === "\x1b[Z",
      alt && payload.length === 1 ? false : alt,
    );
    this.emit(event);
  }
}

// True when this module is the entry point (works under Node ESM + Bun).
export function isMainModule(): boolean {
  if (
    typeof import.meta !== "undefined" &&
    (import.meta as { main?: boolean }).main !== undefined
  ) {
    return Boolean((import.meta as { main?: boolean }).main);
  }
  try {
    const invoked = process.argv[1] ? pathToFileURL(process.argv[1]).href : "";
    return invoked !== "" && import.meta.url === invoked;
  } catch {
    return false;
  }
}

export function createKeyInput(): KeyInput {
  return new KeyInput();
}
