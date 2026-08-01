import { EventEmitter } from "node:events";
import { KeyEvent, PasteEvent } from "./keyHandler";
import type { RawMouseEvent } from "./parseMouse";
import { type StdinEvent, StdinParser } from "./stdinParser";

/**
 * Events emitted by {@link KeyInput}. Mirrors the four kinds of `StdinEvent`
 * produced by the parser so that nothing read from stdin is silently dropped.
 *
 * Historical bug: `KeyInput.drain` previously handled only `type === "key"`
 * and discarded every `mouse` / `paste` / `response` event, which made mouse
 * input, bracketed paste, and terminal-capability replies unreachable from
 * the renderer.
 */
type KeyInputEvents = {
  keypress: [KeyEvent];
  keyrelease: [KeyEvent];
  mouse: [RawMouseEvent, string];
  paste: [PasteEvent];
  response: [string, string];
};

export type { KeyInputEvents };

export class KeyInput extends EventEmitter<KeyInputEvents> {
  private stdinParser: StdinParser;
  private rawMode = false;
  private readonly onDataBound: (data: Buffer) => void;

  constructor() {
    super();
    this.stdinParser = new StdinParser({
      useKittyKeyboard: false,
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
    this.stdinParser.drain((event: StdinEvent) => {
      switch (event.type) {
        case "key": {
          const key = new KeyEvent(event.key);
          if (event.key.eventType === "release") {
            this.emit("keyrelease", key);
          } else {
            // Both "press" and "repeat" (normalized by the parser) surface as
            // a `keypress` event.
            this.emit("keypress", key);
          }
          break;
        }
        case "mouse": {
          this.emit("mouse", event.event, event.raw);
          break;
        }
        case "paste": {
          this.emit("paste", new PasteEvent(event.bytes));
          break;
        }
        case "response": {
          this.emit("response", event.protocol, event.sequence);
          break;
        }
      }
    });
  }
}
