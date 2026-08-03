import type { KeyboardModifiers } from "../keyboard/keyboard";
import type { TestTerminal } from "../terminal/test-terminal";

export enum MouseButton {
  Left = 0,
  Middle = 1,
  Right = 2,
  Release = 3,
  WheelUp = 64,
  WheelDown = 65,
}

export interface MouseOptions {
  modifiers?: KeyboardModifiers;
}

export class MockMouse {
  private currentX = 0;
  private currentY = 0;

  constructor(private readonly terminal: TestTerminal) {}

  public get position(): { x: number; y: number } {
    return { x: this.currentX, y: this.currentY };
  }

  public moveTo(x: number, y: number, _options: MouseOptions = {}): void {
    this.currentX = x;
    this.currentY = y;
    const seq = this.encodeSgr(32, x, y, true);
    this.terminal.emit("mousemove", { x, y, sequence: seq });
    this.terminal.emit("input", seq);
  }

  public async click(
    x: number,
    y: number,
    button = MouseButton.Left,
    options: MouseOptions = {},
  ): Promise<void> {
    this.down(x, y, button, options);
    this.up(x, y, button, options);
  }

  public async doubleClick(
    x: number,
    y: number,
    button = MouseButton.Left,
    options: MouseOptions = {},
  ): Promise<void> {
    await this.click(x, y, button, options);
    await this.click(x, y, button, options);
  }

  public down(x: number, y: number, button = MouseButton.Left, options: MouseOptions = {}): void {
    this.currentX = x;
    this.currentY = y;
    const btnCode = this.applyModifiers(button, options.modifiers);
    const seq = this.encodeSgr(btnCode, x, y, true);
    this.terminal.emit("mousedown", { x, y, button, sequence: seq });
    this.terminal.emit("input", seq);
  }

  public up(x: number, y: number, button = MouseButton.Left, options: MouseOptions = {}): void {
    this.currentX = x;
    this.currentY = y;
    const btnCode = this.applyModifiers(button, options.modifiers);
    const seq = this.encodeSgr(btnCode, x, y, false);
    this.terminal.emit("mouseup", { x, y, button, sequence: seq });
    this.terminal.emit("input", seq);
  }

  public async drag(
    startX: number,
    startY: number,
    endX: number,
    endY: number,
    options: MouseOptions = {},
  ): Promise<void> {
    this.down(startX, startY, MouseButton.Left, options);
    const steps = 5;
    for (let i = 1; i <= steps; i++) {
      const currX = Math.round(startX + ((endX - startX) * i) / steps);
      const currY = Math.round(startY + ((endY - startY) * i) / steps);
      this.moveTo(currX, currY, options);
    }
    this.up(endX, endY, MouseButton.Left, options);
  }

  public scroll(x: number, y: number, direction: "up" | "down", delta = 1): void {
    this.currentX = x;
    this.currentY = y;
    const button = direction === "up" ? MouseButton.WheelUp : MouseButton.WheelDown;
    for (let i = 0; i < delta; i++) {
      const seq = this.encodeSgr(button, x, y, true);
      this.terminal.emit("scroll", { x, y, direction, sequence: seq });
      this.terminal.emit("input", seq);
    }
  }

  private encodeSgr(btn: number, x: number, y: number, press: boolean): string {
    const action = press ? "M" : "m";
    // Terminal 1-based indexing for ANSI SGR mouse coordinates
    return `\x1b[<${btn};${x + 1};${y + 1}${action}`;
  }

  private applyModifiers(btn: number, modifiers?: KeyboardModifiers): number {
    let mod = 0;
    if (modifiers?.shift) mod |= 4;
    if (modifiers?.alt || modifiers?.meta) mod |= 8;
    if (modifiers?.ctrl) mod |= 16;
    return btn + mod;
  }
}
