import { EventEmitter } from "node:events";
import { KeyEvent } from "./keyHandler";
import { StdinParser } from "./stdinParser";

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
    this.stdinParser.drain((event) => {
      if (event.type === "key") {
        this.emit("keypress", new KeyEvent(event.key));
      }
    });
  }
}
