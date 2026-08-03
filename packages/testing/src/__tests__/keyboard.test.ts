import { describe, expect, it } from "vitest";
import { MockKeyboard } from "../keyboard/keyboard";
import { TestTerminal } from "../terminal/test-terminal";

describe("MockKeyboard", () => {
  it("emits ANSI key sequences for arrow keys and enter", () => {
    const terminal = new TestTerminal();
    const keyboard = new MockKeyboard(terminal);

    const emitted: string[] = [];
    terminal.on("input", (_, seq) => emitted.push(seq as string));

    keyboard.press("Enter");
    keyboard.press("ARROW_DOWN");

    expect(emitted).toEqual(["\r", "\x1b[B"]);
  });

  it("types text char by char", async () => {
    const terminal = new TestTerminal();
    const keyboard = new MockKeyboard(terminal);

    const emitted: string[] = [];
    terminal.on("input", (_, seq) => emitted.push(seq as string));

    await keyboard.type("Hi");
    expect(emitted).toEqual(["H", "i"]);
  });

  it("encodes bracketed paste mode", () => {
    const terminal = new TestTerminal();
    const keyboard = new MockKeyboard(terminal);

    let pasted = "";
    terminal.on("input", (_, seq) => {
      pasted = seq as string;
    });

    keyboard.paste("Secret");
    expect(pasted).toBe("\x1b[200~Secret\x1b[201~");
  });
});
