import type { TestTerminal } from "../terminal/test-terminal";
import { KeyCodes } from "./key-codes";

export interface KeyboardModifiers {
  ctrl?: boolean;
  shift?: boolean;
  alt?: boolean;
  meta?: boolean;
}

export interface TypeOptions {
  delayMs?: number;
}

export class MockKeyboard {
  constructor(private readonly terminal: TestTerminal) {}

  public press(key: string, modifiers?: KeyboardModifiers): void {
    const seq = this.encodeKeySequence(key, modifiers);
    this.terminal.emit("keypress", { key, modifiers, sequence: seq });
    this.terminal.emit("input", seq);
  }

  public async type(text: string, options: TypeOptions = {}): Promise<void> {
    const delay = options.delayMs ?? 0;
    for (const char of text) {
      this.press(char);
      if (delay > 0) {
        await new Promise((resolve) => setTimeout(resolve, delay));
      }
    }
  }

  public paste(content: string): void {
    const bracketed = `\x1b[200~${content}\x1b[201~`;
    this.terminal.emit("paste", { content, sequence: bracketed });
    this.terminal.emit("input", bracketed);
  }

  public keyDown(key: string, modifiers?: KeyboardModifiers): void {
    const seq = this.encodeKeySequence(key, modifiers);
    this.terminal.emit("keydown", { key, modifiers, sequence: seq });
  }

  public keyUp(key: string, modifiers?: KeyboardModifiers): void {
    const seq = this.encodeKeySequence(key, modifiers);
    this.terminal.emit("keyup", { key, modifiers, sequence: seq });
  }

  private encodeKeySequence(key: string, modifiers: KeyboardModifiers = {}): string {
    if (key in KeyCodes) {
      const code = KeyCodes[key as keyof typeof KeyCodes];
      if (modifiers.ctrl || modifiers.shift || modifiers.alt || modifiers.meta) {
        return this.applyModifiersToEscapeCode(code, modifiers);
      }
      return code;
    }

    if (key.length === 1) {
      if (modifiers.ctrl) {
        const charCode = key.toUpperCase().charCodeAt(0);
        if (charCode >= 65 && charCode <= 90) {
          return String.fromCharCode(charCode - 64);
        }
      }
      if (modifiers.alt || modifiers.meta) {
        return `\x1b${key}`;
      }
      return key;
    }

    return key;
  }

  private applyModifiersToEscapeCode(baseCode: string, modifiers: KeyboardModifiers): string {
    let modNum = 1;
    if (modifiers.shift) modNum += 1;
    if (modifiers.alt || modifiers.meta) modNum += 2;
    if (modifiers.ctrl) modNum += 4;

    if (baseCode.startsWith("\x1b[") && baseCode.endsWith("~")) {
      const number = baseCode.slice(2, -1);
      return `\x1b[${number};${modNum}~`;
    }
    if (baseCode.startsWith("\x1b[") && baseCode.length === 3) {
      const dir = baseCode.slice(2);
      return `\x1b[1;${modNum}${dir}`;
    }
    return baseCode;
  }
}
